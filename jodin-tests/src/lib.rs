#![cfg(test)]

use jodin_common::error::JodinError;
use jodin_common::init_logging;
use jodin_common::mvp::bytecode::Asm::Push;
use jodin_rs_vm::core_traits::VirtualMachine;
use jodin_rs_vm::mvp::{MinimumALU, MinimumMemory};
use jodin_rs_vm::vm::{VMBuilder, VM};
use jodinc::test_runner::ProjectBuilder;
use log::{info, LevelFilter};
use std::error::Error;
use std::path::PathBuf;

#[test]
fn fibonacci() {
    init_logging(LevelFilter::Warn);
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
            
            fn print(value: int) {
                __NATIVE("print", value);
            }
            
            fn main() -> unsigned int {
                print(factorial(6));
                return 0u;
            }

            "#,
    );

    let dir = match builder.compile() {
        Ok(d) => d,
        Err(e) => match e.downcast::<JodinError>() {
            Ok(e) => {
                panic!("{:#}", e)
            }
            Err(e) => {
                panic!("{}", e)
            }
        },
    };

    let mut vm = VMBuilder::new()
        .memory(MinimumMemory::default())
        .alu(MinimumALU)
        .object_path(dir)
        .build()
        .expect("Should be able to build");

    info!("VM: {:#?}", vm);

    let r = vm.run("main").unwrap();
    println!();
    assert_eq!(r, 0);
}
