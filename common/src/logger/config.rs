use serde::{
    Deserialize,
    Serialize,
};
use structopt::StructOpt;

#[derive(Debug, StructOpt, Clone, Serialize, Deserialize)]
pub struct Config {
    #[structopt(long = "log-file", env = "LOG_FILE", default_value = "")]
    pub file: String,
    #[structopt(long = "log-level", env = "LOG_LEVEL", default_value = "trace")]
    pub level: String,
}
