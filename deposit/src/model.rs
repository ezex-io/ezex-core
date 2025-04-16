use chrono::DateTime;

#[derive(Debug, Clone)]
pub struct WalletAddress {
    pub user_id: String,
    pub chain_id: String,
    pub wallet_id: String,
    pub address: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone)]
pub struct Wallet {
    pub active: bool,
    pub wallet_id: String,
    pub wallet_type: String,
    pub chain_id: String,
    pub description: String,
    // pub created_at: DateTime<>,
}
