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
        let (base, generics, unapplied): (&str, &str, &str) = s.split("::").collect_tuple().ok_or(())?;
        let applied = generics.split("$").map(ToString::to_string).collect::<Vec<String>>();
        Ok(Self {
            base_symbol: base.to_string(),
            applied_generics: applied,
            unapplied_generics: unapplied.parse().map_err(|_| ())?
        })
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}::{}::{}", self.base_symbol, self.applied_generics.join("$"), self.unapplied_generics)
    }
}

impl Symbol {
    pub fn new(base_symbol: String, unapplied_generics: usize) -> Self {
        Symbol { base_symbol, applied_generics: vec![], unapplied_generics }
    }

    pub fn apply_generics<I : IntoIterator<Item=String>>(&self, gens: I) -> Self {
        todo!()
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

