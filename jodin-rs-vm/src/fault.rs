use crate::vm::VM;
use crate::{ArithmeticsTrait, MemoryTrait};
use jodin_asm::mvp::value::Value;
use std::collections::HashMap;

/// A fault is a VM-level exception. The fault should return to the original point of execution once
/// it completes.
pub enum Fault {
    /// The following symbol is missing
    MissingSymbol(String),
}

pub struct FaultHandle<'vm, 'l, M: MemoryTrait, A: ArithmeticsTrait> {
    stored_pc: Vec<usize>,
    stored_stack: Vec<Value>,
    fault: Fault,
    virtual_machine: &'vm mut VM<'l, M, A>,
}

impl<'vm, 'l, M: MemoryTrait, A: ArithmeticsTrait> FaultHandle<'vm, 'l, M, A> {}
