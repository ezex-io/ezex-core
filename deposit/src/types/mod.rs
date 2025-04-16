#[derive(Debug, Clone, Eq, PartialEq)]
pub enum WalletStatus {
    Disabled,
    Enabled,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Wallet {
    pub status: WalletStatus,
    pub wallet_id: String,
    pub chain_id: String,
    pub wallet_type: String, // TODO: Maybe enum here?
    pub description: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Address {
    pub user_id: String,
    pub chain_id: String,
    // Wallet ID is used to generate the address
    pub wallet_id: String,
    pub address: String,
    pub created_at: chrono::NaiveDateTime,
}
