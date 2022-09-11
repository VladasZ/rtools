#[cfg(not(android))]
pub fn init_log() {
    use log::LevelFilter;
    extern crate simplelog;

    use simplelog::{
        ColorChoice, ConfigBuilder, LevelPadding, TermLogger, TerminalMode, ThreadLogMode,
    };

    TermLogger::init(
        LevelFilter::Info,
        ConfigBuilder::new()
            .set_time_level(LevelFilter::Off)
            .set_level_padding(LevelPadding::Right)
            .set_thread_level(LevelFilter::Off)
            .set_thread_mode(ThreadLogMode::Both)
            .set_location_level(LevelFilter::Off)
            .set_target_level(LevelFilter::Off)
            .build(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .expect("Failed to initialize logger");
    trace!("Logger: OK");
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
