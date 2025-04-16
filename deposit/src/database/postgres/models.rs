use super::schema::*;
use diesel::prelude::*;

#[derive(Insertable)]
#[diesel(table_name = wallets)]
pub struct Wallet {
    pub id: uuid::Uuid,
    pub status: i16,
    pub wallet_id: String,
    pub wallet_type: String,
    pub description: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = address_book)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Address {
    pub id: uuid::Uuid,
    pub user_id: String,
    pub chain_id: String,
    pub generated_wallet_id: String,
    pub address: String,
    pub created_at: chrono::NaiveDateTime,
}
