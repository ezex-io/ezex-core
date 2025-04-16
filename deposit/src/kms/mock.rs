use mockall::mock;

use super::provider::KmsProvider;

mock! {
    pub KmsProvider {}

    #[async_trait::async_trait]
    impl KmsProvider for KmsProvider{
        async fn generate_address(&self, wallet_id: &str, coin: &str) -> anyhow::Result<String> ;
    }


}
