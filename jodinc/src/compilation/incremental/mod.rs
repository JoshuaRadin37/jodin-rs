//! This module for the implementation of the incremental compilation of the jodin vm target
//! See feature [#74] for more information on the implementation
//!
//! # Basics:
//! The compiler should allow for incremental compilation.
//!
//! The compiler should allow for directories to be specified that contains compiler jodin code that can be used by later compilation units. This should allow for incremental compilation as well. Unsure how to fully implement this yet.
//!
//! # Implementation
//! The output file type should be `.jobj`, and should output based on namespace and not declaration file.
//! Precompiled units can either be given as individual `.jobj` files, or as zip files using `DEFLATE`
//! with the extension `.jdp`.
//!
//! All `.jobj` files should contain a magic number at the beginning of the file that:
//!
//! Confirms that this is a jodin object file
//! Can be interpreted by the target Jodin Virtual Machine.
//! The compiler should be given directories or .jdp files as inputs for compilation.
//!
//! # Considerations
//! When a file already exists that is being used for compilation, the compiler should rewrite the target file.
//!
//! [#74]: https://github.com/joshradin/jodin-rs/issues/74

use crate::compilation::jodin_vm_compiler::JodinVMCompiler;

use crate::passes::analysis::analyze_with_preload;
use crate::{optimize, JodinError};
use jodin_common::compilation::{Compilable, Compiler};
use jodin_common::compilation_settings::CompilationSettings;
use jodin_common::parsing::parse_program;
use jodin_common::unit::{CompilationObject, TranslationUnit};
use std::cell::RefCell;
use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::BufReader;

use std::path::{Path, PathBuf};

mod compilation_graph;
mod declarations_holder;

pub struct IncrementalCompiler {
    object_path: Vec<PathBuf>,
    output_directory: PathBuf,
    compilation_settings: CompilationSettings,
    translation_units: Vec<TranslationUnit>,
    paths_added: HashSet<PathBuf>,
}

pub trait Incremental {
    fn translation_units(&self) -> Vec<TranslationUnit>;
    fn representative_path(&self) -> PathBuf;
}

impl IncrementalCompiler {
    pub fn new<P: AsRef<Path>>(output_path: P, mut settings: CompilationSettings) -> Self {
        settings.target_directory = output_path.as_ref().to_path_buf();
        Self {
            object_path: vec![],
            output_directory: output_path.as_ref().to_path_buf(),
            compilation_settings: settings,
            translation_units: vec![],
            paths_added: Default::default(),
        }
    }

    /// Compiles a single input into a compilation objects
    pub fn compile_to_object<S: AsRef<str>>(&mut self, input: S) -> Result<(), JodinError> {
        let parsed = parse_program(input)?;
        let (analyzed, _env) = analyze_with_preload(parsed, &self.translation_units)?;

        let optimized = optimize(analyzed)?;

        let mut compiler = JodinVMCompiler::default();

        compiler.compile(&optimized, &self.compilation_settings)
    }

    /// Compiles a single file into a compilation objects
    pub fn compile_file<P: AsRef<Path>>(&mut self, file: P) -> Result<(), JodinError> {
        let input = std::fs::read_to_string(&file)?;
        let parsed = parse_program(input)?;
        let (analyzed, _env) = analyze_with_preload(parsed, &self.translation_units)?;

        let optimized = optimize(analyzed)?;

        let mut compiler = JodinVMCompiler::default();
        compiler.set_originating_file_path(file);

        compiler.compile(&optimized, &self.compilation_settings)
    }

    /// Add an incremental object to the compiler
    pub fn add_incremental<Inc: Incremental>(&mut self, incremental: Inc) {
        let path = incremental.representative_path();
        if self.paths_added.contains(&path) {
            panic!("{:?} already added to compiler", path);
        }
        self.translation_units
            .extend(incremental.translation_units());
        self.paths_added.insert(path);
    }
}
