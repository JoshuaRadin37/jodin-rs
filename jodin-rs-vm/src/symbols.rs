//! Idea behind

use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Index, IndexMut};
use std::str::FromStr;

use crate::compound_types::{FunctionInfo, Pointer};
use crate::symbols::SystemCall::FunctionPointer;
use crate::vm::{Core, VirtualMachine};

use crate::memory::PopFromStack;
use itertools::Itertools;
use regex::{Captures, Match};
use std::collections::hash_map::Entry;
use std::hash::Hash;
use std::iter::FromIterator;

#[derive(Copy, Clone)]
pub enum SystemCall {
    VM(fn(&mut Core)),
    FunctionPointer(Pointer),
}

pub struct SystemCallTable<const N: usize> {
    sys_calls: [SystemCall; N],
}

impl<const N: usize> SystemCallTable<N> {
    pub fn new() -> Self {
        SystemCallTable {
            sys_calls: [FunctionPointer(Pointer(0)); N],
        }
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

#[derive(Debug, Hash, Clone)]
pub struct Symbol<S = String> {
    parent: Option<Box<Symbol<S>>>,
    identifier: S,
    generics: Vec<Option<S>>,
}

impl<S: Eq> Eq for Symbol<S> {}

impl<S> PartialEq for Symbol<S>
where
    S: PartialEq,
{
    fn eq(&self, other: &Symbol<S>) -> bool {
        if self.parent != other.parent {
            return false;
        }

        true
    }
}

impl<'a> From<SingleSymbol> for Symbol<String> {
    fn from(single: SingleSymbol) -> Self {
        let SingleSymbol {
            identifier,
            generics,
        } = single;
        Self {
            parent: None,
            identifier,
            generics,
        }
    }
}

struct SingleSymbol {
    identifier: String,
    generics: Vec<Option<String>>,
}

impl FromStr for SingleSymbol {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use regex::Regex;
        lazy_static::lazy_static! {
            static ref SINGLE_SYMBOL: Regex = Regex::new(r#"(?P<id>\w+)(<(?P<gens>\w+(,\w+)*>))?"#).unwrap();
        }
        let matches: Captures = SINGLE_SYMBOL.captures(s).ok_or(())?;
        let id = matches.name("id").ok_or(())?.as_str();
        let gens = matches
            .name("gens")
            .map(|m| {
                m.as_str()
                    .split(",")
                    .map(|s| match s {
                        "_" => None,
                        other => Some(other.to_string()),
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or(vec![]);
        Ok(Self {
            identifier: id.to_string(),
            generics: gens,
        })
    }
}

impl FromStr for Symbol<String> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use regex::Regex;
        lazy_static::lazy_static! {
            static ref SINGLE_SYMBOL: &'static str = r#"\w+(<(\w+(,\w+)*>))?"#;
            static ref SYMBOL_PATTERN: Regex = Regex::new(format!(r"^{0}(::{0})*$", *SINGLE_SYMBOL).as_str()).unwrap();
        }
        let captures: Captures = SYMBOL_PATTERN.captures(s).ok_or(())?;
        let mut ret: Option<Symbol<String>> = None;
        let id_string = captures.get(0).ok_or(())?.as_str();
        let mut splits = id_string.split("::");

        while let Some(single) = splits.next() {
            let single_symbol = SingleSymbol::from_str(single)?;
            match ret {
                None => {
                    ret = Some(single_symbol.into());
                }
                Some(parent) => {
                    let SingleSymbol {
                        identifier,
                        generics,
                    } = single_symbol;
                    ret = Some(Self {
                        parent: Some(Box::new(parent)),
                        identifier,
                        generics,
                    });
                }
            }
        }

        ret.ok_or(())
    }
}

impl From<CString> for Symbol<String> {
    fn from(cstr: CString) -> Self {
        Symbol::from_str(
            cstr.to_str()
                .expect(format!("{:?} is not a valid str", cstr).as_str()),
        )
        .expect(format!("{:?} is not a valid symbol", cstr).as_str())
    }
}

impl<S: Display> Display for Symbol<S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(parent) = &self.parent {
            write!(f, "{}::", parent)?;
        }
        write!(f, "{}", self.identifier)?;
        if self.generics.len() > 0 {
            write!(f, "<")?;
            let joined = self
                .generics
                .iter()
                .map(|gen| match gen {
                    None => "_".to_string(),
                    Some(g) => format!("{}", g),
                })
                .join(",");
            write!(f, "{}", joined);
            write!(f, ">")?;
        }
        Ok(())
    }
}

impl<S> Symbol<S> {
    pub fn new(base_symbol: S) -> Self {
        Self {
            parent: None,
            identifier: base_symbol,
            generics: vec![],
        }
    }

