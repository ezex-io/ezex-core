use crate::{
    config::Config,
    database::provider::DatabaseProvider,
    kms::provider::KMSProvider,
};
use common::topic::*;
use log::info;

pub struct Deposit<D, K>
where
    D: DatabaseProvider,
    K: KMSProvider,
{
    database: D,
    kms: K,
}

impl<D, K> Deposit<D, K>
where
    D: DatabaseProvider,
    K: KMSProvider,
{
    pub fn new(db: D, kms: K) -> Self {
        Deposit { database: db, kms }
    }

    pub async fn process_address_generate(
        &self,
        message: deposit::address::Generate,
    ) -> anyhow::Result<Vec<Box<dyn TopicMessage>>> {
        let chain_id = match common::utils::coin_to_chain_id(&message.coin) {
            Some(chain) => chain.to_string(),
            None => anyhow::bail!("Unsupported coin: {}", message.coin),
        };
        match self.database.get_address(&message.user_id, &chain_id)? {
            Some(wallet_address) => {
                anyhow::bail!("Duplicated address: {}", wallet_address.address)
            }
            None => {
                let wallet_address = self
                    .kms
                    .generate_address(&message.wallet_id, &message.coin)
                    .await?;

                match self.database.has_address(&wallet_address, &chain_id)? {
                    false => {
                        // Store the address in db
                        self.database.assign_address(
                            &message.user_id,
                            &chain_id,
                            &message.wallet_id,
                            &wallet_address,
                        )?;

                        info!(
                            "A new address created. user: {}, coin: {}, address: {}",
                            message.user_id, message.coin, wallet_address
                        );

                        Ok(vec![Box::new(deposit::address::Generated {
                            user_id: message.user_id,
                            coin: message.coin,
                            chain_id,
                            wallet_id: message.wallet_id,
                            deposit_address: wallet_address,
                        })])
                    }
                    true => anyhow::bail!("Duplicated address: {}", wallet_address),
                }
            }
        }
    }
}

#[cfg(test)]
#[path = "./deposit_test.rs"]
mod deposit_test;
