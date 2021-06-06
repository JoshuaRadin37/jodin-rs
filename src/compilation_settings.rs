use std::iter::FromIterator;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct CompilationSettings {
    pub output_parse_tree: bool,
    pub output_ast: bool,
    pub output_tast: bool,
    pub target_directory: PathBuf,
}

impl CompilationSettings {
    pub fn output_file_path<P: AsRef<Path>>(&self, path: P) -> PathBuf {
        let ret = PathBuf::from_iter(&[&self.target_directory, &PathBuf::from(path.as_ref())]);
        if let Some(parent) = ret.parent() {
            println!("Attempting to create {:?}", parent);
            std::fs::create_dir_all(parent).unwrap();
        }
        ret
    }
}

impl Default for CompilationSettings {
    fn default() -> Self {
        Self {
            output_parse_tree: false,
            output_ast: false,
            output_tast: false,
            target_directory: std::env::current_dir().unwrap(),
        }
    }
}
