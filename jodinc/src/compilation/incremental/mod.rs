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

use crate::compilation::jodin_vm_compiler::{JodinVMCompiler, SingleUseCompiler};

use crate::passes::analysis::analyze_with_preload;
use crate::{optimize, JodinError};
use jodin_common::compilation::{Compilable, Compiler, Context, MicroCompiler, PaddedWriter};
use jodin_common::compilation_settings::CompilationSettings;
use jodin_common::parsing::parse_program;
use jodin_common::unit::{CompilationObject, TranslationUnit};
use std::cell::RefCell;
use std::collections::{HashSet, VecDeque};
use std::fs::{File, OpenOptions};
use std::io::BufReader;

use crate::compilation::incremental::compilation_graph::{CompilationGraph, CompilationNode};
use crate::compilation::object_path::ObjectPath;
use crate::compilation::JodinVM;
use jodin_common::ast::JodinNode;
use jodin_common::core::tags::TagTools;
use jodin_common::error::JodinResult;
use jodin_common::identifier::Identifier;
use jodin_common::utility::PathUtils;
use petgraph::prelude::GraphMap;
use std::path::{Path, PathBuf};

pub mod compilation_graph;
mod declarations_holder;
pub mod variant;

/// Individual files are compiled into this format
pub const COMPILED_FILE_EXTENSION: &str = "grounds";
/// An archive file containing compiled jodin files
pub const ARCHIVE_FILE_EXTENSION: &str = "beans";

pub struct IncrementalCompiler {
    object_path: ObjectPath,
    code_path: PathBuf,
    output_directory: PathBuf,
    compilation_settings: CompilationSettings,
    translation_units: Vec<TranslationUnit>,
}

pub trait Incremental {
    fn translation_units(&self) -> Vec<TranslationUnit>;
    fn representative_path(&self) -> PathBuf;
}

impl IncrementalCompiler {
    pub fn new<P: AsRef<Path>>(
        object_path: ObjectPath,
        code_path: P,
        output_path: P,
        mut settings: CompilationSettings,
    ) -> Self {
        settings.target_directory = output_path.as_ref().to_path_buf();
        Self {
            object_path,
            code_path: code_path.as_ref().to_path_buf(),
            output_directory: output_path.as_ref().to_path_buf(),
            compilation_settings: settings,
            translation_units: vec![],
        }
    }

    // should compile if depended
    fn should_compile(
        &self,
        input: &Path,
        output: &Path,
        graph: &CompilationGraph,
    ) -> crate::Result<bool> {
        if !output.exists() {
            return Ok(true);
        }
        let input_modified = input.metadata()?.modified()?;
        let output_modified = output.metadata()?.modified()?;

        // if output is older that input
        if output_modified < input_modified {
            return Ok(true);
        }

        let mut incoming = graph.dependencies(input);
        for incoming_file in incoming {
            // if incoming output doesn't exist or is older than an incoming file that has been modified
            let incoming_file_output = self.as_output_file(incoming_file);
            if !incoming_file_output.exists()
                || output_modified < incoming_file.metadata()?.modified()?
            {
                return Ok(true);
            }
        }
        Ok(false)
    }

    fn as_output_file(&self, path: &Path) -> PathBuf {
        PathBuf::from_iter(&[
            &self.output_directory,
            &self.code_path.relativize(path).unwrap(),
        ])
        .with_extension(COMPILED_FILE_EXTENSION)
    }

    /// Compile a compilation graph
    pub fn compile(&mut self, graph: CompilationGraph) -> crate::Result<()> {
        let nodes = graph.topological_order();
        for node in nodes {
            let input_path = &node.path;
            let in_module = node
                .parsed_node
                .resolved_id()
                .cloned()
                .unwrap_or(Identifier::empty());

            let output_path = self.as_output_file(input_path);

            if !self.should_compile(input_path, &*output_path, &graph)? {
                continue;
            }

            let mut compile = PreloadCompiler::new(&self.translation_units);
            let mut compilation_object: CompilationObject =
                compile.create_compilable(&node.parsed_node)?;

            let created_units = compilation_object.units.clone();
            self.translation_units.extend(created_units);

            let output_file = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(output_path)?;

            let ref mut padded_writer = PaddedWriter::new(output_file);
            Compilable::<JodinVM>::compile(compilation_object, &Context::new(), padded_writer)?;
        }
        Ok(())
    }
}

pub struct PreloadCompiler<'a> {
    translation_units: &'a Vec<TranslationUnit>,
}

impl<'a> PreloadCompiler<'a> {
    pub fn new(translation_units: &'a Vec<TranslationUnit>) -> Self {
        Self { translation_units }
    }
}

impl<'a> MicroCompiler<JodinVM, CompilationObject> for PreloadCompiler<'a> {
    fn create_compilable(&mut self, tree: &JodinNode) -> JodinResult<CompilationObject> {
        todo!()
    }
}
