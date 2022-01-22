use glam::{IVec2, IVec3};
use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

/// Hexagonal coordinates
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Hash)]
pub struct Hex {
    /// `x` axial coordinate (sometimes called `q` or `i`)
    x: i32,
    /// `y` axial coordinate (sometimes called `r` or `j`)
    y: i32,
}

/// All 6 possible directions in hexagonal space
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Direction {
    /// 0, 1
    Top = 0,
    /// 1, 0
    TopLeft = 1,
    /// 1, -1
    BottomLeft = 2,
    /// 0, -1
    Bottom = 3,
    /// -1, 0
    BottomRight = 4,
    /// -1, 1
    TopRight = 5,
}

impl Hex {
    pub const ZERO: Self = Self::new(0, 0);
    pub const ONE: Self = Self::new(1, 1);
    pub const X: Self = Self::new(1, 0);
    pub const Y: Self = Self::new(0, 1);

    pub const NEIGHBORS_COORDS: [Self; 6] = [
        Self::new(0, 1),
        Self::new(1, 0),
        Self::new(1, -1),
        Self::new(0, -1),
        Self::new(-1, 0),
        Self::new(-1, 1),
    ];

    #[inline]
    #[must_use]
    /// Instantiates a new hexagon from axial coordinates
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
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
    /// `x` coordinate (sometimes called `q` or `i`)
    pub const fn x(self) -> i32 {
        self.x
    }

    #[inline]
    #[must_use]
    /// `y` coordinate (sometimes called `r` or `j`)
    pub const fn y(self) -> i32 {
        self.y
    }

    #[inline]
    #[must_use]
    /// `z` coordinate (sometimes called `s` or `k`).
    ///
    /// This axis is computed as `-x - y`
    pub const fn z(self) -> i32 {
        -self.x - self.y
    }

    #[inline]
    #[must_use]
    pub const fn length(self) -> i32 {
        self.x.abs() + self.y.abs() + self.z().abs()
    }

    #[inline]
    #[must_use]
    pub fn distance_to(self, other: Self) -> i32 {
        (self - other).length()
    }

    #[inline]
    #[must_use]
    pub const fn neighbor_coord(direction: Direction) -> Self {
        Self::NEIGHBORS_COORDS[direction as usize]
    }

    #[inline]
    #[must_use]
    pub fn neighbor(self, direction: Direction) -> Self {
        self + Self::neighbor_coord(direction)
    }

    #[inline]
    #[must_use]
    pub fn all_neighbors(self) -> [Self; 6] {
        Self::NEIGHBORS_COORDS.map(|n| self + n)
    }

    #[inline]
    #[must_use]
    pub const fn rotate_left(self) -> Self {
        Self::new(-self.z(), -self.x)
    }

    #[inline]
    #[must_use]
    pub fn rotate_left_around(self, center: Self) -> Self {
        (self - center).rotate_left() + center
    }

    #[inline]
    #[must_use]
    pub const fn rotate_right(self) -> Self {
        Self::new(-self.y, -self.z())
    }

    #[inline]
    #[must_use]
    pub fn rotate_right_around(self, center: Self) -> Self {
        (self - center).rotate_right() + center
    }
}

impl Add<Self> for Hex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
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
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
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

impl Display for Hex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.x, self.y)
    }
}
