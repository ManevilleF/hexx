use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use hexx::{
    shapes::rombus,
    storage::{HexModMap, HexStore, HexagonalMap, RombusMap},
    *,
};
use std::hint::black_box;

pub fn hexagonal_map_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Hexagonal Storage");
    group
        .warm_up_time(std::time::Duration::from_secs(3))
        .measurement_time(std::time::Duration::from_secs(10))
        .noise_threshold(0.02)
        .sample_size(500);

    let get_value = |h: Hex| h.length();

    for dist in [10, 100, 300] {
        let std_hash_map: std::collections::HashMap<_, _> = Hex::ZERO
            .range(dist)
            .map(|hex| (hex, get_value(hex)))
            .collect();
        let bevy_hash_map: bevy_platform::collections::HashMap<_, _> = Hex::ZERO
            .range(dist)
            .map(|hex| (hex, get_value(hex)))
            .collect();
        let hex_map = HexagonalMap::new(Hex::ZERO, dist, get_value);
        let hexmod_map = HexModMap::new(Hex::ZERO, dist, get_value);
        group.bench_with_input(
            BenchmarkId::new("std::HashMap_get", dist),
            &dist,
            |b, dist| {
                b.iter(|| {
                    for c in black_box(Hex::ZERO).range(*dist) {
                        std_hash_map.get(&c).unwrap();
                    }
                });
            },
        );
        group.bench_with_input(
            BenchmarkId::new("bevy_platform::HashMap_get", dist),
            &dist,
            |b, dist| {
                b.iter(|| {
                    for c in black_box(Hex::ZERO).range(*dist) {
                        bevy_hash_map.get(&c).unwrap();
                    }
                });
            },
        );
        group.bench_with_input(
            BenchmarkId::new("HexagonalMap_get", dist),
            &dist,
            |b, dist| {
                b.iter(|| {
                    for c in black_box(Hex::ZERO).range(*dist) {
                        hex_map.get(c).unwrap();
                    }
                });
            },
        );
        group.bench_with_input(BenchmarkId::new("HexModMap_get", dist), &dist, |b, dist| {
            b.iter(|| {
                for c in black_box(Hex::ZERO).range(*dist) {
                    hexmod_map.get(c).unwrap();
                }
            });
        });
        group.bench_with_input(
            BenchmarkId::new("std::HashMap_iter", dist),
            &dist,
            |b, _| b.iter(|| std_hash_map.iter().collect::<Vec<_>>()),
        );
        group.bench_with_input(
            BenchmarkId::new("bevy_platform::HashMap_iter", dist),
            &dist,
            |b, _| b.iter(|| bevy_hash_map.iter().collect::<Vec<_>>()),
        );
        group.bench_with_input(
            BenchmarkId::new("HexagonalMap_iter", dist),
            &dist,
            |b, _| b.iter(|| hex_map.iter().collect::<Vec<_>>()),
        );
        group.bench_with_input(BenchmarkId::new("HexModMap_iter", dist), &dist, |b, _| {
            b.iter(|| hexmod_map.iter().collect::<Vec<_>>())
        });
    }
    group.finish();
}

pub fn rombus_map_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Rombus Storage");
    group
        .warm_up_time(std::time::Duration::from_secs(3))
        .measurement_time(std::time::Duration::from_secs(10))
        .noise_threshold(0.02)
        .sample_size(500);

    let get_value = |h: Hex| h.length();

    for dist in [10, 100, 500] {
        let std_hash_map: std::collections::HashMap<_, _> = rombus(Hex::ZERO, dist, dist)
            .map(|hex| (hex, get_value(hex)))
            .collect();
        let bevy_hash_map: bevy_platform::collections::HashMap<_, _> =
            rombus(Hex::ZERO, dist, dist)
                .map(|hex| (hex, get_value(hex)))
                .collect();
        let rombus_map = RombusMap::new(Hex::ZERO, dist, dist, get_value);
        group.bench_with_input(
            BenchmarkId::new("std::HashMap_get", dist),
            &dist,
            |b, dist| {
                b.iter(|| {
                    for c in rombus(black_box(Hex::ZERO), *dist, *dist) {
                        std_hash_map.get(&c).unwrap();
                    }
                })
            },
        );
        group.bench_with_input(
            BenchmarkId::new("bevy_platform::HashMap_get", dist),
            &dist,
            |b, dist| {
                b.iter(|| {
                    for c in rombus(black_box(Hex::ZERO), *dist, *dist) {
                        bevy_hash_map.get(&c).unwrap();
                    }
                })
            },
        );
        group.bench_with_input(BenchmarkId::new("RombusMap_get", dist), &dist, |b, dist| {
            b.iter(|| {
                for c in rombus(black_box(Hex::ZERO), *dist, *dist) {
                    rombus_map.get(c).unwrap();
                }
            })
        });
        group.bench_with_input(
            BenchmarkId::new("std::HashMap_iter", dist),
            &dist,
            |b, _| b.iter(|| std_hash_map.iter().collect::<Vec<_>>()),
        );
        group.bench_with_input(
            BenchmarkId::new("bevy_platform::HashMap_iter", dist),
            &dist,
            |b, _| b.iter(|| bevy_hash_map.iter().collect::<Vec<_>>()),
        );
        group.bench_with_input(BenchmarkId::new("RombusMap_iter", dist), &dist, |b, _| {
            b.iter(|| rombus_map.iter().collect::<Vec<_>>())
        });
    }
    group.finish();
}

criterion_group!(benches, hexagonal_map_benchmark, rombus_map_benchmark);
criterion_main!(benches);
