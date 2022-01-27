#![cfg_attr(feature = "strict", deny(warnings))]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(unused_imports)]
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

//! The virtual machine for jodin-rs language.
//!
//! The virtual machine will be stack-based.

#[macro_use]
extern crate num_derive;

#[macro_use]
extern crate log;

use crate::core_traits::{ArithmeticsTrait, MemoryTrait, VirtualMachine};
use crate::function_names::{CALL, RECEIVE_MESSAGE};

pub mod core_traits;
pub use core_traits::*;
pub mod error;
pub mod fault;
pub mod function_names;
pub mod kernel;
pub mod loadables;
pub mod mvp;
pub mod scoped_memory;
pub mod vm;
