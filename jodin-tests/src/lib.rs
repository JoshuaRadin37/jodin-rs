#![cfg(test)]

use jodin_asm::{default_logging, init_logging};
use jodinc::test_runner::ProjectBuilder;
use log::LevelFilter;
use std::error::Error;
use std::path::PathBuf;

#[test]
fn fibonacci() {
    init_logging(LevelFilter::Trace);
    let builder = ProjectBuilder::new("fibonacci").use_string(
        r#"
            in lib;
            fn fibonacci(n: int) -> int {
                if (n < 2) {
                    return n;
                } else {
                    return lib::fibonacci(n - 1) + lib::fibonacci(n - 2);
                }
            }
            "#,
    );

    let dir = match builder.compile() {
        Ok(d) => d,
        Err(e) => {
            panic!("{}", e)
        }
    };
}
