//! Compilation specific errors

use jodin_common::ast::JodinNode;
use jodin_common::error::{JodinError, JodinErrorType};
use jodin_common::identifier::Identifier;
use jodin_common::types::intermediate_type::IntermediateType;
use jodin_common::types::Field;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CompilationError {
    #[error("Found Repeated Declarations (ids = {0:?})")]
    FoundRepeatedDeclarations(Vec<Identifier>),
    #[error("Invalid Tree Given (tree = {0:?})")]
    InvalidTreeGiven(JodinNode),
    #[error(transparent)]
    JodinError(#[from] JodinError),
}

impl From<CompilationError> for JodinErrorType {
    fn from(c: CompilationError) -> Self {
        JodinErrorType::InnerError(Box::new(c))
    }
}
