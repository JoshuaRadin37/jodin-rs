//! The array types that exist within Jodin

use crate::ast::JodinNode;
use crate::core::identifier::Identifier;
use crate::core::types::intermediate_type::IntermediateType;
use crate::core::types::{get_type_id, Type};

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
#[derive(Debug)]
pub struct Array {
    /// The base type of the array
    pub base_type: IntermediateType,
    /// How the array is being created
    pub array_type: ArrayType,
    type_id: u32,
}

impl Array {
    /// Create a new array type
    pub fn new(base_type: IntermediateType, array_type: ArrayType) -> Self {
        Array {
            base_type,
            array_type,
            type_id: get_type_id(),
        }
    }
}

impl Type<'_, '_> for Array {
    fn type_name(&self) -> Identifier {
        format!("[{} array]", self.base_type).into()
    }

    fn type_id(&self) -> u32 {
        self.type_id
    }
}
