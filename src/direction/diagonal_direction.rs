#[allow(clippy::wildcard_imports)]
use super::angles::*;
use crate::{Direction, Hex, HexOrientation};

/// All 6 possible diagonal directions in hexagonal space.
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
/// See [`Hex::DIAGONAL_COORDS`](crate::Hex::DIAGONAL_COORDS)
#[repr(u8)]
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "ser_de", derive(serde::Serialize, serde::Deserialize))]
pub enum DiagonalDirection {
    #[default]
    /// Direction to (2, -1)
    ///
    /// Angles:
    ///
    /// |orientation |radians|degrees|
    /// |------------|-------|-------|
    /// | Flat Top   |   0   |   0   |   
    /// | Pointy Top | 11π/6 |  330  |   
    ///
    /// ```txt
    ///            x Axis
    ///           \___/
    ///      \    /   \    /
    ///       +--+     +--+
    ///    __/    \___/    \__
    ///      \    /   \    /
    ///       +--+     +--+  X
    ///    __/    \___/    \__
    ///      \    /   \    /
    ///       +--+     +--+   y Axis
    ///      /    \___/    \
    /// ```
    Right = 0,
    /// Direction to (1, -2)
    ///
    /// Angles:
    ///
    /// |orientation |radians|degrees|
    /// |------------|-------|-------|
    /// | Flat Top   |  π/3  |  60   |   
    /// | Flat Top   |  π/6  |  30   |   
    ///
    /// ```txt
    ///            x Axis
    ///           \___/  X
    ///      \    /   \    /
    ///       +--+     +--+   
    ///    __/    \___/    \__
    ///      \    /   \    /
    ///       +--+     +--+   
    ///    __/    \___/    \__
    ///      \    /   \    /
    ///       +--+     +--+   y Axis
    ///      /    \___/    \
    /// ```
    TopRight = 1,
    /// Direction to (-1, -1)
    ///
    /// Angles:
    ///
    /// |orientation |radians|degrees|
    /// |------------|-------|-------|
    /// | Flat Top   | 2π/3  |  120  |   
    /// | Pointy Top |  π/2  |  90   |   
    ///
    /// ```txt
    ///            x Axis
    ///        X  \___/
    ///      \    /   \    /
    ///       +--+     +--+
    ///    __/    \___/    \__
    ///      \    /   \    /
    ///       +--+     +--+   
    ///    __/    \___/    \__
    ///      \    /   \    /
    ///       +--+     +--+   y Axis
    ///      /    \___/    \
    /// ```
    TopLeft = 2,
    /// Direction to (-2, 1)
    ///
    /// Angles:
    ///
    /// |orientation |radians|degrees|
    /// |------------|-------|-------|
    /// | Flat Top   |   π   |  180  |   
    /// | Pointy Top | 5π/6  |  150  |   
    ///
    /// ```txt
    ///            x Axis
    ///           \___/
    ///      \    /   \    /
    ///       +--+     +--+
    ///    __/    \___/    \__
    ///      \    /   \    /
    ///    X  +--+     +--+   
    ///    __/    \___/    \__
    ///      \    /   \    /
    ///       +--+     +--+   y Axis
    ///      /    \___/    \
    /// ```
    Left = 3,
    /// Direction to (-1, 2)
    ///
    /// Angles:
    ///
    /// |orientation |radians|degrees|
    /// |------------|-------|-------|
    /// | Flat Top   | 4π/3  |  240  |   
    /// | Pointy Top | 7π/6  |  210  |   
    ///
    /// ```txt
    ///            x Axis
    ///           \___/
    ///      \    /   \    /
    ///       +--+     +--+
    ///    __/    \___/    \__
    ///      \    /   \    /
    ///       +--+     +--+   
    ///    __/    \___/    \__
    ///      \    /   \    /
    ///       +--+     +--+   y Axis
    ///      / X  \___/    \
    /// ```
    BottomLeft = 4,
    /// Direction to (1, 1)
    ///
    /// Angles:
    ///
    /// |orientation |radians|degrees|
    /// |------------|-------|-------|
    /// | Flat Top   | 5π/3  |  300  |   
    /// | Pointy Top | 3π/2  |  270  |   
    ///
    /// ```txt
    ///            x Axis
    ///           \___/
    ///      \    /   \    /
    ///       +--+     +--+
    ///    __/    \___/    \__
    ///      \    /   \    /
    ///       +--+     +--+   
    ///    __/    \___/    \__
    ///      \    /   \    /
    ///       +--+     +--+   y Axis
    ///      /    \___/  X \
    /// ```
    BottomRight = 5,
}

impl DiagonalDirection {
    /// All 6 diagonal directions matching [`Hex::DIAGONAL_COORDS`]
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
    pub const ALL_DIRECTIONS: [Self; 6] = [
        Self::Right,
        Self::TopRight,
        Self::TopLeft,
        Self::Left,
        Self::BottomLeft,
        Self::BottomRight,
    ];

    /// Iterates through all enum variant in order
    pub fn iter() -> impl Iterator<Item = Self> {
        Self::ALL_DIRECTIONS.into_iter()
    }

    #[inline]
    #[must_use]
    /// Computes the opposite direction of `self`
    pub const fn const_neg(self) -> Self {
        match self {
            Self::Right => Self::Left,
            Self::TopRight => Self::BottomLeft,
            Self::TopLeft => Self::BottomRight,
            Self::Left => Self::Right,
            Self::BottomLeft => Self::TopRight,
            Self::BottomRight => Self::TopLeft,
        }
    }

