use criterion::{criterion_group, criterion_main, Criterion};

#[allow(clippy::redundant_closure)]
pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("individual");
    group.sample_size(10);
    group.bench_function("individual", |b| {
        b.iter(|| advent_2019::twenty::twenty_b());
    });
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
