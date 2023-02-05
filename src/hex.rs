use crate::Direction;
use glam::{IVec2, IVec3, Vec2};
use itertools::Itertools;
use std::cmp::{max, min};
use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

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
/// or the [`Div<f32>`] and [`Mul<f32>`] for operations using the [`Hex::round`] method.
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
    #[allow(clippy::cast_possible_wrap)]
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

    #[must_use]
    #[allow(clippy::cast_sign_loss)]
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

impl Add<Self> for Hex {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        self.const_add(rhs)
    }
}

impl Add<i32> for Hex {
    type Output = Self;

    #[inline]
    fn add(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl AddAssign for Hex {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl AddAssign<i32> for Hex {
    #[inline]
    fn add_assign(&mut self, rhs: i32) {
        *self = *self + rhs;
    }
}

impl Sub<Self> for Hex {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        self.const_sub(rhs)
    }
}

impl Sub<i32> for Hex {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x - rhs,
            y: self.y - rhs,
        }
    }
}

impl SubAssign for Hex {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl SubAssign<i32> for Hex {
    #[inline]
    fn sub_assign(&mut self, rhs: i32) {
        *self = *self - rhs;
    }
}

impl Mul<Self> for Hex {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl Mul<i32> for Hex {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Mul<f32> for Hex {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        self.rounded_mul(rhs)
    }
}

impl MulAssign for Hex {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl MulAssign<i32> for Hex {
    #[inline]
    fn mul_assign(&mut self, rhs: i32) {
        *self = *self * rhs;
    }
}

impl MulAssign<f32> for Hex {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl Div<Self> for Hex {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl Div<i32> for Hex {
    type Output = Self;

    #[inline]
    fn div(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl Div<f32> for Hex {
    type Output = Self;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        self.rounded_div(rhs)
    }
}

impl DivAssign for Hex {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl DivAssign<i32> for Hex {
    #[inline]
    fn div_assign(&mut self, rhs: i32) {
        *self = *self / rhs;
    }
}

impl DivAssign<f32> for Hex {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}

impl Rem<Self> for Hex {
    type Output = Self;

    #[inline]
    fn rem(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x % rhs.x,
            y: self.y % rhs.y,
        }
    }
}

impl Rem<i32> for Hex {
    type Output = Self;

    #[inline]
    fn rem(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x % rhs,
            y: self.y % rhs,
        }
    }
}

impl RemAssign for Hex {
    #[inline]
    fn rem_assign(&mut self, rhs: Self) {
        *self = *self % rhs;
    }
}

impl RemAssign<i32> for Hex {
    #[inline]
    fn rem_assign(&mut self, rhs: i32) {
        *self = *self % rhs;
    }
}

impl Neg for Hex {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        self.const_neg()
    }
}

impl From<(i32, i32)> for Hex {
    #[inline]
    fn from((x, y): (i32, i32)) -> Self {
        Self { x, y }
    }
}

impl From<[i32; 2]> for Hex {
    #[inline]
    fn from([x, y]: [i32; 2]) -> Self {
        Self { x, y }
    }
}

impl From<(f32, f32)> for Hex {
    #[inline]
    fn from(v: (f32, f32)) -> Self {
        Self::round(v)
    }
}

impl From<[f32; 2]> for Hex {
    #[inline]
    fn from([x, y]: [f32; 2]) -> Self {
        Self::round((x, y))
    }
}

impl From<Hex> for IVec2 {
    #[inline]
    fn from(hex: Hex) -> Self {
        hex.as_ivec2()
    }
}

impl From<Vec2> for Hex {
    #[inline]
    fn from(value: Vec2) -> Self {
        Self::round((value.x, value.y))
    }
}

impl From<Hex> for IVec3 {
    #[inline]
    fn from(hex: Hex) -> Self {
        hex.as_ivec3()
    }
}

impl From<IVec2> for Hex {
    #[inline]
    fn from(v: IVec2) -> Self {
        Self::new(v.x, v.y)
    }
}

#[cfg(test)]
#[allow(clippy::eq_op)]
mod tests {
    use super::*;

