use crate::types;

use super::schema::*;
use diesel::prelude::*;

#[derive(Selectable, Queryable, Insertable)]
#[diesel(table_name = wallets)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub(super) struct Wallet {
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
pub(super) struct Address {
    pub id: uuid::Uuid,
    pub user_id: String,
    pub wallet_id: String,
    pub chain_id: String,
    pub asset_id: String,
    pub address: String,
    pub created_at: chrono::NaiveDateTime,
}

impl From<Wallet> for types::Wallet {
    fn from(wallet: Wallet) -> Self {
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

impl From<Address> for types::Address {
    fn from(addr: Address) -> Self {
        types::Address {
            user_id: addr.user_id,
            chain_id: addr.chain_id,
            wallet_id: addr.wallet_id,
            address: addr.address.clone(),
            created_at: addr.created_at,
        }
    }
}
