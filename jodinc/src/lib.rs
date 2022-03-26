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
#![deny(rustdoc::broken_intra_doc_links)]
#![allow(dead_code)]
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
extern crate log;

#[macro_use]
extern crate static_assertions;

use crate::error::CompilerError;
use crate::passes::analysis::analyze;
use crate::passes::optimization::optimize;
use jodin_common::ast::JodinNode;
use jodin_common::error::{JodinError, JodinResult};
use jodin_common::types::type_environment::TypeEnvironment;

pub mod cli;
pub mod compilation;
pub mod error;
pub mod error_reporting;
pub mod passes;
pub mod test_runner;

pub type Result<T> = std::result::Result<T, CompilerError>;

/// processes the jodin node tree
pub fn process_jodin_node(
    node: JodinNode,
) -> std::result::Result<(JodinNode, TypeEnvironment), JodinError> {
    let (analyzed, env) = analyze(node)?;
    let optimized = optimize(analyzed)?;
    Ok((optimized, env))
}
