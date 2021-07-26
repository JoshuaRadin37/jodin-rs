use crate::core::error::{JodinError, JodinErrorType};
use crate::parsing::parser::JodinRule;
use std::convert::TryFrom;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Visibility {
    Public = 0,
    Protected = 1,
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
