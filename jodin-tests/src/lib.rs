#![cfg(test)]

use jodin_asm::{default_logging, init_logging};
use jodin_common::test_runner::ProjectBuilder;
use log::LevelFilter;
use std::error::Error;
use std::path::PathBuf;

#[test]
fn fibonacci() {
    init_logging(LevelFilter::Info);
    let builder = ProjectBuilder::new("fibonacci").use_string(
        r#"
            
            fn fibonacci(n: int) -> int {
                if (n < 2) {
                    return n;
                } else {
                    return fibonacci(n - 1) + fibonacci(n - 2);
                }
            }
            
            
            fn factorial(n: int) -> int {
                if (n == 0) { return 1; }
                return factorial(n - 1) * n;
            }
            
            
            fn max(a: int, b: int) -> int {
                if (a < b) {
                    return b;
                } else {
                    return a;
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
