#![allow(clippy::mismatched_target_os)]

use log::LevelFilter;

#[cfg(not(android))]
pub fn init_log(location: bool, level: usize) {
    extern crate simplelog;

    use simplelog::{
        ColorChoice, ConfigBuilder, LevelPadding, TermLogger, TerminalMode, ThreadLogMode,
    };

    TermLogger::init(
        from_usize(level),
        ConfigBuilder::new()
            .set_time_level(LevelFilter::Off)
            .set_level_padding(LevelPadding::Right)
            .set_thread_level(LevelFilter::Off)
            .set_thread_mode(ThreadLogMode::Both)
            .set_location_level(if location {
                LevelFilter::Error
            } else {
                LevelFilter::Off
            })
            .set_target_level(LevelFilter::Off)
            .build(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .expect("Failed to initialize logger");
    trace!("Logger: OK");
}

fn from_usize(u: usize) -> LevelFilter {
    match u {
        0 => LevelFilter::Off,
        1 => LevelFilter::Error,
        2 => LevelFilter::Warn,
        3 => LevelFilter::Info,
        4 => LevelFilter::Debug,
        5 => LevelFilter::Trace,
        _ => panic!("Wrong log level. 5 is max"),
    }
}

#[cfg(android)]
pub fn init_log() {
    use android_logger::{Config, FilterBuilder};
    use log::Level;

    android_logger::init_once(
        Config::default()
            .with_min_level(Level::Trace)
            .with_tag("test_engine")
            .with_filter(
                FilterBuilder::new()
                    .parse("debug,hello::crate=error")
                    .build(),
            ),
    );

    trace!("Android logger: OK");
}
