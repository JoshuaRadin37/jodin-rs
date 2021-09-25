#![deny(rustdoc::broken_intra_doc_links)]
#![deny(unused_imports)]
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
pub mod symbols;
