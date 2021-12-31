//! This module for the implementation of the incremental compilation of the jodin vm target
//! See feature [#74] for more information on the implementation
//!
//! # Basics:
//!The compiler should allow for incremental compilation.
//!
//!The compiler should allow for directories to be specified that contains compiler jodin code that can be used by later compilation units. This should allow for incremental compilation as well. Unsure how to fully implement this yet.
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

use crate::compilation::jodin_vm_compiler::{
    split_by_module, JodinVMCompiler, Module, ObjectCompilerBuilder,
};
use crate::compilation::JodinVM;
use crate::passes::analysis::analyze_with_preload;
use crate::{optimize, JodinError, JodinNode, JodinResult};
use jodin_common::compilation::{
    execute_compiler, Compilable, Compiler, Context, PaddedWriter, Target,
};
use jodin_common::compilation_settings::CompilationSettings;
use jodin_common::parsing::parse_program;
use jodin_common::unit::{CompilationObject, Incremental, TranslationUnit};
use std::cell::RefCell;
use std::collections::{HashSet, VecDeque};
use std::fs::{File, FileType};
use std::path::{Path, PathBuf};

pub struct IncrementalCompiler {
    object_path: Vec<PathBuf>,
    output_directory: PathBuf,
    compilation_settings: CompilationSettings,
    translation_units: Vec<TranslationUnit>,
    paths_added: HashSet<PathBuf>,
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

#[derive(Debug)]
pub struct IncrementalDirectory<'path> {
    dir_path: &'path Path,
    cache: RefCell<Vec<CompilationObject>>,
}

impl<'path> Incremental for IncrementalDirectory<'path> {
    fn translation_units(&self) -> Vec<TranslationUnit> {
        if self.cache.borrow().is_empty() {
            let modules = self
                .to_modules()
                .expect("Could not turn the directory into a compilable");
            let mut cache = self.cache.borrow_mut();
            cache.extend(modules);
        }
        self.cache
            .borrow()
            .iter()
            .flat_map(|modules| modules.units.clone())
            .collect()
    }

    fn representative_path(&self) -> PathBuf {
        self.dir_path.to_path_buf()
    }
}

impl<'path> IncrementalDirectory<'path> {
    /// Create a new incremental directory from a path
    pub fn new(path: &'path Path) -> Option<Self> {
        match path.is_dir() {
            true => Some(IncrementalDirectory {
                dir_path: path,
                cache: RefCell::new(vec![]),
            }),
            false => None,
        }
    }

    /// Converts this directory into a vector of modules
    pub fn to_modules(&self) -> Result<Vec<CompilationObject>, JodinError> {
        let mut output = vec![];
        let mut dir_stack = VecDeque::new();
        dir_stack.push_front(self.dir_path.to_path_buf());

        while let Some(dir) = dir_stack.pop_front() {
            let read = std::fs::read_dir(dir)?;
            for entry in read {
                match entry {
                    Ok(dir_child) => match dir_child.file_type()? {
                        fs if fs.is_dir() => dir_stack.push_back(dir_child.path()),
                        fs if fs.is_file() => {
                            let obj = CompilationObject::try_from(dir_child.path())?;
                            output.push(obj);
                        }
                        _ => unreachable!(),
                    },
                    Err(e) => return Err(e.into()),
                }
            }
        }

        Ok(output)
    }
}
