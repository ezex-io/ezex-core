use clap::Args;
use common::register_config;
use procedural::EnvPrefix;

#[derive(Debug, Clone, Args, EnvPrefix)]
#[env_prefix = "EZEX_DEPOSIT"]
#[group(id = "grpc")]
pub struct Config {
    #[arg(long = "grpc-address", env = "GRPC_ADDRESS")]
    pub address: String,
}

register_config!(Config);
