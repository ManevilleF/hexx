use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use hexx::*;

pub fn sum_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Hex Sum");
    group.significance_level(0.1).sample_size(100);
    let range: u32 = 1000;

    group.bench_with_input(
        BenchmarkId::new("Regular Sum", range),
        &range,
        |b, range| b.iter(|| Hex::ZERO.range(*range).fold(Hex::ZERO, |a, b| a + b)),
    );
    group.bench_with_input(BenchmarkId::new("SIMD Sum", range), &range, |b, range| {
        b.iter(|| Hex::ZERO.range(*range).sum::<Hex>())
    });
    group.finish();
}

criterion_group!(benches, sum_benchmark);
criterion_main!(benches);
