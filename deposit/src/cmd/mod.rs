mod start;

use clap::Parser;
use start::StartArgs;

#[derive(Debug, Parser)]
pub enum Cmd {
    Start(StartArgs),
}

pub async fn handle(cmd: Cmd) {
    match cmd {
        Cmd::Start(args) => {
            args.execute().await;
        }
    }
}
