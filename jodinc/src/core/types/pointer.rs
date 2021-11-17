//! Stores type information for the pointer type
//!

use crate::core::error::JodinResult;
use crate::core::identifier::Identifier;
use crate::core::types::big_object::JBigObject;
use crate::core::types::intermediate_type::IntermediateType;
use crate::core::types::type_environment::TypeEnvironment;
use crate::core::types::{JodinType, Type};
use crate::utility::Visitor;

use super::get_type_id;
lazy_static! {
    static ref POINTER_TYPE_ID: u32 = get_type_id();
}

#[derive(Debug)]
pub struct Pointer {
    inner_jtype: Box<JodinType>,
}

impl Pointer {
    /// Create a new pointer from a pointer
    pub fn new(inner_jtype: JodinType) -> Self {
        Pointer {
            inner_jtype: Box::new(inner_jtype),
        }
    }
}

impl <'t> Visitor<TypeEnvironment, JodinResult<JBigObject<'t>>> for Pointer {
    fn visit(&self, environment: &TypeEnvironment) -> JodinResult<JBigObject<'t>> {
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
        self.inner_jtype.as_intermediate().with_pointer()
    }
}
