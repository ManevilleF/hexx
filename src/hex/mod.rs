/// Type conversions
mod convert;
/// Traits implementations
mod impls;
#[cfg(test)]
mod tests;

pub use impls::{Mean, MeanExt};

use crate::Direction;
use glam::{IVec2, IVec3, Vec2};
use itertools::Itertools;
use std::cmp::{max, min};

/// Hexagonal [axial] coordinates
///
/// # Why Axial ?
///
/// Axial coordinates allow to compute and use *cubic* coordinates with less storage,
/// and allow:
/// - Vector operations
/// - Rotations
/// - Symmetry
/// - Simple algorithms
///
/// when *offset* and *doubled* coordinates don't. Furthermore, it makes the [`Hex`] behave like
/// classic 2D coordinates ([`IVec2`]) and therefore more user friendly.
///
/// Check out this [comparison] article for more information.
///
/// # Conversions
///
///  * Cubic: use [`Self::z`] to compute the third axis
///  * Offset: use [`Self::from_offset_coordinates`] and [`Self::from_offset_coordinates`]
///  * Doubled: use [`Self::from_doubled_coordinates`] and [`Self::from_doubled_coordinates`]
///
/// # Floating point operations
///
/// `Hex` is a primitive type, therefore dividing it by an other integer might not return the
/// expected value, correctly rounded to the correct `Hex`, in various cases like lerping
/// ([`Self::lerp`]).
///
/// In such cases, prefer using the following methods:
/// - [`Self::rounded_div`]
/// - [`Self::rounded_mul`]
///
/// [comparison]: https://www.redblobgames.com/grids/hexagons/#coordinates-comparison
/// [axial]: https://www.redblobgames.com/grids/hexagons/#coordinates-axial
///
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "ser_de", derive(serde::Serialize, serde::Deserialize))]
pub struct Hex {
    /// `x` axial coordinate (sometimes called `q` or `i`)
    pub x: i32,
    /// `y` axial coordinate (sometimes called `r` or `j`)
    pub y: i32,
}

impl Hex {
    /// (0, 0)
    pub const ORIGIN: Self = Self::ZERO;
    /// (0, 0)
    pub const ZERO: Self = Self::new(0, 0);
    /// (1, 1)
    pub const ONE: Self = Self::new(1, 1);
    /// X (Q) axis (1, 0)
    pub const X: Self = Self::new(1, 0);
    /// Y (R) axis (0, 1)
    pub const Y: Self = Self::new(0, 1);
    /// Z (S) axis (0, -1)
    pub const Z: Self = Self::new(0, -1);

    /// Hexagon neighbor coordinates array, following [`Direction`] order
    ///
    /// ```txt
    ///            x Axis
    ///            ___
    ///           /   \
    ///       +--+  1  +--+
    ///      / 2  \___/  0 \
    ///      \    /   \    /
    ///       +--+     +--+
    ///      /    \___/    \
    ///      \ 3  /   \  5 /
    ///       +--+  4  +--+   y Axis
    ///           \___/
    /// ```
    pub const NEIGHBORS_COORDS: [Self; 6] = [
        Self::new(1, -1),
        Self::new(0, -1),
        Self::new(-1, 0),
        Self::new(-1, 1),
        Self::new(0, 1),
        Self::new(1, 0),
    ];

    /// ```txt
    ///            x Axis
    ///           \___/
    ///      \ 2  /   \ 1  /
    ///       +--+     +--+
    ///    __/    \___/    \__
    ///      \    /   \    /
    ///    3  +--+     +--+  0
    ///    __/    \___/    \__
    ///      \    /   \    /
    ///       +--+     +--+   y Axis
    ///      / 4  \___/  5 \
    /// ```
    pub const DIAGONAL_COORDS: [Self; 6] = [
        Self::new(2, -1),
        Self::new(1, -2),
        Self::new(-1, -1),
        Self::new(-2, 1),
        Self::new(-1, 2),
        Self::new(1, 1),
    ];

    #[inline]
    #[must_use]
    /// Instantiates a new hexagon from axial coordinates
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    #[inline]
    #[must_use]
    /// Instantiates a new hexagon with all coordinates set to `v`
    pub const fn splat(v: i32) -> Self {
        Self { x: v, y: v }
    }

