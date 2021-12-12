//! The core traits are the traits the define the different core functionalities of the virtual machine

use jodin_asm::mvp::bytecode::{Asm, Assembly, Bytecode, Decode};
use jodin_asm::mvp::error::BytecodeError;
use jodin_asm::mvp::value::Value;
use num_traits::PrimInt;
use std::hash::Hash;

pub trait VirtualMachine {
    fn interpret_instruction(&mut self, bytecode: Asm) -> Result<(), ()>;

    /// Runs instructions within the virtual machine without any connection to the current state
    /// beyond the global context. Prevents loading new code into the VM during this phase.
    fn enclosed(&mut self, asm: &Assembly) -> Value;

    /// Loads some asm into a the virtual machine for future use
    fn load<A: GetAsm>(&mut self, asm: A);

    /// Runs the VM starting at a label
    fn run(&mut self, start_label: &str) -> Result<u32, ()>;
}

pub trait GetAsm {
    fn get_asm(&self) -> Assembly;
}

impl GetAsm for Assembly {
    fn get_asm(&self) -> Assembly {
        self.clone()
    }
}

impl GetAsm for &Assembly {
    fn get_asm(&self) -> Assembly {
        (*self).clone()
    }
}

/// You can get bytecode from this object.
pub trait GetBytecode {
    fn get_bytecode(&self) -> Result<Bytecode, BytecodeError>;
}

impl<GB: GetBytecode> GetAsm for GB {
    fn get_asm(&self) -> Vec<Asm> {
        let bytecode = self.get_bytecode().expect("Could not get bytecode");
        bytecode.decode()
    }
}

/// Memory defines a way of storing and getting variables.
pub trait MemoryTrait {
    /// Sets the memory to the global scope. Works similarly to a load
    fn global_scope(&self);
    /// Saves the current scope using some value to identify it for later.
    fn save_current_scope<H: Hash>(&mut self, identifier: H);
    /// Loads a scope into memory
    fn load_scope<H: Hash>(&mut self, identifier: H);
    /// Pushes a new scope. New scopes have access to variables in previous scopes.
    fn push_scope(&mut self);
    /// Pops the top-most scope. If scope is not saved anywhere, all information is lost.
    ///
    /// # Panic
    /// Should panic if the current scope is the global scope
    fn pop_scope(&mut self);
    /// After a load, this returns the state of the memory to before the most recent load.
    fn back_scope(&mut self);

    fn set_var(&mut self, var: usize, value: Value);
    fn get_var(&self, var: usize) -> Result<&Value, BytecodeError>;
    fn clear_var(&mut self, var: usize) -> Result<(), BytecodeError>;
    fn next_var_number(&self) -> usize;

    fn push(&mut self, value: Value);
    fn pop(&mut self) -> Option<Value>;
}

/// This defines the way that arithmetics should be performed.
pub trait ArithmeticsTrait {
    fn add(&self, a: Value, b: Value) -> Value;
    fn sub(&self, a: Value, b: Value) -> Value;
    fn mult(&self, a: Value, b: Value) -> Value;
    fn div(&self, a: Value, b: Value) -> Value;
    fn rem(&self, a: Value, b: Value) -> Value;

    fn and(&self, a: Value, b: Value) -> Value;
    fn or(&self, a: Value, b: Value) -> Value;
    fn not(&self, a: Value) -> Value;
    fn xor(&self, a: Value, b: Value) -> Value;

    fn shift_left(&self, a: Value, b: Value) -> Value;
    fn shift_right(&self, a: Value, b: Value) -> Value;
}
