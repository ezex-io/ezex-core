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
        _request: Request<ListBlockchainsRequest>,
    ) -> anyhow::Result<Response<ListBlockchainsResponse>, Status> {
        todo!()
    }

    async fn list_blockchain_assets(
        &self,
        _request: Request<ListBlockchainAssetsRequest>,
    ) -> anyhow::Result<Response<ListBlockchainAssetsResponse>, Status> {
        todo!()
    }

    async fn generate_address(
        &self,
        _request: Request<GenerateAddressRequest>,
    ) -> anyhow::Result<Response<GenerateAddressResponse>, Status> {
        todo!()
    }

    async fn get_address(
        &self,
        _request: Request<GetAddressRequest>,
    ) -> anyhow::Result<Response<GetAddressResponse>, Status> {
        // let user_id = request.get_ref().user_id.to_owned();
        // let chain_id = request.get_ref().chain_id.to_owned();
        // let asset_id = request.get_ref().asset_id.to_owned();
        // if user_id.is_empty() && request.get_ref().asset_id.to_owned().is_empty() {
        //     return Err(Status::new(
        //         Code::InvalidArgument,
        //         "user_id or coin identifier is not valid",
        //     ));
        // }

        // match self
        //     .deposit
        //     .get_address(&user_id, &chain_id, &asset_id)
        //     .await
        //     .map_err(|e| Status::internal(format!("Failed to get address: {}", e)))?
        // {
        //     Some(addr) => Ok(Response::new(GetAddressResponse {
        //         has_address: true,
        //         address: addr.address,
        //     })),
        //     None => Ok(Response::new(GetAddressResponse {
        //         has_address: false,
        //         address: "".to_string(),
        //     })),
        // }

        todo!()
    }

    async fn version(
        &self,
        _request: Request<VersionRequest>,
    ) -> anyhow::Result<Response<VersionResponse>, Status> {
        let response = VersionResponse {
            version: env!("CARGO_PKG_VERSION").to_string(),
        };

        Ok(Response::new(response))
    }
}
