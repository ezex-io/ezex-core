use crate::{
    database::provider::DatabaseProvider, deposit::DepositHandler, kms::provider::KmsProvider,
};
use common::event::*;
use redis_stream_bus::{
    bus::StreamBus, client::RedisClient, config::Config as RedisConfig, entry::Entry,
};
use serde::Serialize;
use tonic::async_trait;

use super::provider::PublisherProvider;

pub struct RedisBus {
    client: RedisClient,
}

impl RedisBus {
    pub fn new(config: &RedisConfig) -> anyhow::Result<Self> {
        // TODO: Why we can't use `?``
        let client = RedisClient::from_config(config).map_err(|e| anyhow::anyhow!(e))?;

        // client.read_loop();
        Ok(Self { client })
    }
}

#[async_trait]
impl PublisherProvider for RedisBus {
    async fn publish(&mut self, event: Box<dyn EventMessage>) -> anyhow::Result<()> {
        let serializer = serde_redis::Serializer;
        let fields = event.serialize(serializer).unwrap();

        let entry = Entry {
            id: None,
            key: event.key(),
            fields: fields,
        };

        self.client.xadd(entry).await.unwrap();

        Ok(())
    }
}
