use clap::Args;
use procedural::EnvPrefix;

#[derive(Debug, Clone, Args, EnvPrefix)]
#[env_prefix = "EZEX_DEPOSIT"]
#[group(id = "grpc")]
pub struct Config {
    #[arg(long, env = "GRPC_ADDRESS")]
    pub address: String,
}
