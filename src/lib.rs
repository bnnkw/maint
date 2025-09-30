use std::path::Path;
use std::str::FromStr;

use rusqlite::Connection;

#[allow(dead_code)]
struct Customer {
    id: u32,
    name: String,
}

#[allow(dead_code)]
struct Contract {
    id: u32,
    customer_id: u32,
    start_date: chrono::NaiveDate,
    end_date: chrono::NaiveDate,
    total_points: u32,
}

#[allow(dead_code)]
struct Request {
    id: u32,
    contract_id: u32,
    description: String,
    request_date: chrono::NaiveDate,
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

impl TryFrom<&rusqlite::Row<'_>> for Customer {
    type Error = rusqlite::Error;

    fn try_from(value: &rusqlite::Row<'_>) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.get(0)?,
            name: value.get(1)?,
        })
    }
}

impl TryFrom<&rusqlite::Row<'_>> for Contract {
    type Error = rusqlite::Error;

    fn try_from(value: &rusqlite::Row<'_>) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.get(0)?,
            customer_id: value.get(1)?,
            start_date: chrono::NaiveDate::from_str(value.get::<_, String>(2)?.as_str()).unwrap(),
            end_date: chrono::NaiveDate::from_str(value.get::<_, String>(3)?.as_str()).unwrap(),
            total_points: value.get(4)?,
        })
    }
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

pub struct DataStore {
    conn: Connection,
}

impl DataStore {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, rusqlite::Error> {
        let is_new_db = !path.as_ref().exists();
        let conn = Connection::open(path)?;

        if is_new_db {
            Self::init(&conn)?;
        }

        Ok(DataStore { conn })
    }

    pub fn add_customer(&self, name: &str) -> Result<usize, rusqlite::Error> {
        self.conn.execute(
            "INSERT INTO customer (name) VALUES (:name)",
            &[(":name", name)],
        )
    }

    pub fn add_contract(
        &self,
        customer_id: &str,
        start_date: &str,
        end_date: &str,
        total_points: &str,
    ) -> Result<usize, rusqlite::Error> {
        self.conn.execute(
            "INSERT INTO contract (customer_id, start_date, end_date, total_points)
                VALUES (:customer_id, :start_date, :end_date, :total_points)",
            &[
                (":customer_id", customer_id),
                (":start_date", start_date),
                (":end_date", end_date),
                (":total_points", total_points),
            ],
        )
    }

    pub fn add_request(
        &self,
        contract_id: &str,
        description: &str,
        request_date: &str,
    ) -> Result<usize, rusqlite::Error> {
        self.conn.execute(
            "INSERT INTO request (contract_id, description, request_date)
                VALUES (:contract_id, :description, :request_date)",
            &[
                (":contract_id", contract_id),
                (":description", description),
                (":request_date", request_date),
            ],
        )
    }

    pub fn add_work(
        &self,
        request_id: &str,
        worker: &str,
        description: &str,
        points_used: &str,
        work_date: &str,
    ) -> Result<usize, rusqlite::Error> {
        self.conn.execute(
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

    fn init(conn: &Connection) -> Result<(), rusqlite::Error> {
        conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS customer (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS contract (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                customer_id INTEGER NOT NULL,
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

    #[allow(dead_code)]
    fn list_customer(&self) -> Result<Vec<Customer>, rusqlite::Error> {
        let query = "select * from customer";
        let mut stmt = self.conn.prepare(query)?;
        let rows = stmt.query_map([], |row| Customer::try_from(row))?;
        let mut customers = Vec::new();
        for customer_result in rows {
            customers.push(customer_result?);
        }

        Ok(customers)
    }

    #[allow(dead_code)]
    fn list_contract(&self) -> Result<Vec<Contract>, rusqlite::Error> {
        let query = "select * from contract";
        let mut stmt = self.conn.prepare(query)?;
        let rows = stmt.query_map([], |row| Contract::try_from(row))?;
        let mut contracts = Vec::new();
        for contract_result in rows {
            contracts.push(contract_result?);
        }

        Ok(contracts)
    }

    #[allow(dead_code)]
    fn list_request(&self) -> Result<Vec<Request>, rusqlite::Error> {
        let query = "select * from request";
        let mut stmt = self.conn.prepare(query)?;
        let rows = stmt.query_map([], |row| Request::try_from(row))?;
        let mut requests = Vec::new();
        for request_result in rows {
            requests.push(request_result?);
        }

        Ok(requests)
    }

    #[allow(dead_code)]
    fn list_work(&self) -> Result<Vec<Work>, rusqlite::Error> {
        let query = "select * from work";
        let mut stmt = self.conn.prepare(query)?;
        let rows = stmt.query_map([], |row| Work::try_from(row))?;
        let mut work_entries = Vec::new();
        for work_result in rows {
            work_entries.push(work_result?);
        }

        Ok(work_entries)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn in_memory_datastore() -> DataStore {
        let conn = Connection::open_in_memory().unwrap();
        DataStore::init(&conn).unwrap();
        DataStore { conn }
    }

    #[test]
    fn test_init() {
        let ds = in_memory_datastore();
        assert!(ds.conn.table_exists(Some("main"), "customer").unwrap());
        assert!(ds.conn.table_exists(Some("main"), "contract").unwrap());
        assert!(ds.conn.table_exists(Some("main"), "request").unwrap());
        assert!(ds.conn.table_exists(Some("main"), "work").unwrap());
    }

    #[test]
    fn test_customer() {
        let ds = in_memory_datastore();
        let nrow = ds.add_customer("test customer").unwrap();
        assert_eq!(1, nrow);
        let customers = ds.list_customer().unwrap();
        let customer = &customers[0];
        assert_eq!(1, customer.id);
        assert_eq!("test customer", customer.name);
    }

    #[test]
    fn test_contract() {
        let ds = in_memory_datastore();
        let nrow = ds
            .add_contract("1", "2025-01-01", "2025-12-31", "10")
            .unwrap();
        assert_eq!(1, nrow);
        let contracts = ds.list_contract().unwrap();
        let contract = &contracts[0];
        assert_eq!(1, contract.id);
        assert_eq!(1, contract.customer_id);
        assert_eq!(
            chrono::NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            contract.start_date
        );
        assert_eq!(
            chrono::NaiveDate::from_ymd_opt(2025, 12, 31).unwrap(),
            contract.end_date
        );
        assert_eq!(10, contract.total_points);
    }

    #[test]
    fn test_request() {
        let ds = in_memory_datastore();
        let nrow = ds.add_request("1", "desc", "2025-01-01").unwrap();
        assert_eq!(1, nrow);
        let requests = ds.list_request().unwrap();
        let request = &requests[0];
        assert_eq!(1, request.id);
        assert_eq!(1, request.contract_id);
        assert_eq!("desc", request.description);
        assert_eq!(
            chrono::NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            request.request_date
        );
    }

    #[test]
    fn test_work() {
        let ds = in_memory_datastore();
        let nrow = ds
            .add_work("1", "worker", "desc", "1", "2025-01-01")
            .unwrap();
        assert_eq!(1, nrow);
        let work_entries = ds.list_work().unwrap();
        let work_entry = &work_entries[0];
        assert_eq!(1, work_entry.id);
        assert_eq!(1, work_entry.request_id);
        assert_eq!("worker", work_entry.worker);
        assert_eq!("desc", work_entry.description);
        assert_eq!(1, work_entry.points_used);
        assert_eq!(
            chrono::NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            work_entry.work_date
        );
    }
}
