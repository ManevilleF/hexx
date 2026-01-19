use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use hexx::*;
use std::hint::black_box;

pub fn rings_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Hex Rings");
    group
        .sample_size(500)
        .warm_up_time(std::time::Duration::from_secs(3))
        .measurement_time(std::time::Duration::from_secs(15))
        .noise_threshold(0.02);
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
