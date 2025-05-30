use common::event::EventMessage;
use tonic::async_trait;

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait PublisherProvider: Sync + Send + 'static {
    async fn publish(&self, event: Box<dyn EventMessage>) -> anyhow::Result<()>;
}
