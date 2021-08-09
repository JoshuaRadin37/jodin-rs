//! Responsible for managing types and translations from intermediate types.
//!
//! Used to determine type checking.

use crate::ast::intermediate_type::{IntermediateType, TypeSpecifier, TypeTail};
use std::collections::HashMap;
use crate::core::identifier::Identifier;
use crate::core::types::JodinType;
use crate::core::types::primitives::Primitive;

/// Stores a lot of information about types and related identifier
pub struct TypeEnvironment {
    types: HashMap<Identifier, JodinType>
}

impl TypeEnvironment {
    /// Create a new type environment
    pub fn new() -> Self {
         Self {
             types: HashMap::new()
         }
    }

    /// Checks whether the first argument can be considered the second type
    ///
    /// # Notable checks for type safety
    /// 1. void* is everything
    pub fn loosely_is(&self, my_type: &IntermediateType, target_type: &IntermediateType) -> bool {
        if Self::is_void_ptr(target_type) && Self::is_ptr(my_type) {
            return true;
        }

        if Self::is_ptr(my_type) && Self::is_ptr(target_type) {
            return self.loosely_is(&my_type.get_deref().unwrap(), &target_type.get_deref().unwrap());
        }



        false
    }

    /// Gets whether this a void*
    pub fn is_void_ptr(inter: &IntermediateType) -> bool {
        if let IntermediateType {
            is_const: false, type_specifier: TypeSpecifier::Primitive(Primitive::Void), generics, tails
        } = inter {
            generics.is_empty() && &*tails == &[TypeTail::Pointer]
        } else {
            false
        }
    }

    /// Whether this a pointer
    pub fn is_ptr(inter: &IntermediateType) -> bool {
        match inter.tails.last() {
            Some(TypeTail::Pointer) => true,
            Some(TypeTail::Array(_)) => true,
            _ => false
        }
    }


}
