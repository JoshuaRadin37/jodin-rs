use crate::core::identifier::Identifier;
use pest::error::Error as PestError;
use std::char::ParseCharError;
use std::error::Error;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum JodinError {
    LiteralParseError,
    AmbiguousIdentifierError {
        given: Identifier,
        found: Vec<Identifier>,
    },
    IdentifierDoesNotExist(Identifier),
    IdentifierAlreadyExists(Identifier),
    MaxNumOfTagExceeded {
        tag_type: String,
        max: u32,
    },
    TagCastError,
    TagNotPresent,
    ParserError(
        pest::error::Error<crate::parsing::parser::Rule>,
        Option<String>,
    ),
    IncompleteParse,
    InnerError(Box<dyn Error>),
}

impl From<pest::error::Error<crate::parsing::parser::Rule>> for JodinError {
    fn from(e: PestError<crate::parsing::parser::Rule>) -> Self {
        Self::ParserError(e, None)
    }
}

macro_rules! wrap_error {
    ($error:ty) => {
        impl From<$error> for JodinError {
            fn from(e: $error) -> Self {
                JodinError::InnerError(Box::new(e))
            }
        }
    };
}

wrap_error!(ParseIntError);
wrap_error!(ParseCharError);
/// Convenience result
pub type JodinResult<T> = Result<T, JodinError>;
