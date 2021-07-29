//! The C Compiler for Jodin

use crate::ast::JodinNode;
use crate::compilation::c_compiler::top_level_declaration_compiler::TopLevelDeclarationCompiler;
use crate::compilation::{Compiler, Context, MicroCompiler, Target};
use crate::compilation_settings::CompilationSettings;
use crate::core::error::JodinResult;

mod top_level_declaration_compiler;

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
        let mut context = Context::new();
        self.compile_part(tree, &mut context)
    }
}

impl MicroCompiler<C99> for C99Compiler {
    fn compile_part(&mut self, tree: &JodinNode, context: &mut Context) -> JodinResult<()> {
        let mut compiler = TopLevelDeclarationCompiler;
        for node in tree {
            compiler.compile_part(node, context)?;
        }
        Ok(())
    }
}
