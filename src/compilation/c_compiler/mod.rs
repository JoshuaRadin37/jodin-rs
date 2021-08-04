//! The C Compiler for Jodin

use crate::ast::JodinNode;
use crate::compilation::{Compilable, Compiler, Context, PaddedWriter, Target};
use crate::compilation_settings::CompilationSettings;
use crate::core::error::JodinResult;

mod c_type_compiler;
mod top_level_declaration_compiler;

mod components;
pub use components::*;

use std::fmt::Write;

/// The C99 target
pub struct C99;

impl Target for C99 {}

/// Compiles Jodin into C99 code.
pub struct C99Compiler {}

impl C99Compiler {
    /// Creates a new instance of the C99 compiler
    pub fn new() -> Self {
        C99Compiler {}
    }
}

impl Compiler<C99> for C99Compiler {
    fn compile(&mut self, tree: &JodinNode, _settings: &CompilationSettings) -> JodinResult<()> {
        let context = Context::new();

        Ok(())
    }
}

/// Something has has both a defintion and a declaration
pub trait SeparableCompilable {
    /// Compiles the instance declaration into a target writer
    fn declaration<W: Write>(&self, context: &Context, w: &mut PaddedWriter<W>) -> JodinResult<()>;

    /// Compiles the instance definition into a target writer
    fn definition<W: Write>(self, context: &Context, w: &mut PaddedWriter<W>) -> JodinResult<()>;
}

impl<SC: SeparableCompilable> Compilable<C99> for SC {
    fn compile<W: Write>(self, context: &Context, w: &mut PaddedWriter<W>) -> JodinResult<()> {
        self.declaration(context, w)?;
        writeln!(w)?;
        self.definition(context, w)
    }
}
