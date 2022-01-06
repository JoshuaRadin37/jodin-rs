//! Stores type information for the pointer type
//!


use crate::identifier::Identifier;
use crate::types::intermediate_type::IntermediateType;
use crate::types::resolved_type::{ResolveType, WeakResolvedType};
use crate::types::type_environment::TypeEnvironment;
use crate::types::{JodinType, Type};



use super::get_type_id;
lazy_static::lazy_static! {
    static ref POINTER_TYPE_ID: u32 = get_type_id();
}

#[derive(Debug, Clone)]
pub struct Pointer {
    inner_jtype: IntermediateType,
}

impl Pointer {
    /// Create a new pointer from a pointer
    pub fn new(inner_jtype: IntermediateType) -> Self {
        Pointer { inner_jtype }
    }
}

impl Into<JodinType> for Pointer {
    fn into(self) -> JodinType {
        JodinType::Pointer(self)
    }
}

impl ResolveType for Pointer {
    fn resolve(&self, _environment: &TypeEnvironment) -> WeakResolvedType {
        todo!()
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
        self.inner_jtype.clone().with_pointer()
    }
}
