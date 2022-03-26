//! Calling the compiler and interpreter

use crate::compilation::object_path::ObjectPath;
use std::collections::{HashMap, HashSet};
use std::ffi::OsStr;
use std::ops::RangeInclusive;
use std::path::PathBuf;
use std::str::FromStr;

/// The jodinc compiler is an incremental compiler that searches for relevant imports from given
/// directories or zipped files. Also searches the directory that files are within.
#[derive(Debug, Parser)]
#[clap(version, author, about)]
pub struct JodinRsApp {
    /// The debug level of the compiler. Ranges from 0 to 5
    #[clap(short = 'd', long = "debug", validator = debug_level_validator, default_value_t = 3)]
    pub debug_level: u8,
    /// The target directory of the compiler
    #[clap(short, long = "--target")]
    pub target_directory: Option<PathBuf>,
    /// The objectpath for the compiler to use. The compiler will search for declarations here.
    #[clap(
        short = 'o',
        long = "objectpath",
        parse(from_os_str = parse_target_directory),
    )]
    pub objectpath: Vec<crate::Result<ObjectPath>>,
    /// Set project properties with `key[=value]`
    #[clap(short = 'P', parse(from_str = custom_key_value_pair), use_value_delimiter(false))]
    properties: Vec<(String, Option<String>)>,
    /// Input files to compile. Can use globbing patterns, like `**/*.jdn`
    pub inputs: Vec<String>,
}

impl JodinRsApp {
    pub fn valued_properties(&self) -> HashMap<&str, &str> {
        self.properties
            .iter()
            .filter_map(|(key, value)| value.as_ref().map(|s| (key.as_str(), s.as_str())))
            .collect()
    }

    pub fn all_properties(&self) -> HashSet<&str> {
        self.properties
            .iter()
            .map(|(key, _)| key.as_str())
            .collect()
    }
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

fn parse_target_directory(input: &OsStr) -> crate::Result<ObjectPath> {
    ObjectPath::try_from_iter(std::env::split_paths(input))
}

fn custom_key_value_pair(input: &str) -> (String, Option<String>) {
    if let Some((key, value)) = input.split_once('=') {
        (key.to_string(), Some(value.to_string()))
    } else {
        (input.to_string(), None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::clap::CommandFactory;
    use clap::Parser;

    #[test]
    fn can_set_properties() {
        let parser = JodinRsApp::parse_from(vec!["--debug", "0"]).boxed();
        println!("{:#?}", parser);
    }
}
