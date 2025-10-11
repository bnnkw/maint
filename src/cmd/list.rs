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
pub struct Customer {}
#[derive(Args)]
pub struct Contract {}
#[derive(Args)]
pub struct Request {}
#[derive(Args)]
pub struct Work {}