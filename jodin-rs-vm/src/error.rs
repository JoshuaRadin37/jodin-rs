use jodin_common::mvp::value::Value;
use std::error::Error as StdError;
use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum VMError {
    #[error("No exit code was provided")]
    NoExitCode,
    #[error("Expected UInteger exit code (found = {0:?})")]
    ExitCodeInvalidType(Value),
    #[error("Invalid type found (expected= {expected}, found= {value:?})")]
    InvalidType { value: Value, expected: String },
    #[error("Given file is incorrect type")]
    WrongFileType,
    #[error("IO Error: {0}")]
    IoError(io::Error),
    #[error(transparent)]
    Other(#[from] Box<dyn StdError>),
}

impl From<io::Error> for VMError {
    fn from(e: io::Error) -> Self {
        Self::IoError(e)
    }
}
