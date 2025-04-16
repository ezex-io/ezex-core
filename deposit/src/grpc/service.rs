use crate::{
    database::provider::DatabaseReader,
    grpc::deposit::{
        deposit_service_server::DepositService,
        *,
    },
};
use tonic::{
    Code,
    Request,
    Response,
    Status,
};

pub struct DepositServiceImpl<D>
where
    D: DatabaseReader + Sync + Send + 'static,
{
    database: D,
}

impl<D> DepositServiceImpl<D>
where
    D: DatabaseReader + Sync + Send + 'static,
{
    pub fn new(database: D) -> Self {
        Self { database }
    }
}

#[tonic::async_trait]
impl<D> DepositService for DepositServiceImpl<D>
where
    D: DatabaseReader + Sync + Send + 'static,
{
    async fn get_address(
        &self,
        request: Request<GetAddressRequest>,
    ) -> anyhow::Result<Response<GetAddressResponse>, Status> {
        let user_id = request.get_ref().user_id.to_owned();
        if user_id.is_empty() && request.get_ref().coin.to_owned().is_empty() {
            return Err(Status::new(
                Code::InvalidArgument,
                "user_id or coin identifier is not valid",
            ));
        }

        let chain_id = match common::utils::coin_to_chain_id(&request.into_inner().coin) {
            Some(chain) => chain,
            None => return Err(Status::new(Code::NotFound, "coin not defined")),
        };
        let address = self
            .database
            .get_address(&user_id, &chain_id)
            .map_err(common::utils::error_to_tonic_status)?;

        match address {
            Some(addr) => Ok(Response::new(GetAddressResponse {
                deposit_address: addr.address,
            })),
            None => Err(Status::new(Code::NotFound, "No deposit address")),
        }
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
