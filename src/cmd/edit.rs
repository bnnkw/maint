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
    /// ID of the customer to edit
    #[arg(long)]
    pub id: u32,

    /// New name for the customer
    #[arg(long)]
    pub name: Option<String>,
}
#[derive(Args)]
pub struct Contract {
    /// ID of the contract to edit
    #[arg(long)]
    pub id: u32,
    // Add other editable fields here as Options
}
#[derive(Args)]
pub struct Request {
    /// ID of the request to edit
    #[arg(long)]
    pub id: u32,
    // Add other editable fields here as Options
}
#[derive(Args)]
pub struct Work {
    /// ID of the work to edit
    #[arg(long)]
    pub id: u32,
    // Add other editable fields here as Options
}
