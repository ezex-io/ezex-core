use clap::Args;

#[derive(Debug, Args)]
pub struct Config {
    #[arg(long, env = "EZEX_DEPOSIT_POSTGRES_DATABASE_URL")]
    pub database_url: String,
    #[arg(long, env = "EZEX_DEPOSIT_POSTGRES_POOL_SIZE", default_value = "10")]
    pub pool_size: u32,
}
