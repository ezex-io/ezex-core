#[derive(Debug, Clone, Eq, PartialEq)]
pub enum WalletStatus {
    Disabled = 0,
    Enabled = 1,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Wallet {
    pub status: WalletStatus,
    pub wallet_id: String,
    pub chain_id: String,
    pub description: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Address {
    pub wallet_id: String,
    pub user_id: String,
    pub chain_id: String,
    pub asset_id: String,
    pub address: String,
    pub created_at: chrono::NaiveDateTime,
}

impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}/{}/{}/{}/{}",
            self.wallet_id, self.user_id, self.chain_id, self.asset_id, self.address
        )
    }
}
