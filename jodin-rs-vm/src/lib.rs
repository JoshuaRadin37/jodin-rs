#![deny(rustdoc::broken_intra_doc_links)]
#![deny(unused_imports)]

//! The virtual machine for jodin-rs language.
//!
//! The virtual machine will be stack-based.

#[macro_use]
extern crate num_derive;

pub mod bytecode;
pub mod chunk;
pub mod frame;
pub mod memory;
pub mod vm;