use crate::types::{self};

use super::schema::*;
use diesel::prelude::*;

#[derive(Selectable, Queryable, Insertable)]
#[diesel(table_name = wallets)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub(super) struct WalletRecord {
    pub id: uuid::Uuid,
    pub status: i16,
    pub wallet_id: String,
    pub chain_id: String,
    pub description: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Selectable, Queryable, Insertable)]
#[diesel(table_name = address_book)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub(super) struct AddressRecord {
    pub id: uuid::Uuid,
    pub user_id: String,
    pub wallet_id: String,
    pub chain_id: String,
    pub asset_id: String,
    pub address: String,
    pub created_at: chrono::NaiveDateTime,
}

impl From<WalletRecord> for types::Wallet {
    fn from(wallet: WalletRecord) -> Self {
        types::Wallet {
            status: match wallet.status {
                0 => types::WalletStatus::Disabled,
                1 => types::WalletStatus::Enabled,
                _ => panic!("Invalid wallet status"),
            },
            wallet_id: wallet.wallet_id,
            chain_id: wallet.chain_id,
            description: wallet.description,
            created_at: wallet.created_at,
        }
    }
}

impl From<AddressRecord> for types::Address {
    fn from(rec: AddressRecord) -> Self {
        types::Address {
            wallet_id: rec.wallet_id.clone(),
            user_id: rec.user_id.clone(),
            chain_id: rec.chain_id.clone(),
            asset_id: rec.asset_id.clone(),
            address: rec.address.clone(),
            created_at: rec.created_at,
        }
    }
}
