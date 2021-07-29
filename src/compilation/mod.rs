//! The final step after the [JodinNode] tree has been completed, converting the AST into something
//! usable
//!
//! [JodinNode]: crate::ast::JodinNode

use crate::ast::JodinNode;
use crate::compilation_settings::CompilationSettings;
use crate::core::error::JodinResult;

mod c_compiler;
pub use c_compiler::{C99Compiler, C99};
use std::fs::File;
use std::io::Write;

/// A target for compilation.
pub trait Target {}

/// Compile code for a target.
pub trait Compiler<T: Target> {
    /// Attempt to compile the AST into something usable.
    fn compile(&mut self, tree: &JodinNode, settings: &CompilationSettings) -> JodinResult<()>;
}

/// Compile a part of the of program, such as a function.
pub trait MicroCompiler<T: Target, Output = ()> {
    /// Compile a part of the program.
    ///
    /// # Arguments
    ///
    /// * `tree`: The AST that is being compiled
    /// * `context`: the context to compile the project in
    ///
    /// returns: Result<Output, JodinError>
    fn compile_part(&mut self, tree: &JodinNode, context: &mut Context) -> JodinResult<Output>;
}

/// The context that the project is being built in.
pub struct Context {
    /// Where to writer files, maybe
    pub maybe_file_writer: Option<IndentedWriter<File>>,
}

impl Context {
    /// Create a new context instance
    pub fn new() -> Self {
        Context {
            maybe_file_writer: None,
        }
    }

    /// Gets the file writer for the context.
    ///
    /// # Panic
    /// Panics if no file writer is set.
    pub fn file_writer(&mut self) -> &mut IndentedWriter<File> {
        match &mut self.maybe_file_writer {
            None => {
                panic!("No file writer has been set")
            }
            Some(v) => v,
        }
    }
}

/// Compile a tree into C99 code.
pub fn compile_c99(tree: JodinNode, compiler_settings: &CompilationSettings) -> JodinResult<()> {
    let compiler = C99Compiler::new();
    execute_compiler(tree, compiler, compiler_settings)
}

/// Execute an arbitrary compiler
pub fn execute_compiler<T, C>(
    tree: JodinNode,
    mut compiler: C,
    compiler_settings: &CompilationSettings,
) -> JodinResult<()>
where
    T: Target,
    C: Compiler<T>,
{
    compiler.compile(&tree, compiler_settings)
}

/// A writer that can set the indent for how things are written
pub struct IndentedWriter<W: Write> {
    writer: W,
    indent_level: usize,
}

impl<W: Write> IndentedWriter<W> {
    /// Create a new indented writer.
    pub fn new(writer: W) -> Self {
        IndentedWriter {
            writer,
            indent_level: 0,
        }
    }
}

impl<W: Write> Write for IndentedWriter<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        todo!()
    }
}
