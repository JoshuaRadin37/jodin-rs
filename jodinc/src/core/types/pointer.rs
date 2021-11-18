//! Stores type information for the pointer type
//!

use crate::core::error::JodinResult;
use crate::core::identifier::Identifier;
use crate::core::types::big_object::JBigObject;
use crate::core::types::intermediate_type::IntermediateType;
use crate::core::types::type_environment::TypeEnvironment;
use crate::core::types::{JodinType, Type};
use crate::utility::Visitor;
use std::sync::{Arc, Weak};

use super::get_type_id;
lazy_static! {
    static ref POINTER_TYPE_ID: u32 = get_type_id();
}

#[derive(Debug, Clone)]
pub struct Pointer {
    inner_jtype: Weak<JodinType>,
}

impl Pointer {
    /// Create a new pointer from a pointer
    pub fn new(inner_jtype: &Arc<JodinType>) -> Self {
        Pointer {
            inner_jtype: Arc::downgrade(inner_jtype),
        }
    }
}

impl<'t> Visitor<'t, TypeEnvironment, JodinResult<JBigObject<'t>>> for Pointer {
    fn visit(&'t self, environment: &'t TypeEnvironment) -> JodinResult<JBigObject<'t>> {
        todo!()
    }
}

impl Into<JodinType> for Pointer {
    fn into(self) -> JodinType {
        JodinType::Pointer(self)
    }
}

impl Type<'_> for Pointer {
    fn type_identifier(&self) -> Identifier {
        Identifier::from("ptr")
    }

    fn type_unique_id(&self) -> u32 {
        *POINTER_TYPE_ID
    }

    fn as_intermediate(&self) -> IntermediateType {
        self.inner_jtype
            .upgrade()
            .unwrap()
            .as_intermediate()
            .with_pointer()
    }
}
