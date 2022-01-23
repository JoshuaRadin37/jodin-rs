use crate::{ArithmeticsTrait, MemoryTrait};

use jodin_common::assembly::value::Value;

/// A fault is a VM-level exception. The fault should return to the original point of execution once
/// it completes.
#[derive(Debug)]
pub enum Fault {
    /// The following symbol is missing
    MissingSymbol(String),
    /// A fault occurred in a fault
    DoubleFault,
}

impl Fault {
    pub fn handle_fault<'vm, 'l, M: MemoryTrait, A: ArithmeticsTrait>(
        &self,
        _handle: &FaultHandle,
    ) {
        match self {
            Fault::MissingSymbol(_s) => {}
            Fault::DoubleFault => panic!("Double Fault Encountered"),
        }
    }
}

#[derive(Debug)]
pub struct FaultHandle {
    pub stored_pc: Vec<usize>,
    pub stored_stack: Vec<Value>,
    pub fault: Fault,
    pub target_function: Value,
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
