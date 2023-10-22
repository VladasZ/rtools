use chrono::Utc;

pub struct Passed {
    stamp: i64,
}

impl Passed {
    pub fn passed(&mut self) -> i64 {
        let now = Utc::now().timestamp_millis();
        let passed = now - self.stamp;
        self.stamp = now;
        passed
    }
}

impl Default for Passed {
    fn default() -> Self {
        Self {
            stamp: Utc::now().timestamp_millis(),
        }
    }
}
