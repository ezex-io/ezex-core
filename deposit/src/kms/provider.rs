use async_trait::async_trait;

#[async_trait]
pub trait KMSProvider: Sync + Send + 'static {
    async fn generate_address(
        &self,
        wallet_id: &str,
        coin: &str,
        forward_version: i32,
    ) -> anyhow::Result<String>;
}
