use async_trait::async_trait;

#[async_trait]
pub trait KmsProvider: Sync + Send + 'static {
    async fn generate_address(
        &self,
        wallet_id: &str,
        chain_id: &str,
        asset_id: &str,
    ) -> anyhow::Result<String>;
}
