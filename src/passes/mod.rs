//! Stores the passes that the compiles run through after the initial AST has been built.

use crate::ast::JodinNode;
use crate::core::error::JodinResult;
use crate::passes::optimization::OrderOfOperationTool;

pub mod analysis;
pub mod frontend;
pub mod optimization;

/// Runs optimizations on a tree
pub fn optimize(mut node: JodinNode) -> JodinResult<JodinNode> {
    let oot = OrderOfOperationTool;

    oot.fix_order_of_operations(&mut node);

    Ok(node)
}
