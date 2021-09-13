//! The frontend of the compiler.
//!


use std::path::PathBuf;


use crate::ast::JodinNode;
use crate::compilation_settings::CompilationSettings;
use crate::core::error::JodinResult;

/// A tool to turn files into a jodin node tree
pub struct FilesToJodinNodeTool<'a> {
    settings: &'a CompilationSettings,
}

impl<'a> FilesToJodinNodeTool<'a> {
    /// Creates a new toolchain that uses the a reference to compilation settings
    pub fn new(settings: &'a CompilationSettings) -> Self {
        Self { settings }
    }

    /// When finish is called, no more files can be added to this JodinNode.
    pub fn finish(self) -> JodinResult<JodinNode> {
        todo!()
    }

    /// invoke the front end
    pub fn invoke<I: IntoIterator<Item = PathBuf>>(&mut self, input_iter: I) -> JodinResult<()> {
        /*
        for path in input_iter {
            let file = File::open(&path)?;
            let mut buffer = String::new();
            BufReader::new(file).read_to_string(&mut buffer)?;
            let pairs = complete_parse(JodinRule::jodin_file, buffer.as_str())?
                .nth(0)
                .unwrap();
            if self.settings.output_parse_tree {
                let string = format!("{:#?}", pairs);
                let mut new_path = PathBuf::from(&path);
                new_path.set_extension("pt");
                let newer_path = self.settings.output_file_path(new_path);
                let mut file = File::create(newer_path)?;
                writeln!(file, "{}", string)?;
            }
            self.builder
                .add_source_string(format!("{}", path.to_str().unwrap()), pairs)?;
        }

         */
        Ok(())
    }
}
