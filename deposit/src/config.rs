use serde::{
    Deserialize,
    Serialize,
};
use structopt::StructOpt;

#[derive(Default, Debug, Clone, Serialize, Deserialize, StructOpt)]
pub struct Config {}
