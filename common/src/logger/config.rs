use clap::Args;
use procedural::EnvPrefix;

#[derive(Debug, Clone, Args, EnvPrefix)]
#[prefix = "EZEX_DEPOSIT"]
#[group(id = "logger")]
pub struct Config {
    #[arg(long = "logger-file", env = "LOGGER_FILE")]
    pub file: String,
    #[arg(
        long = "logger-level",
        env = "LOGGER_LEVEL",
        default_value = "info"
    )]
    pub level: String,
}
