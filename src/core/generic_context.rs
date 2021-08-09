//! Create a generic context for something to exist in

use std::{collections::HashMap, iter::FromIterator};

use super::identifier::Identifier;

/// The generic context something is instantiated in.
#[derive(Debug)]
pub struct GenericContext {
    mapping: HashMap<Identifier, Identifier>,
}

impl FromIterator<(Identifier, Identifier)> for GenericContext {
    fn from_iter<T: IntoIterator<Item = (Identifier, Identifier)>>(iter: T) -> Self {
        Self {
            mapping: iter.into_iter().collect(),
        }
    }
}
