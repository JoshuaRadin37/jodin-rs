//! Store frame information

use std::collections::{HashMap, VecDeque};
use std::ops::Index;
use std::sync::Arc;

/// The actual storage used within a frame
pub struct FrameVars {
    parent: Option<usize>,
    data: Vec<u8>,
    meta: Arc<FrameVarsMetadata>,
}

impl FrameVars {
    fn get<'a, FS: FrameStorage>(&'a self, id: &str, storage: &'a FS) -> Option<&[u8]> {
        if let Some(&(index, size)) = self.meta.id_to_ptr.get(id) {
            let slice = &self.data[index..index + size];
            Some(slice)
        } else if let Some(parent) = self.parent {
            let next_frame = storage
                .get_frame(parent)
                .expect("Frame parent refers to a frame that no longer exists");
            next_frame.vars.get(id, storage)
        } else {
            None
        }
    }

    fn get_mut(&mut self, id: &str) -> Option<&mut [u8]> {
        if let Some(&(index, size)) = self.meta.id_to_ptr.get(id) {
            let slice = &mut self.data[index..index + size];
            Some(slice)
        } else {
            None
        }
    }
    pub fn new(parent: Option<usize>, size: usize, meta: &Arc<FrameVarsMetadata>) -> Self {
        let data = vec![0u8; size];
        FrameVars {
            parent,
            data,
            meta: meta.clone(),
        }
    }
}

/// A context to access local variables
pub struct FrameVarsContext<'a, FS: FrameStorage> {
    frame_store: &'a FS,
    head_frame: usize,
}

impl<'a, 'b: 'a, FS: FrameStorage> From<&'b FrameVarsMutContext<'a, FS>>
    for FrameVarsContext<'a, FS>
{
    fn from(fr: &'b FrameVarsMutContext<'a, FS>) -> Self {
        let fs = &*fr.frame_store;
        FrameVarsContext {
            frame_store: fs,
            head_frame: fr.head_frame,
        }
    }
}

impl<'a, FS: FrameStorage> Index<&str> for FrameVarsContext<'a, FS> {
    type Output = [u8];

    /// Get the slice of data referred to by an id
    fn index(&self, index: &str) -> &Self::Output {
        self.frame_store
            .get_frame(self.head_frame)
            .unwrap()
            .vars
            .get(index, self.frame_store)
            .expect("Could not find variable")
    }
}

/// A context to access local variables
pub struct FrameVarsMutContext<'a, FS: FrameStorage> {
    frame_store: &'a mut FS,
    head_frame: usize,
}

impl<'a, FS: FrameStorage> FrameVarsMutContext<'a, FS> {
    /// Set a variable to a value
    pub fn set_value(&mut self, index: &str, value_as_bytes: &[u8]) {
        let mut frame_pointer = self.frame_store.get_frame_mut(self.head_frame);
        while let Some(ptr) = frame_pointer {
            frame_pointer = None;
            if let Some(var) = ptr.vars.get_mut(index) {
                var.clone_from_slice(value_as_bytes);
                return;
            } else if let Some(next) = ptr.vars.parent {
                frame_pointer = self.frame_store.get_frame_mut(next);
            }
        }
        panic!("Could not variable with name {}", index)
    }
}

impl<'a, FS: FrameStorage> Index<&str> for FrameVarsMutContext<'a, FS> {
    type Output = [u8];

    /// Get the slice of data referred to by an id
    fn index(&self, index: &str) -> &Self::Output {
        self.frame_store
            .get_frame(self.head_frame)
            .unwrap()
            .vars
            .get(index, self.frame_store)
            .expect("Could not find variable")
    }
}

/// Contains a mapping between local variables and their position
#[derive(Clone, Debug)]
pub struct FrameVarsMetadata {
    /// The name of the frame
    pub frame_name: String,
    id_to_ptr: HashMap<String, (usize, usize)>,
    /// The first instruction for the frame
    pub starting_instruction: usize,
}

impl FrameVarsMetadata {
    /// Gets the size of the frame's variables
    pub fn size(&self) -> usize {
        self.id_to_ptr.values().map(|(_, len)| *len).sum()
    }
}

/// Can create frame storage instances
#[derive(Debug)]
pub struct FrameVarFactory {
    meta_data: Arc<FrameVarsMetadata>,
}

impl FrameVarFactory {
    pub fn new_factory(metadata: FrameVarsMetadata) -> Self {
        Self {
            meta_data: Arc::new(metadata),
        }
    }

    /// Create a new frame from this instance
    pub fn new_frame_instance(&self, parent_frame: Option<usize>) -> FrameVars {
        let size = self.meta_data.size();
        FrameVars::new(parent_frame, size, &self.meta_data)
    }

    fn init_frame() -> Self {
        let metadata = FrameVarsMetadata {
            frame_name: "init_frame".to_string(),
            id_to_ptr: Default::default(),
            starting_instruction: 256,
        };
        Self::new_factory(metadata)
    }
}

/// Represents a frame within the Jodin vm
pub struct Frame {
    /// The local variables within the frame
    pub vars: FrameVars,
    pub instruction_pointer: usize,
}

/// A trait that defines the interactions that a frame storage structure can have.
pub trait FrameStorage<FS: FrameStorage = Self> {
    /// Get a frame by index
    fn get_frame(&self, index: usize) -> Option<&Frame>;

    /// Get a mutable frame by index
    fn get_frame_mut(&mut self, index: usize) -> Option<&mut Frame>;

    /// Create a new frame
    fn create_frame(&mut self, factory: FrameVarFactory, parent_frame: Option<usize>) -> usize;

    /// Delete a frame instance
    fn delete_frame(&mut self, index: usize);

    /// Create an instance of the frame storage
    fn instance() -> FS;
}

/// Stores, creates, and deletes frames
pub struct FrameHeap {
    frame_list: Vec<Option<Frame>>,
    free_list: VecDeque<usize>,
}

impl FrameStorage for FrameHeap {
    fn get_frame(&self, index: usize) -> Option<&Frame> {
        let option = self.frame_list.get(index)?;
        option.as_ref()
    }

    fn get_frame_mut(&mut self, index: usize) -> Option<&mut Frame> {
        let option = self.frame_list.get_mut(index)?;
        option.as_mut()
    }

    fn create_frame(&mut self, factory: FrameVarFactory, parent_frame: Option<usize>) -> usize {
        let next_index = self.free_list.pop_front();
        let index = match next_index {
            None => {
                let index = self.frame_list.len();
                self.frame_list.push(None);
                index
            }
            Some(index) => index,
        };
        let frame_vars = factory.new_frame_instance(parent_frame);
        let ip = frame_vars.meta.starting_instruction;
        let frame = Frame {
            vars: frame_vars,
            instruction_pointer: ip,
        };
        self.frame_list[index] = Some(frame);
        index
    }

    fn delete_frame(&mut self, index: usize) {
        if index >= self.frame_list.len() {
            panic!("index {} out of bounds", index)
        }
        match self.frame_list.get_mut(index) {
            None => {}
            Some(frame_opt @ Some(_)) => {
                *frame_opt = None;
                self.free_list.push_back(index);
            }
            Some(None) => {
                panic!("{} is not a live frame", index)
            }
        }
    }

    fn instance() -> Self {
        let mut heap = Self {
            frame_list: vec![],
            free_list: VecDeque::new(),
        };

        let factory = FrameVarFactory::init_frame();
        heap.create_frame(factory, None);
        heap
    }
}
