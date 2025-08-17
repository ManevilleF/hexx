//! Hexagonal tools lib in rust.
//!
//! > Inspired by this [`RedBlobGames` article](https://www.redblobgames.com/grids/hexagons/implementation.html)
//! > and [Sander Evers](https://sanderevers.github.io/) work
//!
//! This lib allows you to:
//!
//! * Manipulate hexagon coordinates
//! * Generate hexagonal maps with custom layouts and orientation
//! * Generate hexagon meshes (planes or columns)
//!
//! I made the choice to use *Axial Coordinates* for performance and utility
//! reasons, but the [`Hex`](crate::hex::Hex) type has conversion utilities with
//! *cubic*, *doubled*, *hexmod* and *offset* coordinates.
//!
//! > See the [hexagonal coordinate systems](https://www.redblobgames.com/grids/hexagons/#coordinates)
//!
//! ## Installation
//!
//! Run `cargo add hexx` in your project or add the following line to your
//! `Cargo.toml`:
//!
//! * `hexx = "0.21"`
//!
//! ### Cargo features
//!
//! `hexx` provides the following cargo features:
//! * `serde`: Enables [serde](https://github.com/serde-rs/serde) support for
//!   most types
//! * `packed`: Makes [`Hex`](crate::hex::Hex) `repr(C)`, useful to use it
//!   accross the FII
//! * `bevy`: Enables [Bevy](https://bevyengine.org/) support
//! * `bevy_platform`: Enables [Bevy Platform](https://docs.rs/bevy_platform/latest/bevy_platform)
//!   for `HashMap`
//! * `bevy_reflect`: Enables [Bevy Reflection](https://docs.rs/bevy_reflect/latest/bevy_reflect)
//!   for most types
//! * `grid`: Enables support for Face/Vertex/Edge [grid handling](https://www.redblobgames.com/grids/parts/#hexagon-coordinates)
//!   using `Hex` as Face, `GridVertex` as vertex and `GridEdge` as edge.
//! * `algorithms`: Enables the [algorithms](crate::algorithms) module with:
//!   * Field of Movement
//!   * A Star Pathfinding
//!   * Field of view
//! * `mesh`: Enables procedural mesh generation
//!
//! _Some features are enabled by default, it is recommended to enable only
//! what is needed for your usage_
//!
//! ## Features
//!
//! `hexx` provides the [`Hex`](crate::hex::Hex) coordinates with:
//!
//! * Distances
//! * Neighbors and directions
//! * Lines
//! * Ranges
//! * Rings
//! * Edges
//! * Wedges
//! * Spirals
//! * Rotation
//! * Symmetry
//! * Vector operations
//! * Conversions to other coordinate systems:
//!     * Cubic coordinates
//!     * Offset coordinates
//!     * Doubled coordinates
//!     * Hexmod coordinates
//! * Multiple hex resolution
//!
//! ## Basic usage
//!
//!```rust
//! use hexx::*;
//!
//! // Declare points in hexagonal spaces
//! let point_a = hex(10, -5); // Equivalent of `Hex::new(10, -5)`
//! let point_b = hex(-8, 15);
//! // Find distance between them
//! let dist = point_a.unsigned_distance_to(point_b);
//! // Compute a line between points
//! let line: Vec<Hex> = point_a.line_to(point_b).collect();
//! // Compute a ring from `point_a` containing `point_b`
//! let ring: Vec<Hex> = point_a.ring(dist).collect();
//! // Rotate `point_b` around `point_a` by 2 times 60 degrees clockwise
//! let rotated = point_b.rotate_cw_around(point_a, 2);
//! // Find the direction between the two points
//! let dir_a = point_a.main_direction_to(point_b);
//! let dir_b = point_b.main_direction_to(point_a);
//! assert!(dir_a == -dir_b);
//! // Compute a wedge from `point_a` to `point_b`
//! let wedge = point_a.wedge_to(point_b);
//! // Get the average value of the wedge
//! let avg = wedge.average();
//! ```
//!
//! ## Layout usage
//!
//! [`HexLayout`](crate::layout::HexLayout) is the bridge between your
//! world/screen/pixel coordinate system and the hexagonal coordinates system.
//!
//!```rust
//! use hexx::*;
//!
//! // Define your layout
//! let layout = HexLayout {
//!     scale: Vec2::new(1.0, 1.0),
//!     orientation: HexOrientation::Flat,
//!     ..Default::default()
//! };
//! // Get the hex coordinate at the world position `world_pos`.
//! let world_pos = Vec2::new(53.52, 189.28);
//! let point = layout.world_pos_to_hex(world_pos);
//! // Get the world position of `point`
//! let point = hex(123, 45);
//! let world_pos = layout.hex_to_world_pos(point);
//! ```
//!
//! ## Wrapping
//!
//! [`HexBounds`](crate::bounds::HexBounds) defines a bounding hexagon around a
//! center coordinate. It can be used for boundary and interesection checks but
//! also for wrapping coordinates.
//! Coordinate wrapping transform a point outside of the bounds to a point
//! inside. This allows for seamless or repeating [wraparound](https://www.redblobgames.com/grids/hexagons/#wraparound)
//! maps.
//!
//! ```rust
//! use hexx::*;
//!
//! let center = hex(23, -45);
//! let radius = 5;
//! let bounds = HexBounds::new(center, radius);
//! let outside_coord = hex(12345, 98765);
//! assert!(!bounds.is_in_bounds(outside_coord));
//! let wrapped_coord = bounds.wrap(outside_coord);
//! assert!(bounds.is_in_bounds(wrapped_coord));
//! ```
//!
//! ## Resolutions and chunks
//!
//! [`Hex`](crate::hex::Hex) support multi-resolution coordinates.
//! In practice this means that you may convert a coordinate to a different
//! resolution:
//!
//! * To a lower resolution, meaning retrieving a *parent* coordinate
//! * to a higher resolution, meaning retrieving the center *child* coordinate
//!
//! Resolutions are abstract, the only useful information is the resolution
//! **radius**.
//!
//! For example, if you use a big grid, with a radius of a 100, you might want
//! to split that grid evenly in larger hexagons containing a 10 radius of
//! coordinates and maybe do operations locally inside of these chunks.
//!
//! So instead of using a big range directly:
//!
//! ```rust
//! use hexx::*;
//!
//! const MAP_RADIUS: u32 = 100;
//!
//! // Our big grid with hundreds of hexagons
//! let big_grid = Hex::ZERO.range(MAP_RADIUS);
//! ```
//!
//! You may define a smaller grid you will then divide to a higher resolution
//!
//! ```rust
//! use hexx::*;
//!
//! const CHUNK_RADIUS: u32 = 10;
//! const MAP_RADIUS: u32 = 20;
//!
//! let chunks = Hex::ZERO.range(MAP_RADIUS);
//! for chunk in chunks {
//!     // We can retrieve the center of that chunk by increasing the resolution
//!     let center = chunk.to_higher_res(CHUNK_RADIUS);
//!     // And retrieve the other coordinates in the chunk
//!     let children = center.range(CHUNK_RADIUS);
//!     // We can retrieve the chunk coordinates from any coordinate..
//!     for coord in children {
//!         // .. by reducing the resolution
//!         assert_eq!(coord.to_lower_res(CHUNK_RADIUS), chunk);
//!     }
//! }
//! ```
//!
//! An other usage could be to draw an infinite hex grid, with different
//! resolutions displayed, dynamically changing according to user zoom level.
//!
//! ## Dense map storage
//!
//! [`Hex`](crate::hex::Hex) implements `Hash`, and most users store hexagonal
//! maps in a `HashMap`. But for some cases `hexx` provides *dense storage*
//!  [collections](crate::storage) with more performant accessors:
//!
//! - [`HexagonalMap<T>`](crate::storage::hexagonal::HexagonalMap)
//! - [`HexModMap<T>`](crate::storage::hexmod::HexModMap)
//! - [`RombusMap<T>`](crate::storage::rombus::RombusMap)
//!
//! ## Procedural meshes
//!
//! > Requires the `mesh` feature
//!
//! `hexx` provides 3 built-in procedural mesh construction utilies:
//! - [`PlaneMeshBuilder`](crate::mesh::plane_builder::PlaneMeshBuilder) for
//!   hexagonal planes
//! - [`ColumnMeshBuilder`](crate::mesh::column_builder::ColumnMeshBuilder)  for
//!   hexagonal columns
//! - [`HeightMapMeshBuilder`](crate::mesh::heightmap_builder::HeightMapMeshBuilder)
//!   for hexagonal height maps
//!
//! All those builders have a lot of customization options and will output a
//! [`MeshInfo`](crate::mesh::MeshInfo) struct containing vertex positions,
//! normals and uvs
//!
//! ### Usage in [Bevy](https://bevyengine.org/)
//!
//! If you want to integrate the procedural meshes in [bevy](bevyengine.org) you
//! may do it this way:
//!
//!```rust
//! use bevy::{
//!     prelude::Mesh,
//!     render::{
//!         mesh::Indices, render_asset::RenderAssetUsages, render_resource::PrimitiveTopology,
//!     },
//! };
//! use hexx::MeshInfo;
//!
//! pub fn hexagonal_mesh(mesh_info: MeshInfo) -> Mesh {
//!     Mesh::new(
//!         PrimitiveTopology::TriangleList,
//!         // Means you won't interact with the mesh on the CPU afterwards
//!         // Check bevy docs for more information
//!         RenderAssetUsages::RENDER_WORLD,
//!     )
//!     .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, mesh_info.vertices)
//!     .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_info.normals)
//!     .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, mesh_info.uvs)
//!     .with_inserted_indices(Indices::U16(mesh_info.indices))
//! }
//! ```
#![forbid(unsafe_code)]
#![warn(
    clippy::nursery,
    clippy::pedantic,
    clippy::cargo,
    missing_docs,
    nonstandard_style,
    future_incompatible,
    // clippy::restriction
    clippy::unwrap_used,
    clippy::lossy_float_literal,
    clippy::return_and_then,
    clippy::same_name_method,
    clippy::str_to_string,
    clippy::try_err,
    clippy::undocumented_unsafe_blocks,
    clippy::use_debug,
    clippy::unseparated_literal_suffix,
    clippy::large_include_file,
    clippy::allow_attributes
)]
#![allow(clippy::module_name_repetitions, clippy::multiple_crate_versions)]
// For lib.rs docs only
#![allow(rustdoc::redundant_explicit_links)]
/// Non exhaustive collection of classic algorithms.
#[cfg(feature = "algorithms")]
pub mod algorithms;
/// Hexagonal range bounds module
pub mod bounds;
/// Hexagonal coordinates conversion module
pub mod conversions;
/// Hexagonal directions module
pub mod direction;
/// Hexagonal coordinates module
pub mod hex;
/// Hexagonal layout module
pub mod layout;
#[cfg(feature = "mesh")]
/// Mesh generation utils module
pub mod mesh;
/// Hexagon oritentation module
pub mod orientation;
/// Map shapes generation functions
pub mod shapes;
pub mod storage;

#[doc(inline)]
pub use bounds::HexBounds;
#[doc(inline)]
pub use conversions::*;
#[doc(inline)]
pub use direction::*;
#[doc(hidden)]
pub use glam::{IVec2, IVec3, Quat, UVec2, Vec2, Vec3};
#[doc(inline)]
#[cfg(feature = "grid")]
pub use hex::{GridEdge, GridVertex};
#[doc(inline)]
pub use hex::{Hex, HexIterExt, hex};
#[doc(inline)]
pub use layout::HexLayout;
#[cfg(feature = "mesh")]
pub use mesh::*;
#[doc(inline)]
pub use orientation::HexOrientation;
