//! To standardize error reporting within the project, all Results should return the [JodinError] type
//! in its E position
//!
//! [JodinError]: crate::core::error::JodinError

use crate::core::identifier::Identifier;
use crate::parsing::JodinRule;
use backtrace::Backtrace;
use std::char::ParseCharError;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::num::ParseIntError;

/// The inner data type for the error that contains specific information required by the error.
#[derive(Debug)]
pub enum JodinErrorType {
    /// Parsing a literal, like `32` or `"hello"`, failed.
    LiteralParseError,
    /// This identifier is ambiguous. This can be the result of a mix of `using` statements and `in` statements.
    AmbiguousIdentifierError {
        /// The identifier to be searched for.
        given: Identifier,
        /// All the identifiers that match.
        found: Vec<Identifier>,
    },
    /// This identifier is not present.
    IdentifierDoesNotExist(Identifier),
    /// An identifier is trying to be redeclared.
    IdentifierAlreadyExists(Identifier),
    /// The maximum number of tags has been reached for a specific tag.
    MaxNumOfTagExceeded {
        /// The type of tag that threw this error.
        tag_type: String,
        /// The maximum number allowed for this type.
        max: u32,
    },
    /// An error casting a tag to a certain type.
    TagCastError,
    /// The requested tag is not present in the node.
    TagNotPresent,
    /// The [pest] parser through an error.
    ///
    /// [pest]: pest
    ParserError(pest::error::Error<crate::parsing::Rule>, Option<String>),
    /// The entire string was not parsed.
    IncompleteParse {
        /// The extra text that was not parsed.
        extra: String,
    },
    /// Some other error not defined in this enum was thrown.
    InnerError(Box<dyn Error>),
    /// An invalid escape sequence was found within a string literal.
    InvalidEscapeSequence(String),
    /// The Jodin tree is empty.
    EmptyJodinTree,
    /// An illegal jodin rule was passed along to attempt to create an AST from.
    InvalidJodinRuleForASTCreation(JodinRule),
    /// This expression can not be evaluated as a constant expression
    NotAConstantExpression,
    /// Attempted to use invalid operator in a constant expression
    InvalidOperatorForConstantExpression,
    /// Attempting to convert a literal into an illegal type
    IncorrectLiteralType,
    /// This identifier can not be directly converted into a C-identifier
    InvalidAsDirectCDeclaration(Identifier),
    /// A circular dependency has been detected
    CircularDependencyDetected,
    /// This rule can not be used for visibility
    InvalidVisibilityRule(JodinRule),
    /// The target identifier is not visible from the originating namespace
    IdentifierProtected {
        /// The targeted import
        target: Identifier,
        /// The originating namespace the import is being made in
        origin_namespace: Identifier,
    },
    /// Extern functions can only be declared in the Base namespace
    ExternFunctionNotDeclaredInBase,
    /// Illegal node type for compiler
    IllegalTreeType,
}

/// Contains both the error type and an approximate backtrace for where the error occurred.
#[derive(Debug)]
pub struct JodinError {
    /// The specific error type.
    pub error_type: JodinErrorType,
    backtrace: Backtrace,
}

impl Display for JodinError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for JodinError {}

impl JodinError {
    /// Creates a new instance of a JodinError using the [Type](JodinErrorType). The backtrace is
    /// created from here
    #[inline]
    pub fn new(error_type: JodinErrorType) -> Self {
        JodinError {
            error_type,
            backtrace: Backtrace::new(),
        }
    }

    /// The backtrace for where this error was created
    pub fn backtrace(&self) -> &Backtrace {
        &self.backtrace
    }

    /// Splits the Error into it's type and the Backtrace
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
wrap_error!(std::fmt::Error);
wrap_error!(pest::error::Error<crate::parsing::Rule>);
/// Convenience result
pub type JodinResult<T> = Result<T, JodinError>;
