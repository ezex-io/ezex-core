use crate::{
    database::provider::DatabaseProvider,
    deposit::Deposit,
    kms::provider::KMSProvider,
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
    deposit: Deposit<D, K>,
}

impl<D, K> RedisBus<D, K>
where
    D: DatabaseProvider,
    K: KMSProvider,
{
    pub fn new(vault: Deposit<D, K>) -> Self {
        Self { deposit: vault }
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

                self.deposit.process_address_generate(message).await
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
