use clap::Args;

#[derive(Debug, Clone, Args)]
#[group(id = "grpc")]
pub struct Config {
    #[arg(long, env = "EZEX_DEPOSIT_GRPC_ADDRESS")]
    pub address: String,
}
