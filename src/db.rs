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
    billnr_int: i32,
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
                amount_str  TEXT NOT NULL,
                billnr_int  INTEGER
            )",
            (),
        )?;
    }

    Ok(())
}

pub fn add_to_db(company: &str, billdate: &NaiveDate, bill_number: &str, amount: &f64, amount_str: &str, billnr_int: &i32) -> DBResult<()> {

    let new_entry = DBEntry {
        id: 0,
        year: billdate.year(),
        month: billdate.month(),
        day: billdate.day(),
        company: company.to_string(),
        bill_number: bill_number.to_string(),
        amount: amount.to_owned(),
        amount_str: amount_str.to_string(),
        billnr_int: billnr_int.clone(),
    };

    let conn = get_connection()?;

    let existing_entries = get_id_if_exists(company, billdate)?;

    if !existing_entries.is_empty() {
        for id in existing_entries {
            delete_entry_by_id(&id)?;
        }
    }

    conn.execute(
        "INSERT INTO bill (year, month, day, company, bill_number, amount, amount_str, billnr_int) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        (&new_entry.year, &new_entry.month, &new_entry.day, &new_entry.company, &new_entry.bill_number, &new_entry.amount, &new_entry.amount_str, &new_entry.billnr_int),
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
            billnr_int: row.get(8)?,
        };
    Ok(res_bill)
    })?;

    let mut bills: Vec<DBEntry> = Vec::new();

    for bill in bill_iter {
        bills.push(bill.unwrap());
    }

    Ok(bills)
}

fn query_for_company_in_month(company: &str, billdate: &NaiveDate) -> DBResult<Vec<DBEntry>> {
    let query_str = format!("SELECT * FROM bill WHERE company == '{company}' AND month == '{month}'", company=company, month = billdate.month());

    let bills = query_db(&query_str)?;

    Ok(bills)
}

fn query_for_month(billdate: &NaiveDate) -> DBResult<Vec<DBEntry>> {
    let query_str = format!("SELECT * FROM bill WHERE month == '{month}'", month = billdate.month());

    let bills = query_db(&query_str)?;

    Ok(bills)
}

fn get_id_if_exists(company: &str, billdate: &NaiveDate) -> DBResult<Vec<i32>> {

    let bills = query_for_company_in_month(company, billdate)?;

    let extracted_ids: Vec<i32> = if !bills.is_empty() {
        bills.iter().map(|item| item.id).collect()
    } else {
        Vec::new()
    };

    println!("Found existing db entries with IDs: {:#?}", extracted_ids);

    Ok(extracted_ids)
}

fn get_first_billnr(vec: &Vec<DBEntry>) -> Option<(String, i32)> {
    vec.get(0).map(|s| (s.bill_number.clone(), s.billnr_int))
}

fn get_all_billnr_ints(bills: &Vec<DBEntry>) -> Vec<i32> {
    let extracted_billnrs: Vec<i32> = if !bills.is_empty() {
        bills.iter().map(|item| item.billnr_int).collect()
    } else {
        Vec::new()
    };
    extracted_billnrs
}

pub fn get_billnr_if_exists(company: &str, billdate: &NaiveDate) -> DBResult<Option<(String, i32)>> {

    let bills = query_for_company_in_month(company, billdate)?;
    let (res) = get_first_billnr(&bills);

    match &res {
        Some(res) => println!("Found existing billnr: {:?}", res),
        None => println!("No existing billnr found")
     }

    Ok(res)
}

pub fn get_new_billnr(billdate: &NaiveDate, billnr_base: &str) -> DBResult<(String, i32)> {
    let bills = query_for_month(billdate)?;
    let billnrs = get_all_billnr_ints(&bills);

    let highest_int = match billnrs.iter().max() {
        Some(&max_value) => {
            let new_billnr_int = max_value + 1;
            new_billnr_int
        }
        None => {1},
    };

    let billnr = format!("{billnr_base}{billnr_int:02}",
        billnr_base = billnr_base,
        billnr_int = &highest_int,
    );
    println!("New billnumber: {:?}", billnr);
    Ok((billnr, highest_int))
}

pub fn print_all_db_entries() -> DBResult<()> {

    let query_str = "SELECT * FROM bill".to_string();

    let bills = query_db(&query_str)?;

    for bill in bills {
        println!("{:?}\n", bill);
    }

    Ok(())
}