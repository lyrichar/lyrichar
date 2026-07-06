use chrono::{DateTime, Datelike, NaiveDate, Utc};

pub type Date = NaiveDate;

pub const BIRTH: Date = Date::from_ymd_opt(2026, 7, 1).unwrap();

pub fn age_from(birth: Date) -> i32 {
    let date = today();

    let mut age = date.year() - birth.year();

    if (date.month(), date.day()) < (birth.month(), birth.day()) {
        age -= 1;
    }

    age
}

pub fn age() -> i32 {
    age_from(BIRTH)
}

pub fn now() -> DateTime<Utc> {
    Utc::now()
}

pub fn today() -> Date {
    now().date_naive()
}

pub fn year() -> i32 {
    today().year()
}
