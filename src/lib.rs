use std::fmt;
use std::path::Path;
use std::str::FromStr;

use chrono::NaiveDate;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Customer {
    #[serde(skip)]
    pub id: u32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Contract {
    #[serde(skip)]
    pub id: u32,
    pub customer_id: u32,
    pub start_date: chrono::NaiveDate,
    pub end_date: chrono::NaiveDate,
    pub total_points: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    #[serde(skip)]
    pub id: u32,
    pub contract_id: u32,
    pub description: String,
    pub request_date: chrono::NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Work {
    #[serde(skip)]
    pub id: u32,
    pub request_id: u32,
    pub worker: String,
    pub description: String,
    pub points_used: u32,
    pub work_date: chrono::NaiveDate,
}

#[derive(Debug, Serialize)]
pub struct CumulativeUsage {
    pub request_date: NaiveDate,
    pub request_description: String,
    pub worker: String,
    pub work_date: NaiveDate,
    pub work_description: String,
    pub points_used: u32,
    pub cumulative_points_used: u32,
}

impl TryFrom<&rusqlite::Row<'_>> for CumulativeUsage {
    type Error = rusqlite::Error;

    fn try_from(value: &rusqlite::Row<'_>) -> Result<Self, Self::Error> {
        Ok(Self {
            request_date: NaiveDate::from_str(value.get::<_, String>(0)?.as_str()).unwrap(),
            request_description: value.get(1)?,
            worker: value.get(2)?,
            work_date: NaiveDate::from_str(value.get::<_, String>(3)?.as_str()).unwrap(),
            work_description: value.get(4)?,
            points_used: value.get(5)?,
            cumulative_points_used: value.get(6)?,
        })
    }
}

impl fmt::Display for CumulativeUsage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_yaml::to_string(self).unwrap())
    }
}

#[derive(Debug, Serialize)]
pub struct ContractUsage {
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub total_points: u32,
    pub cumulative_usage: Vec<CumulativeUsage>,
}

impl fmt::Display for ContractUsage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_yaml::to_string(self).unwrap())
    }
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

impl fmt::Display for Customer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_yaml::to_string(self).unwrap())
    }
}

impl fmt::Display for Contract {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_yaml::to_string(self).unwrap())
    }
}

impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_yaml::to_string(self).unwrap())
    }
}

impl fmt::Display for Work {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_yaml::to_string(self).unwrap())
    }
}

impl FromStr for Customer {
    type Err = serde_yaml::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_yaml::from_str(s)
    }
}

impl FromStr for Contract {
    type Err = serde_yaml::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_yaml::from_str(s)
    }
}

impl FromStr for Request {
    type Err = serde_yaml::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_yaml::from_str(s)
    }
}

impl FromStr for Work {
    type Err = serde_yaml::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_yaml::from_str(s)
    }
}

#[derive(Debug)]
pub enum Error {
    RusqliteError(rusqlite::Error),
}

impl From<rusqlite::Error> for Error {
    fn from(value: rusqlite::Error) -> Self {
        Self::RusqliteError(value)
    }
}

impl std::error::Error for Error {}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{:?}", self)
    }
}

pub struct DataStore {
    conn: Connection,
}

impl DataStore {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let is_new_db = !path.as_ref().exists();
        let conn = Connection::open(path)?;

        if is_new_db {
            Self::init(&conn)?;
        }

