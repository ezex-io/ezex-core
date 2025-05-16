use crate::{kms::provider::KmsProvider, types::Address};
use async_trait::async_trait;
use log::debug;

use super::config::Config;

pub struct DepositKms {}

impl DepositKms {
    pub fn new(_config: &Config) -> anyhow::Result<Self> {
        Ok(DepositKms {})
    }
}

#[cfg_attr(test, async_trait::async_trait)]
#[async_trait]
impl KmsProvider for DepositKms {
    async fn generate_address(
        &self,
        wallet_id: &str,
        user_id: &str,
        chain_id: &str,
        asset_id: &str,
    ) -> anyhow::Result<Address> {
        // TODO: use ezex_kms service to generate address
        debug!("generate_address: {} {} {}", wallet_id, chain_id, asset_id);

        let addr = Address {
            wallet_id: wallet_id.to_string(),
            user_id: user_id.to_string(),
            chain_id: chain_id.to_string(),
            asset_id: asset_id.to_string(),
            address: "Generated_address".to_string(),
            created_at: chrono::Utc::now().naive_utc(),
        };

        Ok(addr)
    }
}
