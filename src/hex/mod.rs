#![allow(clippy::inline_always)]
/// Type conversions
mod convert;
/// Hexagonal grid utilities, like edge and vertices
#[cfg(feature = "grid")]
pub mod grid;
/// Traits implementations
mod impls;
/// Iterator tools module
mod iter;
/// Hex ring utils
mod rings;
/// swizzle utils
mod siwzzle;
#[cfg(test)]
mod tests;

pub(crate) use iter::ExactSizeHexIterator;
pub use iter::HexIterExt;

use crate::{DirectionWay, EdgeDirection, VertexDirection};
use glam::{IVec2, IVec3, Vec2};
#[cfg(feature = "grid")]
pub use grid::{GridEdge, GridVertex};
use std::{
    cmp::{max, min},
    fmt::Debug,
};

/// Hexagonal [axial] coordinates
///
/// # Why Axial ?
///
/// Axial coordinates allow to compute and use *cubic* coordinates with less
/// storage, and allow:
/// - Vector operations
/// - Rotations
/// - Symmetry
/// - Simple algorithms
///
/// when *offset* and *doubled* coordinates don't. Furthermore, it makes the
/// [`Hex`] behave like classic 2D coordinates ([`IVec2`]) and therefore more
/// user friendly.
///
/// Check out this [comparison] article for more information.
///
/// # Conversions
///
///  * Cubic: use [`Self::z`] to compute the third axis
///  * Offset: use [`Self::from_offset_coordinates`] and
///    [`Self::to_offset_coordinates`]
///  * Doubled: use [`Self::from_doubled_coordinates`] and
///    [`Self::to_doubled_coordinates`]
///
/// [comparison]: https://www.redblobgames.com/grids/hexagons/#coordinates-comparison
/// [axial]: https://www.redblobgames.com/grids/hexagons/#coordinates-axial
#[derive(Copy, Clone, Default, Eq, PartialEq)]
#[cfg_attr(not(target_arch = "spirv"), derive(Hash))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "packed", repr(C))]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub struct Hex {
    /// `x` axial coordinate (sometimes called `q` or `i`)
    pub x: i32,
    /// `y` axial coordinate (sometimes called `r` or `j`)
    pub y: i32,
}

#[inline(always)]
#[must_use]
/// Instantiates a new hexagon from axial coordinates
///
/// # Example
///
/// ```rust
/// # use hexx::*;
/// let coord = hex(3, 5);
/// assert_eq!(coord.x, 3);
/// assert_eq!(coord.y, 5);
/// assert_eq!(coord.z(), -3 - 5);
/// ```
pub const fn hex(x: i32, y: i32) -> Hex {
    Hex::new(x, y)
}

impl Hex {
    /// (0, 0)
    pub const ORIGIN: Self = Self::ZERO;
    /// (0, 0)
    pub const ZERO: Self = Self::new(0, 0);
    /// (1, 1)
    pub const ONE: Self = Self::new(1, 1);
    /// (-1, -1)
    pub const NEG_ONE: Self = Self::new(-1, -1);

    /// +X (Q) (1, 0)
    pub const X: Self = Self::new(1, 0);
    /// -X (-Q) (-1, 0)
    pub const NEG_X: Self = Self::new(-1, 0);
    /// +Y (R) (0, 1)
    pub const Y: Self = Self::new(0, 1);
    /// -Y (-R) (0, -1)
    pub const NEG_Y: Self = Self::new(0, -1);

    /// Unit vectors that increase the X axis in clockwise order
    pub const INCR_X: [Self; 2] = [Self::new(1, 0), Self::new(1, -1)];
    /// Unit vectors that increase the Y axis in clockwise order
    pub const INCR_Y: [Self; 2] = [Self::new(0, 1), Self::new(-1, 1)];
    /// Unit vectors that increase the Z axis in clockwise order
    pub const INCR_Z: [Self; 2] = [Self::new(-1, 0), Self::new(0, -1)];

    /// Unit vectors that decrease the X axis in clockwise order
    pub const DECR_X: [Self; 2] = [Self::new(-1, 0), Self::new(-1, 1)];
    /// Unit vectors that decrease the Y axis in clockwise order
    pub const DECR_Y: [Self; 2] = [Self::new(0, -1), Self::new(1, -1)];
    /// Unit vectors that decrease the Z axis in clockwise order
    pub const DECR_Z: [Self; 2] = [Self::new(1, 0), Self::new(0, 1)];

