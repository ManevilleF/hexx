use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use hexx::*;
use std::hint::black_box;

pub fn wedge_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Hex Wedge");
    group
        .sample_size(1000)
        .warm_up_time(std::time::Duration::from_secs(3))
        .noise_threshold(0.02);
    let dist = 500;

    group.bench_with_input(BenchmarkId::new("Wedge", dist), &dist, |b, dist| {
        b.iter(|| {
            Hex::wedge(
                black_box(Hex::ZERO),
                0..=*dist as u32,
                VertexDirection::FLAT_LEFT,
            )
            .collect::<Vec<_>>()
        })
    });
    group.bench_with_input(BenchmarkId::new("Full Wedge", dist), &dist, |b, dist| {
        b.iter(|| {
            Hex::full_wedge(
                black_box(Hex::ZERO),
                *dist as u32,
                VertexDirection::FLAT_LEFT,
            )
            .collect::<Vec<_>>()
        })
    });
    group.bench_with_input(BenchmarkId::new("Triangle", dist), &dist, |b, dist| {
        b.iter(|| shapes::triangle(*dist as u32).collect::<Vec<_>>())
    });
    group.bench_with_input(BenchmarkId::new("Corner Wedge", dist), &dist, |b, dist| {
        b.iter(|| {
            Hex::corner_wedge(
                black_box(Hex::ZERO),
                0..=*dist as u32,
                EdgeDirection::FLAT_TOP,
            )
            .collect::<Vec<_>>()
        })
    });
    group.finish();
}

criterion_group!(benches, wedge_benchmark);
criterion_main!(benches);
