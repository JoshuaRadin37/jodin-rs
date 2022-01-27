#![cfg(test)]

use jodin_common::error::JodinError;
use jodin_common::init_logging;
use std::sync::{Mutex, RwLock};
use std::time::Instant;

use jodin_common::utility::LoggedWrite;
use jodin_rs_vm::core_traits::{ArithmeticsTrait, MemoryTrait, VirtualMachine};
use jodin_rs_vm::mvp::{MinimumALU, MinimumMemory};
use jodin_rs_vm::scoped_memory::VMMemory;
use jodin_rs_vm::vm::{VMBuilder, VM};
use jodinc::test_runner::ProjectBuilder;
use lazy_static::lazy_static;
use log::{debug, info, Level, LevelFilter};

#[test]
fn fibonacci() {
    init_logging(LevelFilter::Off);
    let builder = ProjectBuilder::new("fibonacci").use_string(
        r#"
           
            fn fibonacci(n: int) -> int {
                let output: int = 0;
                if (n < 2) {
                    output = n;
                } else {
                    // output = 1;
                    // let one: int = 0;
                    // let two: int = 1;
                    // let i: int = 2;
                    // while (i <= n) {
                    //     output = one + two; // 1 1
                    //     one = two; // 1 0
                    //     two = output; // 0 1
                    //     i = i + 1;
                    // }
                    return fibonacci(n - 1) + fibonacci(n - 2);
                }
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
                while (index <= 20) {
                    println(fibonacci(index));
                    // println(index);
                    index = index + 1;
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

    let mut logged_buffer = LoggedWrite::new(Level::Info, &mut buffer, "VM".to_string());

    let mut vm = VMBuilder::new()
        .memory(VMMemory::default())
        .alu(MinimumALU)
        .object_path(dir)
        .with_stdout(&mut logged_buffer)
        .build()
        .expect("Should be able to build");

    debug!("VM: {:#?}", vm);
    let (r, duration) = vm.run_with_time("main");
    let r = r.unwrap();
    println!();
    println!(
        "VM completed running in {:.4} seconds",
        duration.as_secs_f32()
    );
    drop(vm);
    drop(logged_buffer);

    assert_eq!(r, 0);

    let output = String::from_utf8(buffer).expect("Output should be utf-8");
    let vm_calculated = output
        .lines()
        .map(|line| {
            line.parse()
                .expect(format!("not an integer (given: {line})").as_str())
        })
        .collect::<Vec<i32>>();

    fn fib(n: i32) -> i32 {
        match n {
            0..=1 => n,
            n => fib(n - 1) + fib(n - 2),
        }
    }

    let start = Instant::now();
    let expected = (0..=20).into_iter().map(|n| fib(n)).collect::<Vec<i32>>();
    let elapsed = start.elapsed();

    println!("Native code ran in {:.4} seconds", elapsed.as_secs_f32());

    println!("Found values = {:?}", vm_calculated);

    assert_eq!(
        vm_calculated, expected,
        "The virtual machine should have calculated the fib(n) value from 0 through 10"
    )
}
