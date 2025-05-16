use log::info;
use tonic::{Request, Response, Status};

use crate::{
    database::provider::DatabaseProvider,
    event_bus::{events, provider::PublisherProvider},
    grpc::deposit::{
        GenerateAddressRequest,
        GenerateAddressResponse,
        GetAddressRequest,
        GetAddressResponse,
    },
    kms::provider::KmsProvider,
    types::Wallet,
};

pub struct DepositHandler {
    database: Box<dyn DatabaseProvider>,
    kms: Box<dyn KmsProvider>,
    publisher: Box<dyn PublisherProvider>,
}

impl DepositHandler {
    pub fn new(
        database: Box<dyn DatabaseProvider>,
        kms: Box<dyn KmsProvider>,
        publisher: Box<dyn PublisherProvider>,
    ) -> Self {
        DepositHandler {
            database,
            kms,
            publisher,
        }
    }

    pub async fn get_address(
        &self,
        req: Request<GetAddressRequest>,
    ) -> anyhow::Result<Response<GetAddressResponse>, Status> {
        let wallet = self.get_wallet(&req.get_ref().chain_id)?;

        let address = match self
            .database
            .get_address(
                &wallet.wallet_id,
                &req.get_ref().user_id,
                &req.get_ref().chain_id,
                &req.get_ref().asset_id,
            )
            .map_err(|e| Status::internal(e.to_string()))?
        {
            Some(address) => address,
            None => {
                return Ok(Response::new(GetAddressResponse {
                    has_address: false,
                    address: "".to_string(),
                }));
            }
        };

        Ok(Response::new(GetAddressResponse {
            has_address: true,
            address: address.address,
        }))
    }

    pub async fn generate_address(
        &self,
        req: Request<GenerateAddressRequest>,
    ) -> anyhow::Result<Response<GenerateAddressResponse>, Status> {
        let wallet = self.get_wallet(&req.get_ref().chain_id)?;

        match self
            .database
            .has_address(
                &wallet.wallet_id,
                &req.get_ref().user_id,
                &req.get_ref().chain_id,
                &req.get_ref().asset_id,
            )
            .map_err(|e| Status::internal(e.to_string()))?
        {
            true => Err(Status::invalid_argument(format!(
                "Duplicated address: {} {} {}",
                req.get_ref().user_id,
                req.get_ref().chain_id,
                req.get_ref().asset_id,
            ))),
            false => {
                let address = self
                    .kms
                    .generate_address(
                        &wallet.wallet_id,
                        &req.get_ref().user_id,
                        &req.get_ref().chain_id,
                        &req.get_ref().asset_id,
                    )
                    .await
                    .map_err(|e| Status::internal(e.to_string()))?;

                // Store the address in db
                self.database
                    .save_address(&address)
                    .map_err(|e| Status::internal(e.to_string()))?;

                // Publish the event
                let event = Box::new(events::address::Generated {
                    user_id: address.user_id.to_string(),
                    chain_id: address.chain_id.to_string(),
                    asset_id: address.asset_id.to_string(),
                    address: address.address.clone(),
                });
                self.publisher
                    .publish(event)
                    .await
                    .map_err(|e| Status::internal(e.to_string()))?;

                info!("address generated. {address}");

                Ok(Response::new(GenerateAddressResponse {
                    address: address.address,
                }))
            }
        }
    }

    fn get_wallet(&self, chain_id: &str) -> anyhow::Result<Wallet, Status> {
        match self
            .database
            .get_wallet(chain_id)
            .map_err(|e| Status::internal(format!("unable to get wallet: {e}")))?
        {
            Some(wallet) => Ok(wallet),
            None => Err(Status::not_found("unable to find wallet_id")),
        }
    }
}

#[cfg(test)]
#[path = "./deposit_test.rs"]
mod deposit_test;