        Ok(DataStore { conn })
    }

    pub fn add_customer(&self, name: &str) -> Result<usize, Error> {
        let rows = self.conn.execute(
            "INSERT INTO customer (name) VALUES (:name)",
            &[(":name", name)],
        )?;

        Ok(rows)
    }

    pub fn add_contract(
        &self,
        customer_id: u32,
        start_date: &NaiveDate,
        end_date: &NaiveDate,
        total_points: u32,
    ) -> Result<usize, Error> {
        let rows = self.conn.execute(
            "INSERT INTO contract (customer_id, start_date, end_date, total_points)
                VALUES (:customer_id, :start_date, :end_date, :total_points)",
            &[
                (":customer_id", &customer_id.to_string()),
                (":start_date", &start_date.to_string()),
                (":end_date", &end_date.to_string()),
                (":total_points", &total_points.to_string()),
            ],
        )?;

        Ok(rows)
    }

    pub fn add_request(
        &self,
        contract_id: u32,
        description: &str,
        request_date: &NaiveDate,
    ) -> Result<usize, Error> {
        let rows = self.conn.execute(
            "INSERT INTO request (contract_id, description, request_date)
                VALUES (:contract_id, :description, :request_date)",
            &[
                (":contract_id", contract_id.to_string().as_str()),
                (":description", description),
                (":request_date", request_date.to_string().as_str()),
            ],
        )?;

        Ok(rows)
    }

    pub fn add_work(
        &self,
        request_id: u32,
        worker: &str,
        description: &str,
        points_used: u32,
        work_date: &NaiveDate,
    ) -> Result<usize, Error> {
        let rows = self.conn.execute(
            "INSERT INTO work (request_id, worker, description, points_used, work_date)
                VALUES (:request_id, :worker, :description, :points_used, :work_date)",
            &[
                (":request_id", request_id.to_string().as_str()),
                (":worker", worker),
                (":description", description),
                (":points_used", points_used.to_string().as_str()),
                (":work_date", work_date.to_string().as_str()),
            ],
        )?;

        Ok(rows)
    }

    fn init(conn: &Connection) -> Result<(), Error> {
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

    pub fn list_customer(&self) -> Result<Vec<Customer>, rusqlite::Error> {
        let query = "select * from customer";
        let mut stmt = self.conn.prepare(query)?;
        let rows = stmt.query_map([], |row| Customer::try_from(row))?;
        let mut customers = Vec::new();
        for customer_result in rows {
            customers.push(customer_result?);
        }

        Ok(customers)
    }

    pub fn list_contract(&self) -> Result<Vec<Contract>, rusqlite::Error> {
        let query = "select * from contract";
        let mut stmt = self.conn.prepare(query)?;
        let rows = stmt.query_map([], |row| Contract::try_from(row))?;
        let mut contracts = Vec::new();
        for contract_result in rows {
            contracts.push(contract_result?);
        }

        Ok(contracts)
    }

    pub fn list_request(&self) -> Result<Vec<Request>, rusqlite::Error> {
        let query = "select * from request";
        let mut stmt = self.conn.prepare(query)?;
        let rows = stmt.query_map([], |row| Request::try_from(row))?;
        let mut requests = Vec::new();
        for request_result in rows {
            requests.push(request_result?);
        }

        Ok(requests)
    }

    pub fn list_work(&self) -> Result<Vec<Work>, rusqlite::Error> {
        let query = "select * from work";
        let mut stmt = self.conn.prepare(query)?;
        let rows = stmt.query_map([], |row| Work::try_from(row))?;
        let mut work_entries = Vec::new();
        for work_result in rows {
            work_entries.push(work_result?);
        }

        Ok(work_entries)
    }

    pub fn get_customer(&self, id: u32) -> Result<Customer, rusqlite::Error> {
        self.conn
            .query_one("select * from customer where id = :id", [id], |r| {
                Customer::try_from(r)
            })
    }

    pub fn get_contract(&self, id: u32) -> Result<Contract, rusqlite::Error> {
        self.conn
            .query_one("select * from contract where id = :id", [id], |r| {
                Contract::try_from(r)
            })
    }

    pub fn get_request(&self, id: u32) -> Result<Request, rusqlite::Error> {
        self.conn
            .query_one("select * from request where id = :id", [id], |r| {
                Request::try_from(r)
            })
    }

    pub fn get_work(&self, id: u32) -> Result<Work, rusqlite::Error> {
        self.conn
            .query_one("select * from work where id = :id", [id], |r| {
                Work::try_from(r)
            })
    }

    pub fn save_customer(&self, entity: Customer) -> Result<usize, rusqlite::Error> {
        let rows = self.conn.execute(
            "UPDATE customer SET name = :name WHERE id = :id",
            &[(":name", &entity.name), (":id", &entity.id.to_string())],
        )?;

        Ok(rows)
    }

    pub fn save_contract(&self, entity: Contract) -> Result<usize, rusqlite::Error> {
        let rows = self.conn.execute(
            "UPDATE contract SET
                customer_id = :customer_id,
                start_date = :start_date,
                end_date = :end_date,
                total_points = :total_points
            WHERE
                id = :id",
            &[
                (":customer_id", &entity.customer_id.to_string()),
                (":start_date", &entity.start_date.to_string()),
                (":end_date", &entity.end_date.to_string()),
                (":total_points", &entity.total_points.to_string()),
                (":id", &entity.id.to_string()),
            ],
        )?;

        Ok(rows)
    }

    pub fn save_request(&self, entity: Request) -> Result<usize, rusqlite::Error> {
        let rows = self.conn.execute(
            "UPDATE request SET
                contract_id = :contract_id,
                description = :description,
                request_date = :request_date
            WHERE
                id = :id",
            &[
                (":contract_id", &entity.contract_id.to_string()),
                (":description", &entity.description),
                (":request_date", &entity.request_date.to_string()),
                (":id", &entity.id.to_string()),
            ],
        )?;

        Ok(rows)
    }

    pub fn save_work(&self, entity: Work) -> Result<usize, rusqlite::Error> {
        let rows = self.conn.execute(
            "UPDATE work SET
                request_id = :request_id,
                worker = :worker,
                description = :description,
                points_used = :points_used,
                work_date = :work_date
            WHERE
                id = :id",
            &[
                (":request_id", &entity.request_id.to_string()),
                (":worker", &entity.worker),
                (":description", &entity.description),
                (":points_used", &entity.points_used.to_string()),
                (":work_date", &entity.work_date.to_string()),
                (":id", &entity.id.to_string()),
            ],
        )?;

        Ok(rows)
    }

    pub fn usage(&self, contract_id: u32) -> Result<ContractUsage, Error> {
        let contract = self.get_contract(contract_id)?;

        let mut stmt = self.conn.prepare(
            "
            SELECT
                request.request_date,
                request.description,
                work.worker,
                work.work_date,
                work.description,
                work.points_used,
                SUM(work.points_used)
                    OVER (
                        PARTITION BY contract.id
                        ORDER BY request.request_date, work.work_date
                        ROWS BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW
                    ) AS cumulative_points_used
            FROM
                work
                INNER JOIN request ON work.request_id = request.id
                INNER JOIN contract ON request.contract_id = contract.id
            WHERE
                contract.id = :id
                AND request.request_date BETWEEN contract.start_date AND contract.end_date
            ",
        )?;

        let rows = stmt.query_map([contract_id], |r| CumulativeUsage::try_from(r))?;
        let mut results = Vec::new();
        for r in rows {
            results.push(r?);
        }

        let contract_usage = ContractUsage {
            start_date: contract.start_date,
            end_date: contract.end_date,
            total_points: contract.total_points,
            cumulative_usage: results,
        };

        Ok(contract_usage)
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
            .add_contract(
                1,
                &"2025-01-01".parse().unwrap(),
                &"2025-12-31".parse().unwrap(),
                10,
            )
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
        let nrow = ds
            .add_request(1, "desc", &"2025-01-01".parse().unwrap())
            .unwrap();
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
            .add_work(1, "worker", "desc", 1, &"2025-01-01".parse().unwrap())
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

    #[test]
    fn test_cumulative_usage() {
        let ds = in_memory_datastore();
        ds.add_contract(
            1,
            &"2025-01-01".parse().unwrap(),
            &"2025-12-31".parse().unwrap(),
            12,
        )
        .unwrap();
        ds.add_customer("customer1").unwrap();
        ds.add_request(1, "req1", &"2025-01-01".parse().unwrap())
            .unwrap();
        ds.add_request(1, "req2", &"2025-12-31".parse().unwrap())
            .unwrap();
        ds.add_work(1, "alice", "work1", 1, &"2025-01-01".parse().unwrap())
            .unwrap();
        ds.add_work(1, "alice", "work1-2", 2, &"2025-01-10".parse().unwrap())
            .unwrap();
        ds.add_work(2, "alice", "work2", 3, &"2025-12-31".parse().unwrap())
            .unwrap();
        let v = ds.usage(1).unwrap();
        let v = v.cumulative_usage;
        assert_eq!(v.len(), 3);
        assert_eq!(v[0].cumulative_points_used, 1);
        assert_eq!(v[1].cumulative_points_used, 3);
        assert_eq!(v[2].cumulative_points_used, 6);
    }
}