    #[inline]
    #[must_use]
    /// Instantiates new hexagonal coordinates in cubic space
    ///
    /// # Panics
    ///
    /// Will panic if the coordinates are invalid, meaning that the sum of coordinates is not equal
    /// to zero
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

    #[must_use]
    #[inline]
    /// Converts `self` to an [`IVec2`].
    /// This operation is a direct mapping of coordinates, no hex to square coordinates are
    /// performed. To convert hex coordinates to world space use [`HexLayout`]
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

    #[allow(clippy::cast_precision_loss)]
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
    /// Negates the coordinate, giving its reflection (symmetry) around the origin.
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
    /// substracts `self` and `other`.
    ///
    /// [`Hex`] implements [`Sub`] (`-` operator) but this method is `const`.
    ///
    /// [`Sub`]: std::ops::Sub
    pub const fn const_sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    #[inline]
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    /// Rounds floating point coordinates to [`Hex`]
    pub fn round((mut x, mut y): (f32, f32)) -> Self {
        let (mut x_r, mut y_r) = (x.round(), y.round());
        x -= x.round(); // remainder
        y -= y.round(); // remainder
        if x * x >= y * y {
            x_r += 0.5_f32.mul_add(y, x).round();
        }
        if x * x < y * y {
            y_r += 0.5_f32.mul_add(x, y).round();
        }
        Self::new(x_r as i32, y_r as i32)
    }

    #[allow(clippy::cast_precision_loss)]
    #[inline]
    #[must_use]
    /// Divides `self` by `rhs`, rounding up to the closest `Hex`
    pub fn rounded_div(self, rhs: f32) -> Self {
        Self::round((self.x as f32 / rhs, self.y as f32 / rhs))
    }

    #[allow(clippy::cast_precision_loss)]
    #[inline]
    #[must_use]
    /// Multiplies `self` by `rhs`, rounding up to the closest `Hex`
    pub fn rounded_mul(self, rhs: f32) -> Self {
        Self::round((self.x as f32 * rhs, self.y as f32 * rhs))
    }

    #[inline]
    #[must_use]
    /// Computes coordinates length as a signed integer
    ///
    /// See [`Self::ulength`] for the unsigned version
    pub const fn length(self) -> i32 {
        (self.x.abs() + self.y.abs() + self.z().abs()) / 2
    }

    #[inline]
    #[must_use]
    #[doc(alias = "unsigned_length")]
    /// Computes coordinates length as an unsigned integer
    ///
    /// See [`Self::length`] for the signed version
    pub const fn ulength(self) -> u32 {
        (self.x.unsigned_abs() + self.y.unsigned_abs() + self.z().unsigned_abs()) / 2
    }

    #[inline]
    #[must_use]
    /// Computes the distance from `self` to `other` in hexagonal space as a signed integer
    ///
    /// See [`Self::unsigned_distance_to`] for the unsigned version
    pub fn distance_to(self, other: Self) -> i32 {
        (self - other).length()
    }

    #[inline]
    #[must_use]
    /// Computes the distance from `self` to `other` in hexagonal space as an unsigned integer
    ///
    /// See [`Self::distance_to`] for the signed version
    pub fn unsigned_distance_to(self, other: Self) -> u32 {
        (self - other).ulength()
    }

    #[inline]
    #[must_use]
    /// Retrieves the hexagonal neighbor coordinates matching the given `direction`
    pub const fn neighbor_coord(direction: Direction) -> Self {
        Self::NEIGHBORS_COORDS[direction as usize]
    }

    #[inline]
    #[must_use]
    /// Retrieves the neighbor coordinates matching the given `direction`
    pub const fn neighbor(self, direction: Direction) -> Self {
        self.const_add(Self::neighbor_coord(direction))
    }

    #[inline]
    #[must_use]
    /// Retrieves the direction of the given neighbor. Will return `None` if `other` is not a neighbor
    /// of `self`
    pub fn neighbor_direction(self, other: Self) -> Option<Direction> {
        Direction::iter().find(|&dir| self.neighbor(dir) == other)
    }

