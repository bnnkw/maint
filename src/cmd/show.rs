use clap::{Args, Parser, Subcommand};

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