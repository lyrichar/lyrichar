pub use chrono::{Datelike, NaiveDate, Utc};

pub type Date = NaiveDate;

pub fn today() -> Date {
    Utc::now().date_naive()
}

pub fn year() -> i32 {
    today().year()
}
