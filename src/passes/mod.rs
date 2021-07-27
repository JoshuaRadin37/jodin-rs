//! Stores the passes that the compiles run through after the initial AST has been built.

use crate::ast::JodinNode;
use crate::core::error::JodinResult;

pub mod analysis;
pub mod frontend;
pub mod optimization;

/// Runs optimizations on a tree
pub fn optimize(node: JodinNode) -> JodinResult<JodinNode> {
    Ok(node)
}
