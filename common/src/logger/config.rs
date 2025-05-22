use clap::Args;
use procedural::EnvPrefix;

use crate::register_config;

#[derive(Debug, Clone, Args, EnvPrefix)]
#[env_prefix = "EZEX_DEPOSIT"]
#[group(id = "logger")]
pub struct Config {
    #[arg(long = "logger-file", env = "LOGGER_FILE", default_value = "")]
    pub file: String,
    #[arg(long = "logger-level", env = "LOGGER_LEVEL", default_value = "info")]
    pub level: String,
}

// Automatically register this config
register_config!(Config);
