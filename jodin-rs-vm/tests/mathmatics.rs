#[macro_use]
extern crate jasm_macros;

use log::LevelFilter;
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
            (expr!(<u, dvar!(0), 2u32)) {
                return_!(dvar!(0))
            } else {
                return_! (
                    expr!(+,
                        call!(~ fibonacci, expr!(-, dvar!(0), 1u32)),
                        call!(~ fibonacci, expr!(-, dvar!(0), 2u32))
                    )
                )
            }
        );
    ];
    println!("{:#?}", block);
    block.normalize()
}

fn dynamic_fibonacci(n: u32) -> u32 {
    let mut output = vec![0; n as usize + 1];
    output[1] = 1;
    for i in 2..(n as usize) {
        output[i] = output[i - 1] + output[i - 2];
    }
    output[n as usize]
}

fn fibonacci_test(n: u32) {
    let asm = create_fib_sequence_asm(n);

    let runner = JVMRunner::default()
        .with_jasm(asm);
    let result = runner.execute().unwrap();
    let expected = dynamic_fibonacci(n);

    assert_eq!(result.exit_code(), expected, "Incorrectly calculated fibonacci({n})");
}

#[test]
fn fibonacci() {
    init_logging(LevelFilter::Info);
    for n in 1..=10 {
        fibonacci_test(n);
    }
}