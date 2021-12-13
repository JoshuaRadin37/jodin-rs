use crate::{ArithmeticsTrait, MemoryTrait};
use jodin_asm::mvp::error::BytecodeError;
use jodin_asm::mvp::value::Value;
use std::cell::RefCell;
use std::hash::Hash;

pub struct MinimumMemory;

impl MemoryTrait for MinimumMemory {
    fn global_scope(&self) {
        todo!()
    }

    fn save_current_scope<H: Hash>(&mut self, identifier: H) {
        todo!()
    }

    fn load_scope<H: Hash>(&mut self, identifier: H) {
        todo!()
    }

    fn push_scope(&mut self) {
        todo!()
    }

    fn pop_scope(&mut self) {
        todo!()
    }

    fn back_scope(&mut self) {
        todo!()
    }

    fn set_var(&mut self, var: usize, value: Value) {
        todo!()
    }

    fn get_var(&self, var: usize) -> Result<RefCell<Value>, BytecodeError> {
        todo!()
    }

    fn clear_var(&mut self, var: usize) -> Result<(), BytecodeError> {
        todo!()
    }

    fn next_var_number(&self) -> usize {
        todo!()
    }

    fn push(&mut self, value: Value) {
        todo!()
    }

    fn pop(&mut self) -> Option<Value> {
        todo!()
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
