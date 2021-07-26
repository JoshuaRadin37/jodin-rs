//! Contains the different analysis based passes.
//!
//! These passes do not modify the structure of the AST in any way

mod identity_resolution_tool;
pub use identity_resolution_tool::{
    BlockIdentifierTag, IdentityResolutionTool, ResolvedIdentityTag,
};
