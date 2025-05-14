use crate::{
    database::provider::DatabaseProvider,
    deposit::DepositHandler,
    kms::provider::KmsProvider,
};
use common::{event::*, redis::redis_bus::EventBus};
use redis_stream_bus::{
    bus::StreamBus,
    client::RedisClient,
    config::Config as RedisConfig,
    entry::Entry,
};
use tonic::async_trait;

use super::provider::PublisherProvider;

pub struct RedisBus {
    client: RedisClient,
}

impl RedisBus {
    pub fn new(config: &RedisConfig) -> anyhow::Result<Self> {
        // TODO: replace from_config with new
        // TODO: Why we can't use `?``
        let client = RedisClient::from_config(config).unwrap();

        // client.read_loop();
        Ok(Self { client })
    }
}

#[async_trait]
impl PublisherProvider for RedisBus {
    async fn publish(&mut self, event: Box<dyn EventMessage>) -> anyhow::Result<()> {
        let mut str = vec![];
        let mut serializer = serde_json::Serializer::new(&mut str);
        event
            .erased_serialize(&mut <dyn erased_serde::Serializer>::erase(&mut serializer))
            .unwrap();

        let entry = Entry {
            id: None,
            key: event.key(),
            fields: redis::Value::Nil, // TODO??
        };

        self.client.xadd(entry).await.unwrap();

        Ok(())
    }
}

// #[async_trait::async_trait]
// impl EventBus for RedisBus {
//     async fn process_Event(
//         &mut self,
//         key: &str,
//         msg: &str,
//     ) -> anyhow::Result<Vec<Box<dyn TopicMessage>>> {
//         match key {
//             topic::general::Ping::name => {
//                 let message: topic::general::Ping = serde_json::from_str(msg)?;

//                 self.client.xadd(entry)
//                     .await
//                     .map_err(|e| anyhow::anyhow!("Failed to send message: {}", e))?;

//                 Ok(())
//             }
//             k => {
//                 log::warn!("unimplemented key: {}", k);
//                 Ok(vec![])
//             }
//         }
//     }

//     fn module_name(&self) -> String {
//         topic::NAME.to_string()
//     }

//     fn module_version(&self) -> String {
//         env!("CARGO_PKG_VERSION").to_string()
//     }
// }
