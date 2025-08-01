use chrono::NaiveDate;
use chrono::Datelike;

pub fn middle_day(year: u32) -> Option<chrono::Weekday> {
    if year % 4 == 0 && year % 100 != 0 {
        return None;
    }
    Some(NaiveDate::from_yo_opt(year as i32, 183).unwrap().weekday())
}

// i just assumed i have to get the day number 183 because its the middle of the year