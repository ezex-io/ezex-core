use crate::{
    api::grpc::{
        config::Config, deposit::deposit_service_server::DepositServiceServer,
        service::DepositServiceImpl,
    },
    database::provider::DatabaseReader,
};
use log::{error, info};
use tonic::transport::Server;

pub async fn start_server<D>(
    config: &Config,
    db: D,
) -> anyhow::Result<(), Box<dyn std::error::Error>>
where
    D: DatabaseReader + Sync + Send + 'static,
{
    // defining address for our service
    let service = DepositServiceImpl::new(db);
    let address = config.address.parse().unwrap();
    info!("Deposit Server listening on {}", address);

    if let Err(e) = Server::builder()
        .add_service(DepositServiceServer::new(service))
        .serve(address)
        .await
    {
        error!("failed to read from socket; err = {:?}", e);
    };

    Ok(())
}
