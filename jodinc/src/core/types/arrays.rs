//! The array types that exist within Jodin

use crate::ast::JodinNode;
use crate::core::error::JodinResult;
use crate::core::identifier::Identifier;
use crate::core::types::big_object::JBigObject;
use crate::core::types::intermediate_type::IntermediateType;
use crate::core::types::type_environment::TypeEnvironment;
use crate::core::types::{get_type_id, JodinType, Type};
use crate::utility::Visitor;

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

impl<'n, 't> Visitor<TypeEnvironment<'n>, JodinResult<JBigObject<'t>>> for Array {
    fn accept(&self, environment: &TypeEnvironment<'n>) -> JodinResult<JBigObject<'t>> {
        todo!()
    }
}

impl Into<JodinType> for Array {
    fn into(self) -> JodinType {
        JodinType::Array(self)
    }
}

impl Type<'_, '_> for Array {
    fn type_identifier(&self) -> Identifier {
        format!("[{} array]", self.base_type).into()
    }

    fn type_unique_id(&self) -> u32 {
        self.type_id
    }

    fn as_intermediate(&self) -> IntermediateType {
        self.base_type.clone().with_abstract_array()
    }
}
