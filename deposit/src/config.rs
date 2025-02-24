use serde::{Deserialize, Serialize};
use structopt::StructOpt;

#[derive(Debug, Clone, Serialize, Deserialize, StructOpt)]
pub struct Config {}

impl Default for Config {
    fn default() -> Self {
        Self {}
    }
}