    /// Hexagon edge neighbor coordinates array, following [`EdgeDirection`]
    /// order
    ///
    /// ```txt
    ///       Z    ___   -Y
    ///           /   \
    ///       +--+  4  +--+
    ///      / 3  \___/  5 \
    ///      \    /   \    /
    ///  -X   +--+     +--+    X
    ///      /    \___/    \
    ///      \ 2  /   \  0 /
    ///       +--+  1  +--+
    ///           \___/
    ///       Y           -Z
    /// ```
    ///
    /// Cubic coordinates:
    ///
    /// ```txt
    ///              (0, -1, 1)
    ///                  ___
    ///                 /   \
    ///  (-1, 0, 1) +--+  4  +--+ (1, -1, 0)
    ///            / 3  \___/  5 \
    ///            \    /   \    /
    ///             +--+     +--+
    ///            /    \___/    \
    /// (-1, 1, 0) \ 2  /   \  0 / (1, 0, -1)
    ///             +--+  1  +--+
    ///                 \___/
    ///               (0, 1, -1)
    /// ```
    pub const NEIGHBORS_COORDS: [Self; 6] = [
        Self::new(1, 0),
        Self::new(0, 1),
        Self::new(-1, 1),
        Self::new(-1, 0),
        Self::new(0, -1),
        Self::new(1, -1),
    ];

    /// Hexagon diagonal neighbor coordinates array, following
    /// [`VertexDirection`] order
    ///
    /// ```txt
    ///       Z           -Y
    ///           \___/
    ///      \ 4  /   \ 5  /
    ///       +--+     +--+
    ///    __/    \___/    \__
    ///      \    /   \    /
    /// -X 3  +--+     +--+  0   X
    ///    __/    \___/    \__
    ///      \    /   \    /
    ///       +--+     +--+
    ///      / 2  \___/  1 \
    ///
    ///       Y           -Z
    /// ```
    pub const DIAGONAL_COORDS: [Self; 6] = [
        Self::new(2, -1),
        Self::new(1, 1),
        Self::new(-1, 2),
        Self::new(-2, 1),
        Self::new(-1, -1),
        Self::new(1, -2),
    ];

    #[inline(always)]
    #[must_use]
    /// Instantiates a new hexagon from axial coordinates
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    /// let coord = Hex::new(3, 5);
    /// assert_eq!(coord.x, 3);
    /// assert_eq!(coord.y, 5);
    /// assert_eq!(coord.z(), -3 - 5);
    /// ```
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    #[inline]
    #[must_use]
    /// Instantiates a new hexagon with all coordinates set to `v`
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    /// let coord = Hex::splat(3);
    /// assert_eq!(coord.x, 3);
    /// assert_eq!(coord.y, 3);
    /// assert_eq!(coord.z(), -3 - 3);
    /// ```
    pub const fn splat(v: i32) -> Self {
        Self { x: v, y: v }
    }

    #[inline]
    #[must_use]
    /// Instantiates new hexagonal coordinates in cubic space
    ///
    /// # Panics
    ///
    /// Will panic if the coordinates are invalid, meaning that the sum of
    /// coordinates is not equal to zero
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    /// let coord = Hex::new_cubic(3, 5, -8);
    /// assert_eq!(coord.x, 3);
    /// assert_eq!(coord.y, 5);
    /// assert_eq!(coord.z(), -8);
    /// ```
    pub const fn new_cubic(x: i32, y: i32, z: i32) -> Self {
        assert!(x + y + z == 0);
        Self { x, y }
    }

    #[inline]
    #[must_use]
    #[doc(alias = "q")]
    /// `x` coordinate (sometimes called `q` or `i`)
    pub const fn x(self) -> i32 {
        self.x
    }

    #[inline]
    #[must_use]
    #[doc(alias = "r")]
    /// `y` coordinate (sometimes called `r` or `j`)
    pub const fn y(self) -> i32 {
        self.y
    }

    #[inline]
    #[must_use]
    #[doc(alias = "s")]
    /// `z` coordinate (sometimes called `s` or `k`).
    ///
    /// This cubic space coordinate is computed as `-x - y`
    pub const fn z(self) -> i32 {
        -self.x - self.y
    }

    #[inline]
    #[must_use]
    /// Creates a [`Hex`] from an array
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    /// let p = Hex::from_array([3, 5]);
    /// assert_eq!(p.x, 3);
    /// assert_eq!(p.y, 5);
    /// ```
    pub const fn from_array([x, y]: [i32; 2]) -> Self {
        Self::new(x, y)
    }

    #[inline]
    #[must_use]
    /// Converts `self` to an array as `[x, y]`
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    /// let coord = Hex::new(3, 5);
    /// let [x, y] = coord.to_array();
    /// assert_eq!(x, 3);
    /// assert_eq!(y, 5);
    /// ```
    pub const fn to_array(self) -> [i32; 2] {
        [self.x, self.y]
    }

    #[inline]
    #[must_use]
    #[expect(clippy::cast_precision_loss)]
    /// Converts `self` to an [`f32`] array as `[x, y]`
    pub const fn to_array_f32(self) -> [f32; 2] {
        [self.x as f32, self.y as f32]
    }

    #[inline]
    #[must_use]
    /// Converts `self` to cubic coordinates array as `[x, y, z]`
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    /// let coord = Hex::new(3, 5);
    /// let [x, y, z] = coord.to_cubic_array();
    /// assert_eq!(x, 3);
    /// assert_eq!(y, 5);
    /// assert_eq!(z, -3 - 5);
    /// ```
    pub const fn to_cubic_array(self) -> [i32; 3] {
        [self.x, self.y, self.z()]
    }

