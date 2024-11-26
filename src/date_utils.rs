use bdays::HolidayCalendar;
use chrono::{Datelike, Duration, Local, NaiveDate};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DateError {
    #[error("No total amount could be computed")]
    CalculationError,
}

fn get_first_of_next_month(year: i32, month: u32) -> Result<NaiveDate, DateError> {
    let first_of_next_month; 

    if month == 12 {
        first_of_next_month = NaiveDate::from_ymd_opt(year+1, 1, 1);
    }
    else {
        first_of_next_month = NaiveDate::from_ymd_opt(year, month+1, 1);
    }
    first_of_next_month.ok_or(DateError::CalculationError)
}

pub fn parse_date_or_default(datestr: &str) -> Result<NaiveDate, Box<dyn std::error::Error>> {
    match NaiveDate::parse_from_str(datestr, "%Y-%m-%d") {
        Ok(date) => Ok(date),
        Err(_) => {
            println!("No date passed. Using the last day of the current month");
            let today = Local::now().naive_local();
            let (year, month) = (today.year(), today.month());

            let first_of_next_month = get_first_of_next_month(year, month)?;

            let last_day_of_current_month = first_of_next_month - Duration::days(1);

            Ok(last_day_of_current_month)
        }
    }
}

pub fn calculate_due_date(date: NaiveDate) -> Result<NaiveDate, Box<dyn std::error::Error>> {
    println!("Advancing {:?} 10 working days", date);

    let cal = bdays::calendars::de::GermanState::BE;
    let due_date  = cal.advance_bdays(date,  9);
    
    println!("Due date is {:?}", due_date);
    Ok(due_date)
}