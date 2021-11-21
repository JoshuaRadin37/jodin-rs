//! The array types that exist within Jodin

use crate::ast::JodinNode;
use crate::core::error::JodinResult;
use crate::core::identifier::Identifier;
use crate::core::types::intermediate_type::IntermediateType;
use crate::core::types::resolved_type::{ResolveType, ResolvedType};
use crate::core::types::type_environment::TypeEnvironment;
use crate::core::types::{get_type_id, AsIntermediate, JodinType, Type};
use crate::utility::Visitor;
use std::sync::{Arc, Weak};

/// An array type
#[derive(Debug)]
pub enum ArrayType {
    /// An array of repeated elements
    Repeated {
        /// The value being repeated
        value: JodinNode,
        /// The number of repeats
        repeats: usize,
    },
    /// An array created from a list of expressions
    List {
        /// The values in the array
        values: Vec<JodinNode>,
    },
}

/// An Array type
#[derive(Debug, Clone)]
pub struct Array {
    /// The base type of the array
    pub base_type: IntermediateType,
    /// How the array is being created
    pub array_size: Option<usize>,
    type_id: u32,
}

impl Array {
    /// Create a new array type
    pub fn new(base_type: IntermediateType, array_size: Option<usize>) -> Self {
        Array {
            base_type,
            array_size,
            type_id: get_type_id(),
        }
    }
}

impl Into<JodinType> for Array {
    fn into(self) -> JodinType {
        JodinType::Array(self)
    }
}

impl ResolveType for Array {
    fn resolve(&self, environment: &TypeEnvironment) -> ResolvedType {
        todo!()
    }
}

impl Type<'_> for Array {
    fn type_identifier(&self) -> Identifier {
        match self.array_size {
            None => Identifier::new(format!("[{}]", self.base_type)),
            Some(size) => Identifier::new(format!("[{}: {}]", self.base_type, size)),
        }
    }

    fn type_unique_id(&self) -> u32 {
        self.type_id
    }

    fn as_intermediate(&self) -> IntermediateType {
        let inter = self.base_type.intermediate_type();
        match self.array_size {
            None => inter.with_abstract_array(),
            Some(size) => inter.with_presized_array(size),
        }
    }
}
