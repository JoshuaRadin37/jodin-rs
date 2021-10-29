//! Traits are interfaces

use crate::core::identifier::Identifier;
use crate::core::types::generic_context::GenericParameter;
use crate::core::types::{Field, Type};
use std::sync::Arc;

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
    pub entries: Vec<Field>,
}

impl Type<'_, '_> for JTrait {
    fn type_name(&self) -> Identifier {
        self.id.clone()
    }

    fn type_id(&self) -> u32 {
        self.type_id
    }
}

#[derive(Debug)]
pub struct JTraitObject {
    owner_type: Identifier,
    jtrait: Arc<JTrait>,
    entries: Vec<Field>,
    type_id: u32,
}

impl Type<'_, '_> for JTraitObject {
    fn type_name(&self) -> Identifier {
        &self.jtrait.id >> &self.owner_type
    }

    fn type_id(&self) -> u32 {
        self.type_id
    }
}
