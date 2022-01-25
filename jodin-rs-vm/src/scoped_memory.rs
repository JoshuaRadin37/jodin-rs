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
    use itertools::Itertools;
    use jodin_common::assembly::value::Value;
    use std::cell::RefCell;
    use std::collections::{HashMap, VecDeque};
    use std::hash::Hash;
    use std::ops::Add;

    #[derive(Debug)]
    pub(super) struct MemNode {
        id: usize,
        num_to_value: HashMap<usize, RefCell<Value>>,
        stack: Vec<Value>,
    }

    impl MemNode {
        pub fn new(id: usize) -> Self {
            MemNode {
                id,
                num_to_value: HashMap::new(),
                stack: vec![],
            }
        }

        pub fn id(&self) -> usize {
            self.id
        }
        pub fn num_to_value(&self) -> &HashMap<usize, RefCell<Value>> {
            &self.num_to_value
        }
        pub fn num_to_value_mut(&mut self) -> &mut HashMap<usize, RefCell<Value>> {
            &mut self.num_to_value
        }

        pub fn stack(&mut self) -> &mut Vec<Value> {
            &mut self.stack
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
            let output = match self.reclaimed.pop_front() {
                None => {
                    if let Some(next_id) = self.next_id {
                        self.next_id = next_id.checked_add(1);
                        Some(next_id)
                    } else {
                        None
                    }
                }
                Some(output) => Some(output),
            };
            info!("Next id: {output:?}");
            output
        }

        pub fn ret_id(&mut self, id: usize) {
            self.reclaimed.push_back(id);
            let mut stored = Vec::from(std::mem::replace(&mut self.reclaimed, VecDeque::new()));
            stored.sort();
            self.reclaimed.extend(stored.into_iter().unique());
            info!("Reclaimed var ids: {:?}", self.reclaimed);
        }

        pub fn ret_ids<I: IntoIterator<Item = usize>>(&mut self, iter: I) {
            self.reclaimed.extend(iter);
            let mut stored = Vec::from(std::mem::replace(&mut self.reclaimed, VecDeque::new()));
            stored.sort();
            self.reclaimed.extend(stored.into_iter().unique());
            info!("Reclaimed var ids: {:?}", self.reclaimed);
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
    id_pool: RefCell<VarIdPool>,
}

impl VMMemory {
    fn current_node_id(&self) -> usize {
        let last = self.mem_node_stack.last().unwrap();
        let &last_last = last.last().unwrap();
        last_last
    }

    fn last_stack(&mut self) -> &mut Vec<usize> {
        match self.mem_node_stack.last_mut() {
            None => {
                panic!()
            }
            Some(s) => s,
        }
    }

    fn last_stack_len(&mut self) -> usize {
        match self.mem_node_stack.last() {
            None => 0,
            Some(s) => s.len(),
        }
    }

    fn is_referenced(&self, id: usize) -> bool {
        for stack in &self.mem_node_stack {
            for &found_id in stack {
                if found_id == id {
                    return true;
                }
            }
        }

        for &value in self.id_to_prev_id.values() {
            if value == id {
                return true;
            }
        }

        false
    }

    fn remove_node(&mut self, node_id: usize) {
        info!("Removing node (id = {node_id})");

        let node = self
            .mem_nodes
            .remove(&node_id)
            .expect(format!("No node with id {node_id}").as_str());

        let node_id = node.id();
        self.id_to_prev_id.remove(&node_id);

        let vars: Vec<usize> = node.num_to_value().keys().copied().collect();

        self.id_pool.borrow_mut().ret_ids(vars);

        let mut hashed = vec![];
        for (&hash, &id) in self.hash_to_id.iter() {
            if id == node_id {
                hashed.push(hash);
            }
        }

        for hash in hashed {
            info!("Removing hash association (hash = {hash}, id = {node_id})");
            self.hash_to_id.remove(&hash);
        }
    }

    fn current_node_mut(&mut self) -> &mut MemNode {
        let id = self.current_node_id();
        self.mem_nodes.get_mut(&id).unwrap()
    }

    fn current_node(&self) -> &MemNode {
        let id = self.current_node_id();
        self.mem_nodes.get(&id).unwrap()
    }

    fn pop_scope_no_reclaim(&mut self) {
        self.last_stack().pop().expect("No mem nodes in stack");
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
        info!("Loading global scope");
        self.load_scope(GLOBAL_SCOPE_IDENTIFIER);
    }

    fn save_current_scope<H: Hash>(&mut self, identifier: H) {
        let id = self.current_node_id();
        let mut hasher = DefaultHasher::default();
        identifier.hash(&mut hasher);
        let hashed = hasher.finish();
        self.hash_to_id.insert(hashed, id);
        info!("Saved current scope to {hashed} (id = {id})");
    }

    fn load_scope<H: Hash>(&mut self, identifier: H) {
        let mut hasher = DefaultHasher::default();
        identifier.hash(&mut hasher);
        let hashed = hasher.finish();
        let mut id = self.hash_to_id.get(&hashed).cloned();

        if id.is_none() {
            self.global_scope();
            self.push_scope();
            self.save_current_scope(identifier);
            self.pop_scope_no_reclaim();
            id = self.hash_to_id.get(&hashed).cloned();
            assert!(id.is_some(), "No scope saved to hash value {}", hashed);
        } else {
            info!("Loading scope {hashed} (id = {id})", id = id.unwrap());
        }

        let mut node_stack = VecDeque::new();

        while let Some(m_id) = id {
            node_stack.push_front(m_id);
            id = self.id_to_prev_id.get(&m_id).cloned();
        }

        let stack = Vec::from_iter(node_stack.into_iter());
        self.mem_node_stack.push(stack);
        info!("Scope stack: {:?}", self.last_stack());
    }

    fn push_scope(&mut self) {
        let next_id = self.mem_nodes.len();
        let memory_node = MemNode::new(next_id);
        let prev_id = self.current_node_id();
        self.id_to_prev_id.insert(next_id, prev_id);
        self.mem_nodes.insert(next_id, memory_node);
        let stack = self.mem_node_stack.last_mut().unwrap();
        stack.push(next_id);
        info!("Pushed scope (id = {next_id})");
        info!("Scope stack: {:?}", self.last_stack());
    }

    fn pop_scope(&mut self) {
        let popped_id = self.last_stack().pop().expect("No mem nodes in stack");
        if !self.is_referenced(popped_id) {
            self.remove_node(popped_id);
        }
        info!("Popped scope (id = {popped_id})");
        info!("Scope stack: {:?}", self.last_stack());
    }

    fn back_scope(&mut self) {
        while self.last_stack_len() > 0 {
            self.pop_scope();
        }
        self.mem_node_stack.pop();
        info!("Went back a scope (id = {})", self.current_node_id());
        info!("Scope stack: {:?}", self.last_stack());
    }

    fn set_var(&mut self, var: usize, value: Value) {
        self.current_node_mut()
            .num_to_value_mut()
            .insert(var, RefCell::new(value));
    }

    fn get_var(&self, var: usize) -> Result<RefCell<Value>, BytecodeError> {
        self.current_node()
            .num_to_value()
            .get(&var)
            .cloned()
            .ok_or(BytecodeError::VariableNotSet(var))
    }

    fn clear_var(&mut self, var: usize) -> Result<(), BytecodeError> {
        self.current_node_mut()
            .num_to_value_mut()
            .remove(&var)
            .ok_or(BytecodeError::VariableNotSet(var))
            .map(|_| ())
    }

    fn next_var_number(&self) -> usize {
        self.id_pool.borrow_mut().next_id().unwrap()
    }

    fn push(&mut self, value: Value) {
        self.current_node_mut().stack().push(value);
    }

    fn pop(&mut self) -> Option<Value> {
        self.current_node_mut().stack().pop()
    }

    fn take_stack(&mut self) -> Vec<Value> {
        std::mem::replace(self.current_node_mut().stack(), vec![])
    }

    fn replace_stack(&mut self, stack: Vec<Value>) {
        std::mem::replace(self.current_node_mut().stack(), stack);
    }
}
