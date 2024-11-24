use std::{
    path::Path,
    error::Error,
    fs::File,
};
use csv::ReaderBuilder;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Record {
    #[allow(unused)]
    pub date: String,
    pub minutes: i32,
    #[allow(unused)]
    pub description: String,
}


pub fn read_csv<P: AsRef<Path>>(path: P) -> Result<i32, Box<dyn Error>> {
    let file = File::open(path)?;

    let mut minutes_total= 0;
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b';')
        .from_reader(file);

    // Iterate over records and add to minutes_minutes total
    for result in rdr.deserialize() {
        let record: Record = result?;
        minutes_total += record.minutes;
    }

    Ok(minutes_total)
}