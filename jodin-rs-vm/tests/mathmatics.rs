#[macro_use]
extern crate jasm_macros;

use jodin_common::assembly::asm_block::AssemblyBlock;
use jodin_common::assembly::instructions::Assembly;
use jodin_common::init_logging;
use jodin_tests_common::jvm_runner::JVMRunner;
use log::{info, trace, LevelFilter};

fn create_fib_sequence_asm(n: u32) -> Assembly {
    let block = block![
        main:
        label!(pub main);
        return_!(call!(~ fibonacci, n));

        label!(pub fibonacci);
        scope!(global);
        scope!(push);
        var!(=> 0);
        if_!(
            (expr!(<, dvar!(0), 2u32)) {
                block![
                    dvar!(0);
                    scope!(back);
                    return_!();
                ]
            } else {
                block![
                        expr!(+,
                                call!(~ fibonacci, expr!(-, dvar!(0), 1u32)),
                                call!(~ fibonacci, expr!(-, dvar!(0), 2u32))
                            );
                        scope!(back);
                        return_! ();
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
        n => rust_fib(n - 1) + rust_fib(n - 2),
    }
}

fn fibonacci_test(n: u32) {
    let asm = create_fib_sequence_asm(n);

    let runner = JVMRunner::default().with_jasm(asm);
    let result = runner.execute().unwrap();
    let expected = rust_fib(n);

    assert_eq!(
        result.exit_code(),
        expected,
        "Incorrectly calculated fibonacci({n})"
    );
}

#[test]
fn fibonacci() {
    for n in 2..=10 {
        fibonacci_test(n);
    }
}
