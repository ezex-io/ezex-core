use anyhow::{Context, Result};
use clap::{Args, Subcommand};
use ezex_deposit::{
    database,
    deposit,
    event_bus,
    kms::{self, kms::DepositKms},
};

#[derive(Debug, Subcommand)]
pub enum AddressCmd {
    Generate {
        database_url: String,
        redis_connection: String,
        wallet_id: String,
        chain_id: String,
        asset_id: String,
    },
}

impl AddressCmd {
    pub async fn execute(&self) {
        if let Err(err) = self.execute_inner().await {
            eprintln!("Error Details: {:#}", err);
            std::process::exit(1);
        }
    }

    async fn execute_inner(&self) -> Result<()> {
        match self.to_owned() {
            AddressCmd::Generate {
                database_url,
                redis_connection,
                wallet_id,
                chain_id,
                asset_id,
            } => {
                let kms_config = kms::config::Config {};
                let kms = kms::kms::DepositKms::new(&kms_config)?;

                let database_config = database::postgres::config::Config {
                    database_url: database_url.clone(),
                    pool_size: 1,
                };
                let database = database::postgres::postgres::PostgresDB::new(&database_config)?;

                let redis_config = redis_stream_bus::config::Config {
                    connection_string: redis_connection.clone(),
                    consumer_name: "test".to_string(),
                    group_name: "test".to_string(),
                };
                let publisher = event_bus::redis::RedisBus::new(&redis_config).unwrap();

                let mut deposit = deposit::DepositHandler::new(
                    Box::new(database),
                    Box::new(kms),
                    Box::new(publisher),
                );

                // Generate address with proper error handling
                let address = deposit
                    .generate_address(wallet_id, chain_id, asset_id)
                    .await
                    .context(format!(
                        "Failed to generate address {} {} {}",
                        wallet_id, chain_id, asset_id
                    ))?;

                println!("address => {}", address);
                Ok(())
            }
        }
    }
}