    #[inline]
    #[must_use]
    #[expect(clippy::cast_precision_loss)]
    /// Converts `self` to cubic [`f32`] coordinates array as `[x, y, z]`
    pub const fn to_cubic_array_f32(self) -> [f32; 3] {
        [self.x as f32, self.y as f32, self.z() as f32]
    }

    /// Creates a [`Hex`] from the first 2 values in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than 2 elements long.
    #[inline]
    #[must_use]
    pub const fn from_slice(slice: &[i32]) -> Self {
        Self::new(slice[0], slice[1])
    }

    /// Writes the elements of `self` to the first 2 elements in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than 2 elements long.
    #[inline]
    pub fn write_to_slice(self, slice: &mut [i32]) {
        slice[0] = self.x;
        slice[1] = self.y;
    }

    #[must_use]
    #[inline]
    /// Converts `self` to an [`IVec2`].
    /// This operation is a direct mapping of coordinates, no hex to square
    /// coordinates are performed. To convert hex coordinates to world space
    /// use [`HexLayout`]
    ///
    /// [`HexLayout`]: crate::HexLayout
    pub const fn as_ivec2(self) -> IVec2 {
        IVec2 {
            x: self.x,
            y: self.y,
        }
    }

    #[must_use]
    #[inline]
    #[doc(alias = "as_cubic")]
    /// Converts `self` to an [`IVec3`] using cubic coordinates.
    /// This operation is a direct mapping of coordinates.
    /// To convert hex coordinates to world space use [`HexLayout`]
    ///
    /// [`HexLayout`]: crate::HexLayout
    pub const fn as_ivec3(self) -> IVec3 {
        IVec3 {
            x: self.x,
            y: self.y,
            z: self.z(),
        }
    }

    #[expect(clippy::cast_precision_loss)]
    #[must_use]
    #[inline]
    /// Converts `self` to a [`Vec2`].
    /// This operation is a direct mapping of coordinates.
    /// To convert hex coordinates to world space use [`HexLayout`]
    ///
    /// [`HexLayout`]: crate::HexLayout
    pub const fn as_vec2(self) -> Vec2 {
        Vec2 {
            x: self.x as f32,
            y: self.y as f32,
        }
    }

