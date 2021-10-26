//! Create a generic context for something to exist in

use crate::core::error::JodinResult;
use crate::core::identifier::Identifier;
use crate::core::types::type_environment::TypeEnvironment;
use crate::core::types::{JodinType, Type};

/// A single generic parameter in a generic instance declaration
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum GenericParameter {
    /// Just a simple identifier being declared
    Invariant(Identifier),
    /// An identifier that extends another type
    Covariant {
        /// The type being declared
        declaration: Identifier,
        /// The super class of this type
        super_class: Identifier,
    },
    /// An identifier that is a super type of another type
    Contravariant {
        /// The type being declared
        declaration: Identifier,
        /// The child class of this type
        child_class: Identifier,
    },
}
impl GenericParameter {
    pub fn as_bound(&self, jtype: &JodinType) -> JodinResult<GenericParameterInstance> {
        match self {
            GenericParameter::Invariant(_) => {
                Ok(GenericParameterInstance::Invariant(jtype.type_name()))
            }
            GenericParameter::Covariant { .. } => {
                Ok(GenericParameterInstance::Covariant(jtype.type_name()))
            }
            GenericParameter::Contravariant { .. } => {
                Ok(GenericParameterInstance::Contravariant(jtype.type_name()))
            }
        }
    }
}
/// Represents an instance of the generic
pub enum GenericParameterInstance {
    /// exactly this type
    Invariant(Identifier),
    /// Child of this type
    Covariant(Identifier),
    /// parent of this type
    Contravariant(Identifier),
}

impl GenericParameterInstance {
    /// Checks whether one generic parameter is valid in the position of the other
    pub fn variance_match(&self, other: &Self, type_env: &TypeEnvironment) -> bool {
        use GenericParameterInstance::*;
        match (self, other) {
            (Invariant(inv1), Invariant(inv2)) => inv1 == inv2,
            (Covariant(covar), Invariant(invar)) => type_env.is_child_type(invar, covar),
            (Covariant(covar), Covariant(child)) => type_env.is_child_type(child, covar),
            (Contravariant(contra), Invariant(inv)) => type_env.is_child_type(contra, inv),
            (Contravariant(contra), Contravariant(inv)) => type_env.is_child_type(contra, inv),
            (_, _) => false,
        }
    }
}
