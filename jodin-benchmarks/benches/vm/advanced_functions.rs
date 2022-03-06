#[macro_use]
extern crate jasm_macros;

use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use jodin_tests_common::jvm_runner::JVMRunner;

pub fn fibonacci(c: &mut Criterion) {



    let fib_sequence =
    jasm![
        label!(pub main);
        push!(5u32);
        label!(pub fibonacci);
        var!(=> 0);
        if_!(
            (expr!(<, dvar!(0), 2usize)) {
                return_!(dvar!(0));
            } else {
                return_! (
                    expr!(+,
                        call!(~ fibonacci, expr!(-, dvar!(0), 1i32)),
                        call!(~ fibonacci, expr!(-, dvar!(0), 2i32))
                    )
                )
            }
        );
    ];

    c.bench_function(
        "fibonacci",
        |b| {
            b.iter_batched(
                || {
                    JVMRunner::default()
                        .with_jasm(fib_sequence.clone())
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
