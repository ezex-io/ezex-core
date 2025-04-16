use super::{
    models::*,
    schema::*,
};
use crate::{
    database::{
        postgres::config::Config,
        provider::{
            DatabaseReader,
            DatabaseWriter,
        },
    },
    types,
};
use diesel::{
    expression::is_aggregate::No,
    pg::PgConnection,
    prelude::*,
    r2d2::{
        ConnectionManager,
        Pool,
    },
};
use diesel_migrations::{
    EmbeddedMigrations,
    MigrationHarness,
    embed_migrations,
};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./src/database/postgres/migrations");

pub struct PostgresDB {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl PostgresDB {
    pub fn new(database_url: &str, pool_size: u32) -> anyhow::Result<Self> {
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder().max_size(pool_size).build(manager)?;

        pool.get()?
            .run_pending_migrations(MIGRATIONS)
            .map_err(|e| anyhow::anyhow!(e))?;

        Ok(PostgresDB { pool })
    }

    fn conn(
        &self,
    ) -> anyhow::Result<diesel::r2d2::PooledConnection<ConnectionManager<PgConnection>>> {
        Ok(self.pool.get()?)
    }

    pub fn from_config(cfg: &Config) -> anyhow::Result<Self> {
        PostgresDB::new(&cfg.database_url, cfg.pool_size)
    }
}

impl DatabaseReader for PostgresDB {
    fn get_wallet(&self, target_chain_id: &str) -> anyhow::Result<types::Wallet> {
        use super::schema::wallets::dsl::*;

        let res = wallets
            .filter(chain_id.eq(target_chain_id))
            .limit(1)
            .load::<Wallet>(&mut self.conn()?)?;

        match res.into_iter().next() {
            Some(wallet) => Ok(wallet.into()),
            None => anyhow::bail!("Wallet not found for chain_id: {}", target_chain_id),
        }
    }

    fn get_address(
        &self,
        target_user_id: &str,
        target_chain_id: &str,
    ) -> anyhow::Result<Option<types::Address>> {
        use super::schema::address_book::dsl::*;

        let target_wallet_id = self.get_wallet(target_chain_id)?.wallet_id;

        let res = address_book
            .filter(
                user_id
                    .eq(target_user_id)
                    .and(wallet_id.eq(target_wallet_id)),
            )
            .limit(1)
            .load::<Address>(&mut self.conn()?)?;

        match res.into_iter().next() {
            Some(addr) => Ok(Some(addr.into())),
            None => Ok(None),
        }
    }

    fn has_address(&self, target_address: &str, target_chain_id: &str) -> anyhow::Result<bool> {
        use super::schema::address_book::dsl::*;

        let target_wallet_id: String = self.get_wallet(target_chain_id)?.wallet_id;

        let res = address_book
            .filter(
                address
                    .eq(target_address)
                    .and(wallet_id.eq(target_wallet_id)),
            )
            .load::<Address>(&mut self.conn()?)?;

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
            id: uuid::Uuid::new_v4(),
            user_id: user_id.to_string(),
            chain_id: chain_id.to_string(),
            wallet_id: wallet_id.to_owned(),
            address: address.to_owned(),
            created_at: chrono::Utc::now().naive_utc(),
        };

        diesel::insert_into(address_book::table)
            .values(&new_address)
            .execute(&mut self.conn()?)?;

        Ok(())
    }
}

#[cfg(test)]
#[path = "./postgres_test.rs"]
mod postgres_test;
