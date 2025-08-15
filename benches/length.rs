use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use hexx::*;

#[inline]
const fn old_hex_length(hex: Hex) -> i32 {
    (hex.x.abs() + hex.y.abs() + hex.z().abs()) / 2
}

pub fn length_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Hex Length");
    group.significance_level(0.1).sample_size(1_000);
    let dist = 1_000_000;

    group.bench_with_input(BenchmarkId::new("Length", dist), &dist, |b, dist| {
        b.iter(|| Hex::length(Hex::splat(*dist)))
    });
    group.bench_with_input(BenchmarkId::new("Old length", dist), &dist, |b, dist| {
        b.iter(|| old_hex_length(Hex::splat(*dist)))
    });
    group.finish();
}

criterion_group!(benches, length_benchmark);
criterion_main!(benches);
