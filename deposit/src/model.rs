#[derive(Eq, PartialEq, Debug, Clone, Default)]
pub struct WalletAddress {
    pub user_id: String,
    pub chain_id: String,
    pub wallet_id: String,
    pub deposit_address: String,
}

#[derive(Eq, PartialEq, Debug, Clone, Default)]
pub struct Wallet {
    pub wallet_id: String,
    pub chain_id: String,
    pub passphrase: String,
}
