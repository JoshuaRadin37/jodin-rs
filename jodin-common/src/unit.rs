//! Am incremental compilation unit is a list of public/protected
//! declarations

use crate::core::privacy::Visibility;
use crate::core::types::intermediate_type::IntermediateType;
use crate::core::types::Field;
use crate::error::{JodinError, JodinErrorType};
use crate::identifier::Identifier;
use crate::mvp::bytecode::Assembly;
use std::borrow::Borrow;
use std::fmt::{format, Debug, Display, Formatter};
use std::io;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::str::FromStr;

/// A translation unit is the smallest public facing unit
#[derive(Debug, PartialEq, Clone)]
pub struct TranslationUnit(Field<IntermediateType>);

/// The separator between each translation unit within a compilation
/// unit
pub const UNIT_SEPARATOR: char = '\n';
/// The separator between the fields within a single translation unit
pub const FIELD_SEPARATOR: char = '|';

const_assert_ne!(UNIT_SEPARATOR, FIELD_SEPARATOR);
assert_impl_all!(TranslationUnit: FromStr, ToString);

impl TranslationUnit {
    /// Creates a new translation unit
    pub fn new<I: Into<Identifier>>(
        visibility: Visibility,
        jtype: IntermediateType,
        id: I,
    ) -> Self {
        Self(Field::new(visibility, jtype, id.into()))
    }
}

impl FromStr for TranslationUnit {
    type Err = JodinError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let as_split: Vec<&str> = s.split(FIELD_SEPARATOR).collect();
        let as_split_slice = as_split.as_slice();
        if let &[name, jtype, visibility] = as_split_slice {
            // Ok(TranslationUnit::new(
            //     Visibility::from_str(visibility)?,
            //     parse_type(jtype)?,
            //     name,
            // ))
            unimplemented!()
        } else {
            Err(JodinErrorType::InvalidCompilationUnit(s.to_string()).into())
        }
    }
}

impl Deref for TranslationUnit {
    type Target = Field<IntermediateType>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for TranslationUnit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let parts: [String; 3] = [
            self.name.to_string(),
            self.jtype.to_string(),
            self.vis.to_string(),
        ];
        let joined = parts.join(&*FIELD_SEPARATOR.to_string());
        write!(f, "{}", joined)
    }
}

/// An iterator that outputs translation units from a string
pub struct TranslationUnitIterator<'s> {
    backing_string: &'s str,
    index: usize,
}

impl<'s> TranslationUnitIterator<'s> {
    pub fn new(s: &'s str) -> Self {
        Self {
            backing_string: s,
            index: 0,
        }
    }
}

impl Iterator for TranslationUnitIterator<'_> {
    type Item = TranslationUnit;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.backing_string.len() {
            return None;
        }
        let next_unit_separator = self.backing_string[self.index..].find(UNIT_SEPARATOR);
        let next_unit = match next_unit_separator {
            Some(i) => {
                let output = &self.backing_string[self.index..(self.index + i)];
                self.index += i + 1;
                output
            }
            None => {
                let output = &self.backing_string[self.index..];
                self.index = self.backing_string.len();
                output
            }
        };
        let unit = TranslationUnit::from_str(next_unit).expect(&*format!(
            "{} can not be parsed into a tranlstion unit",
            next_unit
        ));
        Some(unit)
    }
}

pub fn join_translation_units<S: AsRef<str>, I: IntoIterator<Item = S>>(iterator: I) -> String {
    itertools::join(
        iterator.into_iter().map(|s| s.as_ref().to_string()),
        &*UNIT_SEPARATOR.to_string(),
    )
}

/// The compilation objects are what's produced by the compiler, and can also be interpreted by the
/// incremental compiler to get the the translation units
pub struct CompilationObject {
    magic_number: u64,
    pub file_location: PathBuf,
    /// The module that the translation units are part of
    pub module: Identifier,
    /// The public/protected translation units of the object
    pub units: Vec<TranslationUnit>,
    /// The assembly in the compilation object
    pub jasm: Assembly,
}

impl<R: io::Read> From<R> for CompilationObject {
    fn from(readable: R) -> Self {
        todo!()
    }
}

impl Debug for CompilationObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CompilationObject")
            .field("location", &self.file_location)
            .field("module", &self.module)
            .finish_non_exhaustive()
    }
}

pub trait Incremental {
    fn translation_units(&self) -> Vec<TranslationUnit>;
    fn representative_path(&self) -> PathBuf;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::primitives::Primitive;
    use crate::core::types::Type;
    use jodinc::core::types::primitives::Primitive;
    use jodinc::core::types::Type;

    #[test]
    fn get_translation_units() {
        let strings = vec![
            ["x", "int", "public"].join(&FIELD_SEPARATOR.to_string()),
            ["y", "*int", "private"].join(&FIELD_SEPARATOR.to_string()),
            ["z", "[int]", "protected"].join(&FIELD_SEPARATOR.to_string()),
            ["w", "fn([int]) -> float", "protected"].join(&FIELD_SEPARATOR.to_string()),
        ];
        let joined = join_translation_units(strings);
        let mut unit_iterator = TranslationUnitIterator::new(&joined);
        assert_eq!(
            unit_iterator.next().unwrap(),
            TranslationUnit::new(Visibility::Public, Primitive::Int.as_intermediate(), "x")
        );
        assert_eq!(
            unit_iterator.next().unwrap(),
            TranslationUnit::new(
                Visibility::Private,
                Primitive::Int.as_intermediate().with_pointer(),
                "y"
            )
        );
        assert_eq!(
            unit_iterator.next().unwrap(),
            TranslationUnit::new(
                Visibility::Protected,
                Primitive::Int.as_intermediate().with_abstract_array(),
                "z"
            )
        );
        assert_eq!(
            unit_iterator.next().unwrap(),
            TranslationUnit::new(
                Visibility::Protected,
                Primitive::Float
                    .as_intermediate()
                    .with_function_params([Primitive::Int.as_intermediate().with_abstract_array()]),
                "w"
            )
        );
        assert!(unit_iterator.next().is_none())
    }
}
