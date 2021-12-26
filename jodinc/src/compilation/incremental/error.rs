//! The error module for the incremental compiler

use crate::JodinError;
use std::error::Error;
use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum IncrementalCompilationError {
    #[error("Given invalid string for incremental compilation (string: {0:?})")]
    InvalidCompilationUnit(String),
    #[error(transparent)]
    Other(#[from] Box<dyn Error>),
}

impl From<JodinError> for IncrementalCompilationError {
    fn from(e: JodinError) -> Self {
        IncrementalCompilationError::Other(Box::new(e))
    }
}

impl From<io::Error> for IncrementalCompilationError {
    fn from(e: io::Error) -> Self {
        IncrementalCompilationError::Other(Box::new(e))
    }
}