    #[inline]
    /// Retrieves all directions in the line betzeen `self` and `other`
    pub fn directions_to(self, other: Self) -> impl Iterator<Item = Direction> {
        self.line_to(other)
            .tuple_windows::<(_, _)>()
            .filter_map(|(a, b)| a.neighbor_direction(b))
    }

    #[inline]
    #[must_use]
    /// Retrieves all 6 neighbor coordinates around `self`
    pub fn all_neighbors(self) -> [Self; 6] {
        Self::NEIGHBORS_COORDS.map(|n| self + n)
    }

    #[inline]
    #[must_use]
    /// Retrieves all 6 neighbor diagonal coordinates around `self`
    pub fn all_diagonals(self) -> [Self; 6] {
        Self::DIAGONAL_COORDS.map(|n| self + n)
    }

    #[inline]
    #[must_use]
    /// Rotates `self` around [`Hex::ZERO`] counter clockwise (by -60 degrees)
    pub const fn rotate_left(self) -> Self {
        Self::new(-self.z(), -self.x)
    }

    #[inline]
    #[must_use]
    /// Rotates `self` around `center`  counter clockwise (by -60 degrees)
    pub fn rotate_left_around(self, center: Self) -> Self {
        (self - center).rotate_left() + center
    }

    #[inline]
    #[must_use]
    /// Rotates `self` around [`Hex::ZERO`] clockwise (by 60 degrees)
    pub const fn rotate_right(self) -> Self {
        Self::new(-self.y, -self.z())
    }

    #[inline]
    #[must_use]
    /// Rotates `self` around `center` clockwise (by 60 degrees)
    pub fn rotate_right_around(self, center: Self) -> Self {
        (self - center).rotate_right() + center
    }

    #[inline]
    #[must_use]
    #[doc(alias = "reflect_q")]
    /// Computes the reflection of `self` accross[`Hex::X`]
    pub const fn reflect_x(self) -> Self {
        Self::new(self.x, self.z())
    }

    #[inline]
    #[must_use]
    #[doc(alias = "reflect_r")]
    /// Computes the reflection of `self` accross [`Hex::Y`]
    pub const fn reflect_y(self) -> Self {
        Self::new(self.z(), self.y)
    }

    #[inline]
    #[must_use]
    #[doc(alias = "reflect_s")]
    /// Computes the reflection of `self` accross [`Hex::Z`]
    pub const fn reflect_z(self) -> Self {
        Self::new(self.y, self.x)
    }

    #[allow(clippy::cast_precision_loss)]
    /// Computes all coordinates in a line from `self` to `other`.
    ///
    /// # Example
    /// ```rust
    /// # use hexx::*;
    /// let start = Hex::ZERO;
    /// let end = Hex::new(5, 0);
    ///
    /// let line: Vec<Hex> = start.line_to(end).collect();
    /// assert_eq!(line.len(), 6);
    /// ````
    pub fn line_to(self, other: Self) -> impl Iterator<Item = Self> {
        let distance = self.distance_to(other);
        let [a, b]: [Vec2; 2] = [self.as_vec2(), other.as_vec2()];
        (0..=distance).map(move |step| a.lerp(b, step as f32 / distance as f32).into())
    }

    /// Performs a linear interpolation between `self` and `rhs` based on the value `s`.
    ///
    /// When `s` is `0.0`, the result will be equal to `self`.  When `s` is `1.0`, the result
    /// will be equal to `rhs`. When `s` is outside of range `[0, 1]`, the result is linearly
    /// extrapolated.
    #[doc(alias = "mix")]
    #[inline]
    #[must_use]
    pub fn lerp(self, rhs: Self, s: f32) -> Self {
        let [start, end]: [Vec2; 2] = [self.as_vec2(), rhs.as_vec2()];
        start.lerp(end, s).into()
    }

    #[allow(clippy::cast_possible_wrap)]
    /// Retrieves all [`Hex`] around `self` in a given `range`
    pub fn range(self, range: u32) -> impl Iterator<Item = Self> {
        let range = range as i32;
        (-range..=range).flat_map(move |x| {
            (max(-range, -x - range)..=min(range, range - x)).map(move |y| self + Self::new(x, y))
        })
    }

