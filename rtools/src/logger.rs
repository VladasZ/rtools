#![allow(clippy::mismatched_target_os)]

#[cfg(android)]
#[mutants::skip]
pub fn init_log(_builder: LogBuilder) {
    use android_logger::{Config, FilterBuilder};
    use log::Level;

    android_logger::init_once(
        Config::default()
            .with_max_level(LevelFilter::Error)
            .with_tag("test_engine")
            .with_filter(FilterBuilder::new().parse("debug,hello::crate=error").build()),
    );

    trace!("Android logger: OK");
}
