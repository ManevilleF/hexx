# CHANGELOG

## [Unreleased]

* Added *ordering* wrappers for `Hex` (#147):
  * `OrdByLength`
  * `OrdByXY`
  * `OrdByYX`
* Bumped MSRV to 1.74.1 (#147)

## 0.14.0

* Defined MSRV to be 1.72.1 (#145)
* Made internal `HexOrientationData` matrices `const` (#145)
* (**BREAKING**) Fixed `uv` generation for planes and columns (#145):
  * UV coords, by default, will be correctly wrapped between (0, 0) and (1, 1)
* `MeshInfo::cheap_hexagonal_column` now returns a mesh with 12 vertices (#145)
* Added gizmos to `mesh_builder` example (#145)

### Deprecations

* Deprecated `UVOptions::quad_default` in favor of `UVOptions::new` (#145)
* Deprecated `UVOptions::cap_default` in favor of `UVOptions::new` (#145)

## 0.13.0

### algorithms

* (**BREAKING**) `a_star` `cost` function parameter now takes two adjacent `Hex`
nodes instead of one, allowing for more use cases (#130, #128)
* Fixed `field_of_movement` algorithm (#142, #127)

### Dependencies

* Bumped `bevy_inspector_egui` dev dependency (#129)
* Added `bevy_egui` dev dependency (#143)

### Examples

* Added a `sprite_sheet` bevy example (#135)
* Improved `mesh_builder` example (#143)

### Additions

* Added `HexLayout::rect_size` method (#135)
* Added `ColumnMeshBuilder::center_aligned` option (#139)
* Added `PlaneMeshBuilder::center_aligned` option (#139)
* Added `Hex::to_array_f32` utility method (#141)
* Added `Hex::to_cubic_array_f32` utility method (#141)
* Added `HexLayout::fract_hex_to_world_pos` method (#141, #138, #140)
* Added `HexLayout::world_pos_to_fract_hex` method (#141, #138, #140)
* Added `HexOrientationData::forward` method (#141)
* Added `HexOrientationData::inverse` method (#141)
* Added coordinate expressive const values for `Direction` (#144)
* Added coordinate expressive const values for `DiagonalDirection` (#144)

### Mesh generation

* `ColumnMeshBuilder` now accepts custom `UvOptions` for each 6 sides (#143)
  * Added `ColumnMeshBuilder::with_multi_sides_uv_options` method (#143)
* `UVOptions` changes:
  * (**BREAKING**) changed `flip_x` and `flip_y` fields to `flip` BVec2 (#143)
  * Added `rect` field, to remap the coordinates in specific sections (#143)
  * Added `with_rect` builder method (#143)
  * Changed the order of operations in `alter_uvs` (#143)
* (**BREAKING**) Fixed quad generation which had upside down uvs (#143)

### Deprecations

* Deprecated `MeshInfo::hexagonal_plane` in favor of `PlaneMeshBuilder` (#139)

## 0.12.0

* Added `PlaneMeshBuilder::with_scale` option to scale generated mesh (#121)
* Added `PlaneMeshBuilder::with_rotation` option to rotate generated mesh (#121)
* Added `ColumnMeshBuilder::with_scale` option to scale generated mesh (#121)
* Added `ColumnMeshBuilder::with_rotation` option to rotate generated mesh (#121)
* Mesh transformation follow the SRT order of operations (#121)
* Bumped `bevy` deps to 0.12.x (#123)

## 0.11.0

* Reduced vertice and tri count of mesh generation (#119)
* Use of built-in rust f32 consts instead of custom ones (#113)
* Fix clippy warnings of rust 1.72.0 (#114)
* Added rustfmt config file (#114)
* Fixed `PartialOrd` impl for `a_star` `Node`
* Bumped `bevy_inspector_egui` dev dependency to 0.20

## 0.10.0

* Examples use camera viewport to retrieve cursor world position (#112)
* (**BREAKING**) `Hex::round` now takes a `[f32; 2]` parameter instead of
`(f32, f32)` (#112)
* `HexLayout` now has the option to invert `Hex` x and y axis (#110, #112)
* (**BREAKING**) `Hex::y` axis now correctly points towards the negative world/
pixel/screen `y` coordinates

## 0.9.2

* Fixed some documentation on `Direction` (#111)
* Improved `Hex::round` function and performance (#111)
* `hex_grid` example now shows the hexagon coordinates (#111)

## 0.9.1

* Fixed column mesh normals not being normalized (#108, #109)

## 0.9.0

* Bumped `glam` dependency to 0.24
* Bumped `bevy_reflect` optional dependency to 0.11
* Bumped `bevy` dev dependency to 0.11
* Renamed `fov` example to `field_of_view`
* Removed deprecated methods:
  * `HexOrientation::flat()` deprecated since 0.7.0 in favor of `HexOrientation::Flat`
  * `HexOrientation::pointy()` deprecated since 0.7.0 in favor of `HexOrientation::Pointy`

## 0.8.0

* Added [hexmod](https://observablehq.com/@sanderevers/hexmod-representation)
representation support:
  * `Hex::to_hexmod_coordinates`
  * `Hex::from_hexmod_coordinates`
* Added resolution system ([source](https://observablehq.com/@sanderevers/hexagon-tiling-of-an-hexagonal-grid))
  * `Hex::to_lower_res` returns the coordinates of a *big* hexagon containing the
    given hexagon, effectively reducing its resolution
  * `Hex::to_higher_res` returns the center coordinate of a *big* hexagon in a
    higher resolution
  * `Hex::to_local` returns the coordinate of an hexagon relative to the center
    of its lower resolution (bigger) hexagon
* (**BREAKING**) Removed `HexMap` utility struct:
  * The previous wrapping method was using 6 mirrors, which is not efficient withi
    large maps. The new resolution system allows finding relative positions in
    bounds in a much easier way
  * If you were using `HexMap`, you can use `HexBounds` instead
* Added wrapping methods for `HexBounds`:
  * `HexBounds::wrap` which is the equivalent of `HexMap::wrapped_hex`
  * `HexBounds::wrap_local` which returns the local wrap coordinate relative to
  the bounds
* (**BREAKING**) `Hex::range_count` now returns a `u32` instead of a `usize`
* Added examples
  * `chunks` showcasing the new resolution system
  * `merged_columns` showcasing a simple chunk system with merged meshes

## 0.7.1

* Renamed `ser_de` feature to `serde`. The `ser_de` feature will remain available
until 0.8.0 (#99)
* Added extra documentation to `MeshInfo` (#99)
* Moved the mesh module under a `mesh` feature gate, enabled by default (#99)
* Added Bevy Reflection support with `bevy_reflect` feature (#100)
* Fixed `serde` compilation error introduced in #99 (#101)
* Added `CONTRIBUTING.md` (#102)
* Split CI workflows (#102)

## 0.7.0

* Added `Hex::xrange` for excluding range coordinates (#88)
* Implemented  `PartialEq` For `HexOrientation` (#87)
* `HexOrientation` is now a two variant enum instead of a struct:
  * The inner data (matrices and rotation) are now retrievable through either:
    * `orientation_data()` method
    * `Deref` implementation
* Added conversion from angles for both `Direction` and `DiagonalDirection` (#92):
  * `Direction::from_angle`
  * `Direction::from_pointy_angle`
  * `Direction::from_flat_angle`
  * `Direction::from_angle_degrees`
  * `Direction::from_pointy_angle_degrees`
  * `Direction::from_flat_angle_degrees`
  * `DiagonalDirection::from_angle`
  * `DiagonalDirection::from_pointy_angle`
  * `DiagonalDirection::from_flat_angle`
  * `DiagonalDirection::from_angle_degrees`
  * `DiagonalDirection::from_pointy_angle_degrees`
  * `DiagonalDirection::from_flat_angle_degrees`
* Added missing `Direction::angle_degrees` method (#92)
* Added missing `DiagonalDirection::angle_degrees` method (#92)
* Added `UvOptions` for both planet and column mesh builder (#93):
  * `mesh_builder` example now uses a checker texture and showcases uv options
  * `ColumnMeshBuilder` has uv options for cap and side faces
* (**BREAKING**) All functions that previously required a borrowed `&HexOrientation`
now take an owned `HexOrientation` as the type is now `Copy` (#95)
* (**BREAKING**) Removed all functions deprecated in `0.6.0`(#94)
* Fixed `a_star` algorithm giving inconsisten paths (#96)

## 0.6.0

* Changed logo (#63)
* Drop `itertools` dependency (#58)
* (**BREAKING**) Removed deprecated `Hex::directions_to` method (#58)
* Added `MeshInfo::facing` utils method to rotate hex meshes (#65)
* Added `hex` utils function to create an `Hex`, making it less boilerplate (#66):
  * Before: `Hex::new()`
  * Now: `hex()`
* `Hex` won't derive `Debug` or `Hash` on spirv archs (#66)
* Added `packed` feature to make `Hex` `repr(C)` (#67)
* Added an `algorithms` module with a `a_star` implementation (#69)
* Added field of view algorithms in `algorithms`:
  * `range_fov` omni-directional field of view
  * `directional_fov` directional 120 degrees field of view (`Direction`)
* Added field of movement algorithm in `algorithms`:
  * `field_of_movement` provides the available range of field of movement given
  a `budget` of movement and a movement `cost`
* Renamed rotation functions to follow `cw`/`ccw` terminology
(old versions deprecated) (#78)

### Directions to

The *direction_to* functions were reworked to avoid a
edge case where on some coordinates two directions can be valid (a *tie*).

* Added new `DirectionWay` type
* Added `Hex::way_to` which returns a `DirectionWay<Direction>`
* Added `Hex::diagonal_way_to` which returns a `DirectionWay<DiagonalDirection>`
* Deprecated `Hex::direction_to` in favor of `way_to`
* Deprecated `Hex::diagonal_to` in favor of `diagonal_way_to`

This type can represent the classic *Single* case (One direction) or the *Tie*. This may
seem like an over complication, but `DirectionWay<T>` implements `PartialEq<T>`, which means
you can still do this:

```rust
let a = hex(1, 0);
let b = hex(43, 12);
if a.way_to(b) == Direction::Top {
    //
}
```

But now with accurate results !

### ExactSizeIterator (#68)

* (**BREAKING**) `Hex::ring` now returns a `ExactSizeIterator` instead of a `Vec`
* (**BREAKING**) `Hex::custom_ring` now returns a `ExactSizeIterator` instead of a `Vec`

### Mesh builder (#80)

* Deprecated `MeshInfo::hexagonal_column` and `MeshInfo::partial_hexagonal_column`
* (**BREAKING**) `MeshInfo` fields use `glam` types instead of arrays of float
* Added `ColumnMeshBuilder` to create hex column meshes. This allows more customization options than the previous way:
  * Rotation
  * Offset
  * Sides subdivisions
* Added `PlaneMeshBuilder` to create hex plane meshes. This allows more customization options than the previous way:
  * Rotation
  * Offset
* (**BREAKING**) Removed `MeshInfo::facing` method.
* Added `MeshInfo::rotated` method
* Added `MeshInfo::with_offset` method
* Added `MeshInfo::merge_with` method to merge two meshes

### Examples

* `hex_grid` example now uses a 2d mesh and camera (#65)
* `scroll_map` example now uses a 2d mesh and camera (#65)
* `wrap_map` example now uses a 2d mesh and camera (#65)
* Added an interactive `a_star` example (#69)

## 0.5.3

* Improved `README` examples section
* Added `Hex` swizzle methods:
  * `xx`
  * `yy`
  * `zz`
  * `yx`
  * `yz`
  * `xz`
  * `zx`
  * `zy`
* Improved CI workflow and added a check for a new entry in `CHANGELOG.md`
* Deprecated `Hex::directions_to`

## 0.5.2

* `HexMap::all_coords` now returns a `ExactSizeIterator` instead of a simple `Iterator`
* Added `scroll_map` example
* Added `wrap_map` example

## 0.5.1

* Documentation improvements
* Added `HexMap::bounds` getter method
* Added `glam::IVec3` reexport

## 0.5.0

### ExactSizeIterator

* `Hex::range` now returns a `ExactSizeIterator` instead of a simple `Iterator`
* `Hex::line_to` now returns a `ExactSizeIterator` instead of a simple `Iterator`
* `Hex::wedge_to` now returns a `ExactSizeIterator` instead of a simple `Iterator`
* `Hex::custom_wedge_to` now returns a `ExactSizeIterator` instead of a simple `Iterator`
* `HexBounds::all_coords` now returns a `ExactSizeIterator` instead of a simple `Iterator`
* `shapes::hexagon` now returns a `ExactSizeIterator` instead of a simple `Iterator`
* `shapes::parallelogram` now returns a `ExactSizeIterator` instead of a simple `Iterator`
* `shapes::triangle` now returns a `ExactSizeIterator` instead of a simple `Iterator`
* `shapes::pointy_rectangle` now returns a `ExactSizeIterator` instead of a simple `Iterator`
* `shapes::flat_rectangle` now returns a `ExactSizeIterator` instead of a simple `Iterator`

* (**BREAKING**) `Hex::ring_edge` now returns a `ExactSizeIterator` instead of a `Vec`
* (**BREAKING**) `Hex::custom_ring_edge` now returns a `ExactSizeIterator` instead of a `Vec`
* (**BREAKING**) `Hex::custom_ring_edges` now returns an `Iterator` with an `ExactSizeIterator` item instead of a `Vec`
* (**BREAKING**) `Hex::ring_edges` now returns an `Iterator` with an `ExactSizeIterator` item instead of a `Vec`

### Additions

* Added `Hex::wedge_count` to count the amount of coordinate in a wedge
* Added `Hex::full_wedge` which returns an `ExactSizeIterator`
* Added `Hex::custom_full_wedge` which returns an `ExactSizeIterator`

### Directions

* (**BREAKING**) Removed implementation of `Direction + usize` to rotate clockwise, replaced by the shift right operator `>>`
* (**BREAKING**) Removed implementation of `DiagonalDirection + usize` to rotate clockwise, replaced by the shift right operator `>>`
* (**BREAKING**) Removed implementation of `Direction - usize` to rotate counter clockwise, replaced by the shift left operator `<<`
* (**BREAKING**) Removed implementation of `DiagonalDirection - usize` to rotate counter clockwise, replaced by the shift left operator `<<`
* Added `Hex::diagonal_neigbhor`
* Added `Hex::diagonal_neigbhor_coord`
* Added `Add<Direction>` and `AddAssign<Direction>` impls for `Hex`
* Added `Add<DiagonalDirection>` and `AddAssign<DiagonalDirection>` impls for `Hex`
* Added `Sub<Direction>` and `SubAssign<Direction>` impls for `Hex`
* Added `Sub<DiagonalDirection>` and `SubAssign<DiagonalDirection>` impls for `Hex`
* Added `Mul<i32>` for `Direction` returning its `Hex` vector multiplied
* Added `Mul<i32>` for `DiagonalDirection` returning its `Hex` vector multiplied

### Miscellaneous

* Deprecated `Hex::to_array3` in favor of `Hex::to_cubic_array`
* Made most modules public but kept the wildcard exports at the crate root
* Bump `bevy` dev dependenvy to `0.10` and updated the examples
* Bump `glam` to `0.23`

## 0.4.2

* Fixed `conversion` module was private (#42)
* Fixed `Hex::line_to` returning `(0, 0)` when both ends are identical (#43)

## 0.4.1

### Fix

* Fixed `Product` impl for `Hex` which is always returning (0, 0)

### Additions

* Added `Hex::min` method
* Added `Hex::max` method
* Added `Hex::signum` method
* Added `Hex::dot` method
* Added `Hex::from_slice` method
* Added `Hex::write_to_slice` method
* Added `Hex::dot` method

### Impls

* Added `BitOr` implementations for `Hex`
* Added `BitXor` implementations for `Hex`
* Added `BitAnd` implementations for `Hex`
* Added `Shr` implementations for `Hex`
* Added `Shl` implementations for `Hex`

## 0.4.0

### Directions

* Added `DiagonalDirection` enum with identical features as `Direction`
* Added `Direction::diagonal_left` to retrieve the counter clockwise `DiagonalDirection` neighbor
* Added `Direction::diagonal_right` to retrieve the clockwise `DiagonalDirection` neighbor
* Added `DiagonalDirection::direction_left` to retrieve the counter clockwise `Direction` neighbor
* Added `DiagonalDirection::direction_right` to retrieve the clockwise `Direction` neighbor
* Implement `Neg` for `Direction` to compute the opposite direction
* Implement `Neg` for `DiagonalDirection` to compute the opposite direction
* Implement `Add<usize>` for `Direction` to rotate the direction clockwise
* Implement `Add<usize>` for `DiagonalDirection` to rotate the direction clockwise
* Implement `Sub<usize>` for `Direction` to rotate the direction counter clockwise
* Implement `Sub<usize>` for `DiagonalDirection` to rotate the direction counter clockwise

### Rings

* Added new ring methods:
  * `Hex::rings`
  * `Hex::custom_rings`
  * `Hex::ring_edge`
  * `Hex::custom_ring_edge`
  * `Hex::ring_edges`
  * `Hex::custom_ring_edges`
  * `Hex::cached_ring_edges`
  * `Hex::cached_custom_ring_edges`
  * `Hex::wedge`
  * `Hex::wedge_to`
  * `Hex::custom_wedge`
  * `Hex::custom_wedge_to`
  * `Hex::corner_wedge`
  * `Hex::corner_wedge_to`
* (**BREAKING**) `Hex::custom_spiral_range` now takes a `Iterator<Item = u32>` as range and returns an iterator
* (**BREAKING**) `Hex::spiral_range` now takes a `Iterator<Item = u32>` as range and returns an iterator

### Bounds

* Added `HexBounds` utility struct
* `HexMap` now uses `HexBounds`
* Added `bounds()` iterator util, to compute bounds from multiple hex coordinates

### Misc

* Added `Hexx:to_array` method
* Added `Hexx:to_array3` method
* Added `From<Direction>` impl for `Hex`
* Added `From<DiagonalDirection>` impl for `Hex`
* (**BREAKING**) Grouped all iterator extensions (`MeanExt` and `CenterExt`) in a common `HexIterExt` trait
* `Direction::rotate_left` is now `const`
* `Direction::rotate_right` is now `const`
* `DiagonalDirection::rotate_left` is now `const`
* `DiagonalDirection::rotate_right` is now `const`

## 0.3.0

### Fix

* `Hex::ring` did not work properly with offset coordinates

### Examples

* Added `3d_columns` example

### Division

* (**BREAKING**) Rework `Hex` `Div` impls to respect expected `length`

### Direction

* Added angle methods to `Direction`:
  * `angle_flat` for radian angle in flat orientation
  * `angle_flat_degrees` for degrees angle in flat orientation
  * `angle_pointy` for radian angle in pointy orientation
  * `angle_pointy_degrees` for degrees angle in pointy orientation
  * `angle` for radian angle in a given orientation
* (**BREAKING**) rotated order of `Direction` enum and `Hex::ALL_NEIGHBORS` by 1 to the left, `TopRight` is now the first as `BottomRight` is now last
* Added `left` and `right` methods to `Direction` to get the next direction clockwise and counter clockwise
* Added `rotate_left` and `rotate_right` methods to `Direction` to rotate the direction clockwise and counter clockwise by a custom amount

### New features

* Added `lerp` method to `Hex`
* Added `cached_rings` and `cached_custom_rings` to `Hex` for rings pre-computation
* (**BREAKING**) renamed `Hex::rotate_left` to `Hex::left` and made it `const`
* (**BREAKING**) renamed `Hex::rotate_right` to `Hex::right` and made it `const`
* (**BREAKING**) renamed `Hex::rotate_left_around` to `Hex::left_around` and made it `const`
* (**BREAKING**) renamed `Hex::rotate_right_around` to `Hex::right_around` and made it `const`
* Added `Hex::rotate_left` to rotate counter clockwise by a custom amount
* Added `Hex::rotate_left_around` to rotate counter clockwise by a custom amount and center
* Added `Hex::rotate_right` to rotate clockwise by a custom amount
* Added `Hex::rotate_right_around` to rotate clockwise by a custom amount and center

### Iterator extensions

* Added `average` method for `Hex` iterators to compute their mean value
* Added `center` method for `Hex` iterators to compute their centroid value

### Conversions

* Added `as_ivec2` method to `Hex`
* Added `as_ivec3` method to `Hex`
* Added `as_vec2` method to `Hex`

### Misc

* (**BREAKING**) Removed arbitraty `Display` impl for `Hex`
* Implemented `Rem<Hex>` for `Hex`
* Implemented `RemAssign<Hex>` for `Hex`
* Implemented `Rem<i32>` for `Hex`
* Implemented `RemAssign<i32>` for `Hex`
* Implemented `Sum` for `Hex`
* `Hex::neighbor` is now `const`
* `Hex::distance_to` is now `const`
* `Hex::unsigned_distance_to` is now `const`
* Improved length/distance computation to avoid overflow
* Added `Hex::abs` method

## 0.2.0

* Improved docs
* Improved example
* Added conversion from and to *doubled* coordinates
* Added conversion from and to *offset* coordinates

## 0.1.1

* `bevy` dev dependency fix

## 0.1.0

First version
