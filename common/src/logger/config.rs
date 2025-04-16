use clap::Args;

#[derive(Debug, Clone, Args)]
#[group (id="logger")]
pub struct Config {
    #[arg(long="logger-file", env = "EZEX_DEPOSIT_LOGGER_FILE")]
    pub file: String,
    #[arg(long="logger-level", env = "EZEX_DEPOSIT_LOGGER_LEVEL", default_value = "info")]
    pub level: String,
}
