use crate::Direction;
use glam::{IVec2, IVec3, Vec2};
use itertools::Itertools;
use std::cmp::{max, min};
use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// Hexagonal coordinates
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

    /// ```txt
    ///            x Axis
    ///            ___
    ///           /   \
    ///       +--+  2  +--+
    ///      / 3  \___/  1 \
    ///      \    /   \    /
    ///       +--+     +--+
    ///      /    \___/    \
    ///      \ 4  /   \  0 /
    ///       +--+  5  +--+   y Axis
    ///           \___/
    /// ```
    pub const NEIGHBORS_COORDS: [Self; 6] = [
        Self::new(1, 0),
        Self::new(1, -1),
        Self::new(0, -1),
        Self::new(-1, 0),
        Self::new(-1, 1),
        Self::new(0, 1),
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
    /// Computes coordinates length
    pub const fn length(self) -> i32 {
        (self.x.abs() + self.y.abs() + self.z().abs()) / 2
    }

    #[inline]
    #[must_use]
    /// Computes the distance from `self` to `other` in hexagonal space
    pub fn distance_to(self, other: Self) -> i32 {
        (self - other).length()
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
    pub fn neighbor(self, direction: Direction) -> Self {
        self + Self::neighbor_coord(direction)
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
        let (a, b) = (
            Vec2::new(self.x as f32, self.y as f32),
            Vec2::new(other.x as f32, other.y as f32),
        );
        (0..=distance)
            .map(move |step| a.lerp(b, step as f32 / distance as f32))
            .map(|v| Self::round((v.x, v.y)))
    }

    #[allow(clippy::cast_precision_loss)]
    /// Retrieves all [`Hex`] around `self` in a given `range`
    pub fn range(self, range: i32) -> impl Iterator<Item = Self> {
        (-range..=range).flat_map(move |x| {
            (max(-range, -x - range)..=min(range, range - x)).map(move |y| self + Self::new(x, y))
        })
    }

    #[inline]
    #[must_use]
    /// Counts how many coordinates there are in the given `range`
    pub const fn range_count(range: i32) -> i32 {
        3 * range * (range + 1) + 1
    }

    #[must_use]
    #[allow(clippy::cast_sign_loss)]
    // TODO: benchmark this alogrithm vs range - range n-1
    /// Retrieves one [`Hex`] ring around `self` in a given `range`
    pub fn ring(self, range: i32) -> Vec<Self> {
        if range <= 0 {
            return vec![self];
        }
        let mut hex = self + (Self::neighbor_coord(Direction::BottomLeft) * range);
        let mut res = Vec::with_capacity(Self::ring_count(range) as usize);
        for dir in Self::NEIGHBORS_COORDS {
            (0..range).for_each(|_| {
                res.push(hex);
                hex += dir;
            });
        }
        res
    }

    #[inline]
    #[must_use]
    /// Counts how many coordinates there are in a ring at the given `range`
    pub const fn ring_count(range: i32) -> i32 {
        6 * range
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
    pub fn wrap_in_range(self, radius: i32) -> Self {
        self.wrap_with(radius, &Self::wraparound_mirrors(radius))
    }

    pub(crate) fn wrap_with(self, radius: i32, mirrors: &[Self; 6]) -> Self {
        if self.length() <= radius {
            return self;
        }
        let mut res = self;
        while res.length() > radius {
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
    /// * See [`Self::wrap_in_range`] for a usage
    /// * Use [`HexMap`] for improved wrapping
    /// * See this [article] for more information.
    ///
    /// [`HexMap`]: crate::HexMap
    /// [article]: https://www.redblobgames.com/grids/hexagons/#wraparound
    #[inline]
    #[must_use]
    pub const fn wraparound_mirrors(radius: i32) -> [Self; 6] {
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

    fn add(self, rhs: Self) -> Self::Output {
        self.const_add(rhs)
    }
}

impl Add<i32> for Hex {
    type Output = Self;

    fn add(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl AddAssign for Hex {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl AddAssign<i32> for Hex {
    fn add_assign(&mut self, rhs: i32) {
        self.x += rhs;
        self.y += rhs;
    }
}

impl Sub<Self> for Hex {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.const_sub(rhs)
    }
}

impl Sub<i32> for Hex {
    type Output = Self;

    fn sub(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x - rhs,
            y: self.y - rhs,
        }
    }
}

impl SubAssign for Hex {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl SubAssign<i32> for Hex {
    fn sub_assign(&mut self, rhs: i32) {
        self.x -= rhs;
        self.y -= rhs;
    }
}

impl Mul<Self> for Hex {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl Mul<i32> for Hex {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl MulAssign for Hex {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl MulAssign<i32> for Hex {
    fn mul_assign(&mut self, rhs: i32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Div<Self> for Hex {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl Div<i32> for Hex {
    type Output = Self;

    fn div(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl DivAssign for Hex {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl DivAssign<i32> for Hex {
    fn div_assign(&mut self, rhs: i32) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl Neg for Hex {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.const_neg()
    }
}

impl From<(i32, i32)> for Hex {
    fn from((x, y): (i32, i32)) -> Self {
        Self { x, y }
    }
}

impl From<[i32; 2]> for Hex {
    fn from([x, y]: [i32; 2]) -> Self {
        Self { x, y }
    }
}

impl From<Hex> for IVec2 {
    fn from(hex: Hex) -> Self {
        Self::new(hex.x, hex.y)
    }
}

impl From<Hex> for IVec3 {
    fn from(hex: Hex) -> Self {
        Self::new(hex.x, hex.y, hex.z())
    }
}

impl From<IVec2> for Hex {
    fn from(v: IVec2) -> Self {
        Self::new(v.x, v.y)
    }
}

impl Display for Hex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.x, self.y)
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
                Hex::new(1, 0),
                Hex::new(1, -1),
                Hex::new(0, -1),
                Hex::new(-1, 0),
                Hex::new(-1, 1),
                Hex::new(0, 1),
            ]
        );
        assert_eq!(
            Hex::new(-2, 5).all_neighbors(),
            [
                Hex::new(-1, 5),
                Hex::new(-1, 4),
                Hex::new(-2, 4),
                Hex::new(-3, 5),
                Hex::new(-3, 6),
                Hex::new(-2, 6),
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
        let mut expected = hex.all_neighbors().to_vec();
        expected.rotate_left(4);
        assert_eq!(hex.ring(1), expected);
    }
}
