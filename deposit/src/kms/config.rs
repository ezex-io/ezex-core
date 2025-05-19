use clap::Args;
use common::register_config;
use procedural::EnvPrefix;

#[derive(Debug, Clone, Args, EnvPrefix)]
#[group(id = "kms")]
pub struct Config {
    // TODO: gRPC address of ezex_kms
}

register_config!(Config);
