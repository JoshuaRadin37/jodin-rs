//! The incremental compiler uses an internal compiler that individually compiles parse nodes

use crate::compilation::incremental::unit::CompilationObject;
use crate::compilation::{Compiler, Target};
use crate::compilation_settings::CompilationSettings;
use crate::parsing::parse_program;
use crate::JodinNode;
use std::path::PathBuf;

pub struct IncrementalCompiler {
    object_path: Vec<PathBuf>,
    output_directory: PathBuf,
    compilation_settings: CompilationSettings,
}

impl IncrementalCompiler {
    fn compile_to_object<S: AsRef<str>>(&mut self, input: S) -> Result<CompilationObject, ()> {
        let parsed = parse_program(input)?;
        let optimized =
    }
}
