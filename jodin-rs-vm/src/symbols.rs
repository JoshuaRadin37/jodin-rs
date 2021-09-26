//! Idea behind


use crate::vm::{VirtualMachine, Core};
use crate::compound_types::Pointer;
use crate::symbols::SystemCall::FunctionPointer;
use std::ops::{Index, IndexMut};

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