//! The common package for the jodin-rs project. Contains _common_ structures and traits that
//! should be present in all sub-packages

#![cfg_attr(feature = "strict", deny(warnings))]
#![warn(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
#![allow(dead_code)]

/// A shorthand way of creating a identifiers. Can either use identifiers or literals seperated by any token,
/// token, or an expression seperated by a comma
#[macro_export]
macro_rules! id {
    ($first:ident $($sep:tt $next:ident)*) => {
        $crate::identifier::Identifier::from_iter([stringify!($first), $(stringify!($next)),*])
    };
    ($first:literal $($sep:tt $next:literal)*) => {
        {
            use $crate::identifier::Identifier;
            Identifier::from_iter(&[Identifier::from($first), $(Identifier::from($next)),*])
        }
    };
    ($first:expr $(,$next:expr)*) => {
        {
            use $crate::identifier::Identifier;
            Identifier::from_iter([Identifier::from($first), $(Identifier::from($next)),*])
        }
    };
}

pub mod asm_version;
pub mod assembly;
pub mod ast;
pub mod compilation;
pub mod compilation_settings;
pub mod core;
pub mod error;
pub mod identifier;
pub mod parsing;
pub mod types;
pub mod unit;
pub mod utility;

#[macro_use]
extern crate log;

#[macro_use]
extern crate static_assertions;

#[macro_use]
extern crate serde_derive;

pub use simplelog::*;
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

/// Sets the default logging options for given configurations
pub fn default_logging() {
    if cfg!(debug_assertions) {
        init_logging(LevelFilter::Debug);
    } else if cfg!(release) {
        init_logging(LevelFilter::Warn);
    } else {
        init_logging(LevelFilter::Info);
    }
}
