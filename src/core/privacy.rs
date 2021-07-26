//! Declarations within classes and namespaces can have three different levels of visibility.
//!
//! Public visibility means that code within any namespace can see, and import, this declaration.
//! Protected visibility means that code within this namespace and inner namespaces can see, and
//! important this declaration. Private means only code within this namespace can see this declaration.

use crate::core::error::{JodinError, JodinErrorType};
use crate::parsing::parser::JodinRule;
use std::convert::TryFrom;

/// The visibility of a declaration
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Visibility {
    /// The highest level of visibility, visible to all
    Public = 0,
    /// The default level of visibility, visible to this namespace and it's children.
    Protected = 1,
    /// The lowest level of visibility, only visible to this namespace.
    Private = 2,
}

impl TryFrom<Option<JodinRule>> for Visibility {
    type Error = JodinError;

    fn try_from(value: Option<JodinRule>) -> Result<Self, Self::Error> {
        Ok(match value {
            Some(JodinRule::t_private) => Visibility::Private,
            None => Visibility::Protected,
            Some(JodinRule::t_public) => Visibility::Public,
            _ => return Err(JodinError::new(JodinErrorType::LiteralParseError)),
        })
    }
}

/// Tag a node to marks its visibility
#[derive(Debug)]
pub struct VisibilityTag(Visibility);
