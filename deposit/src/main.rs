use common::topic::deposit;

mod cmd;

#[derive(Parser, Debug)]
#[command(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION")
)]
pub struct Deposit {
    #[command(subcommand)]
    pub commands: cmd::Commands,
}

#[tokio::main]
async fn main() {
    // dotenv::dotenv().ok();

    // let deposit = Deposit::parse();
    // deposit::handle(cli);
    todo!()
}
