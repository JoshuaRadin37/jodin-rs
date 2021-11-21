//! Types that are completely resolved. Formally known as big objects

use crate::core::error::{JodinErrorType, JodinResult};
use crate::core::identifier::Identifier;
use crate::core::types::intermediate_type::IntermediateType;
use crate::core::types::traits::JTraitObject;
use crate::core::types::type_environment::TypeEnvironment;
use crate::core::types::{AsIntermediate, Field, GetResolvedMember, JodinType, Type};
use crate::utility::Visitor;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::sync::{Arc, Weak};

/// A trait that should be implemented to take some jodin type and create a fully resolved type out of it
pub trait ResolveType {
    /// Resolve a type within a type environment.
    fn resolve(&self, environment: &TypeEnvironment) -> ResolvedType;
}

/// A helper trait to build a resolve type. Implementing this trait also implements [ResolveType](ResolveType)
pub trait BuildResolvedType<'types> {
    fn build_resolved_type(&self, builder: &mut ResolvedTypeBuilder);
}

impl<'types, T: BuildResolvedType<'types>> ResolveType for T {
    fn resolve(&self, environment: &TypeEnvironment) -> ResolvedType {
        let mut builder = ResolvedTypeBuilder::new(environment);
        self.build_resolved_type(&mut builder);
        builder.build()
    }
}

/// A JBigObject is a type formed using references
#[derive(Debug)]
pub struct ResolvedType {
    base_type: Weak<JodinType>,
    fields: Vec<Field<ResolvedType>>,
}

impl ResolvedType {
    /// Creates a new resolved type
    pub fn new(base_type: &Arc<JodinType>, fields: Vec<Field<ResolvedType>>) -> Self {
        ResolvedType {
            base_type: Arc::downgrade(base_type),
            fields,
        }
    }

    /// Upgrade a resolved type into a strongly associated type
    pub fn upgrade(&self) -> JodinResult<UpgradedResolvedType> {
        let mut fields = vec![];
        for field in &self.fields {
            fields.push(field.upgrade()?);
        }
        Ok(UpgradedResolvedType {
            base_type: self
                .base_type
                .upgrade()
                .ok_or(JodinErrorType::TypeEnvironmentUnavailable)?,
            fields,
        })
    }
}

impl AsIntermediate for ResolvedType {
    fn intermediate_type(&self) -> IntermediateType {
        self.base_type.upgrade().unwrap().as_intermediate()
    }
}

#[derive(Debug)]
pub struct UpgradedResolvedType {
    base_type: Arc<JodinType>,
    fields: Vec<Field<UpgradedResolvedType>>,
}

impl GetResolvedMember<Field<UpgradedResolvedType>, UpgradedResolvedType> for UpgradedResolvedType {
    fn get_member(&self, member_id: &Identifier) -> JodinResult<&Field<UpgradedResolvedType>> {
        todo!()
    }
}

impl AsIntermediate for UpgradedResolvedType {
    fn intermediate_type(&self) -> IntermediateType {
        self.base_type.as_intermediate()
    }
}

pub struct FieldResolver {}

pub struct ResolvedTypeBuilder<'types> {
    base_type: Option<Arc<JodinType>>,
    parent_object: Option<ResolvedType>,
    // jtraits: BinaryHeap<JTraitObjectWithDistance<'types>>,
    env: &'types TypeEnvironment,
}

impl<'t> ResolvedTypeBuilder<'t> {
    pub(in crate::core::types) fn new(env: &'t TypeEnvironment) -> Self {
        ResolvedTypeBuilder {
            base_type: Default::default(),
            parent_object: Default::default(),
            // jtraits: Default::default(),
            env,
        }
    }

    pub fn set_base_type(&mut self, ty: &Arc<JodinType>) {
        self.base_type = Some(ty.clone());
    }

    pub fn add_parent_type<'types, T: Type<'types>>(&mut self, parent: &'types T) {
        let big_object = parent.resolve(self.env);
        self.parent_object = Some(big_object);
    }

    pub fn build(self) -> ResolvedType {
        if self.base_type.is_none() {
            panic!("Base Type should be set by now.")
        }
        let mut fields: Vec<Field<ResolvedType>> = vec![];
        let big_o_factory = ResolvedTypeFactory::new(self.env);
        fields.extend(self.base_type.as_ref().unwrap().fields().into_iter().map(
            |Field {
                 vis,
                 jtype: ty,
                 name: id,
             }| { Field::new(vis.clone(), self.env.resolve_type(ty), id.clone()) },
        ));
        let parent = self
            .parent_object
            .into_iter()
            .flat_map(|big_obj| big_obj.fields);
        fields.extend(parent);
        let mut traits: Vec<&JTraitObject> = Vec::new();
        // let mut trait_iter = self.jtraits.into_iter();
        // while let Some(next) = trait_iter.next() {
        //     traits.push(next.object);
        // }
        let base = self.base_type.as_ref().unwrap();
        ResolvedType {
            base_type: Arc::downgrade(base),
            fields,
        }
    }
    pub fn env(&self) -> &'t TypeEnvironment {
        self.env
    }
}

/// A factory for building [`JBigObject`]s.
///
/// [`JBigObject`]: ResolvedType
pub struct ResolvedTypeFactory<'types> {
    env: &'types TypeEnvironment,
}

impl<'types> ResolvedTypeFactory<'types> {
    /// Create a new big object factory
    pub fn new(env: &'types TypeEnvironment) -> Self {
        ResolvedTypeFactory { env }
    }

    /// Create a new instance of a j big object
    pub fn new_instance(&self, ty: &Arc<JodinType>) -> ResolvedType {
        ty.resolve(&self.env)
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

#[cfg(test)]
mod tests {
    use crate::core::error::{JodinError, JodinErrorType, JodinResult};
    use crate::core::types::primitives::Primitive;
    use crate::core::types::resolved_type::ResolvedType;
    use crate::core::types::type_environment::TypeEnvironment;
    use crate::core::types::Type;

    #[test]
    fn upgrade_can_fail() {
        let env = TypeEnvironment::new();
        let jtype: ResolvedType = env.resolve_type(&Primitive::Int);
        println!("{:#?}", jtype);
        drop(env);

        match jtype.upgrade() {
            Ok(_) => {
                panic!("Should fail because environment was dropped")
            }
            Err(e) => match e.error_type {
                JodinErrorType::TypeEnvironmentUnavailable => {}
                _ => panic!("Wrong error type to return"),
            },
        }
    }
}
