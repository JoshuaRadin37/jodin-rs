use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn test_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| black_box(20)));
}

criterion_group!(native_calls, test_benchmark);
criterion_main!(native_calls);
