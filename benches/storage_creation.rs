#![allow(missing_docs, clippy::all)]
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use hexx::{
    storage::{HexModMap, HexagonalMap, RombusMap},
    *,
};

pub fn creation_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Storage Map Creation");
    group
        .warm_up_time(std::time::Duration::from_secs(3))
        .measurement_time(std::time::Duration::from_secs(10))
        .noise_threshold(0.02)
        .sample_size(100);

    let get_value = |h: Hex| h.length();

    // ~1,000,000 hexes
    // HexagonalMap: 3*r*(r+1)+1 = 1_000_000 -> r approx 577
    let hex_radius = 577;
    // RombusMap: rows * columns = 1_000_000 -> 1000 * 1000
    let rombus_size = 1000;

    group.bench_with_input(
        BenchmarkId::new("HexagonalMap", hex_radius),
        &hex_radius,
        |b, &r| b.iter(|| HexagonalMap::new(Hex::ZERO, r, get_value)),
    );
    group.bench_with_input(
        BenchmarkId::new("HexagonalMap_Par", hex_radius),
        &hex_radius,
        |b, &r| b.iter(|| HexagonalMap::new_parallel(Hex::ZERO, r, get_value)),
    );

    group.bench_with_input(
        BenchmarkId::new("HexModMap", hex_radius),
        &hex_radius,
        |b, &r| b.iter(|| HexModMap::new(Hex::ZERO, r, get_value)),
    );
    group.bench_with_input(
        BenchmarkId::new("HexModMap_Par", hex_radius),
        &hex_radius,
        |b, &r| b.iter(|| HexModMap::new_parallel(Hex::ZERO, r, get_value)),
    );

    group.bench_with_input(
        BenchmarkId::new("RombusMap", rombus_size),
        &rombus_size,
        |b, &s| b.iter(|| RombusMap::new(Hex::ZERO, s, s, get_value)),
    );
    group.bench_with_input(
        BenchmarkId::new("RombusMap_Par", rombus_size),
        &rombus_size,
        |b, &s| b.iter(|| RombusMap::new_parallel(Hex::ZERO, s, s, get_value)),
    );

    group.finish();
}

criterion_group!(benches, creation_benchmark);
criterion_main!(benches);
