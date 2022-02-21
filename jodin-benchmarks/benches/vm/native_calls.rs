use criterion::{black_box, criterion_group, criterion_main, Criterion, BatchSize};
use jasm_macros::jasm;
use jodin_tests_common::jvm_runner::JVMRunner;

pub fn print(c: &mut Criterion) {
    c.bench_function(
        "print",
        |b| {
            const HELLO_WORLD: &str = "Hello, World!";
            b.iter_batched(
                || {
                    JVMRunner::default()
                        .with_jasm(
                            jasm![
                                ASM_STYLE
                                pub main;
                                    push HELLO_WORLD;
                                    native ("print", 1)
                                    return 0u32;
                                ]
                        )
                },
                |runner| {
                    runner.execute().unwrap()
                },
                BatchSize::SmallInput
            )
        }
    );
}

criterion_group!(native_calls, print);
criterion_main!(native_calls);
