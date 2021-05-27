use crate::core::error::JodinError;
use std::str::FromStr;

#[derive(Debug)]
pub enum Literal {
    String(String),
    Char(char),
    Boolean(bool),
    Float(f32),
    Double(f64),
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    UnsignedByte(u8),
    UnsignedShort(u16),
    UnsignedInt(u32),
    UnsignedLong(u64),
}

impl FromStr for Literal {
    type Err = JodinError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
