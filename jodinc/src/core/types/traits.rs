//! Traits are interfaces

use std::sync::Arc;
use crate::core::types::generic_context::GenericParameter;
use crate::core::identifier::Identifier;
use crate::core::types::{Field, Type};

/// A jodin trait structure
#[derive(Debug)]
pub struct JTrait {
    /// the identifier of the trait
    pub id: Identifier,
    type_id: u32,
    /// The generics of the trait
    pub generics: Vec<GenericParameter>,
    /// The super traits of this trait
    pub extends: Vec<Identifier>,
    pub entries: Vec<Field>
}

impl Type for JTrait {
    fn type_name(&self) -> Identifier {
        self.id.clone()
    }

    fn type_id(&self) -> u32 {
        self.type_id
    }
}

pub struct JTraitObject {
    owner_type: Identifier,
    jtrait: Arc<JTrait>,
    entries: Vec<Field>,
    type_id: u32
}

impl Type for JTraitObject {
    fn type_name(&self) -> Identifier {
        &self.jtrait.id >> &self.owner_type
    }

    fn type_id(&self) -> u32 {
        self.type_id
    }
}