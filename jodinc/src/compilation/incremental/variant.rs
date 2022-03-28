//! The exposed units are the outgoing variants of files

use jodin_common::core::import::Import;
use jodin_common::identifier::Identifier;
use serde::{Deserialize, Serialize};
use std::borrow::Borrow;
use std::cell::Cell;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::path::{Path, PathBuf};

/// An exposed unit represents some data that files expose to each other
trait ExposedUnit {
    fn identifier(&self) -> &Identifier;
}

#[derive(Debug)]
pub struct IncomingExposedUnit {
    pub exposing_file: Option<PathBuf>,
    pub identifier: Identifier,
    _phantom: PhantomData<()>, // can't construct
}

impl ExposedUnit for IncomingExposedUnit {
    fn identifier(&self) -> &Identifier {
        &self.identifier
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum OutgoingVariantType {
    /// Allowed to have more than one declarations
    Module,
    /// Only one of this can exist
    Singleton,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OutgoingExposedUnit {
    pub exposing_file: PathBuf,
    pub identifier: Identifier,
    pub unit_type: OutgoingVariantType,
    _phantom: PhantomData<()>, // can't construct
}

impl ExposedUnit for OutgoingExposedUnit {
    fn identifier(&self) -> &Identifier {
        &self.identifier
    }
}

impl OutgoingExposedUnit {
    pub fn new(
        exposing_file: PathBuf,
        identifier: Identifier,
        unit_type: OutgoingVariantType,
    ) -> Self {
        Self {
            exposing_file,
            identifier,
            unit_type,
            _phantom: PhantomData,
        }
    }
}

pub trait ExposingVariant {
    fn exposed_units(&self) -> &OutgoingVariant;
}

pub trait Variant: ExposingVariant {
    fn incoming_units(&self) -> &IncomingVariant;
    fn match_to_outgoing(&mut self, going: &OutgoingVariant);
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OutgoingVariant {
    exposed: Vec<OutgoingExposedUnit>,
}

impl OutgoingVariant {
    pub fn new<P: AsRef<Path>, Id, I>(path: P, identifiers: I) -> Self
    where
        Id: AsRef<Identifier>,
        I: IntoIterator<Item = Id>,
    {
        let path = path.as_ref().to_path_buf();
        Self {
            exposed: identifiers
                .into_iter()
                .map(|id: Id| OutgoingExposedUnit::new(path.clone(), id.as_ref().clone()))
                .collect(),
        }
    }
}

#[derive(Debug)]
pub struct IncomingVariant {
    looking_for: HashMap<Identifier, IncomingExposedUnit>,
}

impl IncomingVariant {
    pub fn looking_for(&self) -> impl IntoIterator<Item = &IncomingExposedUnit> {
        self.looking_for.values()
    }

    /// Gets a list of incoming exposed units that have an associated file
    pub fn discovered(&self) -> impl IntoIterator<Item = &IncomingExposedUnit> {
        self.looking_for
            .iter()
            .filter_map(|(_, unit)| unit.exposing_file.as_ref().and(Some(unit)))
    }

    /// Match an outgoing variant to this incoming variant.
    pub fn match_outgoing(&mut self, outgoing: &OutgoingVariant) {
        for exposed in &outgoing.exposed {
            let id = exposed.identifier();
            if let Some(entry) = self.looking_for.get_mut(id) {
                match &mut entry.exposing_file {
                    Some(f) => {
                        panic!(
                            "Identifier {} already exposed in {:?}, but redeclared in {:?}",
                            id, entry.exposing_file, exposed.exposing_file
                        )
                    }
                    opt => *opt = Some(exposed.exposing_file.clone()),
                }
            }
        }
    }

    pub fn from_imports<I: Borrow<Import>, It: IntoIterator<Item = I>>(imports: It) -> Self {
        let buffer = imports.into_iter().collect::<Vec<I>>();
        let as_borrow = buffer.iter().map(|i| i.borrow()).collect::<Vec<_>>();
        Self::_from_imports(&as_borrow[..])
    }

    fn _from_imports(imports: &[&Import]) -> Self {
        todo!()
    }
}

impl<I: Borrow<Import>> FromIterator<I> for IncomingVariant {
    fn from_iter<T: IntoIterator<Item = I>>(iter: T) -> Self {
        Self::from_imports(iter)
    }
}
