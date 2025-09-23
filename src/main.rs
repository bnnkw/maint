use std::{env, path::PathBuf, str::FromStr};

use clap::{Args, Parser, Subcommand};
use edit::edit;
use rusqlite::Connection;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Request(RequestArgs),
    Work(WorkArgs),
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

#[allow(dead_code)]
struct Request {
    id: u32,
    contract_id: u32,
    description: String,
    request_date: chrono::NaiveDate,
}

impl TryFrom<&rusqlite::Row<'_>> for Request {
    type Error = rusqlite::Error;

    fn try_from(value: &rusqlite::Row<'_>) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.get(0)?,
            contract_id: value.get(1)?,
            description: value.get(2)?,
            request_date: chrono::NaiveDate::from_str(value.get::<_, String>(3)?.as_str()).unwrap(),
        })
    }
}

#[allow(dead_code)]
struct Work {
    id: u32,
    request_id: u32,
    worker: String,
    description: String,
    points_used: u32,
    work_date: chrono::NaiveDate,
}

impl TryFrom<&rusqlite::Row<'_>> for Work {
    type Error = rusqlite::Error;

    fn try_from(value: &rusqlite::Row<'_>) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.get(0)?,
            request_id: value.get(1)?,
            worker: value.get(2)?,
            description: value.get(3)?,
            points_used: value.get(4)?,
            work_date: chrono::NaiveDate::from_str(value.get::<_, String>(5)?.as_str()).unwrap(),
        })
    }
}

fn get_editor_description(initial_text: &str) -> Result<String, Box<dyn std::error::Error>> {
    let edited_text = edit(initial_text.as_bytes())?;
    Ok(edited_text)
}

fn db() -> PathBuf {
    let path = match env::var("MAINT_DB") {
        Ok(value) => value,
        Err(_) => match env::var("HOME") {
            Ok(value) => format!("{}/.maint.db", value),
            Err(_) => panic!("Failed to get home directory"),
        },
    };
    PathBuf::from(path)
}

fn main() {
    let cli = Cli::parse();
    let db = db();
    let conn = if !db.exists() {
        let conn = Connection::open(&db).unwrap();
        init(&conn).unwrap();
        conn
    } else {
        Connection::open(db).unwrap()
    };

    match cli.command {
        Command::Request(args) => match request(&conn, args) {
            Ok(nrow) => println!("{} row inserted", nrow),
            Err(e) => println!("Error inserting request: {}", e),
        },
        Command::Work(args) => match work(&conn, args) {
            Ok(nrow) => println!("{} row inserted", nrow),
            Err(e) => println!("Error inserting work: {}", e),
        },
    }
}

fn request(conn: &Connection, args: RequestArgs) -> Result<usize, rusqlite::Error> {
    let description = match args.description {
        Some(desc) => desc,
        None => get_editor_description("").expect("Failed to get description from editor"),
    };
    add_request(
        conn,
        &args.contract_id.to_string(),
        &description,
        &args.request_date.to_string(),
    )
}

fn add_request(
    conn: &Connection,
    contract_id: &str,
    description: &str,
    request_date: &str,
) -> Result<usize, rusqlite::Error> {
    conn.execute(
        "INSERT INTO request (contract_id, description, request_date)
            VALUES (:contract_id, :description, :request_date)",
        &[
            (":contract_id", contract_id),
            (":description", description),
            (":request_date", request_date),
        ],
    )
}

fn work(conn: &Connection, args: WorkArgs) -> Result<usize, rusqlite::Error> {
    let description = match args.description {
        Some(desc) => desc,
        None => get_editor_description("").expect("Failed to get description from editor"),
    };
    add_work(
        conn,
        &args.request_id.to_string(),
        &args.worker,
        &description,
        &args.points_used.to_string(),
        &args.work_date.to_string(),
    )
}

fn add_work(
    conn: &Connection,
    request_id: &str,
    worker: &str,
    description: &str,
    points_used: &str,
    work_date: &str,
) -> Result<usize, rusqlite::Error> {
    conn.execute(
        "INSERT INTO work (request_id, worker, description, points_used, work_date)
            VALUES (:request_id, :worker, :description, :points_used, :work_date)",
        &[
            (":request_id", request_id),
            (":worker", worker),
            (":description", description),
            (":points_used", points_used),
            (":work_date", work_date),
        ],
    )
}
#[allow(dead_code)]
fn list_request(conn: &Connection) -> Result<Vec<Request>, rusqlite::Error> {
    let query = "select * from request";
    let mut stmt = conn.prepare(query)?;
    let rows = stmt.query_map([], |row| Request::try_from(row))?;
    let mut requests = Vec::new();
    for request_result in rows {
        requests.push(request_result?);
    }

    Ok(requests)
}

#[allow(dead_code)]
fn list_work(conn: &Connection) -> Result<Vec<Work>, rusqlite::Error> {
    let query = "select * from work";
    let mut stmt = conn.prepare(query)?;
    let rows = stmt.query_map([], |row| Work::try_from(row))?;
    let mut work_entries = Vec::new();
    for work_result in rows {
        work_entries.push(work_result?);
    }

    Ok(work_entries)
}

fn init(conn: &Connection) -> Result<(), rusqlite::Error> {
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

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let conn = Connection::open_in_memory()
            .inspect(|c| init(c).unwrap())
            .unwrap();
        assert!(conn.table_exists(Some("main"), "customer").unwrap());
        assert!(conn.table_exists(Some("main"), "contract").unwrap());
        assert!(conn.table_exists(Some("main"), "request").unwrap());
        assert!(conn.table_exists(Some("main"), "work").unwrap());
    }

    #[test]
    fn test_request() {
        let conn = Connection::open_in_memory()
            .inspect(|c| init(c).unwrap())
            .unwrap();
        let args = RequestArgs {
            contract_id: 1,
            description: Some("desc".to_string()),
            request_date: chrono::Utc::now().date_naive(),
        };
        let nrow = request(&conn, args).unwrap();
        assert_eq!(1, nrow);
        let requests = list_request(&conn).unwrap();
        let request = &requests[0];
        assert_eq!(1, request.id);
        assert_eq!(1, request.contract_id);
        assert_eq!("desc", request.description);
        assert_eq!(chrono::Utc::now().date_naive(), request.request_date);
    }

    #[test]
    fn test_work() {
        let conn = Connection::open_in_memory()
            .inspect(|c| init(c).unwrap())
            .unwrap();
        let args = WorkArgs {
            request_id: 1,
            worker: "worker".to_string(),
            description: Some("desc".to_string()),
            points_used: 1,
            work_date: chrono::Utc::now().date_naive(),
        };
        let nrow = work(&conn, args).unwrap();
        assert_eq!(1, nrow);
        let work_entries = list_work(&conn).unwrap();
        let work_entry = &work_entries[0];
        assert_eq!(1, work_entry.id);
        assert_eq!(1, work_entry.request_id);
        assert_eq!("worker", work_entry.worker);
        assert_eq!("desc", work_entry.description);
        assert_eq!(1, work_entry.points_used);
        assert_eq!(chrono::Utc::now().date_naive(), work_entry.work_date);
    }
}
