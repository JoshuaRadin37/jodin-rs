//! The primitive types that can make up complex jodin types

use crate::identifier::Identifier;

use crate::ast::{JodinNode, JodinNodeType};
use crate::core::literal::Literal;
use crate::error::{JodinError, JodinResult};
use crate::types::intermediate_type::IntermediateType;
use crate::types::resolved_type::{ResolveType, WeakResolvedType};
use crate::types::type_environment::TypeEnvironment;
use crate::types::{BuildType, JodinType, Type};
use crate::utility::Visitor;
use std::fmt::{Display, Formatter};
use strum_macros::EnumIter;

/// A primitive data type within Jodin
#[derive(Debug, Clone, Eq, PartialEq, Hash, EnumIter)]
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

impl ResolveType for Primitive {
    fn resolve(&self, environment: &TypeEnvironment) -> WeakResolvedType {
        let result = environment
            .get_type_by_name(&self.type_identifier())
            .expect("Primitives should be in the type environment");
        WeakResolvedType::new(result, vec![])
    }
}

impl Type<'_> for Primitive {
    fn type_identifier(&self) -> Identifier {
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

    fn type_unique_id(&self) -> u32 {
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
        write!(f, "{}", self.type_identifier())
    }
}

impl<'t> BuildType<'t> for Primitive {
    fn build_type(
        node: &JodinNode,
        env: &TypeEnvironment,
        target_type: Option<&IntermediateType>,
    ) -> JodinResult<Self> {
        let s = "<primitive>";
        match node.inner() {
            JodinNodeType::Literal(l) => match l {
                Literal::Char(_) => Ok(Primitive::Char),
                Literal::Boolean(_) => Ok(Primitive::Boolean),
                Literal::Float(_) => Ok(Primitive::Float),
                Literal::Double(_) => Ok(Primitive::Double),
                Literal::Byte(_) => Ok(Primitive::Byte),
                Literal::Short(_) => Ok(Primitive::Short),
                Literal::Int(_) => Ok(Primitive::Int),
                Literal::Long(_) => Ok(Primitive::Long),
                Literal::UnsignedByte(_) => Ok(Primitive::UnsignedByte),
                Literal::UnsignedShort(_) => Ok(Primitive::UnsignedShort),
                Literal::UnsignedInt(_) => Ok(Primitive::UnsignedInt),
                Literal::UnsignedLong(_) => Ok(Primitive::UnsignedLong),
                _ => Err(JodinError::illegal_type_for_node(Identifier::from(s), node)),
            },
            _ => Err(JodinError::illegal_type_for_node(Identifier::from(s), node)),
        }
    }
}
