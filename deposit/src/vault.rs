use crate::config::Config;
use crate::database::provider::DatabaseProvider;
use crate::kms::provider::KMSProvider;
use common::topic::*;
use log::info;

#[derive(Clone)]
pub struct Vault<D, K>
where
    D: DatabaseProvider,
    K: KMSProvider,
{
    database: D,
    kms: K,
    config: Config,
}

impl<D, K> Vault<D, K>
where
    D: DatabaseProvider,
    K: KMSProvider,
{
    pub fn new(db: D, kms: K, config: Config) -> Self {
        Vault {
            database: db,
            kms,
            config,
        }
    }

    pub async fn process_address_generate(
        &self,
        message: deposit::address::Generate,
    ) -> anyhow::Result<Vec<Box<dyn TopicMessage>>> {
        let chain_id = match common::consts::coin_to_chain_id(&message.coin) {
            Some(chain) => chain.to_string(),
            None => anyhow::bail!("Unsupported coin: {}", message.coin),
        };
        match self.database.get_address(&message.user_id, &chain_id)? {
            Some(wallet_address) => {
                anyhow::bail!("Duplicated address: {}", wallet_address.deposit_address)
            }
            None => {
                // Generate Address for the user in requested wallet
                let mut forward_version = 0;
                if chain_id == common::consts::chain::id::ETHEREUM
                    || chain_id == common::consts::chain::id::ETHEREUM_TESTNET
                {
                    forward_version = 1;
                }
                let wallet_address = self
                    .kms
                    .generate_address(&message.wallet_id, &message.coin, forward_version)
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
#[path = "./vault_test.rs"]
mod vault_test;
