//! Methods for taking literals within jodin code into their jodin literals.
//!
//! There are 13 types of literals
//! 1. `string`
//! 2. `char`
//! 3. `boolean`
//! 4. `float`
//! 5. `double`
//! 6. `byte`
//! 7. `short`
//! 8. `int`
//! 9. `long`
//! 6. `unsigned byte`
//! 7. `unsigned short`
//! 8. `unsigned int`
//! 9. `unsigned long`

use crate::error::{JodinError, JodinErrorType, JodinResult};

use crate::assembly::value::Value;

use regex::Regex;

use std::convert::TryFrom;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// A single instance of a literal
#[derive(Debug, PartialOrd, PartialEq, Clone)]
#[allow(missing_docs)]
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

impl Literal {
    fn parse_escape_sequence(string: &str) -> JodinResult<(char, usize)> {
        match &string[..=1] {
            r"\n" => return Ok(('\n', 2)),
            r"\t" => return Ok(('\t', 2)),
            r"\r" => return Ok(('\r', 2)),
            _ => {}
        }

        if string.starts_with(r"\u") {
            let code_str = &string[2..6];
            let code: u32 = u32::from_str_radix(code_str, 16)?;
            return Ok((
                char::from_u32(code).ok_or(JodinErrorType::InvalidEscapeSequence(
                    string[..6].to_string(),
                ))?,
                6,
            ));
        }

        Err(JodinErrorType::InvalidEscapeSequence(
            string[..=1].to_string(),
        ))?
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::String(v) => {
                write!(f, "{}", v)
            }
            Literal::Char(v) => {
                write!(f, "{}", v)
            }
            Literal::Boolean(v) => {
                write!(f, "{}", v)
            }
            Literal::Float(v) => {
                write!(f, "{}", v)
            }
            Literal::Double(v) => {
                write!(f, "{}", v)
            }
            Literal::Byte(v) => {
                write!(f, "{}", v)
            }
            Literal::Short(v) => {
                write!(f, "{}", v)
            }
            Literal::Int(v) => {
                write!(f, "{}", v)
            }
            Literal::Long(v) => {
                write!(f, "{}", v)
            }
            Literal::UnsignedByte(v) => {
                write!(f, "{}", v)
            }
            Literal::UnsignedShort(v) => {
                write!(f, "{}", v)
            }
            Literal::UnsignedInt(v) => {
                write!(f, "{}", v)
            }
            Literal::UnsignedLong(v) => {
                write!(f, "{}", v)
            }
        }
    }
}

lazy_static::lazy_static! {
    static ref HEX_LITERAL: Regex = Regex::new(r"^0[xX](?P<val>[a-fA-F0-9]+)(?P<ext>[uU]?[lL]?)?$")
        .expect("HEX_LITERAL regular expression string invalid");
    static ref INTEGER_LITERAL: Regex = Regex::new(r"^(?P<val>\d+)(?P<ext>[uU]?[lL]?)?$").unwrap();
    static ref FLOAT_LITERAL: Regex = Regex::new(r"^(\d+\.\d+)([lL]?)$").unwrap();
}

