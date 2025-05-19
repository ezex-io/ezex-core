use serde::{Deserialize, Serialize};
use clap::Parser;

#[derive(Debug, Clone, Serialize, Deserialize, Parser)]
pub struct Config {
    #[arg(long = "database-url", env = "DATABASE_URL")]
    pub db_url: String,
    #[arg(long = "pool-size", env = "DATABASE_POOL_SIZE", default_value = "3")]
    pub pool_size: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            db_url: Default::default(),
            pool_size: 3,
        }
    }
}
