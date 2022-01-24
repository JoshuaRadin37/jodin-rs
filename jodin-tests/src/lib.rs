#![cfg(test)]

use jodin_common::error::JodinError;
use jodin_common::init_logging;
use std::sync::{Mutex, RwLock};

use jodin_rs_vm::core_traits::{ArithmeticsTrait, MemoryTrait, VirtualMachine};
use jodin_rs_vm::mvp::{MinimumALU, MinimumMemory};
use jodin_rs_vm::vm::{VMBuilder, VM};
use jodinc::test_runner::ProjectBuilder;
use lazy_static::lazy_static;
use log::{info, LevelFilter};

#[test]
fn fibonacci() {
    init_logging(LevelFilter::Info);
    let builder = ProjectBuilder::new("fibonacci").use_string(
        r#"
           
            fn fibonacci(n: int) -> int {
                let output: int = 0;
                if (n < 2) {
                    output = n;
                } else {
                    output = fibonacci(n - 1) + fibonacci(n - 2);
                }
                println(output);
                return output;
            }
            
            fn print(value: int) {
                __NATIVE("print", value);
            }
            
            fn println(value: void) {
                print(value);
                print("\n");
            }
            
            fn main() -> unsigned int {
                let index: int = 0;
                while (index <= 10) {
                    println(fibonacci(index));
                }
                
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

    let mut buffer = Vec::<u8>::new();
    let mut vm = VMBuilder::new()
        .memory(MinimumMemory::default())
        .alu(MinimumALU)
        .object_path(dir)
        .with_stdout(&mut buffer)
        .build()
        .expect("Should be able to build");

    info!("VM: {:#?}", vm);
    info!("INSTRUCTIONS: {:#?}", vm.instructions());

    let r = vm.run("main").unwrap();
    drop(vm);
    println!();
    assert_eq!(r, 0);

    let output = String::from_utf8(buffer).expect("Output should be utf-8");
    let vm_calculated = output
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<i32>>();

    fn fib(n: i32) -> i32 {
        match n {
            0..=1 => n,
            n => fib(n - 1) + fib(n - 2),
        }
    }

    let expected = (0..=10).into_iter().map(|n| fib(n)).collect::<Vec<i32>>();

    assert_eq!(
        vm_calculated, expected,
        "The virtual machine should have calculated the fib(n) value from 0 through 10"
    )
}
