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
    pub id: u32,
}

#[derive(Args)]
pub struct Contract {
    /// ID of the contract to show
    pub id: u32,

    #[arg(short, long)]
    usage: bool,
}

#[derive(Args)]
pub struct Request {
    /// ID of the request to show
    pub id: u32,
}

#[derive(Args)]
pub struct Work {
    /// ID of the work to show
    pub id: u32,
}

impl Cmd {
    pub fn run(&self, ds: &DataStore) -> Result<(), Box<dyn std::error::Error>> {
        match &self.arg {
            Arg::Customer(arg) => println!("{:?}", ds.get_customer(arg.id)?),
            Arg::Contract(arg) => {
                if arg.usage {
                    println!("{}", ds.usage(arg.id)?);
                } else {
                    println!("{:?}", ds.get_contract(arg.id)?);
                }
            }
            Arg::Request(arg) => println!("{:?}", ds.get_request(arg.id)?),
            Arg::Work(arg) => println!("{:?}", ds.get_work(arg.id)?),
        };

        Ok(())
    }
}
