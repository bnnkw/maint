use clap::{Args, Parser, Subcommand};
use maint::DataStore;

use crate::cmd::today_utc;

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
    start_date: chrono::NaiveDate,

    /// End date of the contract (e.g., YYYY-MM-DD)
    #[arg(long)]
    end_date: chrono::NaiveDate,

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

    #[arg(default_value_t = today_utc())]
    request_date: chrono::NaiveDate,
}

#[derive(Args)]
struct Work {
    /// ID of the request for this work
    #[arg(long)]
    pub request_id: u32,

    worker: String,

    /// Detailed description of the work performed
    #[arg(long)]
    pub description: Option<String>,

    #[arg(default_value = "1")]
    points_used: u32,

    #[arg(default_value_t = today_utc())]
    work_date: chrono::NaiveDate,
}

impl Cmd {
    pub fn run(&self, ds: &DataStore) -> Result<(), Box<dyn std::error::Error>> {
        match &self.arg {
            Arg::Customer(arg) => ds.add_customer(&arg.name)?,
            Arg::Contract(arg) => ds.add_contract(
                arg.customer_id,
                &arg.start_date,
                &arg.start_date,
                arg.amount,
            )?,
            Arg::Request(arg) => {
                let description = match arg.description {
                    Some(ref description) => description,
                    None => &crate::get_editor_description("")?,
                };
                ds.add_request(arg.contract_id, description, &arg.request_date)?
            }
            Arg::Work(arg) => {
                let description = match arg.description {
                    Some(ref description) => description,
                    None => &crate::get_editor_description("")?,
                };
                ds.add_work(
                    arg.request_id,
                    &arg.worker,
                    description,
                    arg.points_used,
                    &arg.work_date,
                )?
            }
        };

        Ok(())
    }
}
