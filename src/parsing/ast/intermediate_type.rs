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
//! use jodin_rs::parsing::ast::intermediate_type::{IntermediateType, TypeSpecifier, TypeTail};
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

use crate::core::identifier::Identifier;
use crate::core::types::primitives::Primitive;
use crate::parsing::ast::jodin_node::JodinNode;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct IntermediateType {
    pub is_const: bool,
    pub type_specifier: TypeSpecifier,
    pub generics: Vec<IntermediateType>,
    pub tails: Vec<TypeTail>,
}

impl IntermediateType {
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
                self.generics.iter().map(|gen| gen.to_string()).join(",")
            )
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum TypeSpecifier {
    Id(Identifier),
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

#[derive(Debug)]
pub enum TypeTail {
    Pointer,
    Array(Option<JodinNode>),
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
