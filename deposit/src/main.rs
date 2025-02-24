use commands::Command;
mod commands;

#[tokio::main]
async fn main() {
    match Command::from_args() {
        Command::Start(cmd) => cmd.execute().await,
        Command::Address(cmd) => cmd.execute().await,
    }
}
