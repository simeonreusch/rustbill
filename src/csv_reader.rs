use std::{
    path::Path,
    error::Error,
    fs::File,
};
use csv::ReaderBuilder;
use serde::Deserialize;
use chrono::NaiveDate;
use chrono::Datelike;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Record {
    #[allow(unused)]
    pub date: String,
    pub minutes: i32,
    #[allow(unused)]
    pub description: String,
    pub year: Option<i32>,
    pub month: Option<u32>,
    pub day: Option<u32>,
}


pub fn get_ymd(datestr: &str) -> Result<(i32, u32, u32), Box<dyn Error>> {
    let date = NaiveDate::parse_from_str(datestr, "%d.%m.%Y")?;
    let (year, month, day) = (date.year(), date.month(), date.day());
    Ok((year, month, day))
}

pub fn read_csv<P: AsRef<Path>>(path: P) -> Result<Vec<Record>, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut records: Vec<Record> = Vec::new();
    let mut counts: i32 = 0;

    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b';')
        .from_reader(file);

    for result in rdr.deserialize() {
        let mut record: Record = result?;
        let datestr = &record.date;
        let ymd = get_ymd(datestr)?;
        record.year = Some(ymd.0);
        record.month = Some(ymd.1);
        record.day = Some(ymd.2);
        counts += 1;
        println!("{:?}", record);
        records.push(record);
    }
    println!("Found {} table entries", counts);
    Ok(records)
    }

pub fn extract_minutes_total(records: &Vec<Record>) -> Result<i32, Box<dyn Error>> {
    let mut minutes_total = 0;

    for record in records {
        minutes_total += record.minutes;
    }
    println!("Total minutes worked: {}", minutes_total);
    Ok(minutes_total)
}

