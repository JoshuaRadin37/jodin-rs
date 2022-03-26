use jodin_common::error::JodinError;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io;
use std::ops::Add;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CompilerError {
    #[error("The given path is invalid as an object path (path = {0:?})")]
    InvalidObjectPath(PathBuf),
    #[error("{0:?} is missing (error = {1})")]
    MissingObjectPath(PathBuf, #[source] io::Error),
    #[error(transparent)]
    JodinError(#[from] JodinError),
    #[error(transparent)]
    IOError(#[from] io::Error),
    #[error(tranparent)]
    AnyError(Box<dyn Error>),
}

#[derive(Debug)]
pub struct MultiError {
    errors: Vec<Box<dyn Error>>,
}

impl MultiError {
    fn new<E: Error>(errors: Vec<E>) -> Self {
        Self {
            errors: errors.into_iter().map(|e| Box::new(e)).collect(),
        }
    }
}

impl Add for MultiError {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut errors = self.errors;
        errors.extend(rhs.errors);
        Self { errors }
    }
}

impl<E: Error> FromIterator<E> for MultiError {
    fn from_iter<T: IntoIterator<Item = E>>(iter: T) -> Self {
        Self::new(iter.into_iter().collect())
    }
}

impl Display for MultiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Multiple errors found:")?;
        for error in &self.errors {
            writeln!(f, " - {error}")?;
        }
        Ok(())
    }
}

impl<E: Error> From<E> for MultiError {
    fn from(e: E) -> Self {
        Self { errors: vec![e] }
    }
}

#[cfg(test)]
mod multi_error_tests {
    use crate::error::MultiError;
    use anyhow::anyhow;
    use jodin_common::utility::IntoBox;

    #[test]
    fn can_create_multi() {
        let e = anyhow!("test error").boxed();
        let _: MultiError = e.into();
    }
}
