//! The memory of the virtual machine, containing both the heap and the stack


pub union Primitive {
    byte: u8,
    short: u16,
    int: u32,
    long: u64,
    float: f32,
    double: f64
}

/// The stack of the core of the VM
pub struct Stack {
    memory: Vec<Primitive>
}

impl Stack {

    pub fn new() -> Self {
        Self {
            memory: vec![]
        }
    }

    pub fn push(&mut self, primitive: Primitive) {
        self.memory.push(primitive);
    }

    pub fn pop_byte(&mut self) -> u8 {
        unsafe {
            self.memory.pop().unwrap().byte
        }
    }

    pub fn pop_short(&mut self) -> u16 {
        unsafe {
            self.memory.pop().unwrap().short
        }
    }


    pub fn pop_int(&mut self) -> u32 {
        unsafe {
            self.memory.pop().unwrap().int
        }
    }


    pub fn pop_long(&mut self) -> u64 {
        unsafe {
            self.memory.pop().unwrap().long
        }
    }


    pub fn pop_float(&mut self) -> f32 {
        unsafe {
            self.memory.pop().unwrap().float
        }
    }


    pub fn pop_double(&mut self) -> f64 {
        unsafe {
            self.memory.pop().unwrap().double
        }
    }

}

pub struct Heap {
    data: Vec<u8>
}

impl Heap {

    pub fn new() -> Self {
        Self {
            data: vec![]
        }
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
