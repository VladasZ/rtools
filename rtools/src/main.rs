#![feature(linkage)]
#![allow(clippy::mismatched_target_os)]

use std::collections::HashMap;

use lazy_static::lazy_static;
use log::{debug, trace, warn};
use rtools::{init_log, LogBuilder};
use simplelog::error;

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
fn main() {
    init_log(
        LogBuilder::builder()
            .level(5)
            .allow_filter("rtools")
            .target(true)
            .threads(true)
            .location(true)
            .build(),
    );
    trace!("Fargel!");
    debug!("Fargel!");
    warn!("Fargel!");
    error!("Fargel!");
}
