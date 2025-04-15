use crate::database::schema::addresses;

#[derive(Eq, PartialEq, Debug, Clone, Default, Queryable, Selectable)]
#[diesel(table_name = addresses)]
pub struct WalletAddress {
    pub user_id: String,
    pub chain_id: String,
    pub wallet_id: String,
    pub deposit_address: String,
    pub created_at: chrono::NaiveDateTime,
}

// For insertable operations, you might also want:
#[derive(Insertable)]
#[diesel(table_name = addresses)]
pub struct NewWalletAddress<'a> {
    pub user_id: &'a str,
    pub chain_id: &'a str,
    pub wallet_id: &'a str,
    pub deposit_address: &'a str,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Eq, PartialEq, Debug, Clone, Default)]
pub struct Wallet {
    pub wallet_id: String,
    pub chain_id: String,
    pub passphrase: String,
}
