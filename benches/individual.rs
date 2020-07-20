use criterion::{criterion_group, criterion_main, Criterion};

#[allow(clippy::redundant_closure)]
pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("individual", |b| {
        b.iter(|| advent_2019::six::six_a());
        b.iter(|| advent_2019::six::six_b());
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
