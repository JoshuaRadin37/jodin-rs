use jodin_asm::mvp::value::Value;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum VMError {
    #[error("No exit code was provided")]
    NoExitCode,
    #[error("Expected UInteger exit code (found = {0:?})")]
    ExitCodeInvalidType(Value),
    #[error("Invalid type found (expected= {expected}, found= {:?})")]
    InvalidType { value: Value, expected: String },
}
