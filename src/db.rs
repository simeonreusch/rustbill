use std::path::Path;
use rusqlite::{Connection, Result, params};
use chrono::{NaiveDate, Datelike};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DBError{
    #[error("DB could not be created")]
    DBCreationError,
    #[error("DB connection error")]
    DBConnectionError(#[from] rusqlite::Error),
}

#[derive(Debug)]
struct DBEntry {
    // date: NaiveDate,
    id: i32,
    year: i32,
    month: u32,
    day: u32,
    company: String,
    bill_number: String,
    amount: f64,
    amount_str: String,
}

type DBResult<T> = Result<T, DBError>;

static DB_PATH_STR: &str = "db.sql";

fn get_connection() -> DBResult<Connection> {
    let db_path = Path::new(DB_PATH_STR);
    let conn = Connection::open(&db_path)?;

    Ok(conn)
}

pub fn create_db_if_needed() -> DBResult<()> {
    let db_path = Path::new(DB_PATH_STR);
    if db_path.exists() {
        println!("The database exists.")
    } else {
        println!("Creating new database");
        let conn = get_connection()?;

        conn.execute(
            "CREATE TABLE bill (
                id          INTEGER PRIMARY KEY,
                year        INTEGER,
                month       INTEGER,
                day         INTEGER,
                company     TEXT NOT NULL,
                bill_number TEXT NOT NULL,
                amount      FLOAT,
                amount_str  TEXT NOT NULL
            )",
            (),
        )?;
    }

    Ok(())
}

pub fn add_to_db(company: &str, billdate: &NaiveDate, bill_number: &str, amount: &f64, amount_str: &str) -> DBResult<()> {

    let new_entry = DBEntry {
        id: 0,
        year: billdate.year(),
        month: billdate.month(),
        day: billdate.day(),
        company: company.to_string(),
        bill_number: bill_number.to_string(),
        amount: amount.to_owned(),
        amount_str: amount_str.to_string(),
    };

    let conn = get_connection()?;

    let existing_entries = get_id_if_exists(company, billdate)?;

    if !existing_entries.is_empty() {
        for id in existing_entries {
            delete_entry_by_id(&id)?;
        }
    }

    conn.execute(
        "INSERT INTO bill (year, month, day, company, bill_number, amount, amount_str) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        (&new_entry.year, &new_entry.month, &new_entry.day, &new_entry.company, &new_entry.bill_number, &new_entry.amount, &new_entry.amount_str),
    )?;

    Ok(())
}

fn delete_entry_by_id(id: &i32) -> DBResult<()> {
    let conn = get_connection()?;

    println!("Delete existing DB entry with ID: {:?}", id);
    let delete_str = "DELETE FROM bill WHERE id = ?1".to_string();

    conn.execute(&delete_str, params![id])?;

    Ok(())
}

fn query_db(query_str: &str) -> DBResult<Vec<DBEntry>> {

    let conn = get_connection()?;

    let mut stmt = conn.prepare(
    query_str)?;

    let bill_iter = stmt.query_map([], |row| {
        let res_bill = DBEntry {
            id: row.get(0)?,
            year: row.get(1)?,
            month: row.get(2)?,
            day: row.get(3)?,
            company: row.get(4)?,
            bill_number: row.get(5)?,
            amount: row.get(6)?,
            amount_str: row.get(7)?,
        };
    Ok(res_bill)
    })?;

    let mut bills: Vec<DBEntry> = Vec::new();

    for bill in bill_iter {
        bills.push(bill.unwrap());
    }

    Ok(bills)
}

fn get_id_if_exists(company: &str, billdate: &NaiveDate) -> DBResult<Vec<i32>> {

    let query_str = format!("SELECT * FROM bill WHERE company == '{company}' AND month == '{month}'", company=company, month = billdate.month());

    let bills = query_db(&query_str)?;


    let extracted_ids: Vec<i32> = if !bills.is_empty() {
        bills.iter().map(|item| item.id).collect()
    } else {
        Vec::new()
    };

    println!("Found existing db entries with IDs: {:#?}", extracted_ids);

    Ok(extracted_ids)
}

pub fn print_all_db_entries() -> DBResult<()> {

    let query_str = "SELECT * FROM bill".to_string();

    let bills = query_db(&query_str)?;

    for bill in bills {
        println!("{:?}\n", bill);
    }

    Ok(())
}