    #[test]
    fn hex_addition() {
        assert_eq!(Hex::ZERO + Hex::ZERO, Hex::ZERO);
        assert_eq!(Hex::ZERO + Hex::ONE, Hex::ONE);
        assert_eq!(Hex::ONE + Hex::ONE, Hex::new(2, 2));
        assert_eq!(Hex::ONE + Hex::new(3, 4), Hex::new(4, 5));
    }

    #[test]
    fn int_addition() {
        assert_eq!(Hex::ZERO + 1, Hex::ONE);
        assert_eq!(Hex::ONE + 1, Hex::new(2, 2));
    }

    #[test]
    fn hex_subtraction() {
        assert_eq!(Hex::ZERO - Hex::ZERO, Hex::ZERO);
        assert_eq!(Hex::ONE - Hex::ZERO, Hex::ONE);
        assert_eq!(Hex::ONE - Hex::ONE, Hex::ZERO);
        assert_eq!(Hex::ONE - Hex::new(2, 2), Hex::new(-1, -1));
        assert_eq!(Hex::ONE - Hex::new(4, 5), Hex::new(-3, -4));
    }

    #[test]
    fn int_subtraction() {
        assert_eq!(Hex::ONE - 1, Hex::ZERO);
        assert_eq!(Hex::ONE - 2, Hex::splat(-1));
        assert_eq!(Hex::ZERO - 10, Hex::splat(-10));
    }

    #[test]
    fn hex_multiplication() {
        assert_eq!(Hex::ONE * Hex::ZERO, Hex::ZERO);
        assert_eq!(Hex::ONE * Hex::ONE, Hex::ONE);
        assert_eq!(Hex::ONE * Hex::new(2, 2), Hex::new(2, 2));
        assert_eq!(Hex::ONE * Hex::new(5, 6), Hex::new(5, 6));
        assert_eq!(Hex::new(2, 3) * Hex::new(5, 10), Hex::new(10, 30));
    }

    #[test]
    fn int_multiplication() {
        assert_eq!(Hex::ONE * 5, Hex::splat(5));
    }

    #[test]
    fn hex_division() {
        assert_eq!(Hex::ONE / Hex::ONE, Hex::ONE);
        assert_eq!(Hex::new(2, 2) / Hex::new(2, 2), Hex::ONE);
        assert_eq!(Hex::new(10, 30) / Hex::new(2, 6), Hex::new(5, 5));
        assert_eq!(Hex::new(11, 31) / Hex::new(2, 6), Hex::new(5, 5));
    }

    #[test]
    fn hex_rem() {
        for x in 1..30 {
            for y in 1..30 {
                let hex = Hex::new(x, y);
                for x2 in 1..30 {
                    for y2 in 1..30 {
                        // Int
                        let rhs = x2;
                        let div = hex / rhs;
                        let rem = hex % rhs;
                        assert_eq!(div * rhs + rem, hex);
                        // Hex
                        let rhs = Hex::new(x2, y2);
                        let div = hex / rhs;
                        let rem = hex % rhs;
                        assert_eq!(div * rhs + rem, hex);
                    }
                }
            }
        }
    }

    #[test]
    fn int_division() {
        assert_eq!(Hex::new(2, 2) / 2, Hex::ONE);
        assert_eq!(Hex::new(10, 30) / 2, Hex::new(5, 15));
        assert_eq!(Hex::new(11, 31) / 2, Hex::new(5, 15));
    }

    #[test]
    fn neighbors() {
        assert_eq!(
            Hex::ZERO.all_neighbors(),
            [
                Hex::new(1, -1),
                Hex::new(0, -1),
                Hex::new(-1, 0),
                Hex::new(-1, 1),
                Hex::new(0, 1),
                Hex::new(1, 0),
            ]
        );
        assert_eq!(
            Hex::new(-2, 5).all_neighbors(),
            [
                Hex::new(-1, 4),
                Hex::new(-2, 4),
                Hex::new(-3, 5),
                Hex::new(-3, 6),
                Hex::new(-2, 6),
                Hex::new(-1, 5),
            ]
        );
    }

