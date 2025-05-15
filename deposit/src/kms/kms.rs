use crate::kms::provider::KmsProvider;
use async_trait::async_trait;
use log::debug;

use super::config::Config;

pub struct DepositKms {
}

impl DepositKms {
    pub fn new(config: &Config) -> anyhow::Result<Self> {
        Ok(DepositKms {
        })
    }
}

#[async_trait]
impl KmsProvider for DepositKms {
    async fn generate_address(&self, wallet_id: &str,   chain_id: &str, asset_id: &str) -> anyhow::Result<String> {
        // TODO: use ezex_kms service to generate address
        debug!(
            "generate_address: {} {} {}",
            wallet_id, chain_id, asset_id
        );

        Ok("Generated_address".to_string())
    }
}
