//! contains error reporting functionality for better errors

use std::collections::Bound;
use std::ops::{Deref, RangeBounds, RangeTo};

type LineNum = usize;
pub type Line = String;

#[derive(Debug)]
pub struct FileStructure {
    name: Option<String>,
    lines: Vec<String>,
    character_to_line: Vec<LineNum>,
}

impl FileStructure {
    pub fn new(name: Option<String>, contents: &str) -> Self {
        let lines: Vec<_> = contents.lines().map(|s| s.to_string()).collect();
        let mut char_to_inc = vec![false; contents.len()];
        for (char_index, _) in contents
            .char_indices()
            .filter(|(_index, char)| *char == '\n')
        {
            char_to_inc[char_index] = true;
        }
        let mut character_to_line = vec![0; contents.len()];
        let mut current_line: LineNum = 0;

        for i in 0..char_to_inc.len() {
            if char_to_inc[i] {
                current_line += 1;
            }
            character_to_line[i] = current_line;
        }

        Self {
            name,
            lines,
            character_to_line,
        }
    }

    pub fn get_lines<R: RangeBounds<usize>>(&self, char_range: R) -> &[String] {
        let from = match char_range.start_bound() {
            Bound::Included(i) => *i,
            Bound::Excluded(i) => *i + 1,
            Bound::Unbounded => 0,
        };
        let to = match char_range.end_bound() {
            Bound::Included(i) => *i,
            Bound::Excluded(i) => *i - 1,
            Bound::Unbounded => self.character_to_line.len() - 1,
        };
        let line_start = self.character_to_line[from];
        let line_end = self.character_to_line[to];

        &self.lines[line_start..=line_end]
    }
}

pub struct FilePart<'a> {
    parent_structure: &'a FileStructure,
    line_range: LineRange,
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

#[cfg(test)]
mod tests {
    use crate::error_reporting::FileStructure;

    #[test]
    fn get_lines() {
        let string = r#"
<><><><><>

    [] DATA []

<><><><><>
        "#
        .trim();
        println!("{}", string);
        let index = string.find("DATA").unwrap();
        let range = index..(index + "DATA".len());

        let fs = FileStructure::new(None, string);
        println!("{:?}", fs);
        let lines = fs.get_lines(range);
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0], "    [] DATA []")
    }
}
