//! Contains the different analysis based passes.
//!
//! These passes do not modify the structure of the AST in any way

pub use identity_resolution_tool::{
    BlockIdentifierTag, IdentityResolutionTool, ResolvedIdentityTag,
};

use crate::ast::JodinNode;
use crate::core::error::JodinResult;

mod identity_resolution_tool;

/// Performs analysis on the tree, adding tags to the tree where appropriate
///
/// Steps:
/// 1. Perform identity creation and resolution
pub fn analyze(tree: JodinNode) -> JodinResult<JodinNode> {
    let mut identifier_tool = IdentityResolutionTool::new();
    let (tree, id_resolver) = identifier_tool.resolve_identities(tree)?;
    println!("{:#?}", id_resolver);
    Ok(tree)
}
