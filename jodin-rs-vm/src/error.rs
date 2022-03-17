use jodin_common::assembly::value::Value;
use jodin_common::error::JodinError;
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
    #[error("Jodin error: {0}")]
    JodinError(#[from] JodinError),
    #[error(transparent)]
    AnyHowError(#[from] anyhow::Error),
    #[error(transparent)]
    PluginError(#[from] jodin_vm_plugins::Error),
    #[error(transparent)]
    Other(#[from] Box<dyn StdError>),
}

impl From<io::Error> for VMError {
    fn from(e: io::Error) -> Self {
        Self::IoError(e)
    }
}
