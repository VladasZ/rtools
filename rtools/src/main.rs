#![allow(unexpected_cfgs)]

use std::collections::HashMap;

use lazy_static::lazy_static;

lazy_static! {
    static ref HASHMAP: HashMap<u32, &'static str> = {
        let mut m = HashMap::new();
        m.insert(0, "foo");
        m.insert(1, "bar");
        m.insert(2, "baz");
        m
    };
}

#[mutants::skip]
fn main() {}
