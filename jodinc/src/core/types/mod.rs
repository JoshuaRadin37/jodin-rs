//! The basis for the type system that jodin supports.

use std::any::Any;
use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::ops::Index;
use std::rc::Rc;
use std::sync::atomic::{AtomicU32, Ordering};

use crate::ast::JodinNode;
use intermediate_type::IntermediateType;

use crate::ast::tags::Tag;
use crate::core::error::{JodinError, JodinErrorType, JodinResult};
use crate::core::identifier::Identifier;
use crate::core::identifier_resolution::{Registrable, Registry};
use crate::core::privacy::Visibility;
use crate::core::types::arrays::Array;
use crate::core::types::big_object::{JBigObject, JObject};
use crate::core::types::pointer::Pointer;
use crate::core::types::primitives::Primitive;
use crate::core::types::structure::Structure;
use crate::core::types::traits::{JTrait, JTraitObject};
use crate::core::types::type_environment::TypeEnvironment;
use crate::utility::Visitor;

pub mod arrays;
pub mod base_type;
pub mod big_object;
pub mod functions;
pub mod generic_context;
pub mod intermediate_type;
pub mod pointer;
pub mod primitives;
pub mod structure;
pub mod traits;
pub mod type_environment;

/// Common methods within the different types that make up jodin
pub trait Type<'n, 't>:
    Visitor<TypeEnvironment<'n>, JodinResult<JBigObject<'t>>> + Into<JodinType>
{
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

impl AsIntermediate for IntermediateType {
    fn intermediate_type(&self) -> IntermediateType {
        self.clone()
    }
}

impl<'n, 't, T: Type<'n, 't>> AsIntermediate for T {
    fn intermediate_type(&self) -> IntermediateType {
        self.as_intermediate()
    }
}

/// Different types of types within Jodin
#[derive(Debug)]
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
    // /// Gets the type dynamic object
    // pub fn as_inner(&self) -> Box<Type> {
    //     match self {
    //         JodinType::Primitive(p) => p,
    //         JodinType::Structure(s) => s,
    //         JodinType::Array(a) => a,
    //         JodinType::JTraitObject(o) => o,
    //         JodinType::JTrait(t) => t,
    //         JodinType::JObject(o) => o,
    //         JodinType::Pointer(ptr) => ptr,
    //     }
    // }
}

macro_rules! on_inner {
    ($obj:expr, |$var:ident| $cls:expr, $ret_ty:ty) => {{
        fn __apply<'n, 't, T: Type<'n, 't>>($var: &T) -> $ret_ty {
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

impl<'n, 't> Visitor<TypeEnvironment<'n>, JodinResult<JBigObject<'t>>> for JodinType {
    fn accept(&self, environment: &TypeEnvironment<'n>) -> JodinResult<JBigObject<'t>> {
        todo!()
    }
}

impl Type<'_, '_> for JodinType {
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
pub trait CompoundType<'n, 't>: Type<'n, 't> {
    /// Gets all the members of the compound type.
    fn all_members(&self) -> Vec<(&Visibility, &IntermediateType, &Identifier)>;

    /// Try to get a specific member by name
    fn get_field<I: Into<Identifier>>(
        &self,
        id: I,
    ) -> JodinResult<(&Visibility, &IntermediateType, &Identifier)> {
        let id = id.into();
        self.all_members()
            .into_iter()
            .find(|(vis, ty, other_id)| *other_id == &id)
            .ok_or(JodinError::new(JodinErrorType::IdentifierDoesNotExist(id)))
    }
}

pub trait Member: Sized {
    fn jtype(&self) -> &IntermediateType;
    fn id(&self) -> &Identifier;
}

impl Member for (IntermediateType, Identifier) {
    fn jtype(&self) -> &IntermediateType {
        &self.0
    }

    fn id(&self) -> &Identifier {
        &self.1
    }
}

impl Member for (Visibility, IntermediateType, Identifier) {
    fn jtype(&self) -> &IntermediateType {
        &self.1
    }

    fn id(&self) -> &Identifier {
        &self.2
    }
}

impl Member for (&IntermediateType, &Identifier) {
    fn jtype(&self) -> &IntermediateType {
        &self.0
    }

    fn id(&self) -> &Identifier {
        &self.1
    }
}

impl Member for (&Visibility, &IntermediateType, &Identifier) {
    fn jtype(&self) -> &IntermediateType {
        &self.1
    }

    fn id(&self) -> &Identifier {
        &self.2
    }
}

trait GetResolvedMember<M: Member> {
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
#[derive(Debug, PartialEq)]
pub struct Field {
    /// The visibility of the field
    pub vis: Visibility,
    /// The type of the field
    pub jtype: IntermediateType,
    /// The name of the field
    pub name: Identifier,
}

impl Field {
    /// Create a new field instance
    pub fn new(vis: Visibility, jtype: IntermediateType, name: Identifier) -> Self {
        Field { vis, jtype, name }
    }

    /// Turns this field into a tuple
    pub fn into_tuple(self) -> (Visibility, IntermediateType, Identifier) {
        let Field { vis, jtype, name } = self;
        (vis, jtype, name)
    }

    /// Gets this field as a tuple
    pub fn as_tuple(&self) -> (&Visibility, &IntermediateType, &Identifier) {
        let Field {
            ref vis,
            ref jtype,
            ref name,
        } = self;
        (vis, jtype, name)
    }
}

impl Member for Field {
    fn jtype(&self) -> &IntermediateType {
        &self.jtype
    }

    fn id(&self) -> &Identifier {
        &self.name
    }
}

/// Trait to define a way to build a type
pub trait BuildType<'n, 't>: Sized + Type<'n, 't> {
    /// The function that builds this type.
    fn build_type(
        node: &JodinNode,
        env: &TypeEnvironment<'n>,
        target_type: Option<&IntermediateType>,
    ) -> JodinResult<Self>;
}
