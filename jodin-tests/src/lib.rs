#![cfg(test)]

use jodin_common::error::JodinError;
use jodin_common::init_logging;

use jodin_rs_vm::core_traits::VirtualMachine;
use jodin_rs_vm::mvp::{MinimumALU, MinimumMemory};
use jodin_rs_vm::vm::{VMBuilder};
use jodinc::test_runner::ProjectBuilder;
use log::{info, LevelFilter};



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
            
            fn print(value: int) {
                __NATIVE("print", value);
            }
            
            in std
            public fn println(value: void) {
                print(value);
                print("\n");
            }
            
            fn main() -> unsigned int {
                std::println(factorial(6));
                std::println("Hello, World!");
                std::println(fibonacci(7));
                
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
