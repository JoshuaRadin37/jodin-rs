use crate::core::identifier::Identifier;
use crate::parsing::parser::JodinRule;
use backtrace::Backtrace;
use std::char::ParseCharError;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::num::ParseIntError;

#[derive(Debug)]
pub enum JodinErrorType {
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
    IncompleteParse {
        extra: String,
    },
    InnerError(Box<dyn Error>),
    InvalidEscapeSequence(String),
    EmptyJodinTree,
    InvalidJodinRuleForASTCreation(JodinRule),
}

#[derive(Debug)]
pub struct JodinError {
    error_type: JodinErrorType,
    backtrace: Backtrace,
}

impl Display for JodinError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for JodinError {}

impl JodinError {
    #[inline]
    pub fn new(error_type: JodinErrorType) -> Self {
        JodinError {
            error_type,
            backtrace: Backtrace::new(),
        }
    }

    pub fn error_type(&self) -> &JodinErrorType {
        &self.error_type
    }
    pub fn error_type_mut(&mut self) -> &mut JodinErrorType {
        &mut self.error_type
    }
    pub fn backtrace(&self) -> &Backtrace {
        &self.backtrace
    }

    pub fn into_err_and_bt(self) -> (JodinErrorType, Backtrace) {
        let Self {
            error_type,
            backtrace,
        } = self;
        (error_type, backtrace)
    }
}

impl From<JodinErrorType> for JodinError {
    fn from(e: JodinErrorType) -> Self {
        JodinError::new(e)
    }
}

macro_rules! wrap_error {
    ($error:ty) => {
        impl From<$error> for JodinError {
            fn from(e: $error) -> Self {
                JodinError::new(JodinErrorType::InnerError(Box::new(e)))
            }
        }
    };
}

wrap_error!(ParseIntError);
wrap_error!(ParseCharError);
wrap_error!(std::io::Error);
wrap_error!(pest::error::Error<crate::parsing::parser::Rule>);
/// Convenience result
pub type JodinResult<T> = Result<T, JodinError>;
