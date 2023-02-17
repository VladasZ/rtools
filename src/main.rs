#![allow(clippy::mismatched_target_os)]

use log::{debug, trace, warn};
use rtools::{init_log, LogBuilder};
use simplelog::error;

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
