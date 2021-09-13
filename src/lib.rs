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

#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(unused_imports)]
#![deny(unused_mut)]

#![cfg_attr(feature = "strict", deny(warnings))]


#[macro_use]
extern crate clap;

#[macro_use]
extern crate logos;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate lalrpop_util;

pub mod ast;
pub mod cli;
pub mod compilation;
pub mod compilation_settings;
pub mod core;
pub mod parsing;
pub mod passes;
pub mod utility;
