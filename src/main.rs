use clap::Parser;
use std::{env, path::PathBuf, process::ExitCode};

use cmd::Cli;
use edit::edit;
use maint::DataStore;

pub mod cmd;

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

fn main() -> Result<ExitCode, maint::Error> {
    let db_path = db_path();
    let ds = DataStore::open(db_path).unwrap();

    let cli = Cli::parse();
    cli.run()?;

    Ok(ExitCode::SUCCESS)
}

// fn customer(ds: &DataStore, args: CustomerArgs) -> Result<usize, maint::Error> {
//     ds.add_customer(&args.name)
// }

// fn contract(ds: &DataStore, args: ContractArgs) -> Result<usize, maint::Error> {
//     ds.add_contract(
//         &args.customer_id.to_string(),
//         &args.start_date.to_string(),
//         &args.end_date.to_string(),
//         &args.total_points.to_string(),
//     )
// }

// fn request(ds: &DataStore, args: RequestArgs) -> Result<usize, maint::Error> {
//     let description = match args.description {
//         Some(desc) => desc,
//         None => get_editor_description("").expect("Failed to get description from editor"),
//     };
//     ds.add_request(
//         &args.contract_id.to_string(),
//         &description,
//         &args.request_date.to_string(),
//     )
// }

// fn work(ds: &DataStore, args: WorkArgs) -> Result<usize, maint::Error> {
//     let description = match args.description {
//         Some(desc) => desc,
//         None => get_editor_description("").expect("Failed to get description from editor"),
//     };
//     ds.add_work(
//         &args.request_id.to_string(),
//         &args.worker,
//         &description,
//         &args.points_used.to_string(),
//         &args.work_date.to_string(),
//     )
// }
