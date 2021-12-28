use crate::vm::VM;
use crate::{ArithmeticsTrait, MemoryTrait};
use jodin_common::mvp::location::AsmLocation;
use jodin_common::mvp::value::Value;
use std::collections::HashMap;

/// A fault is a VM-level exception. The fault should return to the original point of execution once
/// it completes.
pub enum Fault {
    /// The following symbol is missing
    MissingSymbol(String),
    DoubleFault,
}

impl Fault {
    pub fn handle_fault<'vm, 'l, M: MemoryTrait, A: ArithmeticsTrait>(&self, handle: &FaultHandle) {
        match self {
            Fault::MissingSymbol(s) => {}
            Fault::DoubleFault => panic!("Double Fault Encountered"),
        }
    }
}

pub struct FaultHandle {
    stored_pc: Vec<usize>,
    stored_stack: Vec<Value>,
    fault: Fault,
    target_function: Value,
}

impl FaultHandle {
    pub fn new(
        stored_pc: Vec<usize>,
        stored_stack: Vec<Value>,
        fault: Fault,
        target_function: Value,
    ) -> Self {
        FaultHandle {
            stored_pc,
            stored_stack,
            fault,
            target_function,
        }
    }
}

#[derive(Default)]
pub struct FaultJumpTable;

impl FaultJumpTable {
    /// Gets the value for the fault to jump to
    pub fn get_fault_jump(&self, fault: &Fault) -> Value {
        match fault {
            Fault::MissingSymbol(_) => Value::Native,
            Fault::DoubleFault => {
                panic!("No value for double fault")
            }
        }
    }
}
