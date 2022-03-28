//! Contains the different analysis based passes.
//!
//! These passes do not modify the structure of the AST in any way

pub use identity_resolution_tool::IdentityResolutionTool;
use jodin_common::ast::JodinNode;
use jodin_common::core::identifier_resolution::{IdentifierResolver, Registry};
use jodin_common::core::privacy::Visibility;
pub use jodin_common::core::tags::BlockIdentifierTag;
pub use jodin_common::core::tags::ResolvedIdentityTag;
use jodin_common::error::JodinResult;
use jodin_common::types::type_environment::TypeEnvironment;

use crate::passes::analysis::identity_resolution_tool::IdentifierCreator;
use crate::passes::analysis::type_resolution_tool::TypeResolutionTool;
use jodin_common::unit::TranslationUnit;

mod dependency_tool;
mod identity_resolution_tool;
mod type_resolution_tool;

/// Performs analysis on the tree, adding tags to the tree where appropriate
///
/// Steps:
/// 1. Perform identity creation and resolution
pub fn analyze(tree: JodinNode) -> JodinResult<(JodinNode, TypeEnvironment)> {
    let mut identifier_tool = IdentityResolutionTool::new();
    let (mut tree, _id_resolver) = identifier_tool.resolve_identities(tree)?;

    let mut type_resolution = TypeResolutionTool::new();
    type_resolution.visit(&mut tree)?;
    let environment = type_resolution.finish();

    Ok((tree, environment))
}

pub fn analyze_with_preload<'t, I>(
    tree: JodinNode,
    ids: I,
) -> JodinResult<(JodinNode, TypeEnvironment)>
where
    I: IntoIterator<Item = &'t TranslationUnit>,
{
    let units = ids.into_iter().cloned().collect::<Vec<_>>();
    let mut identifier_tool = IdentityResolutionTool::with_translation_units(&units);
    let (mut tree, _id_resolver) = identifier_tool.resolve_identities(tree)?;

    let mut type_resolution = TypeResolutionTool::with_translation_units(&units);
    type_resolution.visit(&mut tree)?;
    let environment = type_resolution.finish();

    Ok((tree, environment))
}

pub fn with_own_identities(
    tree: JodinNode,
) -> JodinResult<(JodinNode, IdentifierResolver, Registry<Visibility>)> {
    let mut creator = IdentifierCreator::new();
    let mut visibility = IdentityResolutionTool::default_visibility();
    creator
        .start(tree, &mut visibility)
        .map(|(tree, resolver)| (tree, resolver, visibility))
}
