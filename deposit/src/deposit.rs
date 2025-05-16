use crate::{
    // config::Config,
    database::provider::DatabaseProvider,
    event_bus::{events, provider::PublisherProvider},
    kms::provider::KmsProvider,
    types::Address,
};
// use common::event::*;
use log::info;

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
        user_id: &str,
        chain_id: &str,
        asset_id: &str,
    ) -> anyhow::Result<Option<Address>> {
        self.database.get_address(user_id, chain_id, asset_id)
    }

    pub async fn generate_address(
        &mut self,
        user_id: &str,
        chain_id: &str,
        asset_id: &str,
    ) -> anyhow::Result<String> {
        match self.database.get_address(user_id, chain_id, asset_id)? {
            Some(address) => {
                anyhow::bail!("Duplicated address: {}", address.address)
            }
            None => {
                let wallet_id = self.database.get_wallet(chain_id)?.chain_id;
                let address = self
                    .kms
                    .generate_address(&wallet_id, chain_id, asset_id)
                    .await?;

                // Store the address in db
                self.database
                    .assign_address(user_id, &wallet_id, chain_id, asset_id, &address)?;

                // Publish the event
                let event = Box::new(events::address::Generated {
                    user_id: user_id.to_string(),
                    wallet_id: wallet_id.to_string(),
                    chain_id: chain_id.to_string(),
                    asset_id: asset_id.to_string(),
                    address: address.clone(),
                });
                self.publisher.publish(event).await?;

                info!(
                    "A new address generated. {} {} {}, address: {}",
                    user_id, chain_id, asset_id, address
                );

                Ok(address)
            }
        }
    }
}

#[cfg(test)]
#[path = "./deposit_test.rs"]
mod deposit_test;
