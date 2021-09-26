//! Store frame information

use crate::compound_types::Pointer;
use std::collections::HashMap;
use std::ffi::{CString};
use crate::symbols::Symbol;

pub struct Frame {
    pub frame_name: String,
    pub function_name: CString,
    pub within_symbol: Option<Symbol>,
    pub frame_parent: usize,
    pub instruction_pointer: usize,
    pub locals_heap_pointer: usize,
    pub locals_offset_size: HashMap<usize, (usize, usize)>
}

pub struct FrameBuilder {
    frame_name: Option<String>,
    function_name: Option<CString>,
    within_symbol: Option<Symbol>,
    frame_parent: Option<usize>,
    instruction_pointer: usize,
    locals_heap_pointer: Option<Pointer>,
    locals_offset_size: Option<HashMap<usize, (usize, usize)>>
}