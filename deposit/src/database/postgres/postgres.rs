use super::models::*;
use super::schema::*;
use crate::database::postgres::config::Config;
use crate::database::provider::{DatabaseReader, DatabaseWriter};
use crate::model;
use chrono::NaiveDateTime;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./src/database/postgres/migrations");

pub struct PostgresDB {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl PostgresDB {
    pub fn new(database_url: &str, pool_size: u32) -> anyhow::Result<Self> {
        let manager = ConnectionManager::new(database_url);
        let pool = Pool::builder().max_size(pool_size).build(manager)?;

        pool.run_pending_migrations(MIGRATIONS)?;

        Ok(PostgresDB { pool })
    }
    pub fn from_config(cfg: &Config) -> anyhow::Result<Self> {
        PostgresDB::new(&cfg.database_url, cfg.pool_size)
    }
}

impl DatabaseReader for PostgresDB {
    fn get_address(
        &self,
        user_id_: &str,
        chain_id_: &str,
    ) -> anyhow::Result<Option<model::WalletAddress>> {
        use super::schema::address_book::dsl::*;

        let res = address_book
            .filter(user_id.eq(user_id_).and(chain_id.eq(chain_id_)))
            .limit(1)
            .load::<Address>(&mut self.pool.get()?)?;

        match res.get(0) {
            Some(addr) => Ok(Some(addr)),
            None => Ok(None),
        }
    }

    fn get_wallet(&self, chain_id_: &str) -> anyhow::Result<Option<model::Wallet>> {
        use super::schema::wallets::dsl::*;

        let res = wallets
            .filter(chain_id.eq(chain_id_))
            .limit(1)
            .load::<Wallet>(&mut self.pool.get()?)?;

        match res.get(0) {
            Some(wallet) => Ok(Some(wallet.into())),
            None => Ok(None),
        }
    }

    fn has_address(&self, address_: &str, chain_id_: &str) -> anyhow::Result<bool> {
        use super::schema::address_book::dsl::*;
        let res = address_book
            .filter(address.eq(address_).and(chain_id.eq(chain_id_)))
            .load::<Address>(&mut self.pool.get()?)?;

        Ok(!res.is_empty())
    }
}

impl DatabaseWriter for PostgresDB {
    fn assign_address(
        &self,
        user_id: &str,
        chain_id: &str,
        wallet_id: &str,
        address: &str,
    ) -> anyhow::Result<()> {
        let new_address = Address {
            user_id: user_id.to_string(),
            chain_id: chain_id.to_string(),
            generated_wallet_id: wallet_id.to_owned(),
            address: address.to_owned(),
            created_at: chrono::Utc::now().naive_utc(),
        };

        diesel::insert_into(address_book::table)
            .values(&new_address)
            .execute(&mut self.pool.get()?)?;

        Ok(())
    }
}

#[cfg(test)]
#[path = "./postgres_test.rs"]
mod postgres_test;
