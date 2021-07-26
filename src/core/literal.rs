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

use crate::core::error::{JodinError, JodinErrorType, JodinResult};
use regex::Regex;
use std::str::FromStr;

/// A single instance of a literal
#[derive(Debug, PartialOrd, PartialEq)]
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
}