    pub fn new_with_unmapped(base_symbol: S, count: usize) -> Self {
        Self {
            parent: None,
            identifier: base_symbol,
            generics: vec![],
        }
        .with_n_unmapped_generic(count)
    }

    pub fn with_parent(base_symbol: Self, sub: S) -> Self {
        Self {
            parent: Some(Box::new(base_symbol)),
            identifier: sub,
            generics: vec![],
        }
    }

    pub fn with_generic(mut self, generic: S) -> Self {
        self.generics.push(Some(generic));
        self
    }

    pub fn mapped<I: IntoIterator<Item = Option<S>>>(mut self, generics: I) -> Self {
        let vector: Vec<Option<S>> = Vec::from_iter(generics);
        if self.generics.len() != vector.len() {
            panic!("Can't map more generics then available.")
        }
        let zip = vector.into_iter().zip(&mut self.generics);
        for (generic, target) in zip {
            match (generic, target) {
                (None, _) => {}
                (Some(generic), target @ None) => {
                    *target = Some(generic);
                }
                (Some(_generic), Some(_)) => {
                    panic!("Can't remap a generic")
                }
            }
        }

        self
    }

    pub fn apply_generics<I: IntoIterator<Item = S>>(mut self, generics: I) -> Self {
        self.mapped(generics.into_iter().map(|m| Some(m)))
    }

    pub fn pop_generics(&self) -> Self
    where
        S: Clone,
    {
        let mut ret = self.clone();
        ret.generics = (0..self.generics.len()).into_iter().map(|_| None).collect();
        ret
    }

    pub fn with_unmapped_generic(mut self) -> Self {
        self.generics.push(None);
        self
    }

    pub fn with_n_unmapped_generic(mut self, n: usize) -> Self {
        match n {
            0 => self,
            _ => self.with_unmapped_generic().with_n_unmapped_generic(n - 1),
        }
    }
}

impl From<&str> for Symbol<String> {
    fn from(s: &str) -> Self {
        Symbol::from_str(s).expect(format!("{} is not a valid symbol", s).as_str())
    }
}

impl<'a> From<Symbol<&'a str>> for Symbol<String> {
    fn from(s: Symbol<&'a str>) -> Self {
        let Symbol {
            parent,
            identifier,
            generics,
        } = s;
        Self {
            parent: parent.map(|p| Box::new((*p).into())),
            identifier: identifier.into(),
            generics: generics
                .into_iter()
                .map(|gen| gen.map(|s| s.into()))
                .collect(),
        }
    }
}

#[derive(Debug)]
pub enum SymbolInfo {
    Function(FunctionInfo),
    None,
}

#[derive(Debug)]
pub struct SymbolTable {
    mapping: HashMap<Symbol, SymbolInfo>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            mapping: Default::default(),
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
        if !self.mapping.contains_key(symbol) {
            let parent_symbol = symbol.pop_generics();
            let parent_info = self.symbol(&parent_symbol, vm_core);
            if let SymbolInfo::Function(info) = parent_info {
                vm_core.call_function_info(info);
                let function_info: FunctionInfo = vm_core
                    .stack
                    .pop()
                    .expect("Couldn't get function info from the stack");
                self.mapping
                    .insert(symbol.clone(), SymbolInfo::Function(function_info));
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
            assert_eq!(Symbol::new_with_unmapped("base", 0).to_string(), "base");
            assert_eq!(Symbol::new_with_unmapped("base", 1).to_string(), "base<_>");
            assert_eq!(
                Symbol::new_with_unmapped("base", 1)
                    .apply_generics(["int"])
                    .to_string(),
                "base<int>"
            );
            assert_eq!(
                Symbol::new_with_unmapped("base", 2)
                    .mapped([Some("int"), None])
                    .to_string(),
                "base<int,_>"
            );
            assert_eq!(
                Symbol::new_with_unmapped("base", 2)
                    .apply_generics(["int", "int"])
                    .to_string(),
                "base<int,int>"
            );
            let symbol = Symbol::with_parent(Symbol::from("base"), "construct".to_string());
            assert_eq!(symbol.to_string(), "base::construct");
        }

        #[test]
        fn symbol_from_string() {
            assert_eq!(Symbol::new("base"), Symbol::new_with_unmapped("base", 0));
            assert_eq!(
                Symbol::from_str("base<_>").unwrap(),
                Symbol::new_with_unmapped("base", 1).into()
            );
            assert_eq!(
                Symbol::from_str("base<int>").unwrap(),
                Symbol::new_with_unmapped("base", 1)
                    .apply_generics(["int"])
                    .into()
            );
        }
    }
}
