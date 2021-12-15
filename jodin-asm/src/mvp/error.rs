use crate::mvp::value::Value;
use std::error::Error;
use std::num::{ParseIntError, TryFromIntError};
use std::string::FromUtf8Error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BytecodeError {
    #[error("Given value can not be interpreted as a location (value = {0:?})")]
    InvalidLocationFromValue(Value),
    #[error("Variable {0} not set")]
    VariableNotSet(usize),
    #[error(transparent)]
    Other(#[from] Box<dyn Error>),
}

macro_rules! bytecode_error {
    ($err_ty:ty) => {
        impl From<$err_ty> for self::BytecodeError {
            fn from(err: $err_ty) -> Self {
                Self::Other(Box::new(err))
            }
        }
    };
}

bytecode_error!(FromUtf8Error);
bytecode_error!(TryFromIntError);
