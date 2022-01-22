#![forbid(unsafe_code)]
#![warn(
    clippy::all,
    clippy::style,
    clippy::complexity,
    clippy::suspicious,
    clippy::perf,
    clippy::nursery,
    clippy::pedantic,
    clippy::cargo
)]
#![allow(clippy::default_trait_access, clippy::module_name_repetitions)]
mod hex;
mod hex_layout;
mod hex_mesh;
mod hex_orientation;

pub use {hex::*, hex_layout::*, hex_mesh::*, hex_orientation::*};

pub fn parallelogram(min: Hex, max: Hex) -> impl Iterator<Item = Hex> {
    (min.x()..=max.x()).flat_map(move |x| (min.y()..=max.y()).map(move |y| Hex::new(x, y)))
}

pub fn triangle(pos: Hex, size: i32) -> impl Iterator<Item = Hex> {
    (pos.x()..=(pos.x() + size))
        .flat_map(move |x| ((pos.y() - x)..=(pos.y() + size)).map(move |y| Hex::new(x, y)))
}

pub fn hexagon(pos: Hex, radius: i32) -> impl Iterator<Item = Hex> {
    ((pos.x() - radius)..=(pos.x() + radius)).flat_map(move |x| {
        (((pos.y() - radius).max(pos.y() - x - radius))
            ..=((pos.y() + radius).min(pos.y() - x + radius)))
            .map(move |y| Hex::new(x, y))
    })
}
