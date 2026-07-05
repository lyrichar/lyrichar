use chrono::{DateTime, Datelike, NaiveDate, Utc};

pub type Date = NaiveDate;

pub fn now() -> DateTime<Utc> {
    Utc::now()
}

pub fn today() -> Date {
    now().date_naive()
}

pub fn year() -> i32 {
    today().year()
}
