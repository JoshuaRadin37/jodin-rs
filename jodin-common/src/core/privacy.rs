//! Declarations within classes and namespaces can have three different levels of visibility.
//!
//! Public visibility means that code within any namespace can see, and import, this declaration.
//! Protected visibility means that code within this namespace and inner namespaces can see, and
//! important this declaration. Private means only code within this namespace can see this declaration.

use crate::core::tags::Tag;
use crate::error::JodinError;
use crate::error::JodinErrorType;
use crate::identifier::Identifier;
use std::any::Any;
use std::fmt::{write, Debug, Display, Formatter};
use std::str::FromStr;

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
        if protected_path.len() == checking_path.len() {
            if protected_path.parent() == checking_path.parent() {
                // paths in the same namespace are always visible to each-other
                return true;
            }
        }
        match self {
            Visibility::Public => true,
            Visibility::Protected => checking_path >= protected_path,
            Visibility::Private => checking_path == protected_path,
        }
    }
}

impl Display for Visibility {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Visibility::Public => {
                write!(f, "public")
            }
            Visibility::Protected => {
                write!(f, "protected")
            }
            Visibility::Private => {
                write!(f, "private")
            }
        }
    }
}

impl FromStr for Visibility {
    type Err = JodinError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &*s.to_lowercase() {
            "public" | "pu" => Ok(Visibility::Public),
            "protected" | "pro" => Ok(Visibility::Protected),
            "private" | "pri" => Ok(Visibility::Private),
            _ => Err(JodinErrorType::LexerError(s.to_string()).into()),
        }
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
