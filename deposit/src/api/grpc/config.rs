use serde::{Deserialize, Serialize};
use structopt::StructOpt;

#[derive(Debug, Clone, Serialize, Deserialize, StructOpt)]
pub struct Config {
    #[structopt(long = "grpc-address", env = "GRPC_ADDRESS")]
    pub address: String,
}
