use crate::{ArithmeticsTrait, MemoryTrait};
use jodin_asm::mvp::error::BytecodeError;
use jodin_asm::mvp::value::Value;
use std::cell::RefCell;
use std::hash::Hash;

/// Only has stack implementations
#[derive(Default)]
pub struct MinimumMemory {
    stack: Vec<Value>,
}

impl MemoryTrait for MinimumMemory {
    fn global_scope(&self) {}

    fn save_current_scope<H: Hash>(&mut self, identifier: H) {}

    fn load_scope<H: Hash>(&mut self, identifier: H) {}

    fn push_scope(&mut self) {}

    fn pop_scope(&mut self) {}

    fn back_scope(&mut self) {}

    fn set_var(&mut self, var: usize, value: Value) {}

    fn get_var(&self, var: usize) -> Result<RefCell<Value>, BytecodeError> {
        Err(BytecodeError::VariableNotSet(var))
    }

    fn clear_var(&mut self, var: usize) -> Result<(), BytecodeError> {
        Err(BytecodeError::VariableNotSet(var))
    }

    fn next_var_number(&self) -> usize {
        1
    }

    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    fn pop(&mut self) -> Option<Value> {
        self.stack.pop()
    }
}

pub struct MinimumALU;

impl ArithmeticsTrait for MinimumALU {
    fn add(&self, a: Value, b: Value) -> Value {
        todo!()
    }

    fn sub(&self, a: Value, b: Value) -> Value {
        todo!()
    }

    fn mult(&self, a: Value, b: Value) -> Value {
        todo!()
    }

    fn div(&self, a: Value, b: Value) -> Value {
        todo!()
    }

    fn rem(&self, a: Value, b: Value) -> Value {
        todo!()
    }

    fn and(&self, a: Value, b: Value) -> Value {
        todo!()
    }

    fn or(&self, a: Value, b: Value) -> Value {
        todo!()
    }

    fn not(&self, a: Value) -> Value {
        todo!()
    }

    fn xor(&self, a: Value, b: Value) -> Value {
        todo!()
    }

    fn shift_left(&self, a: Value, b: Value) -> Value {
        todo!()
    }

    fn shift_right(&self, a: Value, b: Value) -> Value {
        todo!()
    }
}