    #[inline]
    #[must_use]
    /// Negates the coordinate, giving its reflection (symmetry) around the
    /// origin.
    ///
    /// [`Hex`] implements [`Neg`] (`-` operator) but this method is `const`.
    ///
    /// [`Neg`]: std::ops::Neg
    pub const fn const_neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }

    #[inline]
    #[must_use]
    /// adds `self` and `other`.
    ///
    /// [`Hex`] implements [`Add`] (`+` operator) but this method is `const`.
    ///
    /// [`Add`]: std::ops::Add
    pub const fn const_add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    #[inline]
    #[must_use]
    /// substracts `self` and `rhs`.
    ///
    /// [`Hex`] implements [`Sub`] (`-` operator) but this method is `const`.
    ///
    /// [`Sub`]: std::ops::Sub
    pub const fn const_sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }

    #[inline]
    #[must_use]
    #[expect(clippy::cast_possible_truncation)]
    /// Rounds floating point coordinates to [`Hex`].
    /// This method is used for operations like multiplications and divisions
    /// with floating point numbers.
    /// See the original author Jacob Rus's [article](https://observablehq.com/@jrus/hexround) for
    /// more details
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    /// let point = [0.6, 10.2];
    /// let coord = Hex::round(point);
    /// assert_eq!(coord.x, 1);
    /// assert_eq!(coord.y, 10);
    /// ```
    pub fn round([mut x, mut y]: [f32; 2]) -> Self {
        let [mut x_r, mut y_r] = [x.round(), y.round()];
        x -= x_r;
        y -= y_r;
        if x.abs() >= y.abs() {
            x_r += 0.5_f32.mul_add(y, x).round();
        } else {
            y_r += 0.5_f32.mul_add(x, y).round();
        }
        Self::new(x_r as i32, y_r as i32)
    }

    #[inline]
    #[must_use]
    /// Computes the absolute value of `self`
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    /// let coord = Hex::new(-1, -32).abs();
    /// assert_eq!(coord.x, 1);
    /// assert_eq!(coord.y, 32);
    /// ```
    pub const fn abs(self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }
    /// Returns a vector containing the minimum values for each element of
    /// `self` and `rhs`.
    ///
    /// In other words this computes `[self.x.min(rhs.x), self.y.min(rhs.y),
    /// ..]`.
    #[inline]
    #[must_use]
    pub fn min(self, rhs: Self) -> Self {
        Self {
            x: self.x.min(rhs.x),
            y: self.y.min(rhs.y),
        }
    }

    /// Returns a vector containing the maximum values for each element of
    /// `self` and `rhs`.
    ///
    /// In other words this computes `[self.x.max(rhs.x), self.y.max(rhs.y),
    /// ..]`.
    #[inline]
    #[must_use]
    pub fn max(self, rhs: Self) -> Self {
        Self {
            x: self.x.max(rhs.x),
            y: self.y.max(rhs.y),
        }
    }

    /// Computes the dot product of `self` and `rhs`.
    #[inline]
    #[must_use]
    pub const fn dot(self, rhs: Self) -> i32 {
        (self.x * rhs.x) + (self.y * rhs.y)
    }

    #[inline]
    #[must_use]
    /// Returns a [`Hex`] with elements representing the sign of `self`.
    ///
    ///  - `0` if the number is zero
    ///  - `1` if the number is positive
    ///  - `-1` if the number is negative
    pub const fn signum(self) -> Self {
        Self {
            x: self.x.signum(),
            y: self.y.signum(),
        }
    }

    #[inline]
    #[must_use]
    #[doc(alias = "magnitude")]
    /// Computes coordinates length as a signed integer.
    /// The length of a [`Hex`] coordinate is equal to its distance from the
    /// origin.
    ///
    /// See [`Self::ulength`] for the unsigned version
    ///
    /// # Example
    /// ```rust
    /// # use hexx::*;
    /// let coord = Hex::new(10, 0);
    /// assert_eq!(coord.length(), 10);
    /// ```
    pub const fn length(self) -> i32 {
        let [x, y, z] = [self.x.abs(), self.y.abs(), self.z().abs()];
        if x >= y && x >= z {
            x
        } else if y >= x && y >= z {
            y
        } else {
            z
        }
    }

    #[inline]
    #[must_use]
    #[doc(alias = "unsigned_length")]
    /// Computes coordinates length as an unsigned integer
    /// The length of a [`Hex`] coordinate is equal to its distance from the
    /// origin.
    ///
    /// See [`Self::length`] for the signed version
    ///
    /// # Example
    /// ```rust
    /// # use hexx::*;
    /// let coord = Hex::new(10, 0);
    /// assert_eq!(coord.ulength(), 10);
    /// ```
    pub const fn ulength(self) -> u32 {
        let [x, y, z] = [
            self.x.unsigned_abs(),
            self.y.unsigned_abs(),
            self.z().unsigned_abs(),
        ];
        if x >= y && x >= z {
            x
        } else if y >= x && y >= z {
            y
        } else {
            z
        }
    }

    #[inline]
    #[must_use]
    /// Computes the manhattan distance from `self` to `rhs` in hexagonal space
    /// as a signed integer
    ///
    /// See [`Self::unsigned_distance_to`] for the unsigned version
    pub const fn distance_to(self, rhs: Self) -> i32 {
        self.const_sub(rhs).length()
    }

    #[inline]
    #[must_use]
    /// Computes the distance from `self` to `rhs` in hexagonal space as an
    /// unsigned integer
    ///
    /// See [`Self::distance_to`] for the signed version
    pub const fn unsigned_distance_to(self, rhs: Self) -> u32 {
        self.const_sub(rhs).ulength()
    }

    /// Computes euclidean distance from `self` to `rhs` as a floating point
    /// number in the Cartesian coordinate system.
    /// Euclidean distance can vary for coordinates in the same range, and
    /// can be used for operations outside of the hexagonal space, like
    /// checking if coordinates are in a circular range instead of an
    /// hexagonal range
    ///
    /// Note: For most cases you should use an [`crate::HexLayout`]
    ///
    /// > Source:
    /// > Xiangguo Li's 2013 [Paper]. ([DOI]) gives a formula for Euclidean
    /// > distance
    ///
    /// [Paper]: https://scholar.google.com/scholar?q=Storage+and+addressing+scheme+for+practical+hexagonal+image+processing
    /// [DOI]: https://doi.org/10.1117/1.JEI.22.1.010502
    #[must_use]
    #[expect(clippy::cast_precision_loss)]
    pub fn euclidean_distance_to(self, rhs: Self) -> f32 {
        let dx = (rhs.x - self.x) as f32;
        let dy = (rhs.y - self.y) as f32;
        dx.mul_add(dy, dx.mul_add(dx, dy * dy)).sqrt()
    }

    #[inline]
    #[must_use]
    /// Retrieves the hexagonal neighbor coordinates matching the given
    /// `direction`
    pub const fn neighbor_coord(direction: EdgeDirection) -> Self {
        direction.into_hex()
    }

    #[inline]
    #[must_use]
    /// Retrieves the diagonal neighbor coordinates matching the given
    /// `direction`
    pub const fn diagonal_neighbor_coord(direction: VertexDirection) -> Self {
        direction.into_hex()
    }

    pub(crate) const fn add_dir(self, direction: EdgeDirection) -> Self {
        self.const_add(Self::neighbor_coord(direction))
    }

    pub(crate) const fn add_diag_dir(self, direction: VertexDirection) -> Self {
        self.const_add(Self::diagonal_neighbor_coord(direction))
    }

    #[inline]
    #[must_use]
    /// Retrieves the neighbor coordinates matching the given `direction`
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    /// let coord = Hex::new(10, 5);
    /// let bottom = coord.neighbor(EdgeDirection::FLAT_BOTTOM);
    /// assert_eq!(bottom, Hex::new(10, 6));
    /// ```
    pub const fn neighbor(self, direction: EdgeDirection) -> Self {
        self.const_add(Self::neighbor_coord(direction))
    }

    #[inline]
    #[must_use]
    /// Retrieves the diagonal neighbor coordinates matching the given
    /// `direction`
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    /// let coord = Hex::new(10, 5);
    /// let bottom = coord.diagonal_neighbor(VertexDirection::FLAT_RIGHT);
    /// assert_eq!(bottom, Hex::new(12, 4));
    /// ```
    pub const fn diagonal_neighbor(self, direction: VertexDirection) -> Self {
        self.const_add(Self::diagonal_neighbor_coord(direction))
    }

    #[inline]
    #[must_use]
    /// Retrieves the direction of the given neighbor. Will return `None` if
    /// `other` is not a neighbor of `self`
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    /// let coord = Hex::new(10, 5);
    /// let bottom = coord.neighbor(EdgeDirection::FLAT_BOTTOM);
    /// let dir = coord.neighbor_direction(bottom).unwrap();
    /// assert_eq!(dir, EdgeDirection::FLAT_BOTTOM);
    /// ```
    pub fn neighbor_direction(self, other: Self) -> Option<EdgeDirection> {
        EdgeDirection::iter().find(|&dir| self.neighbor(dir) == other)
    }

    #[must_use]
    /// Find in which [`VertexDirection`] wedge `rhs` is relative to `self`.
    ///
    /// > This method can be innaccurate in case of a *tie* between directions,
    /// > prefer using [`Self::diagonal_way_to`] instead
    pub fn main_diagonal_to(self, rhs: Self) -> VertexDirection {
        self.diagonal_way_to(rhs).unwrap()
    }

    #[must_use]
    /// Find in which [`VertexDirection`] wedge `rhs` is relative to `self`
    pub fn diagonal_way_to(self, rhs: Self) -> DirectionWay<VertexDirection> {
        let [x, y, z] = (rhs - self).to_cubic_array();
        let [xa, ya, za] = [x.abs(), y.abs(), z.abs()];
        match xa.max(ya).max(za) {
            v if v == xa => {
                DirectionWay::way_from(x < 0, xa == ya, xa == za, VertexDirection::FLAT_RIGHT)
            }
            v if v == ya => {
                DirectionWay::way_from(y < 0, ya == za, ya == xa, VertexDirection::FLAT_BOTTOM_LEFT)
            }
            _ => DirectionWay::way_from(z < 0, za == xa, za == ya, VertexDirection::FLAT_TOP_LEFT),
        }
    }

    /// Find in which [`EdgeDirection`] wedge `rhs` is relative to `self`
    ///
    /// > This method can be innaccurate in case of a *tie* between directions,
    /// > prefer using [`Self::way_to`] for accuracy
    #[must_use]
    pub fn main_direction_to(self, rhs: Self) -> EdgeDirection {
        self.way_to(rhs).unwrap()
    }

    #[must_use]
    /// Find in which [`EdgeDirection`] wedge `rhs` is relative to `self`
    pub fn way_to(self, rhs: Self) -> DirectionWay<EdgeDirection> {
        let [x, y, z] = (rhs - self).to_cubic_array();
        let [x, y, z] = [y - x, z - y, x - z];
        let [xa, ya, za] = [x.abs(), y.abs(), z.abs()];
        match xa.max(ya).max(za) {
            v if v == xa => {
                DirectionWay::way_from(x < 0, xa == ya, xa == za, EdgeDirection::FLAT_BOTTOM_LEFT)
            }
            v if v == ya => {
                DirectionWay::way_from(y < 0, ya == za, ya == xa, EdgeDirection::FLAT_TOP)
            }
            _ => {
                DirectionWay::way_from(z < 0, za == xa, za == ya, EdgeDirection::FLAT_BOTTOM_RIGHT)
            }
        }
    }

    #[inline]
    #[must_use]
    /// Retrieves all 6 neighbor coordinates around `self`
    pub fn all_neighbors(self) -> [Self; 6] {
        Self::NEIGHBORS_COORDS.map(|n| self.const_add(n))
    }

    #[inline]
    #[must_use]
    /// Retrieves all 6 neighbor diagonal coordinates around `self`
    pub fn all_diagonals(self) -> [Self; 6] {
        Self::DIAGONAL_COORDS.map(|n| self.const_add(n))
    }

    #[inline]
    #[must_use]
    #[doc(alias = "ccw")]
    /// Rotates `self` around [`Hex::ZERO`] counter clockwise (by -60 degrees)
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    ///
    /// let p = Hex::new(1, 2);
    /// assert_eq!(p.counter_clockwise(), Hex::new(3, -1));
    /// ```
    pub const fn counter_clockwise(self) -> Self {
        Self::new(-self.z(), -self.x)
    }

    #[inline]
    #[must_use]
    /// Rotates `self` around `center` counter clockwise (by -60 degrees)
    pub const fn ccw_around(self, center: Self) -> Self {
        self.const_sub(center).counter_clockwise().const_add(center)
    }

    #[inline]
    #[must_use]
    /// Rotates `self` around [`Hex::ZERO`] counter clockwise by `m` (by `-60 *
    /// m` degrees)
    pub const fn rotate_ccw(self, m: u32) -> Self {
        match m % 6 {
            1 => self.counter_clockwise(),
            2 => self.counter_clockwise().counter_clockwise(),
            3 => self.const_neg(),
            4 => self.clockwise().clockwise(),
            5 => self.clockwise(),
            _ => self,
        }
    }

    #[inline]
    #[must_use]
    /// Rotates `self` around `center` counter clockwise by `m` (by `-60 * m`
    /// degrees)
    pub const fn rotate_ccw_around(self, center: Self, m: u32) -> Self {
        self.const_sub(center).rotate_ccw(m).const_add(center)
    }

    #[inline]
    #[must_use]
    #[doc(alias = "cw")]
    /// Rotates `self` around [`Hex::ZERO`] clockwise (by 60 degrees)
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    ///
    /// let p = Hex::new(1, 2);
    /// assert_eq!(p.clockwise(), Hex::new(-2, 3));
    /// ```
    pub const fn clockwise(self) -> Self {
        Self::new(-self.y, -self.z())
    }

    #[inline]
    #[must_use]
    /// Rotates `self` around `center` clockwise (by 60 degrees)
    pub const fn cw_around(self, center: Self) -> Self {
        self.const_sub(center).clockwise().const_add(center)
    }

    #[inline]
    #[must_use]
    /// Rotates `self` around [`Hex::ZERO`] clockwise by `m` (by `60 * m`
    /// degrees)
    pub const fn rotate_cw(self, m: u32) -> Self {
        match m % 6 {
            1 => self.clockwise(),
            2 => self.clockwise().clockwise(),
            3 => self.const_neg(),
            4 => self.counter_clockwise().counter_clockwise(),
            5 => self.counter_clockwise(),
            _ => self,
        }
    }

    #[inline]
    #[must_use]
    /// Rotates `self` around `center` clockwise by `m` (by `60 * m` degrees)
    pub const fn rotate_cw_around(self, center: Self, m: u32) -> Self {
        self.const_sub(center).rotate_cw(m).const_add(center)
    }

    #[inline]
    #[must_use]
    #[doc(alias = "reflect_q")]
    /// Computes the reflection of `self` accross the `x` axis
    pub const fn reflect_x(self) -> Self {
        Self::new(self.x, self.z())
    }

    #[inline]
    #[must_use]
    #[doc(alias = "reflect_r")]
    /// Computes the reflection of `self` accross the `y` axis
    pub const fn reflect_y(self) -> Self {
        Self::new(self.z(), self.y)
    }

    #[inline]
    #[must_use]
    #[doc(alias = "reflect_s")]
    /// Computes the reflection of `self` accross the `z` axis
    pub const fn reflect_z(self) -> Self {
        Self::new(self.y, self.x)
    }

    #[expect(clippy::cast_precision_loss)]
    #[must_use]
    /// Computes all coordinates in a line from `self` to `other`.
    ///
    /// # Example
    /// ```rust
    /// # use hexx::*;
    /// let start = Hex::ZERO;
    /// let end = Hex::new(5, 0);
    ///
    /// let line = start.line_to(end);
    /// assert_eq!(line.len(), 6);
    /// let line: Vec<Hex> = line.collect();
    /// assert_eq!(line.len(), 6);
    /// ````
    pub fn line_to(self, other: Self) -> impl ExactSizeIterator<Item = Self> {
        let distance = self.unsigned_distance_to(other);
        let dist = distance.max(1) as f32;
        let [a, b]: [Vec2; 2] = [self.as_vec2(), other.as_vec2()];
        ExactSizeHexIterator {
            iter: (0..=distance).map(move |step| a.lerp(b, step as f32 / dist).into()),
            count: distance as usize + 1,
        }
    }

    #[expect(clippy::cast_sign_loss)]
    #[must_use]
    /// Computes all coordinate in a two segment rectiline path from `self` to
    /// `other`
    ///
    /// # Arguments
    ///
    /// * `other` - The destination coordinate
    /// * `clockwise` - If set to `true` the line paths will be clockwise
    ///
    /// # Example
    /// ```rust
    /// # use hexx::*;
    /// let start = Hex::ZERO;
    /// let end = Hex::new(5, 0);
    ///
    /// let line = start.rectiline_to(end, true);
    /// assert_eq!(line.len(), 6);
    /// let line: Vec<Hex> = line.collect();
    /// assert_eq!(line.len(), 6);
    /// assert_eq!(line[0], start);
    /// assert_eq!(line[5], end);
    /// ````
    pub fn rectiline_to(self, other: Self, clockwise: bool) -> impl ExactSizeIterator<Item = Self> {
        let delta = other.const_sub(self);
        let count = delta.length();
        let mut dirs = self.main_diagonal_to(other).edge_directions();
        if !clockwise {
            dirs.rotate_left(1);
        }
        // The two directions to apply
        let [dir_a, dir_b] = dirs;
        // The amount of `da` is the distance between `delta` and the full projection of
        // `db`
        let proj_b = dir_b * count;
        let ca = proj_b.distance_to(delta);

        let iter = std::iter::once(self).chain((0..count).scan(self, move |p, i| {
            if i < ca {
                *p += dir_a;
            } else {
                *p += dir_b;
            }
            Some(*p)
        }));
        ExactSizeHexIterator {
            iter,
            count: (count + 1) as usize,
        }
    }

    /// Performs a linear interpolation between `self` and `rhs` based on the
    /// value `s`.
    ///
    /// When `s` is `0.0`, the result will be equal to `self`.  When `s` is
    /// `1.0`, the result will be equal to `rhs`. When `s` is outside of
    /// range `[0, 1]`, the result is linearly extrapolated.
    #[doc(alias = "mix")]
    #[inline]
    #[must_use]
    pub fn lerp(self, rhs: Self, s: f32) -> Self {
        let [start, end]: [Vec2; 2] = [self.as_vec2(), rhs.as_vec2()];
        start.lerp(end, s).into()
    }

    #[expect(clippy::cast_possible_wrap)]
    #[must_use]
    /// Retrieves all [`Hex`] around `self` in a given `range`.
    /// The number of returned coordinates is equal to `Hex::range_count(range)`
    ///
    /// > See also [`Hex::xrange`] to retrieve all coordinates excluding `self`
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    /// let coord = hex(12, 34);
    /// assert_eq!(coord.range(0).len(), 1);
    /// assert_eq!(coord.range(1).len(), 7);
    /// ```
    pub fn range(self, range: u32) -> impl ExactSizeIterator<Item = Self> {
        let radius = range as i32;
        ExactSizeHexIterator {
            iter: (-radius..=radius).flat_map(move |x| {
                let y_min = max(-radius, -x - radius);
                let y_max = min(radius, radius - x);
                (y_min..=y_max).map(move |y| self.const_add(Self::new(x, y)))
            }),
            count: Self::range_count(range) as usize,
        }
    }

    /// Retrieves all [`Hex`] around `self` in a given circular `range`.
    ///
    /// > See also [`Hex::range`] for hexagonal ranges
    ///
    /// Note that this implementation is very naive and inefficient.
    /// Use sparingly
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    /// let coord = hex(12, 34);
    /// assert_eq!(coord.circular_range(0.0).count(), 1);
    /// assert_eq!(coord.circular_range(1.0).count(), 7);
    /// ```
    #[expect(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    pub fn circular_range(self, range: f32) -> impl Iterator<Item = Self> {
        let radius = range.ceil() as u32;
        // TODO: Improve this computation to have the smallest hexagon
        // which fits the circle
        let hex_range = radius + (range / 6.0).floor() as u32;
        self.range(hex_range)
            .filter(move |h| self.euclidean_distance_to(*h) <= range)
    }

    #[doc(alias = "excluding_range")]
    #[must_use]
    /// Retrieves all [`Hex`] around `self` in a given `range` except `self`.
    /// The number of returned coordinates is equal to
    /// `Hex::range_count(range) - 1`
    ///
    /// > See also [`Hex::range`] to retrieve all coordinates including `self`
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    /// let coord = hex(12, 34);
    /// assert_eq!(coord.xrange(0).len(), 0);
    /// assert_eq!(coord.xrange(1).len(), 6);
    /// ```
    pub fn xrange(self, range: u32) -> impl ExactSizeIterator<Item = Self> {
        let iter = self.range(range);
        ExactSizeHexIterator {
            count: iter.len().saturating_sub(1),
            iter: iter.filter(move |h| *h != self),
        }
    }

    /// Computes the coordinate of a lower resolution hexagon containing `self`
    /// of a given `radius`.
    /// The lower resolution coordinate can be considered *parent* of
    /// the contained higher resolution coordinates.
    /// The `radius` can be thought of as a *chunk size*, as if the grid was
    /// split in hexagonal chunks of that radius. The returned value are the
    /// coordinates of that chunk, in its own coordinates system.
    ///
    /// See the [source] documentation for more information
    ///
    /// > See also [`Self::to_higher_res`] and [`Self::to_local`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    ///
    /// // We define a coordinate
    /// let coord = hex(23, 45);
    /// // We take its *parent* in a coordinate system of size 5
    /// let parent = coord.to_lower_res(5);
    /// // We can then retrieve the center of that parent in the same system as `coord`
    /// let center = parent.to_higher_res(5);
    /// // Therefore the distance between the parent center and `coord` should be lower than 5
    /// assert!(coord.distance_to(center) <= 5);
    /// ```
    ///
    /// [source]: https://observablehq.com/@sanderevers/hexagon-tiling-of-an-hexagonal-grid
    #[must_use]
    #[expect(
        clippy::cast_possible_wrap,
        clippy::cast_precision_loss,
        clippy::cast_possible_truncation
    )]
    #[doc(alias = "downscale")]
    pub fn to_lower_res(self, radius: u32) -> Self {
        let [x, y, z] = self.to_cubic_array();
        let area = Self::range_count(radius) as f32;
        let shift = Self::shift(radius) as i32;
        let [x, y, z] = [
            ((y + shift * x) as f32 / area).floor() as i32,
            ((z + shift * y) as f32 / area).floor() as i32,
            ((x + shift * z) as f32 / area).floor() as i32,
        ];
        let [x, y] = [
            ((1 + x - y) as f32 / 3.0).floor() as i32,
            ((1 + y - z) as f32 / 3.0).floor() as i32,
            // ((1 + z - x) as f32 / 3.0).floor() as i32, -- z
        ];
        // debug_assert_eq!(z, -x - y);
        Self::new(x, y)
    }

    /// Computes the center coordinates of `self` in a higher resolution system
    /// of a given `radius`.
    /// The higher resolution coordinate can be considered as a *child* of
    /// `self` as it is contained by it in a lower resolution coordinates
    /// system. The `radius` can be thought of as a *chunk size*, as if the
    /// grid was split in hexagonal chunks of that radius. The returned
    /// value are the coordinates of the center that chunk, in a higher
    /// resolution coordinates system.
    ///
    /// See the [source] documentation for more information
    ///
    /// > See also [`Self::to_lower_res`] and [`Self::to_local`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    ///
    /// // We define a coordinate
    /// let coord = hex(23, 45);
    /// // We take its *parent* in a coordinate system of size 5
    /// let parent = coord.to_lower_res(5);
    /// // We can then retrieve the center of that parent in the same system as `coord`
    /// let center = parent.to_higher_res(5);
    /// // Therefore the distance between the parent center and `coord` should be lower than 5
    /// assert!(coord.distance_to(center) <= 5);
    /// ```
    ///
    /// [source]: https://observablehq.com/@sanderevers/hexagon-tiling-of-an-hexagonal-grid
    #[must_use]
    #[expect(clippy::cast_possible_wrap)]
    #[doc(alias = "upscale")]
    pub const fn to_higher_res(self, radius: u32) -> Self {
        let range = radius as i32;
        let [x, y, z] = self.to_cubic_array();
        Self::new(x * (range + 1) - range * z, y * (range + 1) - range * x)
    }

    /// Computes the local coordinates of `self` in a lower resolution
    /// coordinates system relative to its containing *parent* hexagon
    ///
    ///
    /// See the [source] documentation for more information
    ///
    /// > See also [`Self::to_lower_res`] and [`Self::to_local`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    ///
    /// // We define a coordinate
    /// let coord = hex(23, 45);
    /// // We can then retrieve the center of that hexagon in a higher res of size 5
    /// let center = coord.to_higher_res(5);
    /// // Therefore, the local coordinates of `center` relative to `coord` should be zero
    /// assert_eq!(center.to_local(5), Hex::ZERO);
    /// ```
    ///
    /// [source]: https://observablehq.com/@sanderevers/hexagon-tiling-of-an-hexagonal-grid
    #[must_use]
    pub fn to_local(self, radius: u32) -> Self {
        let upscale = self.to_lower_res(radius);
        let center = upscale.to_higher_res(radius);
        self.const_sub(center)
    }

    #[inline]
    #[must_use]
    /// Counts how many coordinates there are in the given `range`
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    /// assert_eq!(Hex::range_count(15), 721);
    /// assert_eq!(Hex::range_count(0), 1);
    /// ```
    pub const fn range_count(range: u32) -> u32 {
        3 * range * (range + 1) + 1
    }

    /// Shift constant used for [hexmod] operations
    ///
    /// [hexmod]: https://observablehq.com/@sanderevers/hexmod-representation
    #[inline]
    #[must_use]
    pub(crate) const fn shift(range: u32) -> u32 {
        3 * range + 2
    }

    #[must_use]
    /// Wraps `self` in an hex range around the origin ([`Hex::ZERO`]).
    /// this allows for seamless *wraparound* hexagonal maps.
    /// See this [article] for more information.
    ///
    /// Use [`HexBounds`] for custom wrapping
    ///
    /// [`HexBounds`]: crate::HexBounds
    /// [article]: https://www.redblobgames.com/grids/hexagons/#wraparound
    pub fn wrap_in_range(self, range: u32) -> Self {
        self.to_local(range)
    }
}

#[cfg(not(target_arch = "spirv"))]
impl Debug for Hex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Hex")
            .field("x", &self.x)
            .field("y", &self.y)
            .field("z", &self.z())
            .finish()
    }
}
