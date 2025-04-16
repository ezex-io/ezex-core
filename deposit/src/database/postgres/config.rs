use clap::Args;

#[derive(Debug, Clone, Args)]
#[group(id="postgres")]
pub struct Config {
    #[arg(long="postgres-database-url", env = "EZEX_DEPOSIT_POSTGRES_DATABASE_URL")]
    pub database_url: String,
    #[arg(long="postgres-pool-size", env = "EZEX_DEPOSIT_POSTGRES_POOL_SIZE", default_value = "10")]
    pub pool_size: u32,
}
