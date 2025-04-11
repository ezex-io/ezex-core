use deposit_vault::kms::provider::KMSProvider;
use structopt::StructOpt;

#[derive(Debug, StructOpt, Clone)]
#[structopt(name = "address")]
pub enum AddressCmd {
    Generate {
        wallet_id: String,
        identifier: String,
        forward_version: i32,
    },
}

impl AddressCmd {
    pub async fn execute(&self) {
        match self.to_owned() {
            AddressCmd::Generate {
                wallet_id,
                identifier,
                forward_version,
            } => {
                let address = kms
                    .generate_address(&wallet_id, &identifier, forward_version)
                    .await
                    .unwrap();

                println!("address => {}", address);
            }
        };
    }
}
