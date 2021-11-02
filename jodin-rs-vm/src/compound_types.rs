//! Compound types are any types that can't be directly represented by a primitive type

use std::collections::{BTreeMap, HashMap};
use std::ffi::CString;

use crate::frame::{calculate_offsets, CalculatedLocalVars, UnCalculatedLocalVars};
use crate::memory::{PopFromStack, PushToStack, Stack};

#[derive(PushToStack, PopFromStack, Default, Clone, Copy)]
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
    size: u64,
}

#[derive(Clone, PushToStack)]
pub struct Array<K: PushToStack> {
    pub vector: Vec<K>,
    length: usize,
}

impl<K: PushToStack> Array<K> {
    pub fn new(vector: Vec<K>) -> Self {
        let len = vector.len();
        Array {
            vector,
            length: len,
        }
    }
}

impl<K: PopFromStack> PopFromStack for Array<K> {
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

impl<K: PushToStack> From<Array<K>> for Vec<K> {
    fn from(arr: Array<K>) -> Self {
        let mut vec = Vec::with_capacity(arr.length);
        vec.extend(arr.vector);
        vec
    }
}

pub struct Pair<K, V> {
    pub key: K,
    pub value: V,
}

impl<K, V> Pair<K, V> {
    pub fn new(key: K, value: V) -> Self {
        Pair { key, value }
    }
}

impl<K: PushToStack, V: PushToStack> PushToStack for Pair<K, V> {
    fn push_to_stack(self, stack: &mut Stack) {
        self.key.push_to_stack(stack);
        self.value.push_to_stack(stack);
    }
}

impl<K: PopFromStack, V: PopFromStack> PopFromStack for Pair<K, V> {
    fn pop_from_stack(stack: &mut Stack) -> Option<Self> {
        Some(Self {
            key: stack.pop()?,
            value: stack.pop()?,
        })
    }
}

impl<K, V> From<Pair<K, V>> for (K, V) {
    fn from(p: Pair<K, V>) -> Self {
        let Pair { key, value } = p;
        (key, value)
    }
}

#[derive(PushToStack, PopFromStack)]
pub struct LocalVarsDeclarations {
    map: Array<Pair<usize, usize>>,
}

impl LocalVarsDeclarations {
    pub fn new(map: Array<Pair<usize, usize>>) -> Self {
        LocalVarsDeclarations { map }
    }
}

impl From<HashMap<usize, (usize, usize)>> for LocalVarsDeclarations {
    fn from(vars: HashMap<usize, (usize, usize)>) -> Self {
        let vector: Vec<_> = vars
            .keys()
            .map(|key| {
                let (_offset, size) = &vars[key];
                Pair::new(*key, *size)
            })
            .collect();
        LocalVarsDeclarations {
            map: Array::new(vector),
        }
    }
}

impl From<LocalVarsDeclarations> for HashMap<usize, (usize, usize)> {
    fn from(l: LocalVarsDeclarations) -> Self {
        let vec: Vec<Pair<_, _>> = l.map.into();
        let vec: Vec<(_, _)> = vec.into_iter().map(|p| p.into()).collect();
        calculate_offsets(&vec)
    }
}

#[derive(Debug)]
pub struct FunctionInfo {
    pub instruction_pointer: usize,
    pub locals_offset_size: HashMap<usize, (usize, usize)>,
}

impl PushToStack for FunctionInfo {
    fn push_to_stack(self, stack: &mut Stack) {
        stack.push(LocalVarsDeclarations::from(self.locals_offset_size));
        stack.push(self.instruction_pointer);
    }
}

impl PopFromStack for FunctionInfo {
    fn pop_from_stack(stack: &mut Stack) -> Option<Self> {
        let ip: usize = stack.pop()?;
        let locals: LocalVarsDeclarations = stack.pop()?;
        let hashmap = HashMap::from(locals);
        Some(FunctionInfo {
            instruction_pointer: ip,
            locals_offset_size: hashmap,
        })
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
            vector: vec![1, 2, 3, 4, 5, 6, 7, 8],
        };
        let mut stack = Stack::new();
        stack.push(array.clone());
        println!("{:?}", stack);
        let found_array = stack.pop::<Array<u32>>().unwrap();
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
