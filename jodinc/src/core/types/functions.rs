//! The function types
//!
//! They're actually traits btw

use crate::core::types::intermediate_type::IntermediateType;
use crate::core::identifier::Identifier;
use crate::core::types::generic_context::GenericParameter;
use crate::core::types::traits::JTrait;

/// Functions are traits
pub trait FunctionTrait {
    fn into_trait(self) -> JTrait;
}

pub struct FunctionType {
    name: Identifier,
    generics: Vec<GenericParameter>,
    parameters: Vec<IntermediateType>
}

impl FunctionType {
    pub fn new(name: Identifier, generics: Vec<GenericParameter>, parameters: Vec<IntermediateType>) -> Self {
        FunctionType { name, generics, parameters }
    }

}

impl FunctionTrait for FunctionType {
    fn into_trait(self) -> JTrait {
        todo!()
    }
}


