#[macro_use]
extern crate jasm_macros;

use log::{info, LevelFilter, trace};
use jodin_common::assembly::asm_block::AssemblyBlock;
use jodin_common::assembly::instructions::Assembly;
use jodin_common::init_logging;
use jodin_tests_common::jvm_runner::JVMRunner;

fn create_fib_sequence_asm(n: u32) -> Assembly {
    let block = block![
       label!(pub main);
        return_!(call!(~ fibonacci, n));
        label!(pub fibonacci);
        var!(=> 0);
        if_!(
            (expr!(<, dvar!(0), 2u32)) {
                block![
                    dvar!(0);
                    return_!();
                ]
            } else {
                block![
                        dvar!(0);
                        call!(~ fibonacci, expr!(-, dvar!(0), 1u32));
                        native!("@print_stack");
                        var!(=> 1);
                        var!(=> 0);
                        call!(~ fibonacci, expr!(-, dvar!(0), 1u32));
                        var!(=> 2);
                        var!(=> 0);
                        native!("@print_stack");
                        return_! (expr!(+, dvar!(1), dvar!(2)));
                    ]
            }
        );
    ];
    trace!("{:#?}", block);
    block.normalize()
}

fn rust_fib(n: u32) -> u32 {
    match n {
        0..=1 => n,
        n => rust_fib(n - 1) + rust_fib(n - 2)
    }
}

fn fibonacci_test(n: u32) {
    let asm = create_fib_sequence_asm(n);

    let runner = JVMRunner::default()
        .with_jasm(asm);
    let result = runner.execute().unwrap();
    let expected = rust_fib(n);

    assert_eq!(result.exit_code(), expected, "Incorrectly calculated fibonacci({n})");
}

#[test]
fn fibonacci() {
    init_logging(LevelFilter::Info);
    for n in 2..=10 {
        fibonacci_test(n);
    }
}