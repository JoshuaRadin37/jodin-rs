use jodin_common::compilation_settings::CompilationSettings;
use jodin_common::error::{JodinError, JodinErrorType, JodinResult};
use jodin_common::init_logging;
use jodinc::cli::JodinRsApp;

use jodinc::passes::frontend::FilesToJodinNodeTool;

use jodinc::compilation::incremental::IncrementalCompiler;
use jodinc::process_jodin_node;
use log::{error, info, LevelFilter};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::exit;
use std::str::FromStr;

use clap::Parser;
use jodinc::compilation::object_path::ObjectPath;

fn main() -> jodinc::Result<()> {
    let args: JodinRsApp = JodinRsApp::parse();

    println!("{:#?}", args.valued_properties());

    let mut settings = CompilationSettings::default();

    if let Some(target) = &args.target_directory {
        let path = PathBuf::from(target);
        settings.target_directory = path;
    }

    let level = match &args.debug_level {
        0 => LevelFilter::Off,
        1 => LevelFilter::Error,
        2 => LevelFilter::Warn,
        3 => LevelFilter::Info,
        4 => LevelFilter::Debug,
        5 => LevelFilter::Trace,
        o => panic!("No debug level {}", o),
    };
    init_logging(level);

    let inputs = &args.inputs;
    let mut full_paths = vec![];

    for input in inputs {
        let paths = glob::glob(input)
            .expect(format!("Given input was an invalid path: {}", input).as_str())
            .collect::<Vec<_>>();
        if paths.is_empty() {
            panic!("No file(s) can be found using the path {:?}", input);
        }
        for result in paths {
            match result {
                Ok(path) => {
                    if !path.exists() {
                        panic!("{:?} does not exist!", path)
                    }
                    full_paths.push(path)
                }
                Err(e) => {
                    panic!("{:?}", e)
                }
            }
        }
    }

    let mut object_path: ObjectPath = ObjectPath::empty();
    let objects = args.objectpath.into_iter().collect::<Result<Vec<_>, _>>()?;
    object_path += ObjectPath::from_iter(objects);

    info!("Using object path from cli: {:?}", object_path);
    object_path += ObjectPath::from_files(&full_paths);

    info!("Using object path: {0} ({0:?})", object_path);

    exit(-1);

    let mut incremental = IncrementalCompiler::new(settings.target_directory.clone(), settings);

    let mut errors = vec![];

    for path in full_paths {
        match incremental.compile_file(path) {
            Ok(()) => {}
            Err(e) => {
                errors.push(e);
            }
        }
    }

    match errors.as_slice() {
        &[] => return Ok(()),
        errors => {
            for error in errors {
                error!("{error}");
                for line in format!("{:?}", error.backtrace()).lines() {
                    error!("{}", line);
                }
            }
        }
    }
    Ok(())
}
