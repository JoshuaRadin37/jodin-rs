//! Create a generic context for something to exist in

use super::identifier::Identifier;

/// A single generic parameter in a generic declaration
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
