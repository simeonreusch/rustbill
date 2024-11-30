use std::{
    path::Path,
    error::Error,
    fs,
    fs::File,
};
use csv::ReaderBuilder;
use serde::{Deserialize};
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

    println!("found file {:?}", file);

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

pub fn find_all_companies(dir_path: &Path) -> Result<Vec<String>, Box<dyn Error>> {
    let entries = fs::read_dir(dir_path)?;

    let csv_files: Vec<String> = entries
        .filter_map(Result::ok)
        .filter_map(|entry| {
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("csv") {
                path.file_stem().and_then(|name| name.to_str()).map(|s| s.to_string())
            }
            else {
                None
            }
        })
        .collect();
    
    Ok(csv_files)
}