impl FromStr for Literal {
    type Err = JodinError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains(".") {
            if let Some(captures) = FLOAT_LITERAL.captures(s) {
                let val: &str = captures.get(1).unwrap().as_str();
                let is_long = !captures.get(2).unwrap().as_str().is_empty();
                match is_long {
                    true => {
                        let inner: f64 = val.parse()?;
                        return Ok(Literal::Double(inner));
                    }
                    false => {
                        let inner: f32 = val.parse()?;
                        return Ok(Literal::Float(inner));
                    }
                }
            }
        } else {
            match HEX_LITERAL.captures(s) {
                Some(captures) if captures.get(0).unwrap().as_str() == s => {
                    let val = captures.name("val").unwrap().as_str();
                    let lit: Literal = match captures
                        .name("ext")
                        .map(|m| m.as_str().to_ascii_uppercase())
                        .as_deref()
                    {
                        Some("U") => Literal::UnsignedInt(u32::from_str_radix(val, 16)?),
                        Some("L") => Literal::Long(i64::from_str_radix(val, 16)?),
                        Some("UL") => Literal::UnsignedLong(u64::from_str_radix(val, 16)?),
                        _ => Literal::Int(i32::from_str_radix(val, 16)?),
                    };
                    return Ok(lit);
                }
                _ => {}
            }
            match INTEGER_LITERAL.captures(s) {
                Some(captures) if captures.get(0).unwrap().as_str() == s => {
                    let val = captures.name("val").unwrap().as_str();
                    let lit: Literal = match captures
                        .name("ext")
                        .map(|m| m.as_str().to_ascii_uppercase())
                        .as_deref()
                    {
                        Some("U") => Literal::UnsignedInt(val.parse()?),
                        Some("L") => Literal::Long(val.parse()?),
                        Some("UL") => Literal::UnsignedLong(val.parse()?),
                        _ => Literal::Int(val.parse()?),
                    };
                    return Ok(lit);
                }
                _ => {}
            }
            if s.starts_with("'") && s.ends_with("'") {
                let c_string = &s[1..s.len() - 1];
                let c: char;
                if c_string.starts_with('\\') {
                    c = Literal::parse_escape_sequence(c_string)?.0;
                } else {
                    c = c_string.parse()?;
                }
                return Ok(Literal::Char(c));
            }
            if s.starts_with('"') && s.ends_with('"') {
                let c_string = &s[1..s.len() - 1];
                let mut built = String::new();
                let mut chars: Box<dyn Iterator<Item = (_, _)>> = Box::new(c_string.char_indices());
                while let Some((index, c)) = chars.next() {
                    match c {
                        '\\' => {
                            let (escaped_char, pops) =
                                Literal::parse_escape_sequence(&c_string[index..])?;
                            built.push(escaped_char);
                            chars = Box::new(chars.skip(pops - 1));
                        }
                        c => built.push(c),
                    }
                }
                return Ok(Literal::String(built));
            }
            if s.starts_with("(*\"") && s.ends_with("\"*)") {
                let c_string = &s[3..s.len() - 3];
                return Ok(Literal::String(c_string.to_string()));
            }
        }

        panic!("Shouldn't have been able to find an invalid string for literal here (error string = {})", s)
    }
}

impl From<String> for Literal {
    fn from(s: String) -> Self {
        Literal::String(s)
    }
}

macro_rules! from_type {
    ($ty:ty, $variant:ident) => {
        /*
        impl From<$ty> for Literal {
            fn from(val: $ty) -> Self {
                Literal::$variant(val)
            }
        }

         */

        impl TryFrom<$ty> for Literal {
            type Error = JodinError;
            fn try_from(val: $ty) -> Result<Self, Self::Error> {
                Ok(Literal::$variant(val))
            }
        }
    };
}

from_type!(char, Char);
from_type!(bool, Boolean);

from_type!(u8, UnsignedByte);
from_type!(u16, UnsignedShort);
from_type!(u32, UnsignedInt);
from_type!(u64, UnsignedLong);

from_type!(i8, Byte);
from_type!(i16, Short);
from_type!(i32, Int);
from_type!(i64, Long);

from_type!(f32, Float);
from_type!(f64, Double);

impl TryFrom<Literal> for String {
    type Error = JodinError;

    fn try_from(value: Literal) -> Result<Self, Self::Error> {
        if let Literal::String(str) = value {
            Ok(str)
        } else {
            return Err(JodinErrorType::IncorrectLiteralTypeWithLiteral(value).into());
        }
    }
}

impl TryFrom<Literal> for char {
    type Error = JodinError;

    fn try_from(value: Literal) -> Result<Self, Self::Error> {
        match value {
            Literal::Char(c) => Ok(c),
            Literal::UnsignedByte(b) => Ok(b.into()),
            v => Err(JodinErrorType::IncorrectLiteralTypeWithLiteral(v).into()),
        }
    }
}

impl TryFrom<Literal> for bool {
    type Error = JodinError;

    fn try_from(value: Literal) -> Result<Self, Self::Error> {
        if let Literal::Boolean(b) = value {
            Ok(b)
        } else {
            return Err(JodinErrorType::IncorrectLiteralTypeWithLiteral(value).into());
        }
    }
}

impl TryFrom<Literal> for f32 {
    type Error = JodinError;

    fn try_from(value: Literal) -> Result<Self, Self::Error> {
        match value {
            Literal::Float(f) => Ok(f),
            Literal::Byte(b) => Ok(b.into()),
            Literal::Short(s) => Ok(s.into()),
            Literal::UnsignedByte(b) => Ok(b.into()),
            Literal::UnsignedShort(s) => Ok(s.into()),
            v => Err(JodinErrorType::IncorrectLiteralTypeWithLiteral(v).into()),
        }
    }
}

impl TryFrom<Literal> for f64 {
    type Error = JodinError;

