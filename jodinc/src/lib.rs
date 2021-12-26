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

#[macro_use]
extern crate static_assertions;

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
pub mod error_reporting;
pub mod parsing;
pub mod passes;
pub mod test_runner;
pub mod utility;

/// processes the jodin node tree
pub fn process_jodin_node(mut node: JodinNode) -> Result<(JodinNode, TypeEnvironment), JodinError> {
    let (analyzed, env) = analyze(node)?;
    let optimized = optimize(analyzed)?;
    Ok((optimized, env))
}
