use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
pub struct Cmd {
    #[command(subcommand)]
    arg: Arg,
}

#[derive(Subcommand)]
enum Arg {
    /// Add a new customer
    Customer(Customer),
    /// Add a new contract
    Contract(Contract),
    /// Add a new request
    Request(Request),
    /// Add a new work log
    Work(Work),
}

#[derive(Args)]
struct Customer {
    /// Name of the customer
    #[arg(long)]
    pub name: String,
}

#[derive(Args)]
struct Contract {
    /// ID of the customer for this contract
    #[arg(long)]
    pub customer_id: u32,

    /// Start date of the contract (e.g., YYYY-MM-DD)
    #[arg(long)]
    pub start_date: String,

    /// Monthly amount for the contract
    #[arg(long)]
    pub amount: u32,
}

#[derive(Args)]
struct Request {
    /// ID of the contract for this request
    #[arg(long)]
    pub contract_id: u32,

    /// Detailed description of the request
    #[arg(long)]
    pub description: Option<String>,
}

#[derive(Args)]
struct Work {
    /// ID of the request for this work
    #[arg(long)]
    pub request_id: u32,

    /// Detailed description of the work performed
    #[arg(long)]
    pub description: Option<String>,
}

impl Cmd {
    pub fn run(&self) {
        match &self.arg {
            Arg::Customer(customer) => todo!(),
            Arg::Contract(contract) => todo!(),
            Arg::Request(request) => todo!(),
            Arg::Work(work) => todo!(),
        }
    }
}
