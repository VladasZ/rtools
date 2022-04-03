use chrono::Utc;
use num_integer::Integer;

use crate::IntoF32;

const SEC: f32 = 1_000.0;

#[derive(Debug, Default)]
pub struct Animation {
    start:    f32,
    span:     f32,
    duration: f32,
    stamp:    i64,
}

impl Animation {
    pub fn new(start: impl IntoF32, end: impl IntoF32, duration: impl IntoF32) -> Self {
        let start = start.into_f32() * SEC;
        let end = end.into_f32() * SEC;
        Self {
            start,
            span: end - start,
            duration: duration.into_f32() * SEC,
            stamp: Utc::now().timestamp_millis(),
        }
    }

    pub fn value(&self) -> f32 {
        debug_assert!(self.span != 0.0);
        let now = Utc::now().timestamp_millis();
        let delta = (now - self.stamp) as f32;
        let passed = (delta / self.duration) as u64;
        let even = passed.is_even();
        let passed = passed as f32;
        let delta = delta - (passed * self.duration);
        let ratio = delta / (self.duration);
        let span = if even {
            self.span * ratio
        } else {
            self.span - self.span * ratio
        };
        (self.start + span) / SEC
    }
}
