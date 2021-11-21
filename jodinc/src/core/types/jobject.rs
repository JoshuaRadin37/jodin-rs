//! Contains "big" objects
//!
//! aka classes

use crate::core::error::{JodinError, JodinErrorType, JodinResult};
use crate::core::identifier::{Identifier, Namespaced};
use crate::core::identifier_resolution::Registry;
use crate::core::privacy::Visibility;
use crate::core::types::generic_context::{GenericParameter, Morph};
use crate::core::types::intermediate_type::IntermediateType;
use crate::core::types::resolved_type::{ResolveType, ResolvedType};
use crate::core::types::traits::JTraitObject;
use crate::core::types::type_environment::TypeEnvironment;
use crate::core::types::{
    CompoundType, Field, GetResolvedMember, JodinType, JodinTypeReference, Member, Type,
};
use crate::utility::Visitor;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::ops::Deref;

/// The actual, declaration of the JObject
#[derive(Debug, Clone)]
pub struct JObject {
    id: Identifier,
    generics: Vec<GenericParameter>,
    parent_type: Option<Identifier>,
    type_id: u32,
    fields: Vec<Field<IntermediateType>>,
}

impl Namespaced for JObject {
    fn get_identifier(&self) -> &Identifier {
        &self.id
    }
}

impl Into<JodinType> for JObject {
    fn into(self) -> JodinType {
        JodinType::JObject(self)
    }
}

impl ResolveType for JObject {
    fn resolve(&self, environment: &TypeEnvironment) -> ResolvedType {
        let mut fields = self.fields.iter().collect::<Vec<_>>();
        todo!()
    }
}

impl Type<'_> for JObject {
    fn type_identifier(&self) -> Identifier {
        self.get_identifier().clone()
    }

    fn type_unique_id(&self) -> u32 {
        self.type_id
    }
}

impl CompoundType<'_> for JObject {
    fn all_members(&self) -> Vec<&Field<IntermediateType>> {
        self.fields.iter().collect()
    }
}

impl Morph<'_> for JObject {
    type Morphed = Self;

    fn apply_generics<I>(&self, generics: I) -> Self::Morphed
    where
        I: IntoIterator<Item = (Identifier, Identifier)>,
    {
        todo!()
    }
}

impl Member<IntermediateType> for JTraitObject {
    fn jtype(&self) -> &IntermediateType {
        todo!()
    }

    fn id(&self) -> &Identifier {
        todo!()
    }
}
