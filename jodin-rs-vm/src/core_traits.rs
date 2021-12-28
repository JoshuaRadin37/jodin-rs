//! The core traits are the traits the define the different core functionalities of the virtual machine

use crate::error::VMError;
use crate::fault::Fault;
use crate::vm::VM;
use jodin_common::mvp::bytecode::{Asm, Assembly, Bytecode, Decode, GetAsm};
use jodin_common::mvp::error::BytecodeError;
use jodin_common::mvp::value::Value;
use jodin_common::unit::CompilationObject;
use log::Level;
use num_traits::PrimInt;
use std::cell::RefCell;
use std::fmt::Debug;
use std::hash::Hash;

pub trait VirtualMachine {
    /// Interprets an instruction and returns the next instruction
    fn interpret_instruction(
        &mut self,
        bytecode: &Asm,
        current_instruction: usize,
    ) -> Result<usize, VMError>;

    /// Runs instructions within the virtual machine without any connection to the current state
    /// beyond the global context. Prevents loading new code into the VM during this phase.
    fn enclosed(&mut self, asm: &Assembly) -> Value;

    /// Loads some asm into a the virtual machine for future use
    fn load<A: GetAsm>(&mut self, asm: A);

    /// Runs the VM starting at a label
    fn run(&mut self, start_label: &str) -> Result<u32, VMError>;

    /// Forces the VM to encounter a fault
    fn fault(&mut self, fault: Fault);
}

/// Memory defines a way of storing and getting variables.
pub trait MemoryTrait: Debug {
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
    fn get_var(&self, var: usize) -> Result<RefCell<Value>, BytecodeError>;
    fn clear_var(&mut self, var: usize) -> Result<(), BytecodeError>;
    fn next_var_number(&self) -> usize;

    fn push(&mut self, value: Value);
    fn pop(&mut self) -> Option<Value>;
    fn take_stack(&mut self) -> Vec<Value>;
    fn replace_stack(&mut self, stack: Vec<Value>);
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