    #[test]
    fn diagonals() {
        assert_eq!(
            Hex::ZERO.all_diagonals(),
            [
                Hex::new(2, -1),
                Hex::new(1, -2),
                Hex::new(-1, -1),
                Hex::new(-2, 1),
                Hex::new(-1, 2),
                Hex::new(1, 1),
            ]
        );
        assert_eq!(
            Hex::new(-2, 5).all_diagonals(),
            [
                Hex::new(0, 4),
                Hex::new(-1, 3),
                Hex::new(-3, 4),
                Hex::new(-4, 6),
                Hex::new(-3, 7),
                Hex::new(-1, 6),
            ]
        );
    }

    #[test]
    fn distance_to() {
        assert_eq!(Hex::ZERO.distance_to(Hex::ZERO), 0);
        assert_eq!(Hex::ZERO.distance_to(Hex::ONE), 2);
        assert_eq!(Hex::ZERO.distance_to(Hex::new(2, 2)), 4);
        assert_eq!(Hex::ZERO.distance_to(Hex::new(-2, -2)), 4);
        assert_eq!(Hex::new(-2, -2).distance_to(Hex::new(-4, -4)), 4);
    }

    #[test]
    fn rotate_right() {
        let hex = Hex::new(5, 0);
        let new = hex.rotate_right();
        assert_eq!(new, Hex::new(0, 5));
        let new = new.rotate_right();
        assert_eq!(new, Hex::new(-5, 5));
        let new = new.rotate_right();
        assert_eq!(new, Hex::new(-5, 0));
        let new = new.rotate_right();
        assert_eq!(new, Hex::new(0, -5));
        let new = new.rotate_right();
        assert_eq!(new, Hex::new(5, -5));
        let new = new.rotate_right();
        assert_eq!(new, hex);
    }

    #[test]
    fn rotate_left() {
        let hex = Hex::new(5, 0);
        let new = hex.rotate_left();
        assert_eq!(new, Hex::new(5, -5));
        let new = new.rotate_left();
        assert_eq!(new, Hex::new(0, -5));
        let new = new.rotate_left();
        assert_eq!(new, Hex::new(-5, 0));
        let new = new.rotate_left();
        assert_eq!(new, Hex::new(-5, 5));
        let new = new.rotate_left();
        assert_eq!(new, Hex::new(0, 5));
        let new = new.rotate_left();
        assert_eq!(new, hex);
    }

    #[test]
    fn lerp() {
        let a = Hex::new(0, 0);
        let b = Hex::new(5, 0);

        assert_eq!(a.lerp(b, 0.0), a);
        assert_eq!(a.lerp(b, 1.0), b);
        assert_eq!(a.lerp(b, 2.0), b * 2);
        assert_eq!(a.lerp(b, -1.0), -b);
        assert_eq!(a.lerp(b, -2.0), -b * 2);

        let line = [
            a,
            Hex::new(1, 0),
            Hex::new(2, 0),
            Hex::new(3, 0),
            Hex::new(4, 0),
            b,
        ];
        assert_eq!(a.lerp(b, 0.1), line[0]);
        assert_eq!(a.lerp(b, 0.2), line[1]);
        assert_eq!(a.lerp(b, 0.3), line[1]);
        assert_eq!(a.lerp(b, 0.4), line[2]);
        assert_eq!(a.lerp(b, 0.5), line[2]);
        assert_eq!(a.lerp(b, 0.6), line[3]);
        assert_eq!(a.lerp(b, 0.7), line[3]);
        assert_eq!(a.lerp(b, 0.8), line[4]);
        assert_eq!(a.lerp(b, 0.9), line[4]);
        assert_eq!(a.lerp(b, 0.95), line[5]);
        assert_eq!(a.lerp(b, 1.0), line[5]);
    }

