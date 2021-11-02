//! Contains "big" objects
//!
//! aka classes

use crate::core::error::{JodinError, JodinErrorType, JodinResult};
use crate::core::identifier::{Identifier, Namespaced};
use crate::core::identifier_resolution::Registry;
use crate::core::privacy::Visibility;
use crate::core::types::generic_context::{GenericParameter, Morph};
use crate::core::types::intermediate_type::IntermediateType;
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
#[derive(Debug)]
pub struct JObject {
    id: Identifier,
    generics: Vec<GenericParameter>,
    parent_type: Option<Identifier>,
    type_id: u32,
    fields: Vec<Field>,
}

impl Namespaced for JObject {
    fn get_identifier(&self) -> &Identifier {
        &self.id
    }
}

impl<'n, 't> Visitor<TypeEnvironment<'n>, JodinResult<JBigObject<'t>>> for JObject {
    fn accept(&self, environment: &TypeEnvironment<'n>) -> JodinResult<JBigObject<'t>> {
        todo!()
    }
}

impl Type<'_, '_> for JObject {
    fn type_name(&self) -> Identifier {
        self.get_identifier().clone()
    }

    fn type_id(&self) -> u32 {
        self.type_id
    }
}

impl CompoundType<'_, '_> for JObject {
    fn all_members(&self) -> Vec<(&Visibility, &IntermediateType, &Identifier)> {
        self.fields.iter().map(|field| field.as_tuple()).collect()
    }
}

impl<'node, 'types: 'node> Visitor<TypeEnvironment<'node>, Option<JBigObject<'types>>> for JObject {
    fn accept(&self, environment: &TypeEnvironment<'node>) -> Option<JBigObject<'types>> {
        let mut fields = self.fields.iter().collect::<Vec<_>>();

        todo!()
    }
}

impl Morph<'_, '_> for JObject {
    type Morphed = Self;

    fn apply_generics<I>(&self, generics: I) -> Self::Morphed
    where
        I: IntoIterator<Item = (Identifier, Identifier)>,
    {
        todo!()
    }
}

/// A JBigObject is a type formed using references
#[derive(Debug)]
pub struct JBigObject<'types> {
    base_type: &'types JodinType,
    fields: Vec<&'types Field>,
    traits: Vec<&'types JTraitObject>,
}

impl<'t> GetResolvedMember<Field> for JBigObject<'t> {
    fn get_member(&self, member_id: &Identifier) -> JodinResult<&Field> {
        self.fields
            .iter()
            .find(|trt| &trt.name == member_id)
            .map(|d| d.deref())
            .ok_or(JodinErrorType::IdentifierDoesNotExist(member_id.clone()).into())
    }
}

impl<'t> GetResolvedMember<JTraitObject> for JBigObject<'t> {
    fn get_member(&self, member_id: &Identifier) -> JodinResult<&JTraitObject> {
        self.traits
            .iter()
            .find(|trt| &trt.type_name() == member_id)
            .map(|d| d.deref())
            .ok_or(JodinErrorType::IdentifierDoesNotExist(member_id.clone()).into())
    }
}

impl Member for JTraitObject {
    fn jtype(&self) -> &IntermediateType {
        todo!()
    }

    fn id(&self) -> &Identifier {
        todo!()
    }
}

#[derive(Debug)]
pub struct JTraitObjectWithDistance<'t> {
    object: &'t JTraitObject,
    distance: usize,
}

impl<'t> JTraitObjectWithDistance<'t> {
    pub fn new(object: &'t JTraitObject, distance: usize) -> Self {
        JTraitObjectWithDistance { object, distance }
    }
}

impl PartialEq for JTraitObjectWithDistance<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl Eq for JTraitObjectWithDistance<'_> {}

impl PartialOrd for JTraitObjectWithDistance<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.distance.partial_cmp(&other.distance)
    }
}

impl Ord for JTraitObjectWithDistance<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.cmp(&other.distance)
    }
}

pub struct JBigObjectBuilder<'nodes, 'types> {
    base_type: &'types JodinType,
    parent_object: Option<JBigObject<'types>>,
    jtraits: BinaryHeap<JTraitObjectWithDistance<'types>>,
    type_env: &'types TypeEnvironment<'nodes>,
}

impl<'nodes, 'types> JBigObjectBuilder<'nodes, 'types> {
    pub(super) fn new(
        base_type: &'types JodinType,
        type_env: &'types TypeEnvironment<'nodes>,
    ) -> Self {
        JBigObjectBuilder {
            base_type,
            parent_object: Default::default(),
            jtraits: Default::default(),
            type_env,
        }
    }

    pub fn add_parent_type<'n, 't, T: Type<'n, 't>>(&mut self, parent: &T) {
        let big_object = parent.accept(self.type_env).expect("Could not set parent type");
        self.parent_object = Some(big_object);
    }

    pub fn build(self) -> JBigObject<'types> {
        let fields: Vec<&Field> = self.parent_object_chain.into_iter()
            .flat_map(|big_obj| big_obj.fields)
            .collect();
        let mut traits : Vec<&JTraitObject> = Vec::new();
        let mut trait_iter = self.jtraits.into_iter();
        while let Some(next) = trait_iter.next() {
            traits.push(next.object);
        }
        let base = self.base_type;
        JBigObject {
            base_type: base,
            fields,
            traits
        }
    }
}
