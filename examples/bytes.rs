#![allow(dead_code)]

use rtools::data::{from_bytes, to_bytes};

#[derive(Debug)]
struct Test {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
}

impl Default for Test {
    fn default() -> Self {
        Self {
            a: 11,
            b: 22,
            c: 33,
            d: 44,
        }
    }
}

fn main() {}
