use clap::{Args, Parser, Subcommand};

mod add;
mod edit;
mod list;
mod rm;
mod show;

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

impl Cli {
    pub fn run(&self, ds: &maint::DataStore) -> Result<(), Box<dyn std::error::Error>> {
        match &self.command {
            Command::Add(cmd) => cmd.run(ds),
            Command::Rm(cmd) => cmd.run(ds),
            Command::List(cmd) => cmd.run(ds),
            Command::Show(cmd) => cmd.run(ds),
            Command::Edit(cmd) => cmd.run(ds),
        }?;

        Ok(())
    }
}

#[derive(Subcommand)]
pub enum Command {
    Add(add::Cmd),
    Rm(rm::Cmd),
    List(list::Cmd),
    Show(show::Cmd),
    Edit(edit::Cmd),
}

#[derive(Args)]
struct CustomerArgs {
    name: String,
}

#[derive(Args)]
struct ContractArgs {
    customer_id: u32,
    start_date: chrono::NaiveDate,
    end_date: chrono::NaiveDate,
    total_points: u32,
}

#[derive(Args)]
struct RequestArgs {
    contract_id: u32,

    #[arg(short, long)]
    description: Option<String>,

    #[arg(default_value_t = today_utc())]
    request_date: chrono::NaiveDate,
}

#[derive(Args)]
struct WorkArgs {
    request_id: u32,

    worker: String,

    #[arg(short, long)]
    description: Option<String>,

    #[arg(default_value = "1")]
    points_used: u32,

    #[arg(default_value_t = today_utc())]
    work_date: chrono::NaiveDate,
}

fn today_utc() -> chrono::NaiveDate {
    chrono::Utc::now().date_naive()
}
