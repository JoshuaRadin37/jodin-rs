//! The frontend of the compiler.
//!
//! Takes files as inputs. Each file is first read completely into memory. Then [complete_parse] is
//! called on the string. If parse tree output is set to true, then the `*.pt` file is outputted
//! after this step is successful. After that, the [JodinNodeBuilder] will be called on the resulting
//! parse tree.
//!
//! [complete_parse]: crate::parsing::complete_parse
//! [JodinNodeBuilder]: crate::ast::JodinNodeBuilder

use std::fs::File;
use std::io::Write;
use std::io::{BufReader, Read};
use std::path::PathBuf;

use crate::ast::JodinNode;
use crate::ast::JodinNodeBuilder;
use crate::compilation_settings::CompilationSettings;
use crate::core::error::JodinResult;
use crate::parsing::{complete_parse, JodinRule};
use crate::passes::toolchain::JodinFallibleCollectorTool;

/// A tool to turn files into a jodin node tree
pub struct FilesToJodinNodeTool<'a> {
    builder: JodinNodeBuilder<'a>,
    settings: &'a CompilationSettings,
}

impl<'a> FilesToJodinNodeTool<'a> {
    /// Creates a new toolchain that uses the a reference to compilation settings
    pub fn new(settings: &'a CompilationSettings) -> Self {
        Self {
            builder: JodinNodeBuilder::new(settings),
            settings,
        }
    }

    /// When finish is called, no more files can be added to this JodinNode.
    pub fn finish(self) -> JodinResult<JodinNode> {
        self.builder.finish()
    }
}

impl JodinFallibleCollectorTool for FilesToJodinNodeTool<'_> {
    type Input = PathBuf;
    type Output = ();

    fn invoke<I: IntoIterator<Item = Self::Input>>(
        &mut self,
        input_iter: I,
    ) -> JodinResult<Self::Output> {
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
        Ok(())
    }
}
