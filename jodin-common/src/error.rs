//! To standardize error reporting within the project, all Results should return the [JodinError] type
//! in its E position
//!
//! [JodinError]: crate::core::error::JodinError

use crate::identifier::{Identifier, Namespaced};

use backtrace::Backtrace;

use lalrpop_util::ParseError;
use std::char::ParseCharError;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::num::{ParseFloatError, ParseIntError};
use std::string::FromUtf8Error;

use crate::ast::JodinNode;
use crate::core::literal::Literal;
use thiserror::Error;

/// The inner data type for the error that contains specific information required by the error.
#[derive(Debug, Error)]
pub enum JodinErrorType {
    /// Parsing a literal, like `32` or `"hello"`, failed.
    #[error("Couldn't parse a literal")]
    LiteralParseError,
    /// This identifier is ambiguous. This can be the result of a mix of `using` statements and `in` statements.
    #[error("The given identifier {given} is ambiguous. Could be {found:?}")]
    AmbiguousIdentifierError {
        /// The identifier to be searched for.
        given: Identifier,
        /// All the identifiers that match.
        found: Vec<Identifier>,
    },
    /// This identifier is not present.
    #[error("{0} does not exist")]
    IdentifierDoesNotExist(Identifier),
    /// An identifier is trying to be redeclared.
    #[error("{0} has already been declared")]
    IdentifierAlreadyExists(Identifier),
    /// The maximum number of tags has been reached for a specific tag.
    #[error(
        "The maximum number of this tag has already been reached (tag type: {tag_type}, max: {max})"
    )]
    MaxNumOfTagExceeded {
        /// The type of tag that threw this error.
        tag_type: String,
        /// The maximum number allowed for this type.
        max: u32,
    },
    /// An error casting a tag to a certain type.
    #[error("Tag can not be casted")]
    TagCastError,
    /// The requested tag is not present in the node.
    #[error("Tag is not present")]
    TagNotPresent,
    /// The parser threw an error.
    #[error("The parser threw an error (file: {1:?}): {0}")]
    ParserError(
        #[source] Box<ParseError<usize, String, JodinError>>,
        Option<String>,
    ),
    /// The entire string was not parsed.
    #[error("The entire string wasn't parsed (Remaining: {extra})")]
    IncompleteParse {
        /// The extra text that was not parsed.
        extra: String,
    },
    /// Some other error not defined in this enum was thrown.
    #[error(transparent)]
    InnerError(#[from] Box<dyn Error>),
    /// An invalid escape sequence was found within a string literal.
    #[error("Invalid escape sequence: {0}")]
    InvalidEscapeSequence(String),
    /// The Jodin tree is empty.
    #[error("The jodin tree is empty")]
    EmptyJodinTree,
    /// An illegal jodin rule was passed along to attempt to create an AST from.
    #[error("An illegal jodin rule for ast creation (rule: {0})")]
    InvalidJodinRuleForASTCreation(String),
    /// This expression can not be evaluated as a constant expression
    #[error("Not a constant expression")]
    NotAConstantExpression,
    /// Attempted to use invalid operator in a constant expression
    #[error("Not a valid operator for a constant expression")]
    InvalidOperatorForConstantExpression,
    /// Attempting to convert a literal into an illegal type
    #[error("Attempting to convert a literal into an illegal type")]
    IncorrectLiteralType,
    /// Attempting to convert a literal into an illegal type
    #[error("Attempting to convert a literal into an illegal type (literal: {0})")]
    IncorrectLiteralTypeWithLiteral(Literal),
    /// This identifier can not be directly converted into a C-identifier
    #[error("This is not a valid identifier in C (id: {0})")]
    InvalidAsDirectCDeclaration(Identifier),
    /// A circular dependency has been detected
    #[error("A circular dependency has been found")]
    CircularDependencyDetected,
    /// This rule can not be used for visibility
    #[error("This rule can not be used with visibility")]
    InvalidVisibilityRule(String),
    /// The target identifier is not visible from the originating namespace
    #[error("Can not access {target} from {origin_namespace}")]
    IdentifierProtected {
        /// The targeted import
        target: Identifier,
        /// The originating namespace the import is being made in
        origin_namespace: Identifier,
    },
    /// This can not be expressed as constant expression
    #[error("Not a constant expression (expr: {0})")]
    NotConstantExpression(String),
    /// Extern functions can only be declared in the Base namespace
    #[error("extern functions can only be declared in the base")]
    ExternFunctionNotDeclaredInBase,
    /// Illegal node type for compiler
    #[error("illegal tree type")]
    IllegalTreeType,
    /// This type can't be dereferenced
    #[error("Type can't be dereferenced (type: {0})")]
    TypeCantBeDereferenced(String),
    /// There was a lexing error
    #[error("There was a lexing error: {0}")]
    LexerError(String),
    /// The base type can only be generated once.
    #[error("A base type was already generated")]
    BaseTypeAlreadyGenerated,
    /// Attempted to a build some type using an illegal node type
    #[error("This node was an illegal to create some type (type: {type_name}, info: {node_info})")]
    IllegalNodeToBuildType {
        /// The desired type to build
        type_name: Identifier,
        /// The debug output
        node_info: String,
    },
    /// The type environment is no longer available
    #[error("The type environment is no longer available")]
    TypeEnvironmentUnavailable,
    /// The given tree type is invalid
    #[error("Invalid tree type given to compiler (expected: {0})")]
    InvalidTreeTypeGivenToCompiler(String),
    /// The given incremental unit is invalid in the current context
    #[error("Given invalid string for incremental compilation (string: {0:?})")]
    InvalidCompilationUnit(String),
    /// An anyhow produced error
    #[error(transparent)]
    AnyHowError(#[from] anyhow::Error),
    /// A UTF8 error
    #[error(transparent)]
    UTF8Error(#[from] FromUtf8Error),
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
        if f.alternate() {
            writeln!(f, "Error: {}", self.error_type)?;
            write!(f, "Backtrace: {:?}", self.backtrace)
        } else {
            write!(f, "{}", self.error_type)
        }
    }
}

impl Error for JodinError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.error_type)
    }
}

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

    #[doc(hidden)]
    pub fn illegal_type_for_node<I: Namespaced>(id: I, node: &JodinNode) -> Self {
        Self::new(JodinErrorType::IllegalNodeToBuildType {
            type_name: id.get_identifier().clone(),
            node_info: format!("{:?}", node),
        })
    }
}

impl<E: Into<JodinErrorType>> From<E> for JodinError {
    fn from(e: E) -> Self {
        JodinError::new(e.into())
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
wrap_error!(ParseFloatError);
wrap_error!(std::io::Error);
wrap_error!(std::fmt::Error);

// impl From<lalrpop_util::ParseError<usize, jodinc::parsing::Tok<'_>, JodinError>> for JodinError {
//     fn from(e: lalrpop_util::ParseError<usize, jodinc::parsing::Tok<'_>, JodinError>) -> Self {
//         JodinError::new(JodinErrorType::ParserError(
//             Box::new(e.map_token(|tok| tok.to_string())),
//             None,
//         ))
//     }
// }

// wrap_error!(pest::error::Error<crate::parsing::Rule>);
/// Convenience result
pub type JodinResult<T> = Result<T, JodinError>;
