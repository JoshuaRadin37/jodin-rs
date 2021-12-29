pub mod asm_version;
pub mod ast;
pub mod compilation;
pub mod compilation_settings;
pub mod core;
pub mod error;
pub mod identifier;
pub mod mvp;
pub mod types;
pub mod unit;
pub mod utility;

#[macro_use]
extern crate log;

#[macro_use]
extern crate static_assertions;

#[macro_use]
extern crate num_derive;

#[macro_use]
extern crate serde_derive;

use simplelog::*;
use std::fs::File;
use std::sync::atomic::{AtomicBool, Ordering};

/// Initializes logging for the package
pub fn init_logging(level: LevelFilter) {
    static LOGGING_INIT: AtomicBool = AtomicBool::new(false);
    if let Ok(false) =
        LOGGING_INIT.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
    {
        let log_term_config = ConfigBuilder::new()
            .set_location_level(LevelFilter::Off)
            .set_target_level(LevelFilter::Off)
            .set_thread_level(LevelFilter::Off)
            .build();

        let log_file_config = ConfigBuilder::new()
            .set_thread_mode(ThreadLogMode::Names)
            .set_thread_level(LevelFilter::Error)
            .set_location_level(LevelFilter::Error)
            .set_target_level(LevelFilter::Off)
            .set_level_padding(LevelPadding::Right)
            .build();

        CombinedLogger::init(vec![
            TermLogger::new(
                level,
                log_term_config,
                TerminalMode::Mixed,
                ColorChoice::Auto,
            ),
            WriteLogger::new(
                level,
                log_file_config,
                File::create("compiler.log").expect("Could not open log file"),
            ),
        ])
        .expect("Could not create logger");
    }
}

pub fn default_logging() {
    if cfg!(debug_assertions) {
        init_logging(LevelFilter::Debug);
    } else if cfg!(release) {
        init_logging(LevelFilter::Info);
    }
}
