use std::cell::{Ref, RefCell};
use std::fmt::{Display, Formatter};
use std::rc::Rc;
use std::sync::atomic::{AtomicU32, Ordering};

use crate::core::error::JodinResult;
use crate::core::identifier::Identifier;
use crate::core::privacy::Privacy;
use crate::core::registry::{Registrable, Registry};
use crate::core::types::primitives::Primitive;
use crate::core::types::structure::Structure;
use crate::parsing::ast::tags::Tag;
use std::any::Any;

pub mod primitives;
pub mod structure;

#[derive(Debug)]
pub enum JodinType {
    Primitive(Primitive),
    Structure(Structure),
}

impl JodinType {
    pub fn as_inner(&self) -> &dyn Type {
        match self {
            JodinType::Primitive(p) => p,
            JodinType::Structure(s) => s,
        }
    }
}

pub trait Type {
    fn type_name(&self) -> Identifier;
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

pub type JodinTypeReference = Rc<RefCell<JodinType>>;

pub static NEXT_TYPE_ID: AtomicU32 = AtomicU32::new(10);

pub fn get_type_id() -> u32 {
    NEXT_TYPE_ID.fetch_add(1, Ordering::AcqRel)
}

pub trait CompoundType: Type {
    fn all_members(&self) -> Vec<(Privacy, Identifier, JodinTypeReference)>;
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
            let new_id = Identifier::with_parent(&this_id, field);
            register.insert_with_identifier(field_type.clone(), new_id);
        }
        register.insert_with_identifier(self.into().into(), this_id)
    }
}

pub struct TypeTag {
    jodin_type: JodinTypeReference,
}

impl TypeTag {
    pub fn jodin_type(&self) -> &JodinTypeReference {
        &self.jodin_type
    }

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
