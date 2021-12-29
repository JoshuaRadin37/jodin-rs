//! The function types
//!
//! They're actually traits btw

use crate::identifier::Identifier;
use crate::types::generic_context::GenericParameter;
use crate::types::intermediate_type::IntermediateType;
use crate::types::traits::JTrait;

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
