//! The function types
//!
//! They're actually traits btw

use crate::core::types::generic_context::GenericParameter;
use crate::core::types::intermediate_type::IntermediateType;
use crate::core::types::traits::JTrait;
use crate::identifier::Identifier;

/// Functions are traits
pub trait FunctionTrait {
    /// converts the function into a trait
    fn into_trait(self) -> JTrait;
}

/// A function type
pub struct FunctionType {
    name: Identifier,
    generics: Vec<GenericParameter>,
    parameters: Vec<IntermediateType>,
}

impl FunctionType {
    pub fn new(
        name: Identifier,
        generics: Vec<GenericParameter>,
        parameters: Vec<IntermediateType>,
    ) -> Self {
        FunctionType {
            name,
            generics,
            parameters,
        }
    }
}

impl FunctionTrait for FunctionType {
    fn into_trait(self) -> JTrait {
        todo!()
    }
}