    #[test]
    fn rounded_div() {
        let a = Hex::new(0, 0);
        let b = Hex::new(-5, 7);
        assert_eq!(b / 2, Hex::new(-2, 3)); // Naive
        assert_eq!(b / 2.0, Hex::new(-3, 4)); // Rounded
        assert_eq!(b / 2.0, a.lerp(b, 0.5)); // Lerp
    }

    #[test]
    fn line_to() {
        let a = Hex::new(0, 0);
        let b = Hex::new(5, 0);
        assert_eq!(
            a.line_to(b).collect::<Vec<_>>(),
            vec![
                a,
                Hex::new(1, 0),
                Hex::new(2, 0),
                Hex::new(3, 0),
                Hex::new(4, 0),
                b
            ]
        );
        let b = Hex::new(5, 5);
        assert_eq!(
            a.line_to(b).collect::<Vec<_>>(),
            vec![
                a,
                Hex::new(0, 1),
                Hex::new(1, 1),
                Hex::new(1, 2),
                Hex::new(2, 2),
                Hex::new(2, 3),
                Hex::new(3, 3),
                Hex::new(3, 4),
                Hex::new(4, 4),
                Hex::new(4, 5),
                b
            ]
        );
    }

    #[test]
    fn directions_to() {
        let a = Hex::new(0, 0);
        let b = Hex::new(5, 5);
        assert_eq!(
            a.directions_to(b).collect::<Vec<_>>(),
            vec![
                Direction::Bottom,
                Direction::BottomRight,
                Direction::Bottom,
                Direction::BottomRight,
                Direction::Bottom,
                Direction::BottomRight,
                Direction::Bottom,
                Direction::BottomRight,
                Direction::Bottom,
                Direction::BottomRight
            ]
        );
    }

    #[test]
    fn range_count() {
        assert_eq!(Hex::range_count(0), 1);
        assert_eq!(Hex::range_count(1), 7);
        assert_eq!(Hex::range_count(10), 331);
        assert_eq!(Hex::range_count(15), 721);
    }

    #[test]
    fn ring() {
        let hex = Hex::ZERO;
        assert_eq!(hex.ring(0), vec![hex]);
        let expected = hex.all_neighbors().to_vec();
        assert_eq!(hex.ring(1), expected);

        let radius = 5;
        let mut range: Vec<_> = hex.range(radius).collect();
        let removed: Vec<_> = hex.range(radius - 1).collect();
        range.retain(|h| !removed.contains(h));
        let ring = hex.ring(5);
        assert_eq!(ring.len(), range.len());
        for h in &ring {
            assert!(range.contains(h));
        }
    }

    #[test]
    fn ring_offset() {
        let zero = Hex::ZERO;
        let target = Hex::new(14, 7);

        let expected: Vec<_> = zero.ring(10).into_iter().map(|h| h + target).collect();
        assert_eq!(target.ring(10), expected);
    }

    #[test]
    fn custom_ring() {
        let hex = Hex::ZERO;
        assert_eq!(hex.custom_ring(0, Direction::TopLeft, true), vec![hex]);

        // clockwise
        let mut expected = hex.ring(5);
        expected.reverse();
        expected.rotate_right(1);
        assert_eq!(hex.custom_ring(5, Direction::TopRight, true), expected);
        // offsetted
        let expected = hex.ring(5);
        let ring = hex.custom_ring(5, Direction::BottomLeft, false);
        assert_eq!(expected.len(), ring.len());
        for h in &ring {
            assert!(expected.contains(h));
        }
    }

    #[test]
    fn spiral_range() {
        let expected: Vec<_> = Hex::ZERO.range(10).collect();
        let spiral = Hex::ZERO.spiral_range(10);
        assert_eq!(spiral.len(), expected.len());
        for hex in &expected {
            assert!(spiral.contains(hex));
        }
    }
}
