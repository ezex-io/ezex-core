use crate::{
    api::grpc::{
        config::Config,
        deposit::vault_service_server::VaultServiceServer,
        service::VaultServiceImpl,
    },
    database::provider::DatabaseReader,
};
use log::{
    error,
    info,
};
use tonic::transport::Server;

pub async fn start_server<D>(
    config: &Config,
    db: D,
) -> anyhow::Result<(), Box<dyn std::error::Error>>
where
    D: DatabaseReader + Sync + Send + 'static,
{
    // defining address for our service
    let vault_service_impl = VaultServiceImpl::new(db);
    let address = config.address.parse().unwrap();
    info!("Vault Server listening on {}", address);

    if let Err(e) = Server::builder()
        .add_service(VaultServiceServer::new(vault_service_impl))
        .serve(address)
        .await
    {
        error!("failed to read from socket; err = {:?}", e);
    };

    Ok(())
}
