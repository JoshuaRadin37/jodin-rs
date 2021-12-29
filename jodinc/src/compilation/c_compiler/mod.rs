//! The C Compiler for Jodin

use jodin_common::ast::JodinNode;
use jodin_common::compilation_settings::CompilationSettings;
use jodin_common::error::JodinResult;

mod c_type_compiler;
mod function_compiler;
mod statement_compiler;
mod top_level_declaration_compiler;

pub use c_type_compiler::*;
pub use function_compiler::{FunctionCompiler, MethodCompiler};
pub use statement_compiler::StatementCompiler;
pub use top_level_declaration_compiler::TopLevelDeclarationCompiler;

mod components;
pub mod dependency_graph;

pub use components::*;

use jodin_common::compilation::{
    Compilable, Compiler, Context, MicroCompiler, PaddedWriter, Target,
};
use std::collections::HashMap;
use std::convert::TryFrom;
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

        // println!("Compiling {:?}", tree);
        let mut dependency_graph = DependencyGraph::new();
        let mut id_to_node = HashMap::new();

        for top_level in tree {
            // println!("Compiling Top Level: {:?}", top_level);
            if let Ok(dependency_info) = DependencyInfo::try_from(top_level) {
                let new_units = micro_compiler.create_compilable(top_level)?;
                dependency_graph.add(&dependency_info);
                id_to_node.insert(dependency_info.this.clone(), new_units);
            }
        }

        let units: Vec<_> = dependency_graph
            .dependence_order()?
            .into_iter()
            .flat_map(|id| id_to_node.remove(id).unwrap_or(vec![]))
            .collect();

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