    #[inline]
    #[must_use]
    /// Counts how many coordinates there are in the given `range`
    pub const fn range_count(range: u32) -> usize {
        (3 * range * (range + 1) + 1) as usize
    }

    #[must_use]
    #[allow(clippy::cast_possible_wrap)]
    /// Retrieves one [`Hex`] ring around `self` in a given `range`.
    /// The returned coordinates start from `start_dir` and loop counter clockwise around `self`
    /// unless `clockwise` is set to `true`.
    ///
    /// If you only need the coordinates see [`Self::ring`].
    ///
    /// # Note
    /// The returned vector will be of `6 * radius` ([`Self::ring_count`])
    pub fn custom_ring(self, range: u32, start_dir: Direction, clockwise: bool) -> Vec<Self> {
        if range == 0 {
            return vec![self];
        }
        let mut directions = Self::NEIGHBORS_COORDS;
        // TODO: improve code clarity
        directions.rotate_left(start_dir as usize);
        if clockwise {
            directions.reverse();
            directions.rotate_left(1);
        } else {
            directions.rotate_left(2);
        }

        let mut hex = self + Self::neighbor_coord(start_dir) * range as i32;
        let mut res = Vec::with_capacity(Self::ring_count(range));
        for dir in directions {
            (0..range).for_each(|_| {
                res.push(hex);
                hex += dir;
            });
        }
        res
    }

    #[must_use]
    /// Retrieves one [`Hex`] ring around `self` in a given `range`.
    /// The returned coordinates start from [`Direction::TopRight`] and loop around `self` counter clockwise.
    ///
    /// See [`Self::custom_ring`] for more options.
    ///
    /// # Note
    /// The returned vector will be of `6 * radius` ([`Self::ring_count`])
    pub fn ring(self, range: u32) -> Vec<Self> {
        self.custom_ring(range, Direction::TopRight, false)
    }

    #[allow(clippy::cast_possible_truncation)]
    #[must_use]
    /// Retrieves all successive [`Hex`] rings around `self` in a given `RANGE` as an array of
    /// rings.
    /// The returned rings start from [`Direction::TopRight`] and loop around `self` counter clockwise.
    ///
    /// If you only need the coordinates see [`Self::range`] or [`Self::spiral_range`].
    ///
    /// # Usage
    ///
    /// This function's objective is to pre-compute rings around a coordinate, the returned array
    /// can be used as a cache to avoid extra computation.
    ///
    /// ## Example
    ///
    /// ```rust
    /// # use hexx::*;
    ///
    /// // We cache 10 rings around the origin
    /// let cache = Hex::ORIGIN.cached_rings::<10>();
    /// // We have our target center
    /// let target = Hex::new(11, 24);
    /// // We retrieve the ring of range 5 and offset it to match the target
    /// let five_ring = cache[5].iter().map(|h| *h + target);
    /// ```
    ///
    /// See this [article](https://www.redblobgames.com/grids/hexagons/#rings-spiral) for more
    /// information
    pub fn cached_rings<const RANGE: usize>(self) -> [Vec<Self>; RANGE] {
        std::array::from_fn(|r| self.ring(r as u32))
    }

    #[allow(clippy::cast_possible_truncation)]
    #[must_use]
    /// Retrieves all successive [`Hex`] rings around `self` in a given `RANGE` as an array of
    /// rings.
    /// The returned rings start from `start_dir`] and loop around `self` counter clockwise unless
    /// `clockwise` is set to `true`.
    ///
    /// See also [`Self::cached_rings`]
    /// If you only need the coordinates see [`Self::range`] or [`Self::custom_spiral_range`].
    ///
    /// # Usage
    ///
    /// This function's objective is to pre-compute rings around a coordinate, the returned array
    /// can be used as a cache to avoid extra computation.
    ///
    /// ## Example
    ///
    /// ```rust
    /// # use hexx::*;
    ///
    /// // We cache 10 rings around the origin
    /// let cache = Hex::ORIGIN.cached_custom_rings::<10>(Direction::Top, true);
    /// // We have our target center
    /// let target = Hex::new(11, 24);
    /// // We retrieve the ring of range 5 and offset it to match the target
    /// let five_ring = cache[5].iter().map(|h| *h + target);
    /// ```
    ///
    /// See this [article](https://www.redblobgames.com/grids/hexagons/#rings-spiral) for more
    /// information
    pub fn cached_custom_rings<const RANGE: usize>(
        self,
        start_dir: Direction,
        clockwise: bool,
    ) -> [Vec<Self>; RANGE] {
        std::array::from_fn(|r| self.custom_ring(r as u32, start_dir, clockwise))
    }

