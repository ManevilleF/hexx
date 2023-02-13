<!-- cargo-sync-readme start -->

 # Hexx

 [![workflow](https://github.com/ManevilleF/hexx/actions/workflows/rust.yml/badge.svg)](https://github.com/ManevilleF/hexx/actions/workflows/rust.yml)
 [![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](./LICENSE)
 [![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
 [![Crates.io](https://img.shields.io/crates/v/hexx.svg)](https://crates.io/crates/hexx)
 [![Docs.rs](https://docs.rs/hexx/badge.svg)](https://docs.rs/hexx)
 [![dependency status](https://deps.rs/crate/hexx/0.3.0/status.svg)](https://deps.rs/crate/hexx)

 Hexagonal tools lib in rust.

 > Inspired by this [`RedBlobGames` article](https://www.redblobgames.com/grids/hexagons/implementation.html).

 This lib allows you to:
 - Manipulate hexagon coordinates
 - Generate hexagonal maps with custom layouts and orientation
 - Generate hexagon meshes (planes or columns)

 I made the choice to use *Axial Coordinates* for performance and utility reasons,
 but the [`Hex`] type has conversion utilities with *cubic*, *doubled* and *offset* coordinates.

 > See the [hexagonal coordinate systems](https://www.redblobgames.com/grids/hexagons/#coordinates)

 ## Features

 `hexx` provides the [`Hex`] coordinates with:
 - Distances
 - Neighbors and directions
 - Lines
 - Ranges
 - Rings
 - Edges
 - Wedges
 - Spirals
 - Rotation
 - Symmetry
 - Vector operations
 - Conversions to other coordinate systems

 And the [`HexMap`] utility, for *wraparound* (seamless) hexagonal maps

 ## Usage in [Bevy](https://bevyengine.org/)

 If you want to generate 3D hexagonal mesh and use it in [bevy](bevyengine.org) you may do it this way:

```rust
 use bevy::prelude::Mesh;
 use bevy::render::{mesh::Indices, render_resource::PrimitiveTopology};
 use hexx::{HexLayout, Hex, MeshInfo};

pub fn hexagonal_plane(hex_layout: &HexLayout) -> Mesh {
    // Compute hex plane data for at the origin
    let mesh_info = MeshInfo::hexagonal_plane(hex_layout, Hex::ZERO);
    // Compute the bevy mesh
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, mesh_info.vertices.to_vec());
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_info.normals.to_vec());
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, mesh_info.uvs.to_vec());
    mesh.set_indices(Some(Indices::U16(mesh_info.indices)));
    mesh
}
```

 The [`MeshInfo`] type provides the following mesh generations:
 - [`MeshInfo::hexagonal_plane`] (7 vertices) useful for 2D games
 - [`MeshInfo::cheap_hexagonal_column`] (13 vertices) with merged vertices and useful only for
 unlit games
 - [`MeshInfo::partial_hexagonal_column`] (31 vertices) without the bottom face
 - [`MeshInfo::hexagonal_column`] (38 vertices) with the bottom face

<!-- cargo-sync-readme end -->

> See the [examples](examples) for bevy usage

 ## Example

 ![example](docs/hex_grid.png?)

 > `cargo run --example hex_grid`

 ![example](docs/3d_columns.png?)

 > `cargo run --example 3d_columns`
