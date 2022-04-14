use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    thread,
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

pub fn print_thread_name() {
    let thread = thread::current();
    let thread_name = thread::current().name().unwrap_or_default().to_string();
    dbg!(&thread_name);
    dbg!(thread.id());
    error!("thread name: {:?}, id: {:?}", thread_name, thread.id());
}
