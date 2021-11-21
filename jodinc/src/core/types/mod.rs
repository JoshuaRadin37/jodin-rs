//! The basis for the type system that jodin supports.

use std::any::Any;
use std::cell::RefCell;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Index;
use std::rc::Rc;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Weak;

use crate::ast::JodinNode;
use intermediate_type::IntermediateType;
use resolved_type::ResolvedType;

use crate::ast::tags::Tag;
use crate::core::error::{JodinError, JodinErrorType, JodinResult};
use crate::core::identifier::Identifier;
use crate::core::identifier_resolution::{Registrable, Registry};
use crate::core::privacy::Visibility;
use crate::core::types::arrays::Array;
use crate::core::types::jobject::JObject;
use crate::core::types::pointer::Pointer;
use crate::core::types::primitives::Primitive;
use crate::core::types::resolved_type::{ResolveType, UpgradedResolvedType};
use crate::core::types::structure::Structure;
use crate::core::types::traits::{JTrait, JTraitObject};
use crate::core::types::type_environment::TypeEnvironment;
use crate::utility::Visitor;

pub mod arrays;
pub mod base_type;
pub mod functions;
pub mod generic_context;
pub mod intermediate_type;
pub mod jobject;
pub mod pointer;
pub mod primitives;
pub mod resolved_type;
pub mod structure;
pub mod traits;
pub mod type_environment;

/// Common methods within the different types that make up jodin
pub trait Type<'t>: ResolveType + Into<JodinType> {
    /// The name of the type
    fn type_identifier(&self) -> Identifier;
    /// The unique id for this type
    fn type_unique_id(&self) -> u32;

    /// Creates an intermediate representation of this type
    fn as_intermediate(&self) -> IntermediateType {
        IntermediateType::from(self.type_identifier())
    }
}

pub trait AsIntermediate {
    fn intermediate_type(&self) -> IntermediateType;
}

impl<'t, T: Type<'t>> AsIntermediate for T {
    fn intermediate_type(&self) -> IntermediateType {
        self.as_intermediate()
    }
}

/// Different types of types within Jodin
#[derive(Debug, Clone)]
pub enum JodinType {
    /// A primitive type
    Primitive(Primitive),
    /// An array type
    Array(Array),
    /// The basic [Structure](crate::core::types::structure::Structure) type.
    Structure(Structure),
    Pointer(Pointer),
    /// Effectively a Jtrait with more type info
    JTraitObject(JTraitObject),
    JTrait(JTrait),
    JObject(JObject),
}
impl JodinType {
    /// Tries to get a field. If the base type isn't a compound type a `None` value is returned.
    pub fn get_field<I: Into<Identifier>>(&self, id: I) -> Option<&Field<IntermediateType>> {
        match self {
            JodinType::Structure(s) => s.get_field(id).ok(),
            // JodinType::JTraitObject(o) => o.get_field(id).ok(),
            // JodinType::JTrait(t) => t.get_field(id).ok(),
            JodinType::JObject(o) => o.get_field(id).ok(),
            _ => None,
        }
    }

    pub fn fields(&self) -> Vec<&Field<IntermediateType>> {
        match self {
            JodinType::Structure(s) => s.all_members(),
            // JodinType::JTraitObject(o) => o.get_field(id).ok(),
            // JodinType::JTrait(t) => t.get_field(id).ok(),
            JodinType::JObject(o) => o.all_members(),
            _ => vec![],
        }
    }
}

macro_rules! on_inner {
    ($obj:expr, |$var:ident| $cls:expr, $ret_ty:ty) => {{
        fn __apply<'t, T: Type<'t>>($var: &T) -> $ret_ty {
            $cls
        }
        match &($obj) {
            JodinType::Primitive(p) => __apply(p),
            JodinType::Structure(p) => __apply(p),
            JodinType::Array(p) => __apply(p),
            JodinType::JTraitObject(p) => __apply(p),
            JodinType::JTrait(p) => __apply(p),
            JodinType::JObject(p) => __apply(p),
            JodinType::Pointer(p) => __apply(p),
        }
    }};
}

impl Display for JodinType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.type_identifier())
    }
}

impl ResolveType for JodinType {
    fn resolve(&self, environment: &TypeEnvironment) -> ResolvedType {
        match self {
            JodinType::Primitive(v) => environment.resolve_type(v),
            JodinType::Array(v) => environment.resolve_type(v),
            JodinType::Structure(v) => environment.resolve_type(v),
            JodinType::Pointer(v) => environment.resolve_type(v),
            JodinType::JTraitObject(v) => environment.resolve_type(v),
            JodinType::JTrait(v) => environment.resolve_type(v),
            JodinType::JObject(v) => environment.resolve_type(v),
        }
    }
}

impl Type<'_> for JodinType {
    fn type_identifier(&self) -> Identifier {
        on_inner!(self, |v| v.type_identifier(), Identifier)
    }

    fn type_unique_id(&self) -> u32 {
        on_inner!(self, |v| v.type_unique_id(), u32)
    }

    fn as_intermediate(&self) -> IntermediateType {
        on_inner!(self, |v| v.as_intermediate(), IntermediateType)
    }
}

