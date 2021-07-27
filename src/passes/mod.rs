//! Stores the passes that the compiles run through after the initial AST has been built.

use crate::ast::JodinNode;
use crate::core::error::JodinResult;
use crate::passes::optimization::OrderOfOperationTool;
use crate::passes::toolchain::{map_tool, Tool, ToolchainUtilities};

pub mod analysis;
pub mod frontend;
pub mod optimization;
#[macro_use]
pub mod toolchain;

/// Creates an optimization toolchain that runs only on single nodes
///
/// Currently, only runs an order of operation tool.
pub fn single_node_toolchain() -> impl Tool<Input = JodinNode, Output = JodinResult<JodinNode>> {
    chain_tools![OrderOfOperationTool, map_tool(|node| Ok(node))]
}
