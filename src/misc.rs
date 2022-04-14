use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use backtrace::Backtrace;

pub fn backtrace() {
    let bt = Backtrace::new();
    println!("{:?}", bt);
    error!("{:?}", bt);
}

pub fn hash(obj: impl Hash) -> u64 {
    let mut hasher = DefaultHasher::new();
    obj.hash(&mut hasher);
    hasher.finish()
}
