use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use hexx::*;

pub fn wedge_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Hex Wedge");
    group.significance_level(0.1).sample_size(1_000);
    let dist = 1_000_000;

    group.bench_with_input(BenchmarkId::new("Wedge", dist), &dist, |b, dist| {
        b.iter(|| Hex::full_wedge(Hex::ZERO, *dist as u32, DiagonalDirection::Left))
    });
    group.bench_with_input(BenchmarkId::new("Triangle", dist), &dist, |b, dist| {
        b.iter(|| shapes::triangle(*dist as u32))
    });
    group.finish();
}

criterion_group!(benches, wedge_benchmark);
criterion_main!(benches);
