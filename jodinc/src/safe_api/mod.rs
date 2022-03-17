//! The type-safe api for the compiler.

use crate::safe_api::error::CompilationError;
use jodin_common::ast::JodinNode;
use jodin_common::compilation::{Context, Target};
use jodin_common::utility::ResultBox;
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

    fn compile_to(self) -> Result<T, Self::Error>;
}

pub trait JodinCompilableUtilities: Sized {
    type Error: Error;

    fn compile<T>(self) -> Result<T, <Self as JodinCompilableUtilities>::Error>
    where
        Self: JodinCompilable<T, Error = <Self as JodinCompilableUtilities>::Error>,
    {
        self.compile_to()
    }
    fn compile_boxed<T: Sized>(self) -> Result<Box<T>, <Self as JodinCompilableUtilities>::Error>
    where
        Self: JodinCompilable<T, Error = <Self as JodinCompilableUtilities>::Error>,
    {
        self.compile_to().boxed_ok()
    }
}

impl JodinCompilableUtilities for JodinNode {
    type Error = CompilationError;
}

pub trait EmitTo<T: Target> {
    fn emit_to<W: io::Write>(&self, write_to: W) -> Result<(), CompilationError>;
}
