use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use hexx::*;

pub fn line_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Hex line");
    group.significance_level(0.1).sample_size(1_000);
    let dist = 100_000;

    group.bench_with_input(BenchmarkId::new("Line", dist), &dist, |b, dist| {
        b.iter(|| {
            let p = black_box(Hex::ZERO);
            let _ = p.line_to(black_box(Hex::splat(*dist))).collect::<Vec<_>>();
        })
    });
    group.bench_with_input(BenchmarkId::new("Rectiline", dist), &dist, |b, dist| {
        b.iter(|| {
            let p = black_box(Hex::ZERO);
            let _ = p
                .rectiline_to(black_box(Hex::splat(*dist)), true)
                .collect::<Vec<_>>();
        })
    });
    group.finish();
}

criterion_group!(benches, line_benchmark);
criterion_main!(benches);
