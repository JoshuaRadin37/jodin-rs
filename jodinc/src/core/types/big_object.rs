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
    CompoundType, Field, GetResolvedMember, JField, JodinType, JodinTypeReference, Member, Type,
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
    fields: Vec<JField>,
}

impl Namespaced for JObject {
    fn get_identifier(&self) -> &Identifier {
        &self.id
    }
}

impl<'t> Visitor<'t, TypeEnvironment, JodinResult<JBigObject<'t>>> for JObject {
    fn visit(&'t self, environment: &'t TypeEnvironment) -> JodinResult<JBigObject<'t>> {
        todo!()
    }
}

impl Into<JodinType> for JObject {
    fn into(self) -> JodinType {
        JodinType::JObject(self)
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
    fn all_members(&self) -> Vec<&JField> {
        self.fields.iter().collect()
    }
}

impl<'types> Visitor<'types, TypeEnvironment, Option<JBigObject<'types>>> for JObject {
    fn visit(&'types self, environment: &'types TypeEnvironment) -> Option<JBigObject<'types>> {
        let mut fields = self.fields.iter().collect::<Vec<_>>();

        todo!()
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

/// A JBigObject is a type formed using references
#[derive(Debug)]
pub struct JBigObject<'types> {
    base_type: &'types JodinType,
    fields: Vec<Field<JBigObject<'types>>>,
    traits: Vec<&'types JTraitObject>,
}

impl<'t> GetResolvedMember<Field<JBigObject<'t>>, JBigObject<'t>> for JBigObject<'t> {
    fn get_member(&self, member_id: &Identifier) -> JodinResult<&Field<JBigObject<'t>>> {
        self.fields
            .iter()
            .find(|trt| &trt.name == member_id)
            .map(|d| d)
            .ok_or(JodinErrorType::IdentifierDoesNotExist(member_id.clone()).into())
    }
}

impl<'t> GetResolvedMember<JTraitObject> for JBigObject<'t> {
    fn get_member(&self, member_id: &Identifier) -> JodinResult<&JTraitObject> {
        self.traits
            .iter()
            .find(|trt| &trt.type_identifier() == member_id)
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

pub struct JBigObjectBuilder<'types> {
    base_type: &'types JodinType,
    parent_object: Option<JBigObject<'types>>,
    jtraits: BinaryHeap<JTraitObjectWithDistance<'types>>,
    type_env: &'types TypeEnvironment,
}

impl<'types> JBigObjectBuilder<'types> {
    pub(super) fn new(base_type: &'types JodinType, type_env: &'types TypeEnvironment) -> Self {
        debug!("Creating BigObject builder for {}", base_type);
        JBigObjectBuilder {
            base_type,
            parent_object: Default::default(),
            jtraits: Default::default(),
            type_env,
        }
    }

    pub fn add_parent_type<T: Type<'types>>(&mut self, parent: &'types T) {
        let big_object = parent
            .visit(self.type_env)
            .expect("Could not set parent type");
        self.parent_object = Some(big_object);
    }

    pub fn build(self) -> JBigObject<'types> {
        let mut fields: Vec<Field<JBigObject<'types>>> = vec![];
        let big_o_factory = JBigObjectFactory::new(self.type_env);
        fields.extend(self.base_type.fields().into_iter().map(|(vis, ty, id)| {
            Field::new(
                vis.clone(),
                big_o_factory
                    .new_instance(&self.type_env.jodin_type_from_intermediate(ty).unwrap())
                    .unwrap(),
                id.clone(),
            )
        }));
        let parent = self
            .parent_object
            .into_iter()
            .flat_map(|big_obj| big_obj.fields);
        fields.extend(parent);
        let mut traits: Vec<&JTraitObject> = Vec::new();
        let mut trait_iter = self.jtraits.into_iter();
        while let Some(next) = trait_iter.next() {
            traits.push(next.object);
        }
        let base = self.base_type;
        JBigObject {
            base_type: base,
            fields,
            traits,
        }
    }
}

/// A factory for building [`JBigObject`]s.
///
/// [`JBigObject`]: JBigObject
pub struct JBigObjectFactory<'types> {
    env: &'types TypeEnvironment,
}

impl<'types> JBigObjectFactory<'types> {
    /// Create a new big object factory
    pub fn new(env: &'types TypeEnvironment) -> Self {
        JBigObjectFactory { env }
    }

    /// Create a new instance of a j big object
    pub fn new_instance<T: Type<'types>>(&self, ty: &'types T) -> JodinResult<JBigObject<'types>> {
        ty.visit(self.env)
    }
}
