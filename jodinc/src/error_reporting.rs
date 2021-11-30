//! contains error reporting functionality for better errors

use std::collections::HashMap;
use std::ops::{Deref, RangeInclusive, RangeTo};

type LineNum = usize;
pub type Line = String;

pub struct FileStructure {
    name: Option<String>,
    lines: Vec<String>,
    character_to_line: Vec<LineNum>
}

impl FileStructure {
    pub fn new(name: Option<String>, contents: &str) -> Self {

    }

    pub fn get_lines(&self, from:)
}

pub struct FilePart<'a> {
    parent_structure: &'a FileStructure,
    line_range: LineRange
}

pub struct LineRange(RangeTo<LineNum>);

impl LineRange {
    pub fn new(range: RangeTo<LineNum>) -> Self {
        LineRange(range)
    }
}

impl Deref for LineRange {
    type Target = RangeTo<LineNum>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


