use crate::{
    deposit::DepositHandler,
    grpc::deposit::{deposit_service_server::DepositService, *},
};
use tonic::{Request, Response, Status};

#[allow(dead_code)]
pub struct DepositServiceImpl {
    deposit: DepositHandler,
}

impl DepositServiceImpl {
    pub fn new(deposit: DepositHandler) -> Self {
        Self { deposit }
    }
}

#[tonic::async_trait]
impl DepositService for DepositServiceImpl {
    async fn list_blockchains(
        &self,
        _req: Request<ListBlockchainsRequest>,
    ) -> anyhow::Result<Response<ListBlockchainsResponse>, Status> {
        todo!()
    }

    async fn list_blockchain_assets(
        &self,
        _req: Request<ListBlockchainAssetsRequest>,
    ) -> anyhow::Result<Response<ListBlockchainAssetsResponse>, Status> {
        todo!()
    }

    async fn get_address(
        &self,
        req: Request<GetAddressRequest>,
    ) -> anyhow::Result<Response<GetAddressResponse>, Status> {
        self.deposit.get_address(req).await
    }

    async fn generate_address(
        &self,
        req: Request<GenerateAddressRequest>,
    ) -> anyhow::Result<Response<GenerateAddressResponse>, Status> {
        self.deposit.generate_address(req).await
    }

    async fn version(
        &self,
        _: Request<VersionRequest>,
    ) -> anyhow::Result<Response<VersionResponse>, Status> {
        let response = VersionResponse {
            version: env!("CARGO_PKG_VERSION").to_string(),
        };

        Ok(Response::new(response))
    }
}