    #[must_use]
    /// Retrieves all [`Hex`] around `self` in a given `range` but ordered as successive rings,
    /// starting from `start_dir` and looping counter clockwise unless `clockwise` is set to `true`, forming a spiral
    ///
    /// If you only need the coordinates see [`Self::spiral_range`].
    ///
    /// See this [article](https://www.redblobgames.com/grids/hexagons/#rings-spiral) for more
    /// information
    pub fn custom_spiral_range(
        self,
        range: u32,
        start_dir: Direction,
        clockwise: bool,
    ) -> Vec<Self> {
        let mut res = Vec::with_capacity(Self::range_count(range));
        for i in 0..=range {
            res.extend(self.custom_ring(i, start_dir, clockwise));
        }
        res
    }

    #[must_use]
    #[allow(clippy::cast_sign_loss)]
    /// Retrieves all [`Hex`] around `self` in a given `range` but ordered as successive rings,
    /// starting from [`Direction::TopRight`] and looping counter clockwise, forming a spiral.
    ///
    /// See [`Self::custom_spiral_range`] for more options
    ///
    /// See this [article](https://www.redblobgames.com/grids/hexagons/#rings-spiral) for more
    /// information
    pub fn spiral_range(self, range: u32) -> Vec<Self> {
        self.custom_spiral_range(range, Direction::TopRight, false)
    }

    #[inline]
    #[must_use]
    /// Counts how many coordinates there are in a ring at the given `range`
    pub const fn ring_count(range: u32) -> usize {
        6 * range as usize
    }

    #[must_use]
    /// Wraps `self` in an hex range around the origin ([`Hex::ZERO`]).
    /// this allows for seamless *wraparound* hexagonal maps.
    /// See this [article] for more information.
    ///
    /// Use [`HexMap`] for improved wrapping
    ///
    /// [`HexMap`]: crate::HexMap
    /// [article]: https://www.redblobgames.com/grids/hexagons/#wraparound
    pub fn wrap_in_range(self, radius: u32) -> Self {
        self.wrap_with(radius, &Self::wraparound_mirrors(radius))
    }

    #[must_use]
    /// Wraps `self` in an hex range around the origin ([`Hex::ZERO`]) using custom mirrors.
    ///
    /// # Panics
    ///
    /// Will panic with invalid `mirrors`
    /// Prefer using [`Self::wrap_in_range`] or [`HexMap`] for safe wrapping.
    ///
    /// [`HexMap`]: crate::HexMap
    pub fn wrap_with(self, radius: u32, mirrors: &[Self; 6]) -> Self {
        if self.ulength() <= radius {
            return self;
        }
        let mut res = self;
        while res.ulength() > radius {
            let mirror = mirrors
                .iter()
                .copied()
                .sorted_unstable_by_key(|m| res.distance_to(*m))
                .next()
                .unwrap(); // Safe
            res -= mirror;
        }
        res
    }

    /// Computes the 6 mirror centers of the origin for hexagonal *wraparound* maps
    /// of given `radius`.
    ///
    /// # Notes
    /// * See [`Self::wrap_with`] for a usage
    /// * Use [`HexMap`] for improved wrapping
    /// * See this [article] for more information.
    ///
    /// [`HexMap`]: crate::HexMap
    /// [article]: https://www.redblobgames.com/grids/hexagons/#wraparound
    #[inline]
    #[must_use]
    #[allow(clippy::cast_possible_wrap)]
    pub const fn wraparound_mirrors(radius: u32) -> [Self; 6] {
        let radius = radius as i32;
        let mirror = Self::new(2 * radius + 1, -radius);
        let [center, left, right] = [mirror, mirror.rotate_left(), mirror.rotate_right()];
        [
            left,
            center,
            right,
            left.const_neg(),   // -left
            center.const_neg(), // -center
            right.const_neg(),  // -right
        ]
    }
}
