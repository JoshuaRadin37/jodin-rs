use crate::core::identifier::Identifier;

#[derive(Debug)]
pub enum JodinError {
    LiteralParseError,
    AmbiguousIdentifierError {
        given: Identifier,
        found: Vec<Identifier>,
    },
    IdentifierDoesNotExist(Identifier),
    IdentifierAlreadyExists(Identifier),
}

/// Convenience result
pub type JodinResult<T> = Result<T, JodinError>;
