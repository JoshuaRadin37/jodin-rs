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
//! assert_eq!(as_string, "int(int*,int*)");
//! ```
//!
//! [JodinTypeReference]: crate::core::types::JodinTypeReference

use crate::ast::jodin_node::JodinNode;
use crate::core::identifier::Identifier;
use crate::core::types::primitives::Primitive;
use std::fmt::{Display, Formatter};

use crate::core::error::{JodinErrorType, JodinResult};
use crate::core::types::generic_context::GenericParameter;
use itertools::Itertools;

/// Contains data to represent types without storing any actual type information.
#[derive(Debug, Eq, PartialEq)]
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

    /// Creates a void (empty) type
    pub fn void() -> Self {
        Self::from(Primitive::Void)
    }

    /// Make this type constant if it isn't already
    pub fn into_const(mut self) -> Self {
        self.is_const = true;
        self
    }

    /// Adds generics to a type
    pub fn with_generics<I: IntoIterator<Item = IntermediateType>>(mut self, generics: I) -> Self {
        self.generics.extend(generics);
        self
    }

    /// Adds a pointer to a type
    pub fn with_pointer(mut self) -> Self {
        self.tails.push(TypeTail::Pointer);
        self
    }

    /// Adds an abstract array
    pub fn with_abstract_array(mut self) -> Self {
        self.tails.push(TypeTail::Array(None));
        self
    }

    /// Adds an array with an index
    pub fn with_array(mut self, index: JodinNode) -> Self {
        self.tails.push(TypeTail::Array(Some(index)));
        self
    }

    /// Tries to make this type unsigned
    pub fn into_unsigned(mut self) -> Self {
        let new_type = match self.type_specifier {
            TypeSpecifier::Primitive(Primitive::Char) => Primitive::UnsignedByte,
            TypeSpecifier::Primitive(Primitive::Int) => Primitive::UnsignedShort,
            TypeSpecifier::Primitive(Primitive::Short) => Primitive::UnsignedInt,
            TypeSpecifier::Primitive(Primitive::Long) => Primitive::UnsignedLong,
            r#else => panic!("{:?} can not be made unsigned", r#else),
        };

        self.type_specifier = TypeSpecifier::Primitive(new_type);
        self
    }

    /// Adds parameters to this type
    pub fn with_function_params<I: IntoIterator<Item = IntermediateType>>(
        mut self,
        param_types: I,
    ) -> Self {
        self.tails
            .push(TypeTail::Function(param_types.into_iter().collect()));
        self
    }

    fn lose_info(&self) -> Self {
        let IntermediateType {
            is_const,
            type_specifier,
            generics,
            tails,
        } = self;

        IntermediateType {
            is_const: *is_const,
            type_specifier: type_specifier.clone(),
            generics: generics.iter().map(|int| int.lose_info()).collect(),
            tails: tails
                .iter()
                .map(|tail| match tail {
                    TypeTail::Pointer => TypeTail::Pointer,
                    TypeTail::Array(_) => TypeTail::Array(None),
                    TypeTail::Function(f) => {
                        TypeTail::Function(f.iter().map(|im| im.lose_info()).collect())
                    }
                })
                .collect(),
        }
    }

    /// Get a pointer to this type
    pub fn get_pointer(&self) -> IntermediateType {
        let Self {
            is_const,
            type_specifier,
            generics,
            mut tails,
        } = self.lose_info();
        tails.push(TypeTail::Pointer);
        Self {
            is_const,
            type_specifier,
            generics,
            tails,
        }
    }

    /// Dereference this type
    pub fn get_deref(&self) -> JodinResult<IntermediateType> {
        let Self {
            is_const,
            type_specifier,
            generics,
            mut tails,
        } = self.lose_info();
        match tails.pop() {
            Some(TypeTail::Pointer) => {}
            Some(_) | None => {
                return Err(JodinErrorType::TypeCantBeDereferenced(self.to_string()).into());
            }
        }
        Ok(Self {
            is_const,
            type_specifier,
            generics,
            tails,
        })
    }

    /// Get this type indexed (only works on array types)
    pub fn get_indexed(&self) -> JodinResult<IntermediateType> {
        let Self {
            is_const,
            type_specifier,
            generics,
            mut tails,
        } = self.lose_info();
        match tails.pop() {
            Some(TypeTail::Array(_)) => {}
            Some(_) | None => {
                return Err(JodinErrorType::TypeCantBeDereferenced(self.to_string()).into());
            }
        }
        Ok(Self {
            is_const,
            type_specifier,
            generics,
            tails,
        })
    }

    /// Get this type indexed (only works on array types)
    pub fn get_called(&self) -> JodinResult<IntermediateType> {
        let Self {
            is_const,
            type_specifier,
            generics,
            mut tails,
        } = self.lose_info();
        match tails.pop() {
            Some(TypeTail::Function(_)) => {}
            Some(_) | None => {
                return Err(JodinErrorType::TypeCantBeDereferenced(self.to_string()).into());
            }
        }

        Ok(Self {
            is_const,
            type_specifier,
            generics,
            tails,
        })
    }
}

impl From<Primitive> for IntermediateType {
    fn from(p: Primitive) -> Self {
        Self {
            is_const: false,
            type_specifier: TypeSpecifier::Primitive(p),
            generics: vec![],
            tails: vec![],
        }
    }
}

impl From<Identifier> for IntermediateType {
    fn from(id: Identifier) -> Self {
        Self {
            is_const: false,
            type_specifier: TypeSpecifier::Id(id),
            generics: vec![],
            tails: vec![],
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
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum TypeSpecifier {
    /// An identifier referring to a type, such as `std::object`
    Id(Identifier),
    /// A built-in data type, such as `float` or `unsigned int`
    Primitive(Primitive),
    /// A generic parameter
    Generic(GenericParameter),
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
            TypeSpecifier::Generic(_g) => {
                todo!()
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

impl PartialEq for TypeTail {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (TypeTail::Pointer, TypeTail::Pointer) => true,
            (TypeTail::Array(_), TypeTail::Array(_)) => true,
            (TypeTail::Pointer, TypeTail::Array(_)) => true,
            (TypeTail::Array(_), TypeTail::Pointer) => true,
            (TypeTail::Function(v1), TypeTail::Function(v2)) => v1.eq(v2),
            _ => false,
        }
    }
}

impl Eq for TypeTail {}

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
