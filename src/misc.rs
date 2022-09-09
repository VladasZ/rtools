use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    thread,
    time::Duration,
};

use backtrace::Backtrace;

use crate::IntoF32;

pub fn backtrace() {
    let bt = Backtrace::new();
    println!("{:?}", bt);
    error!("{:?}", bt);
}

pub fn hash(obj: impl ToString + Hash) -> u64 {
    let mut hasher = DefaultHasher::new();
    obj.hash(&mut hasher);
    hasher.finish()
}

pub fn sleep(duration: impl IntoF32) {
    thread::sleep(Duration::from_nanos(
        (duration.into_f32() * 1000000000.0) as _,
    ));
}

pub trait Apply<T> {
    fn apply(self, action: impl FnMut(&mut T));
    fn apply2<U, Second: IntoIterator<Item = U>>(
        self,
        second: Second,
        action: impl FnMut(&mut T, U),
    );
}

impl<T, I: IntoIterator<Item = T>> Apply<T> for I {
    fn apply(self, mut action: impl FnMut(&mut T)) {
        for mut item in self.into_iter() {
            action(&mut item)
        }
    }

    fn apply2<U, Second: IntoIterator<Item = U>>(
        self,
        second: Second,
        mut action: impl FnMut(&mut T, U),
    ) {
        for (mut item, second) in self.into_iter().zip(second.into_iter()) {
            action(&mut item, second);
        }
    }
}
