use jodin_rs::cli::JodinRsApp;
use jodin_rs::compilation_settings::CompilationSettings;
use jodin_rs::core::error::{JodinErrorType, JodinResult};
use jodin_rs::passes::frontend::FilesToJodinNodeTool;
use jodin_rs::passes::optimize;
use std::path::PathBuf;
use std::process::exit;

fn main() -> JodinResult<()> {
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

    let (_command, command_args) = match matches.subcommand() {
        (command, Some(args)) => (command, args),
        _ => panic!("No subcommand used, must either use run or check"),
    };

    let inputs = command_args.values_of("INPUT").unwrap();
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
    println!("{:?}", full_paths);
    let mut builder = FilesToJodinNodeTool::new(&settings);
    let result = builder.invoke(full_paths);
    if let Err(e) = result {
        match e.into_err_and_bt() {
            (JodinErrorType::ParserError(e, file), bt) => {
                let error = if let Some(path) = file {
                    e.with_path(path.as_str())
                } else {
                    e
                };

                eprintln!("{:#}", error);
                eprintln!("{:?}", bt);
                exit(-1);
            }
            (error, bt) => {
                eprintln!("{:?}", error);
                eprintln!("{:?}", bt);
                exit(-1);
            }
        }
    }

    let node = builder.finish()?;
    println!("{:?}", node);

    let optimized = optimize(node)?;
    println!("{:?}", optimized);

    Ok(())
}