    fn try_from(value: Literal) -> Result<Self, Self::Error> {
        match value {
            Literal::Float(f) => Ok(f.into()),
            Literal::Double(d) => Ok(d),
            Literal::Byte(b) => Ok(b.into()),
            Literal::Short(s) => Ok(s.into()),
            Literal::Int(i) => Ok(i.into()),
            Literal::UnsignedByte(u) => Ok(u.into()),
            Literal::UnsignedShort(s) => Ok(s.into()),
            Literal::UnsignedInt(i) => Ok(i.into()),
            v => Err(JodinErrorType::IncorrectLiteralTypeWithLiteral(v).into()),
        }
    }
}

impl TryFrom<Literal> for u8 {
    type Error = JodinError;

    fn try_from(value: Literal) -> Result<Self, Self::Error> {
        match value {
            Literal::Boolean(b) => Ok(b.into()),
            Literal::UnsignedByte(b) => Ok(b),
            v => Err(JodinErrorType::IncorrectLiteralTypeWithLiteral(v).into()),
        }
    }
}
impl TryFrom<Literal> for u16 {
    type Error = JodinError;

    fn try_from(value: Literal) -> Result<Self, Self::Error> {
        match value {
            Literal::Boolean(b) => Ok(b.into()),
            Literal::UnsignedByte(b) => Ok(b.into()),
            Literal::UnsignedShort(b) => Ok(b),
            v => Err(JodinErrorType::IncorrectLiteralTypeWithLiteral(v).into()),
        }
    }
}

impl TryFrom<Literal> for u32 {
    type Error = JodinError;

    fn try_from(value: Literal) -> Result<Self, Self::Error> {
        match value {
            Literal::Boolean(b) => Ok(b.into()),
            Literal::UnsignedByte(b) => Ok(b.into()),
            Literal::UnsignedShort(b) => Ok(b.into()),
            Literal::UnsignedInt(b) => Ok(b),
            v => Err(JodinErrorType::IncorrectLiteralTypeWithLiteral(v).into()),
        }
    }
}

impl TryFrom<Literal> for u64 {
    type Error = JodinError;

    fn try_from(value: Literal) -> Result<Self, Self::Error> {
        match value {
            Literal::Boolean(b) => Ok(b.into()),
            Literal::UnsignedByte(b) => Ok(b.into()),
            Literal::UnsignedShort(b) => Ok(b.into()),
            Literal::UnsignedInt(b) => Ok(b.into()),
            Literal::UnsignedLong(b) => Ok(b),
            v => Err(JodinErrorType::IncorrectLiteralTypeWithLiteral(v).into()),
        }
    }
}

impl TryFrom<Literal> for i8 {
    type Error = JodinError;

    fn try_from(value: Literal) -> Result<Self, Self::Error> {
        match value {
            Literal::Boolean(b) => Ok(b.into()),
            Literal::Byte(b) => Ok(b),
            v => Err(JodinErrorType::IncorrectLiteralTypeWithLiteral(v).into()),
        }
    }
}

impl TryFrom<Literal> for i16 {
    type Error = JodinError;

    fn try_from(value: Literal) -> Result<Self, Self::Error> {
        match value {
            Literal::Boolean(b) => Ok(b.into()),
            Literal::Byte(b) => Ok(b.into()),
            Literal::Short(b) => Ok(b),
            v => Err(JodinErrorType::IncorrectLiteralTypeWithLiteral(v).into()),
        }
    }
}

impl TryFrom<Literal> for i32 {
    type Error = JodinError;

    fn try_from(value: Literal) -> Result<Self, Self::Error> {
        match value {
            Literal::Boolean(b) => Ok(b.into()),
            Literal::Byte(b) => Ok(b.into()),
            Literal::Short(b) => Ok(b.into()),
            Literal::Int(b) => Ok(b),
            v => Err(JodinErrorType::IncorrectLiteralTypeWithLiteral(v).into()),
        }
    }
}

impl TryFrom<Literal> for i64 {
    type Error = JodinError;

    fn try_from(value: Literal) -> Result<Self, Self::Error> {
        match value {
            Literal::Boolean(b) => Ok(b.into()),
            Literal::Byte(b) => Ok(b.into()),
            Literal::Short(b) => Ok(b.into()),
            Literal::Int(b) => Ok(b.into()),
            Literal::Long(b) => Ok(b),
            v => Err(JodinErrorType::IncorrectLiteralTypeWithLiteral(v).into()),
        }
    }
}

impl Into<Value> for &Literal {
    fn into(self) -> Value {
        (*self).clone().into()
    }
}

/// A trait to enable constant casts between types (at compile time)
pub trait ConstantCast<T>: Sized {
    /// Try to cast this value to another value
    fn try_constant_cast(self) -> JodinResult<T>;
}

