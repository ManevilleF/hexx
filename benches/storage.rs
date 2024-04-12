use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use hexx::{
    shapes::rombus,
    storage::{HexagonalMap, RombusMap},
    *,
};
use std::collections::HashMap;

pub fn hexagonal_map_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Hexagonal Storage");
    group
        .significance_level(0.1)
        .target_time(6.0)
        .sample_size(100);

    let get_value = |h: Hex| h.length();

    for dist in [10, 50, 100, 300] {
        let hash_map: HashMap<_, _> = Hex::ZERO
            .range(dist)
            .map(|hex| (hex, get_value(hex)))
            .collect();
        group.bench_with_input(BenchmarkId::new("HashMap", dist), &dist, |b, dist| {
            b.iter(|| {
                for c in black_box(Hex::ZERO).range(*dist) {
                    hash_map.get(&c).unwrap();
                }
            })
        });
        let hex_map = HexagonalMap::new(Hex::ZERO, dist, get_value);
        group.bench_with_input(BenchmarkId::new("HexagonalMap", dist), &dist, |b, dist| {
            b.iter(|| {
                for c in black_box(Hex::ZERO).range(*dist) {
                    hex_map.get(c).unwrap();
                }
            })
        });
    }
    group.finish();
}

pub fn rombus_map_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Rombus Storage");
    group.significance_level(0.1).sample_size(100);

    let get_value = |h: Hex| h.length();

    for dist in [10, 100, 500] {
        let hash_map: HashMap<_, _> = rombus(Hex::ZERO, dist, dist)
            .map(|hex| (hex, get_value(hex)))
            .collect();
        group.bench_with_input(BenchmarkId::new("HashMap", dist), &dist, |b, dist| {
            b.iter(|| {
                for c in rombus(black_box(Hex::ZERO), *dist, *dist) {
                    hash_map.get(&c).unwrap();
                }
            })
        });
        let hex_map = RombusMap::new(Hex::ZERO, dist, dist, get_value);
        group.bench_with_input(BenchmarkId::new("RombusMap", dist), &dist, |b, dist| {
            b.iter(|| {
                for c in rombus(black_box(Hex::ZERO), *dist, *dist) {
                    hex_map.get(c).unwrap();
                }
            })
        });
    }
    group.finish();
}

criterion_group!(benches, hexagonal_map_benchmark, rombus_map_benchmark);
criterion_main!(benches);
