use crate::compilation_settings::CompilationSettings;
use crate::core::error::JodinResult;
use crate::parsing::ast::jodin_node::JodinNode;
use crate::parsing::ast::JodinNodeBuilder;
use crate::parsing::parser::{complete_parse, JodinRule};
use crate::passes::toolchain::JodinFallibleCollectorTool;

use std::fs::File;
use std::io::Write;
use std::io::{BufReader, Read};

use std::path::PathBuf;

pub struct FilesToJodinNodeTool<'a> {
    builder: JodinNodeBuilder<'a>,
    settings: &'a CompilationSettings,
}

impl<'a> FilesToJodinNodeTool<'a> {
    pub fn new(settings: &'a CompilationSettings) -> Self {
        Self {
            builder: JodinNodeBuilder::new(settings),
            settings,
        }
    }

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
