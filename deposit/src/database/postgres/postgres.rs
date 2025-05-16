use super::{models::*, schema::*};
use crate::{
    database::{
        postgres::config::Config,
        provider::{DatabaseReader, DatabaseWriter},
    },
    types::{self, Address, WalletStatus},
};
use diesel::{
    pg::PgConnection,
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./src/database/postgres/migrations");

pub struct PostgresDB {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl PostgresDB {
    pub fn new(cfg: &Config) -> anyhow::Result<Self> {
        let manager = ConnectionManager::<PgConnection>::new(&cfg.database_url);
        let pool = Pool::builder().max_size(cfg.pool_size).build(manager)?;

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
}

impl DatabaseReader for PostgresDB {
    fn get_wallet(&self, target_chain_id: &str) -> anyhow::Result<Option<types::Wallet>> {
        use super::schema::wallets::dsl::*;

        let res = wallets
            .filter(chain_id.eq(target_chain_id))
            .filter(status.eq(WalletStatus::Enabled as i16))
            .limit(1)
            .load::<WalletRecord>(&mut self.conn()?)?;

        match res.into_iter().next() {
            Some(wallet) => Ok(Some(wallet.into())),
            None => Ok(None),
        }
    }

    fn get_address(
        &self,
        target_wallet_id: &str,
        target_user_id: &str,
        target_chain_id: &str,
        target_asset_id: &str,
    ) -> anyhow::Result<Option<Address>> {
        use super::schema::address_book::dsl::*;

        let res = address_book
            .filter(
                wallet_id
                    .eq(target_wallet_id)
                    .and(user_id.eq(&target_user_id))
                    .and(chain_id.eq(&target_chain_id))
                    .and(asset_id.eq(&target_asset_id)),
            )
            .limit(1)
            .load::<AddressRecord>(&mut self.conn()?)?;

        match res.into_iter().next() {
            Some(addr) => Ok(Some(addr.into())),
            None => Ok(None),
        }
    }

    fn has_address(
        &self,
        target_wallet_id: &str,
        target_user_id: &str,
        target_chain_id: &str,
        target_asset_id: &str,
    ) -> anyhow::Result<bool> {
        use super::schema::address_book::dsl::*;

        let res = address_book
            .filter(
                wallet_id
                    .eq(target_wallet_id)
                    .and(user_id.eq(&target_user_id))
                    .and(chain_id.eq(&target_chain_id))
                    .and(asset_id.eq(&target_asset_id)),
            )
            .load::<AddressRecord>(&mut self.conn()?)?;

        Ok(!res.is_empty())
    }
}

impl DatabaseWriter for PostgresDB {
    fn save_address(&self, address: &Address) -> anyhow::Result<()> {
        let new_address = AddressRecord {
            id: uuid::Uuid::new_v4(),
            wallet_id: address.wallet_id.to_owned(),
            user_id: address.user_id.to_string(),
            chain_id: address.chain_id.to_string(),
            asset_id: address.asset_id.to_owned(),
            address: address.address.to_owned(),
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
