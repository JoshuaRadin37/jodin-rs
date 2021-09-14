//! The memory of the virtual machine, containing both the heap and the stack

use std::ffi::{CStr, CString};
use std::mem::ManuallyDrop;
use std::ops::Deref;
use std::num::NonZeroU8;

/// The stack of the core of the VM
#[derive(Debug)]
pub struct Stack {
    memory: Vec<u8>,
}

impl Stack {
    pub fn new() -> Self {
        Self { memory: vec![] }
    }

    pub fn push<T: PushToStack>(&mut self, value: T) {
        T::push_to_stack(value, self)
    }

    pub fn pop<T: PopFromStack>(&mut self) -> Option<T> {
        T::pop_from_stack(self)
    }

    pub fn len(&self) -> usize {
        self.memory.len()
    }

    pub fn is_empty(&self) -> bool {
        self.memory.is_empty()
    }
}

pub struct Heap {
    data: Vec<u8>,
}

impl Heap {
    pub fn new() -> Self {
        Self { data: vec![] }
    }

    pub fn increase_capacity(&mut self, size: usize) {
        self.data.extend(vec![0u8; size])
    }

    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut Vec<u8> {
        &mut self.data
    }
}

/// A trait to convert a primitive type into bytes
pub trait PushToStack {
    /// Turn this value into bytes
    fn push_to_stack(self, stack: &mut Stack);
}

impl<const N: usize> PushToStack for [u8; N] {
    fn push_to_stack(mut self, stack: &mut Stack) {
        self.reverse();
        stack.memory.extend(self)
    }
}

impl PushToStack for &[u8] {
    fn push_to_stack(self, stack: &mut Stack) {
        let reverse = self.iter().rev();
        for byte in reverse {
            byte.push_to_stack(stack);
        }
    }
}

impl PushToStack for &CStr {
    fn push_to_stack(self, stack: &mut Stack) {
        self.to_bytes_with_nul().push_to_stack(stack)
    }
}

impl PushToStack for CString {
    fn push_to_stack(self, stack: &mut Stack) {
        self.to_bytes_with_nul().push_to_stack(stack)
    }
}


macro_rules! primitive_push {
    ($ty:ty) => {
        impl PushToStack for $ty {
            fn push_to_stack(self, stack: &mut Stack) {
                self.to_le_bytes().push_to_stack(stack)
            }
        }
    };
}

primitive_push!(u8);
primitive_push!(u16);
primitive_push!(u32);
primitive_push!(u64);
primitive_push!(f32);
primitive_push!(f64);

/// A trait to pop a type from the stack
pub trait PopFromStack: PushToStack + Sized {
    /// the pop action
    fn pop_from_stack(stack: &mut Stack) -> Option<Self>;
}

impl<const N: usize> PopFromStack for [u8; N] {
    fn pop_from_stack(stack: &mut Stack) -> Option<Self> {
        if stack.memory.len() < N {
            return None;
        }
        let mut buffer = [0u8; N];
        let range = (stack.memory.len() - N)..stack.memory.len();
        let drain = stack.memory.drain(range);
        buffer.copy_from_slice(drain.as_slice());
        buffer.reverse();
        Some(buffer)
    }
}

impl PopFromStack for CString {
    fn pop_from_stack(stack: &mut Stack) -> Option<Self> {
        let mut buffer = vec![];
        let mut zero_found = false;
        while !stack.is_empty() {
            let pop = stack.pop::<u8>().unwrap();


            if pop == 0 {
                zero_found = true;
                break;
            } else {
                buffer.push(NonZeroU8::new(pop).unwrap());
            }
        }

        match zero_found {
            true => { Some(CString::from(buffer)) }
            false => { None }
        }
    }
}

macro_rules! primitive_pop {
    ($ty:ty) => {
        impl PopFromStack for $ty {
            fn pop_from_stack(stack: &mut Stack) -> Option<Self> {
                Some(<$ty>::from_le_bytes(stack.pop()?))
            }
        }
    };
}

primitive_pop!(u8);
primitive_pop!(u16);
primitive_pop!(u32);
primitive_pop!(u64);
primitive_pop!(f32);
primitive_pop!(f64);

#[cfg(test)]
mod tests {
    use crate::memory::Stack;
    use std::ffi::{CStr, CString};

    #[test]
    fn push_primitives() {
        let mut stack = Stack::new();
        stack.push::<u8>(32);
        println!("{:x?}", stack);
        assert_eq!(stack.pop::<u8>(), Some(32));
        assert!(stack.memory.is_empty());

        stack.push::<u64>(4096);
        assert_eq!(stack.len(), 8);
        println!("{:x?}", stack);
        assert_eq!(stack.pop::<u32>(), Some(4096));
    }

    #[test]
    fn push_strings() {
        let string = CStr::from_bytes_with_nul(concat!("Hello, World!", '\0').as_bytes()).unwrap();
        let mut stack = Stack::new();
        stack.push(string);
        println!("{:x?}", stack);
        let cstr: CString = stack.pop().unwrap();
        assert_eq!(cstr.to_str().unwrap(), "Hello, World!")
    }
}
