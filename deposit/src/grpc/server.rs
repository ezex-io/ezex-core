use crate::{
    deposit::DepositHandler,
    grpc::{
        config::Config,
        deposit::deposit_service_server::DepositServiceServer,
        service::DepositServiceImpl,
    },
};
use log::{error, info};
use tonic::transport::Server;

pub async fn start_server(config: &Config, deposit: DepositHandler) -> anyhow::Result<()> {
    // defining address for our service
    let service = DepositServiceImpl::new(deposit);
    let address = config.address.parse()?;
    info!("Deposit Server listening on {address}");

    if let Err(e) = Server::builder()
        .add_service(DepositServiceServer::new(service))
        .serve(address)
        .await
    {
        error!("failed to read from socket; err = {e:?}");
    };

    Ok(())
}
