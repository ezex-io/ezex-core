mod address;
mod start;

use clap::{Parser};
use start::StartArgs;
use address::AddressCmd;

#[derive(Debug, Parser)]
pub enum Cmd {
    Start(StartArgs),
    #[command(subcommand)]
    Address(AddressCmd),
}

pub async fn handle(cmd: Cmd) {
    match cmd {
        Cmd::Start(args) => { todo!()},
        Cmd::Address(cmd) =>{

        } ,
    }
}
