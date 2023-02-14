# CHANGELOG

## Unreleased

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
  - `angle_flat` for radian angle in flat orientation
  - `angle_flat_degrees` for degrees angle in flat orientation
  - `angle_pointy` for radian angle in pointy orientation
  - `angle_pointy_degrees` for degrees angle in pointy orientation
  - `angle` for radian angle in a given orientation
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
