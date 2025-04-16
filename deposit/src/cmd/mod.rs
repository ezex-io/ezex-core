mod address;
mod start;

#[derive(Subcommand, Debug)]
pub enum Commands {
    StartCmd,
    AddressCmd,
}
