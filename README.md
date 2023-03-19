<p align="center">
  <img src="docs/logo.png" alt="logo">
</p>

[![workflow](https://github.com/ManevilleF/hexx/actions/workflows/rust.yml/badge.svg)](https://github.com/ManevilleF/hexx/actions/workflows/rust.yml)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](./LICENSE)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
[![Crates.io](https://img.shields.io/crates/v/hexx.svg)](https://crates.io/crates/hexx)
[![Docs.rs](https://docs.rs/hexx/badge.svg)](https://docs.rs/hexx)
[![dependency status](https://deps.rs/crate/hexx/0.5.2/status.svg)](https://deps.rs/crate/hexx)

<!-- cargo-sync-readme start -->

 Hexagonal tools lib in rust.

 > Inspired by this [`RedBlobGames` article](https://www.redblobgames.com/grids/hexagons/implementation.html).

 This lib allows you to:
 - Manipulate hexagon coordinates
 - Generate hexagonal maps with custom layouts and orientation
 - Generate hexagon meshes (planes or columns)

 I made the choice to use *Axial Coordinates* for performance and utility reasons,
 but the [`Hex`] type has conversion utilities with *cubic*, *doubled* and *offset* coordinates.

 > See the [hexagonal coordinate systems](https://www.redblobgames.com/grids/hexagons/#coordinates)

 ## Installation

 Run `cargo add hexx` in your project or add the following line to your `Cargo.toml`:

 - `hexx = "0.5"`

 ### Cargo features

 `hexx` supports serialization and deserialization of most types using [serde](https://github.com/serde-rs/serde),
 through the `ser_de` feature gate. To enable it add the following line to your `Cargo.toml`:

 - `hexx = { version = "0.5", features = ["ser_de"] }`

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

 ## Basic usage

```rust
 use hexx::*;

 // Declare points in hexagonal spaces
 let point_a = Hex::new(10, -5);
 let point_b = Hex::new(-8, 15);
 // Find distance between them
 let dist = point_a.unsigned_distance_to(point_b);
 // Compute a line between points
 let line: Vec<Hex> = point_a.line_to(point_b).collect();
 // Compute a ring from `point_a` containing `point_b`
 let ring: Vec<Hex> = point_a.ring(dist);
 // Rotate `point_b` around `point_a` by 2 times 60 degrees clockwise
 let rotated = point_b.rotate_right_around(point_a, 2);
 // Find the direction between the two points
 let dir_a = point_a.direction_to(point_b);
 let dir_b = point_b.direction_to(point_a);
 assert!(dir_a == -dir_b);
 // Compute a wedge from `point_a` to `point_b`
 let wedge = point_a.wedge_to(point_b);
 // Get the average value of the wedge
 let avg = wedge.average();
```

 ## Layout usage

 [`HexLayout`] is the bridge between your world/screen/pixel coordinate system and the hexagonal
 coordinates system.

```rust
 use hexx::*;

 // Define your layout
 let layout = HexLayout {
    hex_size: Vec2::new(1.0, 1.0),
    orientation: HexOrientation::flat(),
    ..Default::default()
 };
 // Get the hex coordinate at the world position `world_pos`.
 let world_pos = Vec2::new(53.52, 189.28);
 let hex = layout.world_pos_to_hex(world_pos);
 // Get the world position of `hex`
 let hex = Hex::new(123, 45);
 let world_pos = layout.hex_to_world_pos(hex);
```

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

 ## Examples

  <img src='docs/hex_grid.png?' width=500/>

 > `cargo run --example hex_grid`

  https://user-images.githubusercontent.com/26703856/225945177-7fb8eb73-0bca-47c8-af68-9643c7b229f2.mp4

 > `cargo run --example scroll_map`

  https://user-images.githubusercontent.com/26703856/225945154-8395f5cb-9de3-42c8-86b0-674bc1a1c499.mp4

 > `cargo run --example wrap_map`

  <img src='docs/3d_columns.png?' width=500/>

 > `cargo run --example 3d_columns`
