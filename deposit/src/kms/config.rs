use clap::Args;
use procedural::EnvPrefix;

#[derive(Debug, Clone, Args, EnvPrefix)]
#[group(id = "kms")]
pub struct Config {
}
