use bdays::HolidayCalendar;
use chrono::{NaiveDate, Local, Datelike, Duration};

pub fn parse_date_or_default(datestr: &str) -> NaiveDate {
    match NaiveDate::parse_from_str(datestr, "%Y-%m-%d") {
        Ok(date) => date,
        Err(_) => {
            println!("No date passed. Using the last day of the current month");
            let today = Local::now().naive_local();
            let (year, month) = (today.year(), today.month());

            let first_of_next_month = 
                if month == 12 {
                        NaiveDate::from_ymd_opt(year+1, 1, 1).unwrap()
                    } 
                else {
                    NaiveDate::from_ymd_opt(year, month+1, 1).unwrap()
                    };

            let last_day_of_current_month = first_of_next_month - Duration::days(1);

            last_day_of_current_month
        }
    }
}

pub fn calculate_due_date(date: NaiveDate) -> NaiveDate {
    println!("Advancing {:?} 10 working days", date);

    let cal = bdays::calendars::de::GermanState::BE;
    let due_date  = cal.advance_bdays(date,  9);
    
    println!("Due date is {:?}", due_date);
    due_date
}