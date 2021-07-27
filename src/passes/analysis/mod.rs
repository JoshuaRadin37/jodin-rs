//! Contains the different analysis based passes.
//!
//! These passes do not modify the structure of the AST in any way

mod identity_resolution_tool;
use crate::ast::JodinNode;
use crate::core::error::JodinResult;
pub use identity_resolution_tool::{
    BlockIdentifierTag, IdentityResolutionTool, ResolvedIdentityTag,
};

/// Performs analysis on the tree, adding tags to the tree where appropriate
///
/// Steps:
/// 1. Perform identity creation and resolution
pub fn analyze(tree: JodinNode) -> JodinResult<JodinNode> {
    let mut identifier_tool = IdentityResolutionTool::new();
    let (tree, id_resolver) = identifier_tool.resolve_identities(tree)?;

    Ok(tree)
}
