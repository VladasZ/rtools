#![allow(clippy::mismatched_target_os)]

use log::LevelFilter;
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
pub struct LogBuilder {
    #[builder(default)]
    location:      bool,
    #[builder(default)]
    threads:       bool,
    #[builder(default)]
    target:        bool,
    #[builder(default = 4)]
    level:         usize,
    #[builder(default, setter(strip_option))]
    allow_filter:  Option<&'static str>,
    #[builder(default, setter(strip_option))]
    ignore_filter: Option<&'static str>,
}

#[cfg(not(android))]
#[mutants::skip]
pub fn init_log(builder: LogBuilder) {
    extern crate simplelog;

    use simplelog::{ColorChoice, ConfigBuilder, LevelPadding, TermLogger, TerminalMode, ThreadLogMode};

    let mut config = ConfigBuilder::new();

    config
        .set_time_level(LevelFilter::Off)
        .set_level_padding(LevelPadding::Right)
        .set_thread_level(if builder.threads {
            LevelFilter::Error
        } else {
            LevelFilter::Off
        })
        .set_thread_mode(ThreadLogMode::Both)
        .set_location_level(if builder.location {
            LevelFilter::Error
        } else {
            LevelFilter::Off
        })
        .set_target_level(if builder.target {
            LevelFilter::Error
        } else {
            LevelFilter::Off
        });

    if let Some(filter) = builder.allow_filter {
        config.add_filter_allow_str(filter);
    }

    if let Some(filter) = builder.ignore_filter {
        config.add_filter_ignore_str(filter);
    }

    let config = config.build();

    let res = TermLogger::init(
        from_usize(builder.level),
        config,
        TerminalMode::Mixed,
        ColorChoice::Auto,
    );
    trace!("Logger: {:?}", res);
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
