//! Contains "big" objects
//!
//! aka classes

use crate::core::error::JodinResult;
use crate::core::identifier::{Identifier, Namespaced};
use crate::core::identifier_resolution::Registry;
use crate::core::privacy::Visibility;
use crate::core::types::{CompoundType, Field, GetResolvedMember, JodinType, JodinTypeReference, Type};
use crate::core::types::generic_context::{GenericParameter, Morph};
use crate::core::types::intermediate_type::IntermediateType;
use crate::core::types::traits::JTraitObject;
use crate::core::types::type_environment::{TypeEnvironment};
use crate::utility::Visitor;


/// The actual, declaration of the JObject
#[derive(Debug)]
pub struct JObject {
    id: Identifier,
    generics: Vec<GenericParameter>,
    parent_type: Option<Identifier>,
    type_id: u32,
    fields: Vec<Field>
}


impl Namespaced for JObject {
    fn get_identifier(&self) -> &Identifier {
        &self.id
    }
}

impl Type for JObject {
    fn type_name(&self) -> Identifier {
        self.get_identifier().clone()
    }

    fn type_id(&self) -> u32 {
        self.type_id
    }
}


impl CompoundType for JObject {
    fn all_members(&self) -> Vec<(&Visibility, &IntermediateType, &Identifier)> {
        self.fields
            .iter()
            .map(|field| field.as_tuple())
            .collect()
    }
}

impl<'node, 'types : 'node> Visitor<TypeEnvironment<'node>, Option<JBigObject<'types>>> for JObject {
    fn accept(&self, environment: &TypeEnvironment<'node>) -> Option<JBigObject<'types>> {
        let mut fields = self.fields.iter().collect::<Vec<_>>();

    }
}

impl Morph for JObject {
    type Morphed = Self;

    fn apply_generics<I>(&self, generics: I) -> Self::Morphed where I: IntoIterator<Item=(Identifier, Identifier)> {
        todo!()
    }
}


/// A JBigObject is a type formed using references
#[derive(Debug)]
pub struct JBigObject<'types> {
    base_type: &'types JodinType,
    fields: Vec<&'types Field>,
    traits: Vec<&'types JTraitObject>
}

impl<'t> GetResolvedMember<Field> for JBigObject<'t> {
    fn get_member(&self, member_id: &Identifier) -> JodinResult<&Field> {
        todo!()
    }
}
