use crate::{ArithmeticsTrait, MemoryTrait};
use jodin_common::mvp::error::BytecodeError;
use jodin_common::mvp::value::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;

/// Only has stack implementations and non-scoped variables
#[derive(Default, Debug)]
pub struct MinimumMemory {
    stack: Vec<Value>,
    vars: HashMap<usize, RefCell<Value>>,
}

impl MemoryTrait for MinimumMemory {
    fn global_scope(&self) {}

    fn save_current_scope<H: Hash>(&mut self, identifier: H) {}

    fn load_scope<H: Hash>(&mut self, identifier: H) {}

    fn push_scope(&mut self) {}

    fn pop_scope(&mut self) {}

    fn back_scope(&mut self) {}

    fn set_var(&mut self, var: usize, value: Value) {
        self.vars.insert(var, RefCell::new(value));
    }

    fn get_var(&self, var: usize) -> Result<RefCell<Value>, BytecodeError> {
        self.vars
            .get(&var)
            .cloned()
            .ok_or(BytecodeError::VariableNotSet(var))
    }

    fn clear_var(&mut self, var: usize) -> Result<(), BytecodeError> {
        self.vars
            .remove(&var)
            .map(|_| ())
            .ok_or(BytecodeError::VariableNotSet(var))
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

    fn take_stack(&mut self) -> Vec<Value> {
        std::mem::replace(&mut self.stack, Vec::new())
    }
    fn replace_stack(&mut self, stack: Vec<Value>) {
        std::mem::replace(&mut self.stack, stack);
    }
}

pub struct MinimumALU;

impl ArithmeticsTrait for MinimumALU {
    fn add(&self, a: Value, b: Value) -> Value {
        match (a, b) {
            (Value::UInteger(a), Value::UInteger(b)) => Value::UInteger(u64::wrapping_add(a, b)),
            (Value::Integer(a), Value::Integer(b)) => Value::Integer(i64::wrapping_add(a, b)),
            (a, b) => panic!("Invalid operation operands (l: {}, b: {})", a, b),
        }
    }

    fn sub(&self, a: Value, b: Value) -> Value {
        match (a, b) {
            (Value::UInteger(a), Value::UInteger(b)) => Value::UInteger(u64::wrapping_sub(a, b)),
            (Value::Integer(a), Value::Integer(b)) => Value::Integer(i64::wrapping_sub(a, b)),
            (a, b) => panic!("Invalid operation operands (l: {}, b: {})", a, b),
        }
    }

    fn mult(&self, a: Value, b: Value) -> Value {
        match (a, b) {
            (Value::UInteger(a), Value::UInteger(b)) => Value::UInteger(u64::wrapping_mul(a, b)),
            (Value::Integer(a), Value::Integer(b)) => Value::Integer(i64::wrapping_mul(a, b)),
            (a, b) => panic!("Invalid operation operands (l: {}, b: {})", a, b),
        }
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
        match a {
            Value::Byte(b) => Value::Byte(if b != 0 { 0 } else { 1 }),
            v => {
                panic!("Invalid operation operand (v: {})", v)
            }
        }
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