macro_rules! constant_cast {
    ($dest_type:ty, $($inner_id:ident),+) => {
        impl ConstantCast<$dest_type> for Literal {
            fn try_constant_cast(self) -> JodinResult<$dest_type> {
                use Literal::*;
                match self {
                    $($inner_id (v) => Ok(v as $dest_type),)*
                    e => Err(JodinErrorType::NotConstantExpression(format!("{:?}", e)).into())
                }
            }
        }
    };
}

constant_cast!(f32, Float);
constant_cast!(f64, Float, Double);
constant_cast!(u8, Char, Boolean, UnsignedByte);
constant_cast!(char, Char, UnsignedByte);
constant_cast!(bool, Boolean);
constant_cast!(u16, Char, UnsignedByte, Int, UnsignedInt);
constant_cast!(u32, Char, Boolean, UnsignedByte, UnsignedShort, Short);
constant_cast!(
    u64,
    Char,
    UnsignedByte,
    Int,
    UnsignedInt,
    UnsignedShort,
    Short,
    UnsignedLong,
    Long
);
constant_cast!(i8, Char, Boolean, UnsignedByte);
constant_cast!(i16, Char, UnsignedByte, Int, UnsignedInt);
constant_cast!(i32, Char, Boolean, UnsignedByte, UnsignedShort, Short);
constant_cast!(
    i64,
    Char,
    UnsignedByte,
    Int,
    UnsignedInt,
    UnsignedShort,
    Short,
    UnsignedLong,
    Long
);

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    use crate::parsing::parse_expression;
    use crate::utility::Visitor;

    #[test]
    fn parse_hex_literals() {
        assert_eq!(Literal::Int(0xFFFF), "0xFFFF".parse().unwrap());
        assert_eq!(Literal::UnsignedInt(0xFFFF), "0xFFFFu".parse().unwrap());
        assert_eq!(Literal::Long(0xFFFF), "0xFFFFl".parse().unwrap());
        assert_eq!(Literal::UnsignedLong(0xFFFF), "0xFFFFul".parse().unwrap());
    }

    #[test]
    fn parse_integer_literals() {
        assert_eq!(Literal::Int(12345), "12345".parse().unwrap());
        assert_eq!(Literal::UnsignedInt(12345), "12345u".parse().unwrap());
        assert_eq!(Literal::Long(12345), "12345l".parse().unwrap());
        assert_eq!(Literal::UnsignedLong(12345), "12345ul".parse().unwrap());
    }

    #[test]
    fn parse_chars() {
        assert_eq!(Literal::Char('c'), "'c'".parse().unwrap());
        assert_eq!(Literal::Char('\n'), "'\\n'".parse().unwrap());
        assert_eq!(Literal::Char('\u{298}'), "'\\u0298'".parse().unwrap());
    }

    #[test]
    fn parse_strings() {
        assert_eq!(
            Literal::String("Hello, World!".to_string()),
            "\"Hello, World!\"".parse().unwrap()
        );
        assert_eq!(
            Literal::String("Hello\n\tWorld!".to_string()),
            "\"Hello\\n\\tWorld!\"".parse().unwrap()
        );
        assert_eq!(
            Literal::String("Hello\"World!".to_string()),
            "(*\"Hello\"World!\"*)".parse().unwrap()
        );
    }

    #[test]
    fn constant_expressions() {
        let lit_node =
            parse_expression("(3+2+-2)/3 - 1").expect("This should be parsable as a expression");
        let as_literal = lit_node
            .visit(&HashMap::new())
            .expect("All literals and operations involved are constant");
        let val: i32 = as_literal.try_into().unwrap();
        assert_eq!(val, 0i32);
        let cast_expression =
            parse_expression("(16 as long)").expect("Couldn't parse a cast expression");
        let as_literal = cast_expression
            .visit(&HashMap::new())
            .expect("Int to Long should be valid");
        let val: i64 = as_literal.try_into().unwrap();
        assert_eq!(val, 16i64);

        let floating_point_node = parse_expression("3.2").expect("Floats supported now");
        let as_float: f32 = floating_point_node
            .visit(&HashMap::new())
            .expect("All literals and operations involved are constant")
            .try_into()
            .expect("Couldn't treat this as f32");

        assert_eq!(as_float, 3.2f32);

        let floating_point_node = parse_expression("6.4l").expect("Floats supported now");
        println!("{:?}", floating_point_node);
        let as_float: f64 = floating_point_node
            .visit(&HashMap::new())
            .expect("All literals and operations involved are constant")
            .try_into()
            .expect("Couldn't treat this as f64");

        assert_eq!(as_float, 6.4f64);
    }
}
