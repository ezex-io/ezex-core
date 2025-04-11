use crate::{
    database::provider::DatabaseProvider,
    kms::provider::KMSProvider,
    vault::Vault,
};
use common::{
    redis::redis_bus::RedisBusTrait,
    topic::*,
};

pub struct RedisBus<D, K>
where
    D: DatabaseProvider,
    K: KMSProvider,
{
    vault: Vault<D, K>,
}

impl<D, K> RedisBus<D, K>
where
    D: DatabaseProvider,
    K: KMSProvider,
{
    pub fn new(vault: Vault<D, K>) -> Self {
        Self { vault }
    }
}

#[async_trait::async_trait]
impl<D, K> RedisBusTrait for RedisBus<D, K>
where
    D: DatabaseProvider + Send + Sync,
    K: KMSProvider + Send + Sync,
{
    async fn process_message(
        &mut self,
        key: &str,
        msg: &str,
    ) -> anyhow::Result<Vec<Box<dyn TopicMessage>>> {
        match key {
            deposit::address::Generate::name => {
                let message: deposit::address::Generate = serde_json::from_str(msg)?;

                self.vault.process_address_generate(message).await
            }
            k => {
                log::warn!("unimplemented key: {}", k);
                Ok(vec![])
            }
        }
    }

    fn module_name(&self) -> String {
        common::topic::deposit::NAME.to_string()
    }

    fn module_version(&self) -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }
}
