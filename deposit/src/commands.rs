pub mod address;
pub mod start;
use crate::commands::start::StartCmd;
use async_trait::async_trait;
use structopt::StructOpt;

use self::address::AddressCmd;

#[async_trait]
pub trait EzexVaultCommand {
    /// Returns the result of the command execution.
    async fn execute(self);
}

#[derive(Debug, StructOpt)]
pub enum Command {
    ///Start the vault service
    #[structopt(name = "start")]
    Start(StartCmd),
    #[structopt(name = "address")]
    Address(AddressCmd),
}

impl Command {
    /// Wrapper around `StructOpt::from_args` method.
    pub fn from_args() -> Self {
        <Self as StructOpt>::from_args()
    }
}

#[async_trait]
impl EzexVaultCommand for Command {
    async fn execute(self) {
        match self {
            Self::Start(command) => command.execute().await,
            Self::Address(command) => command.execute().await,
        }
    }
}
