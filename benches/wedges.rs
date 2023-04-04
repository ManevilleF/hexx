use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use hexx::*;

pub fn wedge_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Hex Wedge");
    group.significance_level(0.1).sample_size(100);
    let dist = 1000;

    group.bench_with_input(BenchmarkId::new("Wedge", dist), &dist, |b, dist| {
        b.iter(|| {
            Hex::wedge(
                black_box(Hex::ZERO),
                0..=*dist as u32,
                DiagonalDirection::Left,
            )
            .collect::<Vec<_>>()
        })
    });
    group.bench_with_input(BenchmarkId::new("Full Wedge", dist), &dist, |b, dist| {
        b.iter(|| {
            Hex::full_wedge(black_box(Hex::ZERO), *dist as u32, DiagonalDirection::Left)
                .collect::<Vec<_>>()
        })
    });
    group.bench_with_input(BenchmarkId::new("Triangle", dist), &dist, |b, dist| {
        b.iter(|| shapes::triangle(*dist as u32).collect::<Vec<_>>())
    });
    group.bench_with_input(BenchmarkId::new("Corner Wedge", dist), &dist, |b, dist| {
        b.iter(|| {
            Hex::corner_wedge(black_box(Hex::ZERO), 0..=*dist as u32, Direction::Top)
                .collect::<Vec<_>>()
        })
    });
    group.finish();
}

criterion_group!(benches, wedge_benchmark);
criterion_main!(benches);