/// A type that allows for multiple references to the same type
pub type JodinTypeReference = Rc<RefCell<JodinType>>;

/// The next type id.
pub static NEXT_TYPE_ID: AtomicU32 = AtomicU32::new(100);

/// Get a type id
pub fn get_type_id() -> u32 {
    NEXT_TYPE_ID.fetch_add(1, Ordering::AcqRel)
}

/// Common methods for compound types in jodin.
pub trait CompoundType<'t>: Type<'t> {
    /// Gets all the members of the compound type.
    fn all_members(&self) -> Vec<&Field<IntermediateType>>;

    /// Try to get a specific member by name
    fn get_field<I: Into<Identifier>>(&self, id: I) -> JodinResult<&Field<IntermediateType>> {
        let id = id.into();
        self.all_members()
            .into_iter()
            .find(
                |Field {
                     vis: _,
                     jtype: _,
                     name: other_id,
                 }| other_id == &id,
            )
            .ok_or(JodinError::new(JodinErrorType::IdentifierDoesNotExist(id)))
    }
}

pub trait Member<T>: Sized {
    fn jtype(&self) -> &T;
    fn id(&self) -> &Identifier;
}

impl Member<IntermediateType> for (IntermediateType, Identifier) {
    fn jtype(&self) -> &IntermediateType {
        &self.0
    }

    fn id(&self) -> &Identifier {
        &self.1
    }
}

impl Member<IntermediateType> for (Visibility, IntermediateType, Identifier) {
    fn jtype(&self) -> &IntermediateType {
        &self.1
    }

    fn id(&self) -> &Identifier {
        &self.2
    }
}

impl Member<IntermediateType> for (&IntermediateType, &Identifier) {
    fn jtype(&self) -> &IntermediateType {
        &self.0
    }

    fn id(&self) -> &Identifier {
        &self.1
    }
}

impl Member<IntermediateType> for (&Visibility, &IntermediateType, &Identifier) {
    fn jtype(&self) -> &IntermediateType {
        &self.1
    }

    fn id(&self) -> &Identifier {
        &self.2
    }
}

pub trait GetResolvedMember<M, T>
where
    M: Member<T>,
{
    /// Get's a resolved member
    fn get_member(&self, member_id: &Identifier) -> JodinResult<&M>;
}

impl From<JodinType> for JodinTypeReference {
    fn from(t: JodinType) -> Self {
        Rc::new(RefCell::new(t))
    }
}

/// A tag for assigning a type to an AST node
pub struct TypeTag {
    jodin_type: IntermediateType,
}

impl TypeTag {
    /// The type.
    pub fn jodin_type(&self) -> &IntermediateType {
        &self.jodin_type
    }

    /// Create a new type tag.
    pub fn new(jodin_type: IntermediateType) -> Self {
        TypeTag { jodin_type }
    }
}

impl Tag for TypeTag {
    fn tag_type(&self) -> String {
        "Type".to_string()
    }

    fn tag_info(&self) -> String {
        format!("[{}]", self.jodin_type)
    }

    fn max_of_this_tag(&self) -> u32 {
        1
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

/// Represents a way of storing variable
#[derive(Debug)]
pub enum StorageModifier {
    /// A locally stored variable
    Local,
    /// A global variable
    Static,
    /// An immutable variable (also global)
    Const,
}

/// A field in some sort of compound structure
#[derive(Debug, PartialEq, Clone)]
pub struct Field<T> {
    /// The visibility of the field
    pub vis: Visibility,
    /// The type of the field
    pub jtype: T,
    /// The name of the field
    pub name: Identifier,
}

impl<T> Field<T> {
    /// Create a new field instance
    pub fn new(vis: Visibility, jtype: T, name: Identifier) -> Self {
        Field { vis, jtype, name }
    }

    /// Turns this field into a tuple
    pub fn into_tuple(self) -> (Visibility, T, Identifier) {
        let Field { vis, jtype, name } = self;
        (vis, jtype, name)
    }

    /// Gets this field as a tuple
    pub fn as_tuple(&self) -> (&Visibility, &T, &Identifier) {
        let Field {
            ref vis,
            ref jtype,
            ref name,
        } = self;
        (vis, jtype, name)
    }
}

impl Field<ResolvedType> {
    pub fn upgrade(&self) -> JodinResult<Field<UpgradedResolvedType>> {
        let upgraded = self.jtype.upgrade()?;
        Ok(Field {
            vis: self.vis.clone(),
            jtype: upgraded,
            name: self.name.clone(),
        })
    }
}

impl<T> Member<T> for Field<T> {
    fn jtype(&self) -> &T {
        &self.jtype
    }

    fn id(&self) -> &Identifier {
        &self.name
    }
}

/// Trait to define a way to build a type
pub trait BuildType<'t>: Sized + Type<'t> {
    /// The function that builds this type.
    fn build_type(
        node: &JodinNode,
        env: &TypeEnvironment,
        target_type: Option<&IntermediateType>,
    ) -> JodinResult<Self>;
}
