use crate::parsing::error::ParseError;
use std::str::FromStr;

#[derive(Debug)]
pub enum Keyword {
    Let,
    Return,
    Class,
    If,
    While,
    For,
    Abstract,
    Enum,
    Public,
    Private,
    Use,
    Break,
    Case,
    Continue,
    Default,
    Else,
    Signed,
    Unsigned,
    Sizeof,
    Switch,
    Typedef,
    Union,
    Struct,
    Super,
    Trait,
    Is,
}

impl Keyword {
    pub fn is_keyword<S: AsRef<str>>(s: S) -> bool {
        Keyword::from_str(s.as_ref()).is_ok()
    }
}

impl FromStr for Keyword {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        unimplemented!()
    }
}
