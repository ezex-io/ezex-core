use common::event::EventMessage;
use tonic::async_trait;

#[cfg(test)]
use mockall::mock;

#[cfg_attr(test, mockall::)]
#[async_trait]
pub trait PublisherProvider: Sync + Send + 'static {
    async fn publish(&self, event: Box<dyn EventMessage>) -> anyhow::Result<()>;
}
