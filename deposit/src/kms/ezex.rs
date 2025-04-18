use crate::kms::provider::KmsProvider;
use async_trait::async_trait;
use log::debug;

pub struct ezexKms {}

impl ezexKms {
    pub fn new() -> anyhow::Result<Self> {
        Ok(ezexKms {})
    }
}

#[async_trait]
impl KmsProvider for ezexKms {
    async fn generate_address(&self, wallet_id: &str, coin: &str) -> anyhow::Result<String> {
        debug!("generate_address: {} {}", wallet_id, coin);

        todo!()
    }
}
