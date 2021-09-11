//! Store frame information

use std::collections::HashMap;
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
    /*

    /// Get the slice of data referred to by an id
    fn get_mut<'a, FS: FrameStorage>(
        &'a mut self,
        id: &str,
        storage: &'a mut FS,
    ) -> Option<&mut [u8]> {
        if let Some(&(index, size)) = self.meta.id_to_ptr.get(id) {
            let slice = &mut self.data[index..index + size];
            Some(slice)
        } else if let Some(parent) = self.parent {
            let next_frame = storage
                .get_frame_mut(parent)
                .expect("Frame parent refers to a frame that no longer exists");
            next_frame.vars.get_mut(id, storage)
        } else {
            None
        }
    }

     */

    fn get_mut(&mut self, id: &str) -> Option<&mut [u8]> {
        if let Some(&(index, size)) = self.meta.id_to_ptr.get(id) {
            let slice = &mut self.data[index..index + size];
            Some(slice)
        } else {
            None
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
    id_to_ptr: HashMap<String, (usize, usize)>,
    frame_name: String,
}

/// Can create frame storage instances
#[derive(Debug)]
pub struct FrameVarFactory {
    meta_data: Arc<FrameVarsMetadata>,
}

/// Represents a frame within the Jodin vm
pub struct Frame {
    /// The local variables within the frame
    pub vars: FrameVars,
}

/// A trait that defines the interactions that a frame storage structure can have.
pub trait FrameStorage {
    /// Get a frame by index
    fn get_frame(&self, index: usize) -> Option<&Frame>;

    /// Get a mutable frame by index
    fn get_frame_mut(&mut self, index: usize) -> Option<&mut Frame>;

    /// Create a new frame
    fn create_frame(&mut self, factory: FrameVarFactory) -> usize;
}
