//! # Hexx
//!
//! [![workflow](https://github.com/ManevilleF/hexx/actions/workflows/rust.yml/badge.svg)](https://github.com/ManevilleF/hexx/actions/workflows/rust.yml)
//! [![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](./LICENSE)
//! [![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
//!
//! Hexagonal tools lib in rust.
//!
//! > Inspired by this [`RedBlobGames` article](https://www.redblobgames.com/grids/hexagons/implementation.html).
//!
//! This lib allows you to:
//! - Manipulate hexagon coordinates
//! - Generate hexagonal maps with custom layouts and orientation
//! - Generate hexagon meshes (planes or columns)
//!
//! I made the choice to use *Axial Coordinates* for performance and utility reasons,
//! but the `Hex` type allows you to use computes *Cubic coordinates*. (See the [hexagonal coordinate systems](https://www.redblobgames.com/grids/hexagons/#coordinates))
//!
//! The `Hex` type gives you access to most hexagonal arithmetics like:
//! - Distances
//! - Neighbors and directions
//! - Lines
//! - Ranges
//! - Rings
//! - Rotation
//!
//! ## TODO list:
//!
//! - [ ] Complete test coverage (Required for `v0.1.0` release)
//! - [X] Complete documentation (Required for `v0.1.0` release)
//! - [ ] Hexagonal symmetry
//! - [ ] Obstacles and pathfinding
//! - [X] Decent UV mapping
//!
//! ## Usage in bevy
//!
//! If you want to generate 3D hexagonal mesh and use it in [bevy](bevyengine.org) you may do it this way:
//!
//!```rust
//! use bevy::prelude::Mesh;
//! use bevy::render::{mesh::Indices, render_resource::PrimitiveTopology};
//! use hexx::{HexLayout, Hex, MeshInfo};
//!
//!pub fn hexagonal_plane(hex: Hex, hex_layout: &HexLayout) -> Mesh {
//!    let mesh_info = MeshInfo::hexagonal_plane(
//!        hex_layout,
//!        hex,
//!    );
//!    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
//!    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, mesh_info.vertices.to_vec());
//!    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_info.normals.to_vec());
//!    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, mesh_info.uvs.to_vec());
//!    mesh.set_indices(Some(Indices::U16(mesh_info.indices)));
//!    mesh
//!}
//!```
#![forbid(unsafe_code)]
#![warn(clippy::nursery, clippy::pedantic, clippy::cargo, missing_docs)]
#![allow(clippy::default_trait_access, clippy::module_name_repetitions)]
mod direction;
mod hex;
mod layout;
mod mesh;
mod orientation;

pub use {direction::*, hex::*, layout::*, mesh::*, orientation::*};

/// Generates a parallelogram layout from `min` to `max`
pub fn parallelogram(min: Hex, max: Hex) -> impl Iterator<Item = Hex> {
    (min.x()..=max.x()).flat_map(move |x| (min.y()..=max.y()).map(move |y| Hex::new(x, y)))
}

/// Generates a triangle layout from `pox` with a custom `size`
pub fn triangle(pos: Hex, size: i32) -> impl Iterator<Item = Hex> {
    (pos.x()..=(pos.x() + size))
        .flat_map(move |x| ((pos.y() - x)..=(pos.y() + size)).map(move |y| Hex::new(x, y)))
}

/// Generates an hexagonal layout arout `pos` with a custom `radius`
pub fn hexagon(pos: Hex, radius: i32) -> impl Iterator<Item = Hex> {
    ((pos.x() - radius)..=(pos.x() + radius)).flat_map(move |x| {
        (((pos.y() - radius).max(pos.y() - x - radius))
            ..=((pos.y() + radius).min(pos.y() - x + radius)))
            .map(move |y| Hex::new(x, y))
    })
}

/// Generates a rectangle with the given bounds for "pointy topped" hexagons
pub fn pointy_rectangle([left, right, top, bottom]: [i32; 4]) -> impl Iterator<Item = Hex> {
    (top..=bottom).flat_map(move |y| {
        let y_offset = y >> 1;
        ((left - y_offset)..=(right - y_offset)).map(move |x| Hex::new(x, y))
    })
}

/// Generates a rectangle with the given bounds for "flat topped" hexagons
pub fn flat_rectangle([left, right, top, bottom]: [i32; 4]) -> impl Iterator<Item = Hex> {
    (left..=right).flat_map(move |x| {
        let x_offset = x >> 1;
        ((top - x_offset)..=(bottom - x_offset)).map(move |y| Hex::new(x, y))
    })
}
