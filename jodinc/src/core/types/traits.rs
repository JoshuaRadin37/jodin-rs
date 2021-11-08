//! Traits are interfaces

use crate::core::error::JodinResult;
use crate::core::identifier::Identifier;
use crate::core::types::big_object::JBigObject;
use crate::core::types::generic_context::GenericParameter;
use crate::core::types::type_environment::TypeEnvironment;
use crate::core::types::{Field, JodinType, Type};
use crate::utility::Visitor;
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

impl Into<JodinType> for JTrait {
    fn into(self) -> JodinType {
        JodinType::JTrait(self)
    }
}

impl<'n, 't> Type<'n, 't> for JTrait {
    fn type_name(&self) -> Identifier {
        self.id.clone()
    }

    fn type_id(&self) -> u32 {
        self.type_id
    }
}

impl<'n, 't> Visitor<TypeEnvironment<'n>, JodinResult<JBigObject<'t>>> for JTrait {
    fn accept(&self, environment: &TypeEnvironment<'n>) -> JodinResult<JBigObject<'t>> {
        todo!()
    }
}

#[derive(Debug)]
pub struct JTraitObject {
    owner_type: Identifier,
    jtrait: Arc<JTrait>,
    entries: Vec<Field>,
    type_id: u32,
}

impl<'n, 't> Visitor<TypeEnvironment<'n>, JodinResult<JBigObject<'t>>> for JTraitObject {
    fn accept(&self, environment: &TypeEnvironment<'n>) -> JodinResult<JBigObject<'t>> {
        todo!()
    }
}

impl Into<JodinType> for JTraitObject {
    fn into(self) -> JodinType {
        JodinType::JTraitObject(self)
    }
}

impl Type<'_, '_> for JTraitObject {
    fn type_name(&self) -> Identifier {
        &self.jtrait.id >> &self.owner_type
    }

    fn type_id(&self) -> u32 {
        self.type_id
    }
}
