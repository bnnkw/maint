use clap::{Args, Parser, Subcommand};
use maint::DataStore;

#[derive(Parser)]
pub struct Cmd {
    #[command(subcommand)]
    pub arg: Arg,
}

#[derive(Subcommand)]
pub enum Arg {
    Customer(Customer),
    Contract(Contract),
    Request(Request),
    Work(Work),
}

#[derive(Args)]
pub struct Customer {
    /// ID of the customer to show
    #[arg(long)]
    pub id: u32,
}
#[derive(Args)]
pub struct Contract {
    /// ID of the contract to show
    #[arg(long)]
    pub id: u32,
}
#[derive(Args)]
pub struct Request {
    /// ID of the request to show
    #[arg(long)]
    pub id: u32,
}
#[derive(Args)]
pub struct Work {
    /// ID of the work to show
    #[arg(long)]
    pub id: u32,
}
impl Cmd {
    pub fn run(&self, _ds: &DataStore) -> Result<(), Box<dyn std::error::Error>> {
        match &self.arg {
            Arg::Customer(_arg) => todo!(),
            Arg::Contract(_arg) => todo!(),
            Arg::Request(_arg) => todo!(),
            Arg::Work(_arg) => todo!(),
        }
    }
}
