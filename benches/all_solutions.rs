use criterion::{criterion_group, criterion_main, Criterion};

#[allow(clippy::redundant_closure)]
pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("all solutions", |b| {
        b.iter(|| advent_2019::run_all_solutions())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
