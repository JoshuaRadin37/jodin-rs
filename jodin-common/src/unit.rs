//! Am incremental compilation unit is a list of public/protected
//! declarations

use crate::asm_version::Version;
use crate::assembly::instructions::{Assembly, Bytecode, Decode, Encode, GetAsm};
use crate::compilation::{Compilable, Context, PaddedWriter, Target};
use crate::core::privacy::Visibility;
use crate::error::{JodinError, JodinErrorType, JodinResult};
use crate::identifier::Identifier;
use crate::types::intermediate_type::IntermediateType;
use crate::types::Field;
use anyhow::anyhow;

use std::fmt::{Debug, Display, Formatter};
use std::fs::File;
use std::io::Write;
use std::mem;
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
        if let &[_name, _jtype, _visibility] = as_split_slice {
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
#[derive(Debug)]
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

impl CompilationObject {
    pub fn new(
        file_location: PathBuf,
        module: Identifier,
        units: Vec<TranslationUnit>,
        jasm: Assembly,
    ) -> Self {
        CompilationObject {
            magic_number: Version.to_magic_number(),
            file_location,
            module,
            units,
            jasm,
        }
    }

}

impl Display for CompilationObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let path: PathBuf = match std::env::current_dir() {
            Ok(current) => {
                let file_location = pathdiff::diff_paths(current, &self.file_location).unwrap();

                if file_location.ancestors().count() < self.file_location.ancestors().count() {
                    file_location
                } else {
                    self.file_location.clone()
                }
            }
            Err(_) => self.file_location.clone(),
        };
        f.debug_struct("CompilationObject")
            .field("units", &self.units.len())
            .field("instructions", &self.jasm.len())
            .field("location", &path)
            .finish()
    }
}

impl<T: Target> Compilable<T> for CompilationObject {
    fn compile<W: Write>(self, _context: &Context, w: &mut PaddedWriter<W>) -> JodinResult<()> {
        let magic_num_as_bytes = self.magic_number.to_be_bytes();
        trace!("Wrote magic num {:?} to file", magic_num_as_bytes);
        w.write_all(&magic_num_as_bytes)?;
        writeln!(w)?;
        writeln!(w, "{:?}", &self.file_location)?;
        writeln!(w, "{}", &self.module)?;
        write!(w, "{{")?;
        for unit in &self.units {
            write!(w, "{};", unit)?;
        }
        write!(w, "}}")?;
        let encoded = self.jasm.encode();
        w.write_all(&*encoded)?;
        w.flush()?;
        Ok(())
    }
}

impl TryFrom<&[u8]> for CompilationObject {
    type Error = JodinError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let mut bytes = [0u8; mem::size_of::<u64>()];
        bytes.copy_from_slice(&value[0..8]);
        let magic_num = u64::from_be_bytes(bytes);
        if !Version.verify_magic_number(magic_num) {
            return Err(anyhow!(
                "Incorrect magic number for for {} (found: {}, expected: {})",
                Version.version_string(),
                magic_num,
                Version.to_magic_number()
            )
            .into());
        }

        let translation_unit_start_index = value
            .iter()
            .position(|&p| p == b'{')
            .ok_or(anyhow!("No position found for {{"))?;
        let translation_unit_end_offset = value
            .iter()
            .skip(translation_unit_start_index)
            .position(|&p| p == b'}')
            .ok_or(anyhow!("No position found for }}"))?;

        let translation_units_end = translation_unit_start_index + translation_unit_end_offset;
        let translation_unit_string =
            &value[translation_unit_start_index + 1..(translation_units_end)];
        let translation_unit_string = String::from_utf8(translation_unit_string.to_vec())?;
        let tu_strings: Vec<&str> = translation_unit_string
            .trim()
            .split(UNIT_SEPARATOR)
            .filter(|&s| !s.is_empty())
            .collect();
        debug!("Translation Unit Strings: {:#?}", tu_strings);
        let mut translation_units: Vec<TranslationUnit> = vec![];
        for translation_unit_string in tu_strings {
            info!("unit: {:?}", translation_unit_string);
            let field_split_vec = translation_unit_string
                .split(FIELD_SEPARATOR)
                .collect::<Vec<_>>();
            if let &[name, jtype, visibility] = &*field_split_vec {
                let id: Identifier = name.parse().unwrap();
                let jtype: IntermediateType = jtype.parse()?;
                let visibility: Visibility = visibility.parse()?;

                translation_units.push(TranslationUnit::new(visibility, jtype, id));
            }
        }

        let header_bytes = value[8..translation_unit_start_index].to_vec();
        // Header should be in utf-8
        let header = String::from_utf8(header_bytes)?;
        let mut split = header.lines().skip(1);
        let file_location: PathBuf = PathBuf::from(split.next().unwrap().replace('"', ""));
        let module: Identifier = Identifier::from(split.next().unwrap());

        let bytecode_raw = &value[translation_units_end + 1..];
        let bytecode: Bytecode = Bytecode::from(bytecode_raw);
        let assembly: Assembly = bytecode.decode();
        let output = CompilationObject::new(file_location, module, translation_units, assembly);
        info!("Generated {}", output);
        Ok(output)
    }
}

impl TryFrom<PathBuf> for CompilationObject {
    type Error = JodinError;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        CompilationObject::try_from(value.as_path())
    }
}

impl TryFrom<&Path> for CompilationObject {
    type Error = JodinError;

    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        info!("Attempting to load file at {:?}", value);
        if value.is_file() {
            let buffer = std::fs::read(value)?;
            Self::try_from(&*buffer)
        } else {
            Err(anyhow!(
                "{:?} is a directory and not be made directly into an CompilationObject",
                value
            ))?
        }
    }
}

impl GetAsm for CompilationObject {
    fn get_asm(&self) -> Assembly {
        self.jasm.clone()
    }
}

pub trait Incremental {
    fn translation_units(&self) -> Vec<TranslationUnit>;
    fn representative_path(&self) -> PathBuf;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::primitives::Primitive;
    use crate::types::Type;

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
