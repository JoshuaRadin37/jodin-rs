//! Store frame information

use std::sync::Arc;
use std::collections::HashMap;
use std::ops::Index;

/// The actual storage used within a frame
pub struct FrameVars {
    parent: Option<usize>,
    data: Vec<u8>,
    meta: Arc<FrameVarsMetadata>
}

impl FrameVars {

    /// Get the slice of data referred to by an id
    pub fn get<'a, FS : FrameStorage>(&'a self, id: &str, storage: &'a FS) -> Option<&[u8]> {
        if let Some(&(index, size)) = self.meta.id_to_ptr.get(id) {
            let slice = &self.data[index..index+size];
            Some(slice)
        } else if let Some(parent) = self.parent {
            let next_frame = storage.get_frame(parent).expect("Frame parent refers to a frame that no longer exists");
            next_frame.vars.get(id, storage)
        } else {
            None
        }
    }

    /// Get the slice of data referred to by an id
    pub fn get_mut<'a, FS : FrameStorage>(&'a mut self, id: &str, storage: &'a mut FS) -> Option<&mut [u8]> {
        if let Some(&(index, size)) = self.meta.id_to_ptr.get(id) {
            let slice = &mut self.data[index..index+size];
            Some(slice)
        } else if let Some(parent) = self.parent {
            let next_frame = storage.get_frame_mut(parent).expect("Frame parent refers to a frame that no longer exists");
            next_frame.vars.get_mut(id, storage)
        } else {
            None
        }
    }
}


/// Contains a mapping between local variables and their position
#[derive(Clone, Debug)]
pub struct FrameVarsMetadata {
    id_to_ptr: HashMap<String, (usize, usize)>,
    frame_name: String
}

/// Can create frame storage instances
#[derive(Debug)]
pub struct FrameVarFactory {
    meta_data: Arc<FrameVarsMetadata>
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