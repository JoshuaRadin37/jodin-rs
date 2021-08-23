//! Declarations within classes and namespaces can have three different levels of visibility.
//!
//! Public visibility means that code within any namespace can see, and import, this declaration.
//! Protected visibility means that code within this namespace and inner namespaces can see, and
//! important this declaration. Private means only code within this namespace can see this declaration.

use crate::ast::tags::Tag;
use crate::core::error::{JodinError, JodinErrorType};
use crate::core::identifier::Identifier;
#[cfg(feature = "pest_parser")]
use crate::parsing::JodinRule;
use std::any::Any;
use std::convert::TryFrom;
use std::fmt::Debug;

/// The visibility of a declaration
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Visibility {
    /// The highest level of visibility, visible to all
    Public = 2,
    /// The default level of visibility, visible to this namespace and it's children.
    Protected = 1,
    /// The lowest level of visibility, only visible to this namespace.
    Private = 0,
}

impl Visibility {
    /// Checks whether a path is visible from another, checking path
    pub fn is_visible(&self, protected_path: &Identifier, checking_path: &Identifier) -> bool {
        match self {
            Visibility::Public => true,
            Visibility::Protected => checking_path >= protected_path,
            Visibility::Private => checking_path == protected_path,
        }
    }
}
#[cfg(feature = "pest_parser")]
impl TryFrom<Option<JodinRule>> for Visibility {
    type Error = JodinError;

    fn try_from(value: Option<JodinRule>) -> Result<Self, Self::Error> {
        Ok(match value {
            Some(JodinRule::t_private) => Visibility::Private,
            None => Visibility::Protected,
            Some(JodinRule::t_public) => Visibility::Public,
            Some(other) => {
                return Err(JodinError::new(JodinErrorType::InvalidVisibilityRule(
                    other,
                )))
            }
        })
    }
}

/// Tag a node to marks its visibility
#[derive(Debug, Clone)]
pub struct VisibilityTag(Visibility);

impl VisibilityTag {
    /// Creates a new visibility tag
    pub fn new(vis: Visibility) -> Self {
        VisibilityTag(vis)
    }

    /// Get the visibility of the tag
    pub fn visibility(&self) -> &Visibility {
        &self.0
    }
}

impl Tag for VisibilityTag {
    fn tag_type(&self) -> String {
        "VisibilityTag".to_string()
    }

    fn tag_info(&self) -> String {
        format!("[Visibility: {:?}]", self.0)
    }

    fn max_of_this_tag(&self) -> u32 {
        1
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
