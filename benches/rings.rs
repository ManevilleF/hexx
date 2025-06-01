use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use hexx::*;

pub fn rings_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Hex Rings");
    group.significance_level(0.1).sample_size(1000);
    let dist: u32 = 1000;

    group.bench_with_input(BenchmarkId::new("Range", dist), &dist, |b, dist| {
        b.iter(|| Hex::range(black_box(Hex::ZERO), *dist).collect::<Vec<_>>())
    });
    group.bench_with_input(BenchmarkId::new("XRange", dist), &dist, |b, dist| {
        b.iter(|| Hex::xrange(black_box(Hex::ZERO), *dist).collect::<Vec<_>>())
    });
    group.bench_with_input(BenchmarkId::new("Ring", dist), &dist, |b, dist| {
        b.iter(|| Hex::ring(black_box(Hex::ZERO), *dist).collect::<Vec<_>>())
    });
    group.bench_with_input(BenchmarkId::new("Rings", dist), &dist, |b, dist| {
        b.iter(|| Hex::rings(black_box(Hex::ZERO), 0..=*dist).collect::<Vec<_>>())
    });
    group.bench_with_input(BenchmarkId::new("Spiral Rings", dist), &dist, |b, dist| {
        b.iter(|| Hex::spiral_range(black_box(Hex::ZERO), 0..=*dist).collect::<Vec<_>>())
    });
    group.finish();
}

criterion_group!(benches, rings_benchmark);
criterion_main!(benches);
