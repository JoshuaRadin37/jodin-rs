//! An intermediate type is a way of creating a type without actually *creating* the type.
//!
//! Intermediate types will eventually turned into a [JodinTypeReference] during one of the passes of the
//! compiler, after all identifiers have been resolved.
//!
//! The intermediate type can represent any canonical type that is supported by Jodin. Intermediate
//! types are used to declare types, and only refer to them.
//!
//! # Examples
//!
//! A function pointer that takes in two integer pointers as input and an integer as an output would be
//! defined as `int (int*, int*)` in jodin. This would results in an `IntermediateType` with this value:
//! ```
//! use jodin_rs::ast::intermediate_type::{IntermediateType, TypeSpecifier, TypeTail};
//! use jodin_rs::core::types::primitives::Primitive;
//! let i_type = IntermediateType {
//!     is_const: false,
//!     type_specifier: TypeSpecifier::Primitive(Primitive::Int),
//!     generics: vec![],
//!     tails: vec![TypeTail::Function(vec![
//!         IntermediateType {
//!             is_const: false,
//!             type_specifier: TypeSpecifier::Primitive(Primitive::Int),
//!             generics: vec![],
//!             tails: vec![TypeTail::Pointer]
//!         },
//!         IntermediateType {
//!             is_const: false,
//!             type_specifier: TypeSpecifier::Primitive(Primitive::Int),
//!             generics: vec![],
//!             tails: vec![TypeTail::Pointer]
//!         }
//!     ])]
//! };
//!
//! let as_string = i_type.to_string();
//! assert_eq!(as_string, "int(int*, int*)");
//! ```
//!
//! [JodinTypeReference]: crate::core::types::JodinTypeReference

use crate::ast::jodin_node::JodinNode;
use crate::core::identifier::Identifier;
use crate::core::types::primitives::Primitive;
use std::fmt::{Display, Formatter};

use itertools::Itertools;

/// Contains data to represent types without storing any actual type information.
#[derive(Debug)]
pub struct IntermediateType {
    /// Whether this type is constant.
    pub is_const: bool,
    /// The type specifier.
    pub type_specifier: TypeSpecifier,
    /// The generics elements of this type.
    pub generics: Vec<IntermediateType>,
    /// The "tails", which are either a pointer `*`, an array declaration, or a function call
    pub tails: Vec<TypeTail>,
}

impl IntermediateType {
    /// Creates a new intermediate type instance
    pub fn new(
        is_const: bool,
        type_specifier: TypeSpecifier,
        generics: Vec<IntermediateType>,
        tails: Vec<TypeTail>,
    ) -> Self {
        IntermediateType {
            is_const,
            type_specifier,
            generics,
            tails,
        }
    }
}

impl Display for IntermediateType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.is_const {
            write!(f, "const ")?;
        }
        write!(f, "{}", self.type_specifier)?;
        if !self.generics.is_empty() {
            write!(
                f,
                "<{}>",
                Itertools::intersperse(
                    self.generics.iter().map(|gen| gen.to_string()),
                    ",".to_string()
                )
                .collect::<String>()
            )?;
        }
        for tail in &self.tails {
            write!(f, "{}", tail)?;
        }
        Ok(())
    }
}

/// A type specifier can either be a built in primitive, or an identifier
#[derive(Debug)]
pub enum TypeSpecifier {
    /// An identifier referring to a type, such as `std::object`
    Id(Identifier),
    /// A built-in data type, such as `float` or `unsigned int`
    Primitive(Primitive),
}

impl Display for TypeSpecifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeSpecifier::Id(id) => {
                write!(f, "{}", id)
            }
            TypeSpecifier::Primitive(p) => {
                write!(f, "{}", p)
            }
        }
    }
}

/// Contains a tail for the type, which are modifiers on a base type that expands the functionality of it
#[derive(Debug)]
pub enum TypeTail {
    /// A pointer is one level of indirection from a data type.
    Pointer,
    /// An array is a contiguous block of memory of a type. The size is optional. An array with no
    /// size is equivalent to a pointer.
    Array(Option<JodinNode>),
    /// Turns the type into a function pointer.
    Function(Vec<IntermediateType>),
}

impl Display for TypeTail {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeTail::Pointer => {
                write!(f, "*")
            }
            TypeTail::Array(inner) => match inner {
                None => {
                    write!(f, "[]")
                }
                Some(_) => {
                    write!(f, "[...]")
                }
            },
            TypeTail::Function(input_types) => {
                write!(
                    f,
                    "({})",
                    input_types
                        .iter()
                        .map(|input_type| { format!("{}", input_type) })
                        .join(",")
                )
            }
        }
    }
}