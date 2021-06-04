use crate::core::error::JodinError;
use regex::Regex;
use std::str::FromStr;

#[derive(Debug, PartialOrd, PartialEq)]
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

lazy_static! {
    static ref HEX_LITERAL: Regex = Regex::new(r"0[xX](?P<val>[a-fA-F0-9]+)(?P<ext>[uU]?[lL]?)?")
        .expect("HEX_LITERAL regular expression string invalid");
    static ref INTEGER_LITERAL: Regex = Regex::new(r"(?P<val>\d+)(?P<ext>[uU]?[lL]?)?").unwrap();
}

impl FromStr for Literal {
    type Err = JodinError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains(".") {
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
                let c: char = c_string.parse()?;
                return Ok(Literal::Char(c));
            }
        }

        panic!("Shouldn't have been able to find an invalid string for literal here (error string = {})", s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    }
}
