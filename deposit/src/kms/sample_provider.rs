use crate::kms::provider::KMSProvider;
use async_trait::async_trait;

pub struct SampleProvider {}

impl SampleProvider {
    pub fn new() -> Self {
        Self {}
    }
}
#[async_trait]
impl KMSProvider for SampleProvider {
    async fn generate_address(
        &self,
        _wallet_id: &str,
        _identifier: &str,
    ) -> Result<String, anyhow::Error> {
        todo!()
    }
}
