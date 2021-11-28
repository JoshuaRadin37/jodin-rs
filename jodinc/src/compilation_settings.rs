//! Compilation settings affect what files will be outputted and where, and enable/disable
//! experimental features

use std::iter::FromIterator;
use std::path::{Path, PathBuf};

/// Compilation settings
#[derive(Debug)]
pub struct CompilationSettings {
    /// Whether to output the parse tree as a ".pt" file
    pub output_parse_tree: bool,
    /// Whether to output the abstract syntax tree as a ".ast" file
    pub output_ast: bool,
    /// Whether to output the final tagged syntax tree as a ".tast" file
    pub output_tast: bool,
    /// The directory to output files in
    pub target_directory: PathBuf,
}

impl CompilationSettings {
    /// Creates a file path relative to the target directory
    ///
    /// # Arguments
    ///
    /// * `path`: the path within the target directory
    ///
    /// returns: The path relative to the target directory
    ///
    /// # Example
    ///
    /// ```
    /// use std::path::{Path, PathBuf};
    /// use jodinc::ompilation_settings::CompilationSettings;
    /// use std::iter::FromIterator;
    /// let file = Path::new("file.txt");
    /// let mut settings = CompilationSettings::default();
    /// settings.target_directory = PathBuf::from("target");
    /// assert_eq!(
    ///     settings.output_file_path(file),
    ///     PathBuf::from_iter(&["target", "file.txt"])
    /// )
    /// ```
    pub fn output_file_path<P: AsRef<Path>>(&self, path: P) -> PathBuf {
        let ret = PathBuf::from_iter(&[&self.target_directory, &PathBuf::from(path.as_ref())]);
        if let Some(parent) = ret.parent() {
            println!("Attempting to create {:?}", parent);
            std::fs::create_dir_all(parent).unwrap();
        }
        ret
    }

    /// Creates a new settings instance.
    ///
    /// The default target directory is the working directory.
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for CompilationSettings {
    /// The default target directory is the working directory
    fn default() -> Self {
        Self {
            output_parse_tree: false,
            output_ast: false,
            output_tast: false,
            target_directory: std::env::current_dir().unwrap(),
        }
    }
}
