use anyhow::{Context, Result};
use ezex_deposit::kms::create_provider;

#[derive(Debug)]
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
                // Get provider type from environment variable, with a default
                let provider_type =
                    std::env::var("KMS_PROVIDER_TYPE").unwrap_or_else(|_| "sample".to_string());

                // Create provider with proper error handling
                let provider = create_provider(&provider_type).context(format!(
                    "Failed to create KMS provider of type '{}'",
                    provider_type
                ))?;

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
