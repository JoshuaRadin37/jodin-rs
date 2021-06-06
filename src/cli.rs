use clap::{App, ArgMatches};
use std::ops::{Deref, DerefMut};

pub struct JodinRsApp<'a, 'b: 'a>(App<'a, 'b>);

impl<'a, 'b: 'a> Deref for JodinRsApp<'a, 'b> {
    type Target = App<'a, 'b>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, 'b: 'a> DerefMut for JodinRsApp<'a, 'b> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a, 'b: 'a> JodinRsApp<'a, 'b> {
    /// Creates the clap application to be used for the cli
    pub fn new() -> Self {
        Self(clap_app!(jodin_rs =>
            (version: "0.1.0")
            (author: "Joshua Radin <jradin16@gmail.com>")
            (about: "Interpreter for jodin written in rust")
            (@arg debug: -d --debug_level +takes_value "set the debug level, from 0 to 5")
            (@arg tast: --tast "outputs the tast (after id and type pass) into a file")
            (@arg ast: --ast "outputs the ast (before any passes) into a file")
            (@arg pt: --pt "outputs the parse tree into a file")
            (@arg include: -I --include +takes_value ... "include the contents of a directory for indexing")
            (@arg target_dir: -T --target_dir +takes_value "where generated files should be emitted")
            (@subcommand check =>
                (about: "Runs the parser, doesn't run any passes over the ast besides building it")
                (@arg INPUT: +required +takes_value ... "the file inputs")
            )
            (@subcommand run =>
                (about: "Runs the interpreter")
                (@arg INPUT: +required +takes_value ... "the file inputs")
            )
        ))
    }

    pub fn get_matches(self) -> ArgMatches<'a> {
        self.0.get_matches()
    }
}