    #[inline]
    #[must_use]
    #[doc(alias = "clockwise")]
    /// Returns the next direction in clockwise order
    pub const fn right(self) -> Self {
        match self {
            Self::Right => Self::BottomRight,
            Self::TopRight => Self::Right,
            Self::TopLeft => Self::TopRight,
            Self::Left => Self::TopLeft,
            Self::BottomLeft => Self::Left,
            Self::BottomRight => Self::BottomLeft,
        }
    }

    #[inline]
    #[must_use]
    #[doc(alias = "counterclockwise")]
    /// Returns the next direction in counter clockwise order
    pub const fn left(self) -> Self {
        match self {
            Self::Right => Self::TopRight,
            Self::TopRight => Self::TopLeft,
            Self::TopLeft => Self::Left,
            Self::Left => Self::BottomLeft,
            Self::BottomLeft => Self::BottomRight,
            Self::BottomRight => Self::Right,
        }
    }

    #[inline]
    #[must_use]
    /// Rotates `self` counter clockwise by `offset` amount.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    /// assert_eq!(Direction::Top, Direction::Top.rotate_left(6));
    /// ```
    pub const fn rotate_left(self, offset: usize) -> Self {
        match offset % 6 {
            1 => self.left(),
            2 => self.left().left(),
            3 => self.const_neg(),
            4 => self.right().right(),
            5 => self.right(),
            _ => self,
        }
    }

    #[inline]
    #[must_use]
    /// Rotates `self` clockwise by `offset` amount.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    /// assert_eq!(Direction::Top, Direction::Top.rotate_right(6));
    /// ```
    pub const fn rotate_right(self, offset: usize) -> Self {
        match offset % 6 {
            1 => self.right(),
            2 => self.right().right(),
            3 => self.const_neg(),
            4 => self.left().left(),
            5 => self.left(),
            _ => self,
        }
    }

    const FLAT_ANGLES_DEGREES: [f32; 6] = [
        0.0,
        DIRECTION_ANGLE_DEGREES,
        DIRECTION_ANGLE_DEGREES * 2.0,
        DIRECTION_ANGLE_DEGREES * 3.0,
        DIRECTION_ANGLE_DEGREES * 4.0,
        DIRECTION_ANGLE_DEGREES * 5.0,
    ];

    const FLAT_ANGLES: [f32; 6] = [
        0.0,
        DIRECTION_ANGLE_RAD,
        DIRECTION_ANGLE_RAD * 2.0,
        DIRECTION_ANGLE_RAD * 3.0,
        DIRECTION_ANGLE_RAD * 4.0,
        DIRECTION_ANGLE_RAD * 5.0,
    ];

    #[inline]
    #[must_use]
    /// Returns the angle in radians of the given direction for *flat* hexagons
    ///
    /// See [`Self::angle_pointy`] for *pointy* hexagons
    pub const fn angle_flat(self) -> f32 {
        Self::FLAT_ANGLES[self as usize]
    }

    #[inline]
    #[must_use]
    /// Returns the angle in radians of the given direction for *pointy* hexagons
    ///
    /// See [`Self::angle_flat`] for *flat* hexagons
    pub fn angle_pointy(self) -> f32 {
        self.angle_flat() - DIRECTION_ANGLE_OFFSET
    }

    #[inline]
    #[must_use]
    /// Returns the angle in degrees of the given direction for *pointy* hexagons
    ///
    /// See [`Self::angle_flat`] for *flat* hexagons
    pub const fn angle_flat_degrees(self) -> f32 {
        Self::FLAT_ANGLES_DEGREES[self as usize]
    }

    #[inline]
    #[must_use]
    /// Returns the angle in degrees of the given direction for *pointy* hexagons
    ///
    /// See [`Self::angle_flat`] for *flat* hexagons
    pub fn angle_pointy_degrees(self) -> f32 {
        self.angle_flat_degrees() - DIRECTION_ANGLE_OFFSET_DEGREES
    }

    #[inline]
    #[must_use]
    /// Returns the angle in radians of the given direction in the given `orientation`
    pub fn angle(self, orientation: &HexOrientation) -> f32 {
        self.angle_pointy() - orientation.angle_offset
    }

    #[inline]
    #[must_use]
    /// Computes the counter clockwise [`Direction`] neighbor of `self`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    /// let dir = DiagonalDirection::Right.direction_left();
    /// assert_eq!(dir, Direction::TopRight);
    /// ```
    pub const fn direction_left(self) -> Direction {
        match self {
            Self::Right => Direction::TopRight,
            Self::TopRight => Direction::Top,
            Self::TopLeft => Direction::TopLeft,
            Self::Left => Direction::BottomLeft,
            Self::BottomLeft => Direction::Bottom,
            Self::BottomRight => Direction::BottomRight,
        }
    }

    #[inline]
    #[must_use]
    /// Computes the clockwise [`Direction`] neighbor of `self`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    /// let dir = DiagonalDirection::Right.direction_right();
    /// assert_eq!(dir, Direction::BottomRight);
    /// ```
    pub const fn direction_right(self) -> Direction {
        match self {
            Self::Right => Direction::BottomRight,
            Self::TopRight => Direction::TopRight,
            Self::TopLeft => Direction::Top,
            Self::Left => Direction::TopLeft,
            Self::BottomLeft => Direction::BottomLeft,
            Self::BottomRight => Direction::Bottom,
        }
    }
}

impl From<DiagonalDirection> for Hex {
    fn from(value: DiagonalDirection) -> Self {
        Self::DIAGONAL_COORDS[value as usize]
    }
}
