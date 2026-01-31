use criterion::{Criterion, criterion_group, criterion_main};
use hexx::{DoubledHexMode, Hex, HexOrientation, OffsetHexMode};
use std::hint::black_box;

fn conversion_roundtrip(c: &mut Criterion) {
    let mut group = c.benchmark_group("coordinate_roundtrip");

    let hexes: Vec<Hex> = Hex::ZERO.range(50).collect();

    // Test roundtrip for all conversion types
    group.bench_function("doubled_roundtrip", |b| {
        b.iter(|| {
            for &hex in &hexes {
                for mode in [DoubledHexMode::DoubledWidth, DoubledHexMode::DoubledHeight] {
                    let doubled = hex.to_doubled_coordinates(mode);
                    let converted = Hex::from_doubled_coordinates(doubled, mode);
                    black_box(converted);
                }
            }
        })
    });

    group.bench_function("offset_roundtrip", |b| {
        b.iter(|| {
            for &hex in &hexes {
                for mode in [OffsetHexMode::Even, OffsetHexMode::Odd] {
                    for orientation in [HexOrientation::Flat, HexOrientation::Pointy] {
                        let offset = hex.to_offset_coordinates(mode, orientation);
                        let converted = Hex::from_offset_coordinates(offset, mode, orientation);
                        black_box(converted);
                    }
                }
            }
        })
    });

    group.bench_function("hexmod_roundtrip", |b| {
        b.iter(|| {
            for &hex in &hexes {
                let hexmod = hex.to_hexmod_coordinates(50);
                let converted = Hex::from_hexmod_coordinates(hexmod, 50);
                black_box(converted);
            }
        })
    });

    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default()
        .sample_size(1000)
        .warm_up_time(std::time::Duration::from_secs(2))
        .measurement_time(std::time::Duration::from_secs(5));
    targets = conversion_roundtrip,
);
criterion_main!(benches);
