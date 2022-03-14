//! The type-safe api for the compiler.

use crate::safe_api::error::CompilationError;
use jodin_common::ast::JodinNode;
use jodin_common::compilation::{Context, Target};
use std::error::Error;
use std::io;

pub mod error;
pub mod translation_object;
pub mod units;

pub trait CompileTo<T: Target> {
    type Output: EmitTo<T>;
    fn compile(self) -> Self::Output;
}

pub trait JodinCompilable<T> {
    type Error: Error;

    fn compile(self) -> Result<T, Self::Error>;
}

pub trait EmitTo<T: Target> {
    fn emit_to<W: io::Write>(&self, write_to: W) -> Result<(), CompilationError>;
}
