extern crate simplelog;

use log::LevelFilter;
use simplelog::{
    ColorChoice, ConfigBuilder, LevelPadding, TermLogger, TerminalMode, ThreadLogMode,
};

pub fn init_log() {
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
}
