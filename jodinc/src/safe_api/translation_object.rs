//! Translation Objects are the "buildables" of compiled jodin files


use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::ops::{Add, AddAssign};
use num_traits::MulAddAssign;
use jodin_common::assembly::instructions::Assembly;
use jodin_common::compilation::{PaddedWriter, Target};
use jodin_common::error::JodinError;
use jodin_common::identifier::Identifier;
use jodin_common::types::{Field, Member};
use jodin_common::types::intermediate_type::IntermediateType;
use jodin_common::unit::CompilationObject;
use crate::safe_api::error::CompilationError;

pub trait EmitTo<T : Target> {

    fn emit_to<W : io::Write>(&self, write_to: W) -> Result<(), CompilationError>;
}


#[derive(Debug, Default)]
pub struct TranslationObject {
    related_decs: Vec<Field<IntermediateType>>,
    assembly: Assembly
}

impl TranslationObject {

    /// Try to merge another translation object into this one
    pub fn try_merge_into(&mut self, other: Self) -> Result<(), JodinError> {
        let this_ids = self.ids();
        let other_ids = other.ids();
        let intersection = this_ids.intersection(&other_ids).collect::<Vec<_>>();
        if !intersection.is_empty() {
            return Err(CompilationError::FoundRepeatedDeclarations(intersection
                .into_iter()
                .map(|&s| s.clone())
                .collect()).into())
        }

        self.related_decs.extend(other.related_decs);
        self.assembly.extend(other.assembly);

        Ok(())
    }

    pub fn try_merge(mut self, other: Self) -> Result<Self, CompilationError> {
        self.try_merge_into(other)?;
        Ok(self)
    }

    /// The ids of the related declarations
    pub fn ids(&self) -> HashSet<&Identifier> {
        self.related_decs
            .iter()
            .map(|field| field.id())
            .collect()
    }
}

impl Add for TranslationObject {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.try_merge(rhs).unwrap()
    }
}

impl AddAssign for TranslationObject {
    fn add_assign(&mut self, rhs: Self) {
        self.try_merge_into(rhs).unwrap()
    }
}
