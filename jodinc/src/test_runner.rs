use crate::compilation::incremental::incremental_compiler::IncrementalCompiler;
use crate::compilation_settings::CompilationSettings;
use crate::test_runner::ProjectBuilderInput::{Directory, File, Raw};
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug)]
pub struct ProjectBuilder {
    project_name: String,
    object_paths: Vec<PathBuf>,
    input: Option<ProjectBuilderInput>,
    settings: Option<CompilationSettings>,
}

#[derive(Debug)]
enum ProjectBuilderInput {
    Raw(String),
    File(PathBuf),
    Directory(PathBuf),
}

impl ProjectBuilder {
    pub fn new(name: &str) -> Self {
        ProjectBuilder {
            project_name: name.to_string(),
            object_paths: vec![],
            input: None,
            settings: None,
        }
    }

    pub fn use_string<S: AsRef<str>>(mut self, s: S) -> Self {
        if std::mem::replace(&mut self.input, Some(Raw(s.as_ref().to_string()))).is_some() {
            panic!("Input already set")
        }
        self
    }

    pub fn use_file<P: AsRef<Path>>(mut self, s: P) -> Self {
        if std::mem::replace(&mut self.input, Some(File(s.as_ref().to_path_buf()))).is_some() {
            panic!("Input already set")
        }
        self
    }

    pub fn use_dir<P: AsRef<Path>>(mut self, s: P) -> Self {
        if std::mem::replace(&mut self.input, Some(Directory(s.as_ref().to_path_buf()))).is_some() {
            panic!("Input already set")
        }
        self
    }

    pub fn settings(mut self, settings: CompilationSettings) -> Self {
        self.settings = Some(settings);
        self
    }

    pub fn objects_path<S: AsRef<str>>(mut self, path: S) -> Self {
        let path = path.as_ref();
        let paths = std::env::split_paths(path);
        self.object_paths.extend(paths);
        self
    }

    pub fn objects_paths<'s, I: IntoIterator<Item = &'s Path>>(mut self, paths: I) -> Self {
        for path in paths {
            self.object_paths.push(path.to_path_buf());
        }
        self
    }

    /// Runs a jodin compiler on the input
    ///
    /// # Result
    ///
    /// If the compiler finished, the output is Ok(path to the compiled files),
    /// otherwise an error is returned
    pub fn compile(self) -> Result<PathBuf, Box<dyn Error>> {
        let cargo_program = std::env::var("CARGO").expect("No cargo installed on this system");
        let cargo_output = Command::new(&cargo_program)
            .arg("locate-project")
            .arg("--workspace")
            .output()?;

        if !cargo_output.status.success() {
            return Err(format!(
                "cargo locate-workspace failed ({})",
                String::from_utf8(cargo_output.stderr)?
            )
            .into());
        }
        let json_string = String::from_utf8(cargo_output.stdout)?;
        let dict: HashMap<String, &Path> = serde_json::from_str(&json_string)?;
        let &full_path = dict
            .get("root")
            .ok_or("Unexpected output found for locate-workspace")?;

        println!("{:?}", full_path);

        let parent_path = PathBuf::from(full_path.parent().unwrap());
        let meta_data_output = Command::new(&cargo_program)
            .arg("metadata")
            .current_dir(parent_path)
            .output()?;
        if !meta_data_output.status.success() {
            return Err(format!(
                "cargo meta-data failed ({})",
                String::from_utf8(cargo_output.stderr)?
            )
            .into());
        }

        let metadata: HashMap<String, Value> =
            serde_json::from_str(&String::from_utf8(meta_data_output.stdout)?)?;

        let mut output_path = PathBuf::new();
        if let Some(val) = metadata.get("target_directory") {
            if let Value::String(s) = val {
                output_path.push(s);
            } else {
                return Err("target_directory not a string".into());
            }
        } else {
            return Err("target_directory is not present".into());
        }
        output_path.push("jodin-tests");
        output_path.push(self.project_name);

        std::fs::create_dir_all(&output_path)?;
        trace!("Outputting to directory: {:?}", output_path);
        let mut compiler = IncrementalCompiler::new(
            &output_path,
            self.settings.unwrap_or(CompilationSettings::default()),
        );

        match self.input {
            None => {
                panic!("Input must be given for project builder");
            }
            Some(Raw(s)) => {
                debug!("Compiling a string...");
                compiler.compile_to_object(s)?;
            }
            Some(File(p)) => {
                let reading_file = std::fs::File::open(p)?;
                let mut buffer = Vec::new();
                let mut buffered_reader = BufReader::new(reading_file);
                buffered_reader.read_to_end(&mut buffer)?;
                let as_string = String::from_utf8(buffer)?;
                compiler.compile_to_object(as_string)?;
            }
            Some(Directory(p)) => {
                todo!()
            }
        }

        Ok(output_path)
    }
}
