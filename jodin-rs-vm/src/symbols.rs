//! Idea behind


use std::collections::HashMap;
use std::ffi::CString;
use std::fmt::{Display, Formatter};
use std::ops::{Index, IndexMut};
use std::str::FromStr;

use crate::compound_types::{Pointer, FunctionInfo};
use crate::symbols::SystemCall::FunctionPointer;
use crate::vm::{Core, VirtualMachine};

use itertools::Itertools;
use regex::{Captures, Match};
use std::collections::hash_map::Entry;
use crate::memory::PopFromStack;

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

    /// Pop applied generics until none left. Returns None if no applied generic
    pub fn pop_generic(&self) -> Option<Self> {
        if self.applied_generics.len() == 0 {
            return None;
        }
        let mut applied = self.applied_generics.clone();
        applied.pop();
        Some(Self {
            base_symbol: self.base_symbol.clone(),
            applied_generics: applied,
            unapplied_generics: self.unapplied_generics + 1,
        })
    }

    /// Pop applied generics until none left. Returns None if no applied generic
    pub fn pop_generics(&self) -> Option<Self> {
        if self.applied_generics.len() == 0 {
            return None;
        }
        let applied = self.applied_generics.clone();
        Some(Self {
            base_symbol: self.base_symbol.clone(),
            applied_generics: vec![],
            unapplied_generics: self.unapplied_generics + applied.len(),
        })
    }

    fn num_applied(&self) -> usize {
        self.applied_generics.len()
    }
}




#[derive(Debug)]
pub enum SymbolInfo {
    Function(FunctionInfo),
    None,
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

    pub fn insert_symbol(&mut self, symbol: Symbol) -> &mut SymbolInfo {
        if self.mapping.contains_key(&symbol) {
            panic!("{} already exists", symbol);
        }
        self.mapping.entry(symbol).or_insert(SymbolInfo::None)
    }

    pub fn get_symbol(&self, symbol: &Symbol) -> Option<&SymbolInfo> {
        self.mapping.get(symbol)
    }

    pub fn symbol(&mut self, symbol: &Symbol, vm_core: &mut Core) -> &SymbolInfo {
        if !self.mapping
            .contains_key(symbol) {
            let parent_symbol = symbol.pop_generics().expect("No parent symbol, using insert_symbol for base symbol");
            let parent_info = self.symbol(&parent_symbol, vm_core);
            if let SymbolInfo::Function(info) = parent_info {
                vm_core.call_function_info(info);
                let function_info: FunctionInfo = vm_core.stack.pop().expect("Couldn't get function info from the stack");
                self.mapping.insert( symbol.clone(), SymbolInfo::Function(function_info));
            } else {
                panic!("Parent symbol must be a function")
            }
        }

        return &self.mapping[symbol];
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

