//! Calling the compiler and interpreter

use crate::compilation::object_path::ObjectPath;
use std::ffi::OsStr;
use std::ops::RangeInclusive;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, Parser)]
#[clap(version, author, about)]
pub struct JodinRsApp {
    /// The debug level of the compiler. Ranges from 0 to 5
    #[clap(short = 'd', long = "debug", validator = debug_level_validator, default_value_t = 3)]
    pub debug_level: u8,
    /// The target directory of the compiler
    #[clap(short, long = "--target")]
    pub target_directory: Option<PathBuf>,
    /// The objectpath for the compiler to use.
    ///
    /// The compiler will search for declartions here.
    #[clap(
        short = 'o',
        long = "objectpath",
        parse(from_os_str = parse_target_directory),
        default_value_t = ObjectPath::project_dir()
    )]
    pub objectpath: ObjectPath,
    /// Input files to compile. Can use globbing patterns, like `**/*.jdn`
    pub inputs: Vec<String>,
}

const DEBUG_RANGE: RangeInclusive<u8> = 0..=5;

fn debug_level_validator(s: &str) -> Result<(), String> {
    u8::from_str(s)
        .map(|level| DEBUG_RANGE.contains(&level))
        .map_err(|l| l.to_string())
        .and_then(|result| match result {
            true => Ok(()),
            false => Err(format!(
                "Debug level not in range {}-{}",
                DEBUG_RANGE.start(),
                DEBUG_RANGE.end()
            )),
        })
}

fn parse_target_directory(input: &OsStr) -> ObjectPath {
    ObjectPath::from_iter(std::env::split_paths(input))
}
