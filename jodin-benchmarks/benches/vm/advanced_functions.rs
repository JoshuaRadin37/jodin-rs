#[macro_use]
extern crate jasm_macros;

use criterion::{BatchSize, BenchmarkId, Criterion, criterion_group, criterion_main};
use jasm_macros::Assembly;
use jodin_tests_common::jvm_runner::JVMRunner;


fn create_fib_sequence_asm(n: u32) -> Assembly {
    let block = block![
        main:
        label!(pub main);
        return_!(call!(~ fibonacci, n));
        scope!(global);
        scope!(push);
        label!(pub fibonacci);
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
                        dvar!(0);
                        expr!(+,
                                call!(~ fibonacci, expr!(-, dvar!(0), 1u32)),
                                call!(~ fibonacci, expr!(-, dvar!(0), 2u32))
                            );
                        var!(=> 1);
                        var!(=> 0);
                        var!(1);
                        return_! ();
                    ]
            }
        );
    ];
    block.normalize()
}

pub fn fibonacci(c: &mut Criterion) {
    let fib_value = 10;

    c.bench_with_input(
        BenchmarkId::new("fibonacci", fib_value),
        &fib_value,
        |b, &n| {
            b.iter_batched(
                || {
                    let asm = create_fib_sequence_asm(n);
                    JVMRunner::default()
                        .with_jasm(asm)
                },
                |runner| {
                    runner.execute().unwrap()
                },
                BatchSize::SmallInput
            )
        }
    );
}

criterion_group!(mathmatics, fibonacci);
criterion_main!(mathmatics);
