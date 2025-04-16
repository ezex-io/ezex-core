use anyhow::{
    Context,
    Result,
};
use clap::{Args, Subcommand};
use ezex_deposit::kms::{self, ezex::ezexKms, provider::KMSProvider};


#[derive(Debug, Subcommand)]
pub enum AddressCmd {
    Generate {
        wallet_id: String,
        identifier: String,
        forward_version: i32,
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
                wallet_id,
                identifier,
                forward_version: _,
            } => {
                // Create provider with proper error handling
                let provider = ezexKms::new()?;

                // Generate address with proper error handling
                let address = provider
                    .generate_address(&wallet_id, &identifier)
                    .await
                    .context(format!(
                        "Failed to generate address for wallet '{}' and identifier '{}'",
                        wallet_id, identifier
                    ))?;

                println!("address => {}", address);
                Ok(())
            }
        }
    }
}
