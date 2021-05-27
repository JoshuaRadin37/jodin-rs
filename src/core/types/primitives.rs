use crate::core::error::JodinResult;
use crate::core::identifier::Identifier;
use crate::core::registry::{Registrable, Registry};
use crate::core::types::{JodinType, JodinTypeReference, Type};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Primitive {
    Void,
    Boolean,
    Char,
    Byte,
    Short,
    Int,
    Long,
    UnsignedByte,
    UnsignedShort,
    UnsignedInt,
    UnsignedLong,
}

impl Type for Primitive {
    fn type_name(&self) -> Identifier {
        let str: &str = match self {
            Primitive::Void => "void",
            Primitive::Boolean => "boolean",
            Primitive::Char => "char",
            Primitive::Byte => "byte",
            Primitive::Short => "short",
            Primitive::Int => "int",
            Primitive::Long => "long",
            Primitive::UnsignedByte => "ubyte",
            Primitive::UnsignedShort => "ushort",
            Primitive::UnsignedInt => "uint",
            Primitive::UnsignedLong => "ulong",
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
        }
    }
}

impl From<Primitive> for JodinType {
    fn from(p: Primitive) -> Self {
        JodinType::Primitive(p)
    }
}
