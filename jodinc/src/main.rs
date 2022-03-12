use jodin_common::compilation_settings::CompilationSettings;
use jodin_common::error::{JodinError, JodinErrorType, JodinResult};
use jodin_common::init_logging;
use jodinc::cli::JodinRsApp;

use jodinc::passes::frontend::FilesToJodinNodeTool;

use jodinc::process_jodin_node;
use log::{error, LevelFilter};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::exit;
use std::str::FromStr;
use jodinc::compilation::incremental::IncrementalCompiler;

fn main() {
    let cli = JodinRsApp::new();
    let matches = cli.into_matches();
    let mut settings = CompilationSettings::default();

    if let Some(target) = matches.value_of("target") {
        let path = PathBuf::from(target);
        settings.target_directory = path;
    }
    if matches.is_present("pt") {
        settings.output_parse_tree = true;
    }
    if matches.is_present("ast") {
        settings.output_ast = true;
    }
    if matches.is_present("tast") {
        settings.output_tast = true;
    }
    if matches.is_present("debug") {
        let level = match u8::from_str(matches.value_of("debug").unwrap()) {
            Ok(0) => LevelFilter::Off,
            Ok(1) => LevelFilter::Error,
            Ok(2) => LevelFilter::Warn,
            Ok(3) => LevelFilter::Info,
            Ok(4) => LevelFilter::Debug,
            Ok(5) => LevelFilter::Trace,
            Ok(o) => panic!("No debug level {}", o),
            Err(_) => panic!("invalid value for debug"),
        };
        init_logging(level)
    } else {
        init_logging(LevelFilter::Info)
    }


    let inputs = matches.values_of("INPUT").unwrap();
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
        &[] => return,
        errors => {
            for error in errors {
                error!("{}", error);
                error!("{:?}", error.backtrace())
            }
        }
    }

}
