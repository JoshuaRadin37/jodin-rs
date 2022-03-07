use crate::compilation::incremental::IncrementalCompiler;
use jodin_common::compilation_settings::CompilationSettings;
use jodin_common::identifier::Identifier;
use jodin_common::unit::CompilationObject;
use jodin_rs_vm::core_traits::VirtualMachine;
use jodin_rs_vm::mvp::{MinimumALU, MinimumMemory};
use jodin_rs_vm::vm::VMBuilder;
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use std::fs::FileType;

use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use temp_dir::TempDir;
use walkdir::WalkDir;
use jodin_common::assembly::instructions::{Assembly, GetAsm};

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
        if std::mem::replace(
            &mut self.input,
            Some(ProjectBuilderInput::Raw(s.as_ref().to_string())),
        )
        .is_some()
        {
            panic!("Input already set")
        }
        self
    }

    pub fn use_file<P: AsRef<Path>>(mut self, s: P) -> Self {
        if std::mem::replace(
            &mut self.input,
            Some(ProjectBuilderInput::File(s.as_ref().to_path_buf())),
        )
        .is_some()
        {
            panic!("Input already set")
        }
        self
    }

    pub fn use_dir<P: AsRef<Path>>(mut self, s: P) -> Self {
        if std::mem::replace(
            &mut self.input,
            Some(ProjectBuilderInput::Directory(s.as_ref().to_path_buf())),
        )
        .is_some()
        {
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

    pub fn compile_to_jasm(self) -> Result<Assembly, Box<dyn Error>> {
        let temp_dir = TempDir::new()?;
        let compiled = self.compile_to_path(temp_dir.path().to_path_buf())?;
        let mut output = Assembly::new();
        for entry in WalkDir::new(compiled) {
            let entry = entry?.into_path();
            if std::fs::metadata(&entry)?.is_file() {
                let compilable = CompilationObject::try_from(entry)?;
                output.extend(compilable.get_asm());
            }
        }
        Ok(output)
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
        let dict: HashMap<String, String> = serde_json::from_str(&json_string)?;
        let full_path: PathBuf = dict
            .get("root")
            .ok_or("Unexpected output found for locate-workspace")?
            .into();

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
        output_path.push(self.project_name.clone());

        std::fs::create_dir_all(&output_path)?;

        self.compile_to_path(output_path)
    }

    fn compile_to_path(self, output_path: PathBuf) -> Result<PathBuf, Box<dyn Error>> {
        trace!("Outputting to directory: {:?}", output_path);
        let mut compiler = IncrementalCompiler::new(
            &output_path,
            self.settings.unwrap_or(CompilationSettings::default()),
        );

        match self.input {
            None => {
                panic!("Input must be given for project builder");
            }
            Some(ProjectBuilderInput::Raw(s)) => {
                debug!("Compiling a string...");
                compiler.compile_to_object(s)?;
            }
            Some(ProjectBuilderInput::File(p)) => {
                let reading_file = std::fs::File::open(p)?;
                let mut buffer = Vec::new();
                let mut buffered_reader = BufReader::new(reading_file);
                buffered_reader.read_to_end(&mut buffer)?;
                let as_string = String::from_utf8(buffer)?;
                compiler.compile_to_object(as_string)?;
            }
            Some(ProjectBuilderInput::Directory(_p)) => {
                todo!()
            }
        }

        Ok(output_path)
    }

    /// Compiles then executes
    pub fn execute(self, function: Identifier) -> Result<(u32, String, String), Box<dyn Error>> {
        let path = self.compile()?;
        let mut stdout = Vec::<u8>::new();
        let mut stderr = Vec::<u8>::new();
        let mut virtual_machine = VMBuilder::new()
            .memory(MinimumMemory::default())
            .alu(MinimumALU)
            .with_stdout(&mut stdout)
            .with_stderr(&mut stderr)
            .build()?;

        let obj = CompilationObject::try_from(path)?;
        virtual_machine.load(obj);
        let start = function
            .os_compat_str()
            .ok_or("Function name incompatible")?;
        let result = virtual_machine.run(&start)?;
        drop(virtual_machine);
        Ok((
            result,
            String::from_utf8(stdout)?,
            String::from_utf8(stderr)?,
        ))
    }
}
