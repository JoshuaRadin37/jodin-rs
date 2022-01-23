//! The final step after the [JodinNode] tree has been completed, converting the AST into something
//! usable
//!
//! [JodinNode]: crate::ast::JodinNode

// pub use c_compiler::{C99Compiler, C99};

#[cfg(feature = "c_compiler")]
pub mod c_compiler;
#[cfg(feature = "c_compiler")]
pub use c_compiler::C99;

#[cfg(feature = "jodin_vm_compiler")]
pub mod jodin_vm_compiler;
#[cfg(feature = "jodin_vm_compiler")]
pub use jodin_vm_compiler::JodinVM;

#[cfg(feature = "incremental")]
pub mod incremental;

/*
/// Compile a tree into C99 code.
pub fn compile_c99<W: std::fmt::Write>(
    tree: JodinNode,
    compiler_settings: &CompilationSettings,
    writer: W,
) -> JodinResult<()> {
    let compiler = C99Compiler::new(writer);
    execute_compiler(tree, compiler, compiler_settings)
}

 */
