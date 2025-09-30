use std::{env, path::PathBuf, process::ExitCode};

use clap::{Args, Parser, Subcommand};
use edit::edit;
use maint::DataStore;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Customer(CustomerArgs),
    Contract(ContractArgs),
    Request(RequestArgs),
    Work(WorkArgs),
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

fn get_editor_description(initial_text: &str) -> Result<String, Box<dyn std::error::Error>> {
    let edited_text = edit(initial_text.as_bytes())?;
    Ok(edited_text)
}

fn db_path() -> PathBuf {
    let path = match env::var("MAINT_DB") {
        Ok(value) => value,
        Err(_) => match env::var("HOME") {
            Ok(value) => format!("{}/.maint.db", value),
            Err(_) => panic!("Failed to get home directory"),
        },
    };
    PathBuf::from(path)
}

fn main() -> Result<ExitCode, rusqlite::Error> {
    let cli = Cli::parse();
    let db_path = db_path();
    let ds = DataStore::open(db_path).unwrap();

    match cli.command {
        Command::Customer(args) => customer(&ds, args)?,
        Command::Contract(args) => contract(&ds, args)?,
        Command::Request(args) => request(&ds, args)?,
        Command::Work(args) => work(&ds, args)?,
    };

    Ok(ExitCode::SUCCESS)
}

fn customer(ds: &DataStore, args: CustomerArgs) -> Result<usize, rusqlite::Error> {
    ds.add_customer(&args.name)
}

fn contract(ds: &DataStore, args: ContractArgs) -> Result<usize, rusqlite::Error> {
    ds.add_contract(
        &args.customer_id.to_string(),
        &args.start_date.to_string(),
        &args.end_date.to_string(),
        &args.total_points.to_string(),
    )
}

fn request(ds: &DataStore, args: RequestArgs) -> Result<usize, rusqlite::Error> {
    let description = match args.description {
        Some(desc) => desc,
        None => get_editor_description("").expect("Failed to get description from editor"),
    };
    ds.add_request(
        &args.contract_id.to_string(),
        &description,
        &args.request_date.to_string(),
    )
}

fn work(ds: &DataStore, args: WorkArgs) -> Result<usize, rusqlite::Error> {
    let description = match args.description {
        Some(desc) => desc,
        None => get_editor_description("").expect("Failed to get description from editor"),
    };
    ds.add_work(
        &args.request_id.to_string(),
        &args.worker,
        &description,
        &args.points_used.to_string(),
        &args.work_date.to_string(),
    )
}
