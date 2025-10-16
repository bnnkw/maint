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
pub struct Customer {}
#[derive(Args)]
pub struct Contract {}
#[derive(Args)]
pub struct Request {}
#[derive(Args)]
pub struct Work {}

impl Cmd {
    pub fn run(&self, ds: &DataStore) -> Result<(), Box<dyn std::error::Error>> {
        match &self.arg {
            Arg::Customer(_arg) => println!("{:?}", ds.list_customer()?),
            Arg::Contract(_arg) => println!("{:?}", ds.list_contract()?),
            Arg::Request(_arg) => println!("{:?}", ds.list_request()?),
            Arg::Work(_arg) => println!("{:?}", ds.list_work()?),
        };

        Ok(())
    }
}
