//! Types that are completely resolved. Formally known as big objects

use crate::error::{JodinErrorType, JodinResult};
use crate::identifier::Identifier;
use crate::types::intermediate_type::IntermediateType;
use crate::types::traits::JTraitObject;
use crate::types::type_environment::TypeEnvironment;
use crate::types::{AsIntermediate, Field, GetResolvedMember, JodinType, Type};

use std::cmp::Ordering;

use std::sync::{Arc, Weak};

/// A trait that should be implemented to take some jodin type and create a fully resolved type out of it
pub trait ResolveType {
    /// Resolve a type within a type environment.
    fn resolve(&self, environment: &TypeEnvironment) -> WeakResolvedType;
}

/// A helper trait to build a resolve type. Implementing this trait also implements [ResolveType](ResolveType)
pub trait BuildResolvedType<'types> {
    fn build_resolved_type(&self, builder: &mut ResolvedTypeBuilder);
}

impl<'types, T: BuildResolvedType<'types>> ResolveType for T {
    fn resolve(&self, environment: &TypeEnvironment) -> WeakResolvedType {
        let mut builder = ResolvedTypeBuilder::new(environment);
        self.build_resolved_type(&mut builder);
        builder.build()
    }
}

/// A JBigObject is a type formed using references
#[derive(Debug)]
pub struct WeakResolvedType {
    base_type: Weak<JodinType>,
    fields: Vec<Field<WeakResolvedType>>,
}

impl WeakResolvedType {
    /// Creates a new resolved type
    pub fn new(base_type: &Arc<JodinType>, fields: Vec<Field<WeakResolvedType>>) -> Self {
        WeakResolvedType {
            base_type: Arc::downgrade(base_type),
            fields,
        }
    }

    /// Upgrade a resolved type into a strongly associated type
    pub fn upgrade(&self) -> JodinResult<ResolvedType> {
        let mut fields = vec![];
        for field in &self.fields {
            fields.push(field.upgrade()?);
        }
        Ok(ResolvedType {
            base_type: self
                .base_type
                .upgrade()
                .ok_or(JodinErrorType::TypeEnvironmentUnavailable)?,
            fields,
        })
    }
}

impl AsIntermediate for WeakResolvedType {
    fn intermediate_type(&self) -> IntermediateType {
        self.base_type.upgrade().unwrap().as_intermediate()
    }
}

#[derive(Debug)]
pub struct ResolvedType {
    base_type: Arc<JodinType>,
    fields: Vec<Field<ResolvedType>>,
}

impl GetResolvedMember<Field<ResolvedType>, ResolvedType> for ResolvedType {
    fn get_member(&self, member_id: &Identifier) -> JodinResult<&Field<ResolvedType>> {
        self.fields
            .iter()
            .find(|field| field.name.clone().this() == member_id.to_string())
            .ok_or(JodinErrorType::IdentifierDoesNotExist(member_id.clone()).into())
    }
}

impl AsIntermediate for ResolvedType {
    fn intermediate_type(&self) -> IntermediateType {
        self.base_type.as_intermediate()
    }
}

pub struct ResolvedTypeBuilder<'types> {
    base_type: Option<Arc<JodinType>>,
    parent_object: Option<WeakResolvedType>,
    // jtraits: BinaryHeap<JTraitObjectWithDistance<'types>>,
    env: &'types TypeEnvironment,
}

impl<'t> ResolvedTypeBuilder<'t> {
    pub fn new(env: &'t TypeEnvironment) -> Self {
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

    pub fn build(self) -> WeakResolvedType {
        if self.base_type.is_none() {
            panic!("Base Type should be set by now.")
        }
        let mut fields: Vec<Field<WeakResolvedType>> = vec![];
        let _big_o_factory = ResolvedTypeFactory::new(self.env);
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
        let _traits: Vec<&JTraitObject> = Vec::new();
        // let mut trait_iter = self.jtraits.into_iter();
        // while let Some(next) = trait_iter.next() {
        //     traits.push(next.object);
        // }
        let base = self.base_type.as_ref().unwrap();
        WeakResolvedType {
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
    pub fn new_instance(&self, ty: &Arc<JodinType>) -> WeakResolvedType {
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
    use crate::error::JodinErrorType;
    use crate::types::primitives::Primitive;
    use crate::types::resolved_type::WeakResolvedType;
    use crate::types::type_environment::TypeEnvironment;

    #[test]
    fn upgrade_can_fail() {
        let env = TypeEnvironment::new();
        let jtype: WeakResolvedType = env.resolve_type(&Primitive::Int);
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
