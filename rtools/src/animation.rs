use chrono::Utc;

use crate::IntoF32;

const SEC: f32 = 1_000.0;

#[derive(Default, Debug)]
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

    pub fn finished(&self) -> bool {
        Utc::now().timestamp_millis() >= self.stamp + self.duration as i64
    }

    pub fn value(&self) -> f32 {
        debug_assert!(self.span != 0.0);
        if self.finished() {
            return (self.start + self.span) / SEC;
        }
        let now = Utc::now().timestamp_millis();
        let delta = (now - self.stamp) as f32;
        let passed = (delta / self.duration) as u64;
        let even = passed % 2 == 0;
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

#[cfg(test)]
mod test {
    use crate::{sleep, Animation};

    #[test]
    fn test() {
        let anim = Animation::new(0, 1, 0.5);

        assert_eq!(anim.finished(), false);
        assert_eq!(anim.value(), 0.0);

        sleep(0.25);

        assert_eq!(anim.finished(), false);
        assert!(anim.value() >= 0.5 && anim.value() <= 0.51);

        sleep(0.10);

        assert_eq!(anim.finished(), false);
        assert!(anim.value() >= 0.71 && anim.value() <= 0.72);

        sleep(0.15);

        assert_eq!(anim.finished(), true);
        assert_eq!(anim.value(), 1.0);
    }
}
