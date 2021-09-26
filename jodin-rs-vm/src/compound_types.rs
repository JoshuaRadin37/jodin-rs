//! Compound types are any types that can't be directly represented by a primitive type

use std::collections::{BTreeMap, HashMap};
use std::ffi::CString;

use crate::memory::{PopFromStack, PushToStack, Stack};

#[derive(PushToStack, PopFromStack)]
#[derive(Default, Clone, Copy)]
pub struct Pointer(pub usize);

impl Pointer {
    pub fn nullptr() -> Self {
        Self(0)
    }

    pub fn is_null(&self) -> bool {
        self.0 == 0
    }
}


#[derive(PushToStack, PopFromStack)]
pub struct SizedPointer {
    ptr: u64,
    size: u64
}

#[derive(Clone, PushToStack)]
pub struct Array<K : PushToStack> {
    pub vector: Vec<K>,
    length: usize,
}

impl<K: PushToStack> Array<K> {
    pub fn new(vector: Vec<K>) -> Self {
        let len = vector.len();
        Array { vector, length: len }
    }
}

impl<K : PushToStack + PopFromStack> PopFromStack for Array<K> {
    fn pop_from_stack(stack: &mut Stack) -> Option<Self> {
        let length = usize::pop_from_stack(stack)?;
        let mut bytes = vec![];
        for _ in 0..length {
            bytes.push(stack.pop()?);
        }
        Some(Array {
            length,
            vector: bytes,
        })
    }
}


pub struct Pair<K, V> {
    pub key: K,
    pub value: V
}

impl<K, V> Pair<K, V> {
    pub fn new(key: K, value: V) -> Self {
        Pair { key, value }
    }
}

impl<K : PushToStack, V : PushToStack> PushToStack for Pair<K, V> {
    fn push_to_stack(self, stack: &mut Stack) {
        self.key.push_to_stack(stack);
        self.value.push_to_stack(stack);
    }
}

impl<K : PopFromStack, V : PopFromStack> PopFromStack for Pair<K, V> {
    fn pop_from_stack(stack: &mut Stack) -> Option<Self> {
        Some(
            Self {
                key: stack.pop()?,
                value: stack.pop()?
            }
        )
    }
}


#[derive(PushToStack, PopFromStack)]
pub struct LocalVarsDeclarations {
    map: Array<Pair<usize, usize>>
}

impl LocalVarsDeclarations {
    pub fn new(map: Array<Pair<usize, usize>>) -> Self {
        LocalVarsDeclarations { map }
    }
}


#[cfg(test)]
mod tests {
    use crate::compound_types::{Array, Pair};
    use crate::memory::Stack;

    #[test]
    fn stack_array() {
        let array: Array<u32> = Array {
            length: 8,
            vector: vec![1, 2, 3, 4, 5, 6, 7, 8]
        };
        let mut stack = Stack::new();
        stack.push(array.clone());
        println!("{:?}", stack);
        let found_array= stack.pop::<Array<u32>>().unwrap();
        assert_eq!(found_array.vector, array.vector);
    }

    #[test]
    fn stack_pair() {
        let mut stack = Stack::new();
        let pair = Pair {
            key: 0u64,
            value: 0u64,
        };
        stack.push(pair);
    }
}