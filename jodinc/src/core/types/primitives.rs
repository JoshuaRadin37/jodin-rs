//! The primitive types that can make up complex jodin types

use crate::core::identifier::Identifier;

use crate::core::types::{JodinType, Type};
use std::fmt::{Display, Formatter};
use crate::core::error::JodinResult;
use crate::core::types::big_object::JBigObject;
use crate::core::types::intermediate_type::IntermediateType;
use crate::core::types::type_environment::TypeEnvironment;
use crate::utility::Visitor;

/// A primitive data type within Jodin
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Primitive {
    /// An empty type, can not be used as an explicit canonical type
    Void,
    /// `true` or `false` value
    Boolean,
    /// A character, such as `'a'` or `'\n'`
    Char,
    /// An i8
    Byte,
    /// An i16
    Short,
    /// An i32
    Int,
    /// An i64
    Long,
    /// A u8
    UnsignedByte,
    /// A u16
    UnsignedShort,
    /// A u32
    UnsignedInt,
    /// A u64
    UnsignedLong,
    /// An f32
    Float,
    /// An f64
    Double,
    /// Varargs
    VaList,
}

impl Visitor<TypeEnvironment<'_>, JodinResult<JBigObject<'_>>> for Primitive {
    fn accept(&self, environment: &TypeEnvironment<'_>) -> JodinResult<JBigObject<'_>> {
        todo!()
    }
}

impl Type<'_, '_> for Primitive {
    fn type_name(&self) -> Identifier {
        let str: &str = match self {
            Primitive::Void => "void",
            Primitive::Boolean => "boolean",
            Primitive::Char => "char",
            Primitive::Byte => "byte",
            Primitive::Short => "short",
            Primitive::Int => "int",
            Primitive::Long => "long",
            Primitive::UnsignedByte => "unsigned byte",
            Primitive::UnsignedShort => "unsigned short",
            Primitive::UnsignedInt => "unsigned int",
            Primitive::UnsignedLong => "unsigned long",
            Primitive::Float => "float",
            Primitive::Double => "double",
            Primitive::VaList => "...",
        };
        Identifier::from(str)
    }

    fn type_id(&self) -> u32 {
        match self {
            Primitive::Void => 0,
            Primitive::Boolean => 1,
            Primitive::Char => 2,
            Primitive::Byte => 3,
            Primitive::Short => 4,
            Primitive::Int => 5,
            Primitive::Long => 6,
            Primitive::UnsignedByte => 7,
            Primitive::UnsignedShort => 8,
            Primitive::UnsignedInt => 9,
            Primitive::UnsignedLong => 10,
            Primitive::Float => 11,
            Primitive::Double => 12,
            Primitive::VaList => {
                panic!("VA LIST doesn't have a type id")
            }
        }
    }

    fn as_intermediate(&self) -> IntermediateType {
        IntermediateType::from(self.clone())
    }
}

impl From<Primitive> for JodinType {
    fn from(p: Primitive) -> Self {
        JodinType::Primitive(p)
    }
}

impl Display for Primitive {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.type_name())
    }
}
