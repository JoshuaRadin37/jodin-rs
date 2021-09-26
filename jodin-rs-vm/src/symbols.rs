//! Idea behind


use std::collections::HashMap;
use std::ffi::CString;
use std::fmt::{Display, Formatter};
use std::ops::{Index, IndexMut};
use std::str::FromStr;

use crate::compound_types::Pointer;
use crate::symbols::SystemCall::FunctionPointer;
use crate::vm::{Core, VirtualMachine};

use itertools::Itertools;
use regex::{Captures, Match};

#[derive(Copy, Clone)]
pub enum SystemCall {
    VM(fn(&mut Core)),
    FunctionPointer(Pointer)
}

pub struct SystemCallTable<const N: usize> {
    sys_calls: [SystemCall; N]
}

impl<const N: usize> SystemCallTable<N> {
    pub fn new() -> Self {
        SystemCallTable { sys_calls: [FunctionPointer(Pointer(0)); N] }
    }
}

impl<const N: usize> Index<usize> for SystemCallTable<N> {
    type Output = SystemCall;

    fn index(&self, index: usize) -> &Self::Output {
        &self.sys_calls[index]
    }
}

impl<const N: usize> IndexMut<usize> for SystemCallTable<N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.sys_calls[index]
    }
}


#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct Symbol {
    base_symbol: String,
    applied_generics: Vec<String>,
    unapplied_generics: usize,
}

impl FromStr for Symbol {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use regex::Regex;
        lazy_static::lazy_static! {
            static ref SYMBOL_PATTERN: Regex = Regex::new(r#"^(\w+)(<(?P<applied>\w+(,\w+)*)>)?(::(?P<unapplied>\d+))?$"#).unwrap();
        }
        let captures: Captures = SYMBOL_PATTERN.captures(s).ok_or(())?;

        let base = captures.get(1).unwrap().as_str();
        let applied = match captures.name("applied") {
            None => { vec![]}
            Some(applied) => {
                let applied_string = applied.as_str();
                applied_string.split(",").map(|s| s.to_string()).collect()
            }
        };
        let unnapplied: usize = match captures.name("unapplied") {
            None => {
                0
            }
            Some(unapplied) => {
                unapplied.as_str().parse().map_err(|e| ())?
            }
        };
        Ok(
            Self {
                base_symbol: base.to_string(),
                applied_generics: applied,
                unapplied_generics: unnapplied
            }
        )
    }
}

impl From<CString> for Symbol {
    fn from(cstr: CString) -> Self {
        Symbol::from_str(cstr.to_str().expect(format!("{:?} is not a valid str", cstr).as_str()))
            .expect(format!("{:?} is not a valid symbol", cstr).as_str())
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.base_symbol)?;
        if self.applied_generics.len() > 0 {
            let gens = self.applied_generics.join(",");
            write!(f, "<{}>", gens)?;
        }
        if self.unapplied_generics > 0 {
            write!(f, "::{}", self.unapplied_generics)?;
        }
        Ok(())
    }
}

impl Symbol {
    pub fn new<S : AsRef<str>>(base_symbol: S, unapplied_generics: usize) -> Self {
        Symbol { base_symbol: base_symbol.as_ref().to_string(), applied_generics: vec![], unapplied_generics }
    }

    pub fn apply_generics<S : AsRef<str>, I : IntoIterator<Item=S>>(&self, gens: I) -> Self {
        let mut ret = self.clone();
        let orig_len = ret.applied_generics.len();
        ret.applied_generics.extend(gens.into_iter().map(|s: S| s.as_ref().to_string()));
        let less = ret.applied_generics.len() - orig_len;
        if less > ret.unapplied_generics {
            panic!("Can't apply more symbols than available")
        }
        ret.unapplied_generics -= less;
        ret
    }

    fn num_applied(&self) -> usize {
        self.applied_generics.len()
    }
}


#[derive(Debug)]
pub struct SymbolInfo {
    pub instruction_pointer: usize,
    pub locals_offset_size: HashMap<usize, (usize, usize)>
}

#[derive(Debug)]
pub struct SymbolTable {
    mapping: HashMap<Symbol, SymbolInfo>
}

impl SymbolTable {

    pub fn new() -> Self {
        Self {
            mapping: Default::default()
        }
    }


}


#[cfg(test)]
mod tests {
    pub use super::*;

    mod symbol_tests {
        use super::*;

        #[test]
        fn symbol_to_string() {
            assert_eq!(
                Symbol::new("base", 0).to_string(),
                "base"
            );
            assert_eq!(
                Symbol::new("base", 1).to_string(),
                "base::1"
            );
            assert_eq!(
                Symbol::new("base", 1)
                    .apply_generics(["int"])
                    .to_string(),
                "base<int>"
            );
            assert_eq!(
                Symbol::new("base", 2)
                    .apply_generics(["int"])
                    .to_string(),
                "base<int>::1"
            );
            assert_eq!(
                Symbol::new("base", 2)
                    .apply_generics(["int", "int"])
                    .to_string(),
                "base<int,int>"
            );
        }

        #[test]
        fn symbol_from_string() {
            assert_eq!(
                Symbol::from_str("base").unwrap(),
                Symbol::new("base", 0)
            );
            assert_eq!(
                Symbol::from_str("base::1").unwrap(),
                Symbol::new("base", 1)
            );
            assert_eq!(
                Symbol::from_str("base<int>").unwrap(),
                Symbol::new("base", 1).apply_generics(["int"])
            );
            assert_eq!(
                Symbol::from_str("base<int>::1").unwrap(),
                Symbol::new("base", 2).apply_generics(["int"])
            );
            assert_eq!(
                Symbol::from_str("base<int,int>").unwrap(),
                Symbol::new("base", 2).apply_generics(["int", "int"])
            );
        }
    }
}

