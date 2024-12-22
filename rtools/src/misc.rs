use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    thread,
    time::Duration,
};

pub fn hash(obj: impl ToString + Hash) -> u64 {
    let mut hasher = DefaultHasher::new();
    obj.hash(&mut hasher);
    hasher.finish()
}

pub fn sleep(duration: impl Into<f32>) {
    let duration = duration.into();
    assert!(duration >= 0.0);

    if duration == 0.0 {
        return;
    }

    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    thread::sleep(Duration::from_micros((duration * 1_000_000.0) as _));
}

pub trait Toggle {
    fn toggle(&mut self) -> bool;
}

/// Returns old value
impl Toggle for bool {
    fn toggle(&mut self) -> bool {
        *self = !*self;
        !*self
    }
}

#[cfg(test)]
mod test {

    use crate::{hash, passed::Passed, random::Random, sleep, Toggle};

    #[test]
    fn toggle() {
        let mut val = bool::random();

        for _ in 0..10 {
            let prev = val;
            assert_eq!(val.toggle(), prev);
            assert_eq!(val, !prev);
        }
    }

    #[test]
    fn misc() {
        assert_eq!(397663693774528972, hash(54325));
        assert_eq!(13713634450856707654, hash(8657865));
        assert_eq!(5949921715258702887, hash(87));
    }

    #[test]
    fn test_sleep() {
        let mut pass = Passed::default();

        sleep(0.2);
        let passed = pass.passed();
        assert!(passed >= 200);
        assert!(passed < 220);
    }
}
