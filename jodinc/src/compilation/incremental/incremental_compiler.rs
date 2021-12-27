//! The incremental compiler uses an internal compiler that individually compiles parse nodes

use crate::compilation::incremental::unit::{CompilationObject, Incremental, TranslationUnit};
use crate::compilation::incremental::Error;
use crate::compilation::jodin_vm_compiler::{
    split_by_module, CompiledModule, JodinVMCompiler, Module, ModuleCompiler,
};
use crate::compilation::{
    execute_compiler, Compilable, Compiler, Context, JodinVM, PaddedWriter, Target,
};
use crate::compilation_settings::CompilationSettings;
use crate::parsing::parse_program;
use crate::passes::analysis::analyze_with_preload;
use crate::{optimize, JodinError, JodinNode, JodinResult};
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
    pub fn to_modules(&self) -> Result<Vec<CompilationObject>, Error> {
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
                            let file = File::open(dir_child.path())?;
                            let obj = CompilationObject::from(file);
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
