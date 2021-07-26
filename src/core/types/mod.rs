//! The basis for the type system that jodin supports.

use std::any::Any;
use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::rc::Rc;
use std::sync::atomic::{AtomicU32, Ordering};

use crate::ast::tags::Tag;
use crate::core::error::JodinResult;
use crate::core::identifier::Identifier;
use crate::core::identifier_resolution::{Registrable, Registry};
use crate::core::privacy::Visibility;
use crate::core::types::primitives::Primitive;
use crate::core::types::structure::Structure;

pub mod primitives;
pub mod structure;
pub mod type_tracker;

/// Different types of types within Jodin
#[derive(Debug)]
pub enum JodinType {
    /// A primitive type
    Primitive(Primitive),
    /// The basic [Structure](crate::core::types::structure::Structure) type.
    Structure(Structure),
}

impl JodinType {
    /// Gets the type dynamic object
    pub fn as_inner(&self) -> &dyn Type {
        match self {
            JodinType::Primitive(p) => p,
            JodinType::Structure(s) => s,
        }
    }
}

/// Common methods within the different types that make up jodin
pub trait Type {
    /// The name of the type
    fn type_name(&self) -> Identifier;
    /// The unique id for this type
    fn type_id(&self) -> u32;
}

impl Display for JodinType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.type_name())
    }
}

impl Type for JodinType {
    fn type_name(&self) -> Identifier {
        self.as_inner().type_name()
    }

    fn type_id(&self) -> u32 {
        self.as_inner().type_id()
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
pub trait CompoundType: Type {
    /// Gets all the members of the compound type.
    fn all_members(&self) -> Vec<(Visibility, Identifier, JodinTypeReference)>;
}

impl From<JodinType> for JodinTypeReference {
    fn from(t: JodinType) -> Self {
        Rc::new(RefCell::new(t))
    }
}

impl<C: CompoundType + Into<JodinType>> Registrable<JodinTypeReference> for C {
    fn register(self, register: &mut Registry<JodinTypeReference>) -> JodinResult<Identifier> {
        let this_id = self.type_name();
        for (_, field, field_type) in self.all_members() {
            let new_id = Identifier::new_concat(&this_id, field);
            register.insert_with_identifier(field_type.clone(), new_id);
        }
        register.insert_with_identifier(self.into().into(), this_id)
    }
}

/// A tag for assigning a type to an AST node
pub struct TypeTag {
    jodin_type: JodinTypeReference,
}

impl TypeTag {
    /// The type.
    pub fn jodin_type(&self) -> &JodinTypeReference {
        &self.jodin_type
    }

    /// Create a new type tag.
    pub fn new(jodin_type: JodinTypeReference) -> Self {
        TypeTag { jodin_type }
    }
}

impl Tag for TypeTag {
    fn tag_type(&self) -> String {
        "Type".to_string()
    }

    fn tag_info(&self) -> String {
        format!("[{}]", self.jodin_type.borrow().type_name())
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
