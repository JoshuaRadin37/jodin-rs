//! Store frame information

use crate::compound_types::Pointer;
use std::collections::HashMap;
use std::ffi::{CString};

pub struct Frame {
    pub frame_name: String,
    pub function_name: CString,
    pub frame_parent: usize,
    pub instruction_pointer: usize,
    pub starting_stack_length: usize,
    pub locals_heap_pointer: usize,
    pub locals_offset_size: HashMap<usize, (usize, usize)>
}

pub struct FrameBuilder {
    frame_name: Option<String>,
    function_name: Option<CString>,
    frame_parent: Option<usize>,
    instruction_pointer: usize,
    starting_stack_length: Option<usize>,
    locals_heap_pointer: Option<Pointer>,
    locals_offset_size: Option<HashMap<usize, (usize, usize)>>
}