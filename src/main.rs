use std::path::Path;

use clap::{Args, Parser, Subcommand};
use rusqlite::Connection;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Request(RequestArgs),
}

#[derive(Args)]
struct RequestArgs {
    contract_id: u32,
    description: Option<String>,
    request_date: Option<chrono::NaiveDate>,
}

impl Default for RequestArgs {
    fn default() -> Self {
        RequestArgs {
            contract_id: 0,
            description: None,
            request_date: Some(chrono::Utc::now().date_naive()),
        }
    }
}

fn main() {
    println!("Hello, world!");
}

fn request(conn: &Connection, args: &RequestArgs) -> Result<usize, rusqlite::Error> {
    conn.execute(
        "INSERT INTO request (contract_id, description, request_date)
            VALUES (:contract_id, :description, :request_date)",
        &[
            (":contract_id", &args.contract_id.to_string()),
            (
                ":description",
                &"specified -d option or edited with external editor".to_string(),
            ),
            (":request_date", &args.request_date.unwrap().to_string()),
        ],
    )
}

fn init<P>(path: Option<P>) -> Result<Connection, rusqlite::Error>
where
    P: AsRef<Path>,
{
    let conn = match path {
        Some(p) => Connection::open(p),
        None => Connection::open_in_memory(),
    }?;
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS customer (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS contract (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            customer_id TEXT NOT NULL,
            start_date DATE NOT NULL,
            end_date DATE NOT NULL,
            total_points INTEGER NOT NULL,
            FOREIGN KEY (customer_id) REFERENCES customer(id)
        );
        CREATE TABLE IF NOT EXISTS request (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            contract_id INTEGER NOT NULL,
            description TEXT NOT NULL,
            request_date DATE NOT NULL,
            FOREIGN KEY (contract_id) REFERENCES contract(id)
        );
        CREATE TABLE IF NOT EXISTS work (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            request_id INTEGER NOT NULL,
            worker TEXT NOT NULL,
            description TEXT NOT NULL,
            points_used INTEGER NOT NULL,
            work_date DATE NOT NULL,
            FOREIGN KEY (request_id) REFERENCES request(id)
        );
        ",
    )?;
    Ok(conn)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let conn = init::<&str>(None).unwrap();
        assert!(conn.table_exists(Some("main"), "customer").unwrap());
        assert!(conn.table_exists(Some("main"), "contract").unwrap());
        assert!(conn.table_exists(Some("main"), "request").unwrap());
        assert!(conn.table_exists(Some("main"), "work").unwrap());
    }

    #[test]
    fn test_request() {
        let conn = init::<&str>(None).unwrap();
        let args = RequestArgs::default();
        let nrow = request(&conn, &args).unwrap();
        assert_eq!(1, nrow);
    }
}
