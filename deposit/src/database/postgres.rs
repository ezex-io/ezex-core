use anyhow::anyhow;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use structopt::StructOpt;
use crate::database::provider::{DatabaseReader, DatabaseWriter};
use crate::model::*;
use std::error::Error;
use tokio::task;
use std::sync::Arc;

#[derive(Default, Debug, Clone, Serialize, Deserialize, StructOpt)]
pub struct Config {
    #[structopt(long = "postgres-url", env = "POSTGRES_URL")]
    pub url: String,
    #[structopt(flatten)]
    pub connections: ConnectionsConfig,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, StructOpt)]
pub struct ConnectionsConfig {
    #[structopt(long = "postgres-max-connections", env = "POSTGRES_MAX_CONNECTIONS", default_value = "10")]
    pub max: u32,
}
pub struct PostgresDB {
    pool: Arc<Pool<ConnectionManager<PgConnection>>>,
}

impl PostgresDB {
    pub fn from_config(config: &Config) -> Result<Self, Box<dyn Error>> {
        let manager = ConnectionManager::<PgConnection>::new(&config.url);
        let pool = Pool::builder()
            .max_size(config.connections.max)
            .build(manager)?;

        Ok(Self { pool: Arc::new(pool) })
    }

    fn get_conn(&self) -> Result<PooledConnection<ConnectionManager<PgConnection>>, Box<dyn Error>> {
        self.pool.get().map_err(|e| e.into())
    }

    // a helper method to execute a database operation in a blocking task
    fn db_opt_exec<F, T>(&self, operation: F) -> anyhow::Result<T>
    where
        F: FnOnce(&mut PgConnection) -> anyhow::Result<T> + Send + 'static,
        T: Send + 'static,
    {
        let pool = Arc::clone(&self.pool);

        task::block_in_place(move || {
            let mut conn = pool.get().map_err(|e| anyhow!(e.to_string()))?;
            operation(&mut conn)
        })
    }
}

impl DatabaseReader for PostgresDB {
    // Example implementation for get_address method
    fn get_address(&self, user_id: &str, chain_id: &str) -> anyhow::Result<Option<WalletAddress>> {
        // let pool = Arc::clone(&self.pool);
        let user_id = user_id.to_string();
        let chain_id = chain_id.to_string();

        self.db_opt_exec(move |conn| {
            use crate::database::schema::addresses::dsl::*;

            addresses
                .filter(user_id.eq(&user_id))
                .filter(chain_id.eq(&chain_id))
                .first::<WalletAddress>(conn)
                .optional()
                .map_err(|e| anyhow!(e.to_string()))
        })
    }

    fn get_wallet(&self, chain_id: &str) -> anyhow::Result<Option<Wallet>> {
        todo!()
    }

    fn has_address(&self, address: &str, chain_id: &str) -> anyhow::Result<bool> {
        todo!()
    }

    // Implement other methods similarly
}

impl DatabaseWriter for PostgresDB {
    fn assign_address(
        &self,
        user_id: &str,
        chain_id: &str,
        wallet_id: &str,
        wallet_address: &str,
    ) -> anyhow::Result<()> {
        let pool = Arc::clone(&self.pool);
        let user_id = user_id.to_string();
        let chain_id = chain_id.to_string();
        let wallet_id = wallet_id.to_string();
        let wallet_address = wallet_address.to_string();

        task::block_in_place(move || {
            let mut conn = pool.get().map_err(|e| anyhow!(e.to_string()))?;

            use crate::database::schema::addresses;

            // Check if address already exists
            let existing = addresses::table
                .filter(addresses::user_id.eq(&user_id))
                .filter(addresses::chain_id.eq(&chain_id))
                .first::<WalletAddress>(&mut conn)
                .optional()
                .map_err(|e| anyhow!(e.to_string()))?;

            if existing.is_some() {
                return Err(anyhow!("Address already exists for user_id: {} and chain_id: {}", user_id, chain_id));
            }

            // Create new wallet address record
            let new_address = NewWalletAddress {
                user_id: &user_id,
                chain_id: &chain_id,
                wallet_id: &wallet_id,
                deposit_address: &wallet_address,
                created_at: chrono::Utc::now().naive_utc(),
            };

            diesel::insert_into(addresses::table)
                .values(&new_address)
                .execute(&mut conn)
                .map_err(|e| anyhow!(e.to_string()))?;

            Ok(())
        })
    }
}
