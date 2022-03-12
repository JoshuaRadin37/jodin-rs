//! Compilation specific errors

use thiserror::Error;
use jodin_common::error::JodinErrorType;
use jodin_common::identifier::Identifier;
use jodin_common::types::Field;
use jodin_common::types::intermediate_type::IntermediateType;

#[derive(Debug, Error)]
pub enum CompilationError {
    #[error("Found Repeated Declarations (ids = {0:?})")]
    FoundRepeatedDeclarations(Vec<Identifier>)
}

impl From<CompilationError> for JodinErrorType {
    fn from(c: CompilationError) -> Self {
        JodinErrorType::InnerError(Box::new(c))
    }
}