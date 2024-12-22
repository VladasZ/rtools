use chrono::Utc;

const SEC: f32 = 1_000.0;

#[derive(Default, Debug)]
pub struct Animation {
    start:    f32,
    span:     f32,
    duration: f32,
    stamp:    i64,
}

impl Animation {
    pub fn new(start: impl Into<f32>, end: impl Into<f32>, duration: impl Into<f32>) -> Self {
        let start = start.into() * SEC;
        let end = end.into() * SEC;
        let span = end - start;
        assert!(span != 0.0);
        Self {
            start,
            span,
            duration: duration.into() * SEC,
            stamp: Utc::now().timestamp_millis(),
        }
    }

    #[allow(clippy::cast_possible_truncation)]
    pub fn finished(&self) -> bool {
        Utc::now().timestamp_millis() >= self.stamp + self.duration as i64
    }

    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_precision_loss)]
    pub fn value(&self) -> f32 {
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
        let anim = Animation::new(0.0, 1.0, 0.5);

        assert_eq!(anim.finished(), false);
        assert_eq!(anim.value(), 0.0);

        sleep(0.25);

        assert_eq!(anim.finished(), false);
        assert!(anim.value() >= 0.48 && anim.value() <= 0.52);

        sleep(0.10);

        assert_eq!(anim.finished(), false);
        assert!(anim.value() >= 0.70 && anim.value() <= 0.725);

        sleep(0.15);

        assert_eq!(anim.finished(), true);
        assert!(anim.value() >= 0.96 && anim.value() <= 1.04);

        sleep(0.25);

        assert_eq!(anim.finished(), true);
        assert!(anim.value() >= 0.40 && anim.value() <= 0.60);
    }
}
