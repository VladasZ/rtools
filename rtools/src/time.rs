use chrono::Utc;

pub struct Time {}

impl Time {
    pub fn now() -> i64 {
        Utc::now().timestamp_micros()
    }
}
