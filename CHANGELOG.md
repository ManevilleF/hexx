# CHANGELOG

## Unreleased

* Added `3d_columns` example
* Implemented `Rem<Hex>` for `Hex`
* Implemented `RemAssign<Hex>` for `Hex`
* Implemented `Rem<i32>` for `Hex`
* Implemented `RemAssign<i32>` for `Hex`
* Added angle methods to `Direction`:
  - `angle_flat` for radian angle in flat orientation
  - `angle_flat_degrees` for degrees angle in flat orientation
  - `angle_pointy` for radian angle in pointy orientation
  - `angle_pointy_degrees` for degrees angle in pointy orientation
  - `angle` for radian angle in a given orientation
* (**BREAKING**) rotated order of `Direction` enum and `Hex::ALL_NEIGHBORS` by 1 to the left, `TopRight` is now the first as `BottomRight` is now last
* Added `left` and `right` methods to `Direction` to get the next direction clockwise and counter clockwise
* Added `rotate_left` and `rotate_right` methods to `Direction` to rotate the direction clockwise and counter clockwise by a custom amount

* (**BREAKING**) Removed arbitraty `Display` impl for `Hex`

## 0.2.0

* Improved docs
* Improved example
* Added conversion from and to *doubled* coordinates
* Added conversion from and to *offset* coordinates

## 0.1.1

* `bevy` dev dependency fix

## 0.1.0

First version
