//! The scoped memory module is the improved memory abstraction for the VM

use crate::MemoryTrait;
use jodin_common::assembly::error::BytecodeError;
use jodin_common::assembly::value::Value;
use std::cell::RefCell;
use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, VecDeque};
use std::hash::{BuildHasher, Hash, Hasher, SipHasher};

const GLOBAL_SCOPE_IDENTIFIER: &str = "@@GLOBAL_SCOPE";

mod helper_structs {
    use jodin_common::assembly::value::Value;
    use std::cell::RefCell;
    use std::collections::{HashMap, VecDeque};
    use std::hash::Hash;
    use std::ops::Add;

    #[derive(Debug)]
    pub(super) struct MemNode {
        id: usize,
        num_to_value: HashMap<usize, RefCell<Value>>,
    }

    impl MemNode {
        pub fn new(id: usize) -> Self {
            MemNode {
                id,
                num_to_value: HashMap::new(),
            }
        }
    }

    impl MemNode {
        pub fn used_var_ids(self) -> Vec<usize> {
            self.num_to_value.keys().copied().collect()
        }
    }

    #[derive(Debug)]
    pub(super) struct VarIdPool {
        next_id: Option<usize>,
        reclaimed: VecDeque<usize>,
    }

    impl Default for VarIdPool {
        fn default() -> Self {
            Self {
                next_id: Some(0),
                reclaimed: VecDeque::new(),
            }
        }
    }

    impl VarIdPool {
        pub fn next_id(&mut self) -> Option<usize> {
            match self.reclaimed.pop_front() {
                None => {
                    if let Some(next_id) = self.next_id {
                        self.next_id = next_id.checked_add(1);
                        Some(next_id)
                    } else {
                        None
                    }
                }
                Some(output) => Some(output),
            }
        }

        pub fn ret_id(&mut self, id: usize) {
            self.reclaimed.push_back(id);
            let mut stored = Vec::from(std::mem::replace(&mut self.reclaimed, VecDeque::new()));
            stored.sort();
            self.reclaimed.extend(stored);
        }

        pub fn ret_ids<I: IntoIterator<Item = usize>>(&mut self, iter: I) {
            self.reclaimed.extend(iter);
            let mut stored = Vec::from(std::mem::replace(&mut self.reclaimed, VecDeque::new()));
            stored.sort();
            self.reclaimed.extend(stored);
        }
    }
}
use helper_structs::*;

#[derive(Debug)]
pub struct VMMemory {
    mem_nodes: HashMap<usize, MemNode>,
    global_scope_id: usize,
    hash_to_id: HashMap<u64, usize>,
    id_to_prev_id: HashMap<usize, usize>,
    mem_node_stack: Vec<Vec<usize>>,
    id_pool: VarIdPool,
}

impl VMMemory {
    fn current_node_id(&self) -> usize {
        let last = self.mem_node_stack.last().unwrap();
        let &last_last = last.last().unwrap();
        last_last
    }
}

impl Default for VMMemory {
    fn default() -> Self {
        let node = MemNode::new(0);
        let mut output = Self {
            mem_nodes: Default::default(),
            global_scope_id: 0,
            hash_to_id: Default::default(),
            id_to_prev_id: Default::default(),
            mem_node_stack: vec![vec![0]],
            id_pool: Default::default(),
        };
        output.mem_nodes.insert(0, node);
        output.save_current_scope(GLOBAL_SCOPE_IDENTIFIER);
        output
    }
}

impl MemoryTrait for VMMemory {
    fn global_scope(&mut self) {
        self.load_scope(GLOBAL_SCOPE_IDENTIFIER);
    }

    fn save_current_scope<H: Hash>(&mut self, identifier: H) {
        let id = self.current_node_id();
        let mut hasher = DefaultHasher::default();
        identifier.hash(&mut hasher);
        let hashed = hasher.finish();
        self.hash_to_id.insert(hashed, id);
    }

    fn load_scope<H: Hash>(&mut self, identifier: H) {
        let mut hasher = DefaultHasher::default();
        identifier.hash(&mut hasher);
        let hashed = hasher.finish();
        let mut id = self.hash_to_id.get(&hashed);

        let mut stack = VecDeque::new();

        while let Some(m_id) = id {
            stack.push_front(m_id);
            id = self.id_to_prev_id.get(m_id);
        }

        let stack = Vec::from_iter(stack.into_iter().copied());
        self.mem_node_stack.push(stack);
    }

    fn push_scope(&mut self) {
        let next_id = self.mem_nodes.len();
        let memory_node = MemNode::new(next_id);
        let prev_id = self.current_node_id();
        self.id_to_prev_id.insert(next_id, prev_id);
        self.mem_nodes.insert(next_id, memory_node);
        let stack = self.mem_node_stack.last_mut().unwrap();
        stack.push(next_id);
    }

    fn pop_scope(&mut self) {
        self.mem_node_stack.last_mut().unwrap().pop();
    }

    fn back_scope(&mut self) {
        let mut last_stack =
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

    fn take_stack(&mut self) -> Vec<Value> {
        todo!()
    }

    fn replace_stack(&mut self, stack: Vec<Value>) {
        todo!()
    }
}
