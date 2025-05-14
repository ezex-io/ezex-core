use common::event::EventMessage;
use tonic::async_trait;

#[async_trait]
pub trait PublisherProvider: Sync + Send + 'static {
    async fn publish(&mut self, event: Box<dyn EventMessage>) -> anyhow::Result<()>;
}
