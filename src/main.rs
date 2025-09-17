use std::path::Path;

use rusqlite::Connection;

fn main() {
    println!("Hello, world!");
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
}
