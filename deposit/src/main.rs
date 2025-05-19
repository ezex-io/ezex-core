use clap::Parser;

mod cmd;

#[derive(Debug, Parser)]
#[command(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION")
)]
pub struct CLI {
    #[command(subcommand)]
    pub cmd: cmd::Cmd,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let cli = CLI::parse();
    cmd::handle(cli.cmd).await;
}
