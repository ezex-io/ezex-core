use clap::Args;
use common::register_config;
use procedural::EnvPrefix;

#[derive(Debug, Clone, Args, EnvPrefix)]
#[env_prefix = "EZEX_DEPOSIT"]
#[group(id = "postgres")]
pub struct Config {
    #[arg(long = "postgres-database-url", env = "POSTGRES_DATABASE_URL")]
    pub database_url: String,
    #[arg(
        long = "postgres-pool-size",
        env = "POSTGRES_POOL_SIZE",
        default_value = "10"
    )]
    pub pool_size: u32,
}

register_config!(Config);
