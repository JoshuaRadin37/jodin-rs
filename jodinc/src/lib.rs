//! The library for the jodin compiler/interpreter.
//!
//! Jodin is an object-oriented programming language designed with the intent to be compiled into
//! C code.
//!
//! Code is compi
//!
//! # Example
//!
//! ```cpp
//! class Notify {
//!     virtual void notify();
//! }
//!
//! class HelloWorld : Notify {
//!     virtual void notify() {
//!         stdout::println("Hello, World!");
//!     }
//! }
//!
//! public void main(int argc, String[] argv) {
//!     Notify obj = new HelloWorld();
//!     obj->notify();
//! }
//!
//! ```

#![cfg_attr(feature = "strict", deny(warnings))]
#![warn(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
#![warn(
    bad_style,
    const_err,
    dead_code,
    improper_ctypes,
    non_shorthand_field_patterns,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    private_in_public,
    unconditional_recursion,
    unused,
    unused_allocation,
    unused_comparisons,
    unused_parens,
    while_true
)]

#[macro_use]
extern crate clap;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate lalrpop_util;

#[macro_use]
extern crate log;

use crate::ast::JodinNode;
use crate::core::error::{JodinError, JodinResult};
use crate::core::types::type_environment::TypeEnvironment;
use crate::passes::analysis::analyze;
use crate::passes::optimization::optimize;
use std::fs::File;

#[macro_export]
macro_rules! id {
    ($first:ident $($sep:tt $next:ident)*) => {
        $crate::core::identifier::Identifier::from_iter([stringify!($first), $(stringify!($next)),*])
    };
    ($first:literal $($sep:tt $next:literal)*) => {
        {
            use $crate::core::identifier::Identifier;
            Identifier::from_iter(&[Identifier::from($first), $(Identifier::from($next)),*])
        }
    };
    ($first:expr $(,$next:expr)*) => {
        {
            use $crate::core::identifier::Identifier;
            Identifier::from_iter([Identifier::from($first), $(Identifier::from($next)),*])
        }
    };
}

pub mod ast;
pub mod cli;
pub mod compilation;
pub mod compilation_settings;
pub mod core;
pub mod parsing;
pub mod passes;
pub mod utility;

use simplelog::*;

/// Initializes logging for the package
pub fn init_logging(level: LevelFilter) {
    let log_term_config = ConfigBuilder::new()
        .set_thread_mode(ThreadLogMode::Names)
        .set_location_level(LevelFilter::Off)
        .set_target_level(LevelFilter::Off)
        .build();

    let log_file_config = ConfigBuilder::new()
        .set_thread_mode(ThreadLogMode::Names)
        .set_thread_padding(ThreadPadding::Right(15))
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

pub fn default_logging() {
    if cfg!(debug_assertions) {
        init_logging(LevelFilter::Debug);
    } else if cfg!(release) {
        init_logging(LevelFilter::Info);
    }
}

/// processes the jodin node tree
pub fn process_jodin_node(mut node: JodinNode) -> Result<(JodinNode, TypeEnvironment), JodinError> {
    let (analyzed, env) = analyze(node)?;
    let optimized = optimize(analyzed)?;
    Ok((optimized, env))
}
