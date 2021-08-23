use chrono::{DateTime, NaiveDateTime, Utc};

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    let new_time = start.timestamp() + 1_000_000_000;
    let time = NaiveDateTime::from_timestamp(new_time, 0);
    return DateTime::from_utc(time, Utc);
}
