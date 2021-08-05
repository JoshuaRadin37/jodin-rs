//! The C Compiler for Jodin

use crate::ast::JodinNode;
use crate::compilation::{Compilable, Compiler, Context, MicroCompiler, PaddedWriter, Target};
use crate::compilation_settings::CompilationSettings;
use crate::core::error::JodinResult;

mod c_type_compiler;
mod top_level_declaration_compiler;

pub use c_type_compiler::*;
pub use top_level_declaration_compiler::TopLevelDeclarationCompiler;

mod components;
pub use components::*;

use std::fmt::Write;

/// The C99 target
pub struct C99;

impl Target for C99 {}

/// Compiles Jodin into C99 code.
pub struct C99Compiler<W: Write> {
    writer: PaddedWriter<W>,
}

impl<W: Write> C99Compiler<W> {
    /// Creates a new instance of the C99 compiler
    pub fn new(writer: W) -> Self {
        C99Compiler {
            writer: PaddedWriter::new(writer),
        }
    }
}

impl<W: Write> Compiler<C99> for C99Compiler<W> {
    fn compile(&mut self, tree: &JodinNode, _settings: &CompilationSettings) -> JodinResult<()> {
        let context = Context::new();

        let mut micro_compiler = TopLevelDeclarationCompiler;

        println!("Compiling {:?}", tree);
        let mut units = vec![];

        for top_level in tree {
            println!("Compiling Top Level: {:?}", top_level);
            let new_units = micro_compiler.create_compilable(top_level)?;
            units.extend(new_units);
        }

        for unit in &units {
            unit.declaration(&context, &mut self.writer)?;
        }

        for unit in units {
            unit.definition(&context, &mut self.writer)?;
        }

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
