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

#[macro_use]
extern crate jodin_asm_derive;

#[macro_use]
extern crate log;

use crate::core_traits::{ArithmeticsTrait, MemoryTrait, VirtualMachine};
use crate::function_names::{CALL, RECEIVE_MESSAGE};
use jodin_common::mvp::bytecode::{Asm, Assembly, Decode, GetAsm};
use jodin_common::mvp::location::AsmLocation;
use jodin_common::mvp::value::Value;
use std::cell::RefCell;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};

use std::io::{stderr, stdin, stdout, Read, Write};

pub mod core_traits;
pub mod error;
pub mod fault;
pub mod function_names;
pub mod mvp;
pub mod vm;
