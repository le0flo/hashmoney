use chrono::{DateTime, Local, NaiveDateTime};

pub(super) fn current_naive_date_time() -> NaiveDateTime {
    return DateTime::parse_from_rfc3339(Local::now().to_rfc3339().as_str())
        .unwrap()
        .naive_local();
}
