use chrono::Utc;

use crate::IntoF32;

const SEC: f32 = 1_000.0;

#[derive(Debug)]
pub struct Animation {
    start:    f32,
    span:     f32,
    duration: f32,
    stamp:    i64,
}

impl Animation {
    pub fn new(start: impl IntoF32, end: impl IntoF32, duration: impl IntoF32) -> Self {
        assert!(end.into_f32() > start.into_f32());

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
        let duration = self.duration;
        let now = Utc::now().timestamp_millis();
        let delta = (now - self.stamp) as f32;
        let passed = (delta / duration) as u64 as f32;
        let delta = delta - (passed * duration);
        let ratio = delta / (self.duration);
        let span = self.span * ratio;
        (self.start + span) / SEC
    }
}
