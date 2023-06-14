#[allow(clippy::wildcard_imports)]
use super::angles::*;
use crate::{DiagonalDirection, HexOrientation};

/// All 6 possible directions in hexagonal space.
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
///
/// See [`Hex::NEIGHBORS_COORDS`](crate::Hex::NEIGHBORS_COORDS)
///
///
/// ## Operations
///
/// Directions can be:
///  - rotated *clockwise* with:
///     - [`Self::clockwise`] and [`Self::rotate_cw`]
///     - The shift right `>>` operator
///  - rotated *counter clockwise* with:
///     - [`Self::counter_clockwise`] and [`Self::rotate_ccw`]
///     - The shift left `<<` operator
///  - negated using the minus `-` operator
///  - multiplied by an `i32`, returning a [`Hex`](crate::Hex) vector
///
/// Example:
/// ```rust
/// # use hexx::*;
/// let direction = Direction::Top;
/// assert_eq!(-direction, Direction::Bottom);
/// assert_eq!(direction >> 1, Direction::TopRight);
/// assert_eq!(direction << 1, Direction::TopLeft);
/// ```
#[repr(u8)]
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "ser_de", derive(serde::Serialize, serde::Deserialize))]
pub enum Direction {
    #[default]
    /// Direction to (1, -1)
    ///
    /// Angles:
    ///
    /// |orientation |radians|degrees|
    /// |------------|-------|-------|
    /// | Flat Top   | π/6   |  30   |   
    /// | Pointy Top |   0   |   0   |   
    ///
    /// ```txt
    ///            x Axis
    ///            ___
    ///           /   \
    ///       +--+     +--+
    ///      /    \___/  X \
    ///      \    /   \    /
    ///       +--+     +--+
    ///      /    \___/    \
    ///      \    /   \    /
    ///       +--+     +--+   y Axis
    ///           \___/
    /// ```
    #[doc(alias = "NorthEast")]
    TopRight = 0,
    /// Direction to (0, -1)
    ///
    /// Angles:
    ///
    /// |orientation |radians|degrees|
    /// |------------|-------|-------|
    /// | Flat Top   |  π/2  |  90   |   
    /// | Pointy Top |  π/3  |  60   |   
    ///
    /// ```txt
    ///            x Axis
    ///            ___
    ///           /   \
    ///       +--+  X  +--+
    ///      /    \___/    \
    ///      \    /   \    /
    ///       +--+     +--+
    ///      /    \___/    \
    ///      \    /   \    /
    ///       +--+     +--+   y Axis
    ///           \___/
    /// ```
    #[doc(alias = "North")]
    Top = 1,
    /// Direction to (-1, 0)
    ///
    /// Angles:
    ///
    /// |orientation |radians|degrees|
    /// |------------|-------|-------|
    /// | Flat Top   | 5π/6  |  150  |   
    /// | Pointy Top | 2π/3  |  120  |   
    ///
    /// ```txt
    ///            x Axis
    ///            ___
    ///           /   \
    ///       +--+     +--+
    ///      / X  \___/    \
    ///      \    /   \    /
    ///       +--+     +--+
    ///      /    \___/    \
    ///      \    /   \    /
    ///       +--+     +--+   y Axis
    ///           \___/
    /// ```
    #[doc(alias = "NorthWest")]
    TopLeft = 2,
    /// Direction to (-1, 1)
    ///
    /// Angles:
    ///
    /// |orientation |radians|degrees|
    /// |------------|-------|-------|
    /// | Flat Top   | 7π/6  |  210  |   
    /// | Pointy Top |   π   |  180  |   
    ///
    /// ```txt
    ///            x Axis
    ///            ___
    ///           /   \
    ///       +--+     +--+
    ///      /    \___/    \
    ///      \    /   \    /
    ///       +--+     +--+
    ///      /    \___/    \
    ///      \ X  /   \    /
    ///       +--+     +--+   y Axis
    ///           \___/
    /// ```
    #[doc(alias = "SouthWest")]
    BottomLeft = 3,
    /// Direction to (0, 1)
    ///
    /// Angles:
    ///
    /// |orientation |radians|degrees|
    /// |------------|-------|-------|
    /// | Flat Top   | 3π/2  |  270  |   
    /// | Pointy Top | 4π/3  |  240  |   
    ///
    /// ```txt
    ///            x Axis
    ///            ___
    ///           /   \
    ///       +--+     +--+
    ///      /    \___/    \
    ///      \    /   \    /
    ///       +--+     +--+
    ///      /    \___/    \
    ///      \    /   \    /
    ///       +--+  X  +--+   y Axis
    ///           \___/
    /// ```
    #[doc(alias = "South")]
    Bottom = 4,
    /// Drection to (1, 0)
    ///
    /// Angles:
    ///
    /// |orientation |radians|degrees|
    /// |------------|-------|-------|
    /// | Flat Top   | 11π/6 | 330   |
    /// | Pointy Top | 5π/3  | 300   |
    ///
    /// ```txt
    ///            x Axis
    ///            ___
    ///           /   \
    ///       +--+     +--+
    ///      /    \___/    \
    ///      \    /   \    /
    ///       +--+     +--+
    ///      /    \___/    \
    ///      \    /   \  X /
    ///       +--+     +--+   y Axis
    ///           \___/
    /// ```
    #[doc(alias = "SouthEast")]
    BottomRight = 5,
}

impl Direction {
    /// All 6 hexagonal directions matching [`Hex::NEIGHBORS_COORDS`](crate::Hex::NEIGHBORS_COORDS)
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
    pub const ALL_DIRECTIONS: [Self; 6] = [
        Self::TopRight,
        Self::Top,
        Self::TopLeft,
        Self::BottomLeft,
        Self::Bottom,
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
            Self::TopRight => Self::BottomLeft,
            Self::Top => Self::Bottom,
            Self::TopLeft => Self::BottomRight,
            Self::BottomLeft => Self::TopRight,
            Self::Bottom => Self::Top,
            Self::BottomRight => Self::TopLeft,
        }
    }

    #[deprecated(since = "0.6.0", note = "Use Direction::clockwise")]
    #[inline]
    #[must_use]
    /// Returns the next direction in clockwise order
    pub const fn right(self) -> Self {
        self.clockwise()
    }

    #[inline]
    #[must_use]
    #[doc(alias = "cw")]
    /// Returns the next direction in clockwise order
    pub const fn clockwise(self) -> Self {
        match self {
            Self::TopRight => Self::BottomRight,
            Self::Top => Self::TopRight,
            Self::TopLeft => Self::Top,
            Self::BottomLeft => Self::TopLeft,
            Self::Bottom => Self::BottomLeft,
            Self::BottomRight => Self::Bottom,
        }
    }

    #[deprecated(since = "0.6.0", note = "Use Direction::counter_clockwise")]
    #[inline]
    #[must_use]
    /// Returns the next direction in counter clockwise order
    pub const fn left(self) -> Self {
        self.counter_clockwise()
    }

    #[inline]
    #[must_use]
    #[doc(alias = "ccw")]
    /// Returns the next direction in counter clockwise order
    pub const fn counter_clockwise(self) -> Self {
        match self {
            Self::TopRight => Self::Top,
            Self::Top => Self::TopLeft,
            Self::TopLeft => Self::BottomLeft,
            Self::BottomLeft => Self::Bottom,
            Self::Bottom => Self::BottomRight,
            Self::BottomRight => Self::TopRight,
        }
    }

    #[deprecated(since = "0.6.0", note = "Use Direction::rotate_ccw")]
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
        self.rotate_ccw(offset)
    }

    #[inline]
    #[must_use]
    /// Rotates `self` counter clockwise by `offset` amount.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    /// assert_eq!(Direction::Top, Direction::Top.rotate_ccw(6));
    /// ```
    pub const fn rotate_ccw(self, offset: usize) -> Self {
        match offset % 6 {
            1 => self.counter_clockwise(),
            2 => self.counter_clockwise().counter_clockwise(),
            3 => self.const_neg(),
            4 => self.clockwise().clockwise(),
            5 => self.clockwise(),
            _ => self,
        }
    }

    #[deprecated(since = "0.6.0", note = "Use Direction::rotate_cw")]
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
        self.rotate_cw(offset)
    }

    #[inline]
    #[must_use]
    /// Rotates `self` clockwise by `offset` amount.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    /// assert_eq!(Direction::Top, Direction::Top.rotate_cw(6));
    /// ```
    pub const fn rotate_cw(self, offset: usize) -> Self {
        match offset % 6 {
            1 => self.clockwise(),
            2 => self.clockwise().clockwise(),
            3 => self.const_neg(),
            4 => self.counter_clockwise().counter_clockwise(),
            5 => self.counter_clockwise(),
            _ => self,
        }
    }

    const POINTY_ANGLES_DEGREES: [f32; 6] = [
        0.0,
        DIRECTION_ANGLE_DEGREES,
        DIRECTION_ANGLE_DEGREES * 2.0,
        DIRECTION_ANGLE_DEGREES * 3.0,
        DIRECTION_ANGLE_DEGREES * 4.0,
        DIRECTION_ANGLE_DEGREES * 5.0,
    ];

    const POINTY_ANGLES: [f32; 6] = [
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
    pub fn angle_flat(self) -> f32 {
        self.angle_pointy() + DIRECTION_ANGLE_OFFSET
    }

    #[inline]
    #[must_use]
    /// Returns the angle in radians of the given direction for *pointy* hexagons
    ///
    /// See [`Self::angle_flat`] for *flat* hexagons
    pub const fn angle_pointy(self) -> f32 {
        Self::POINTY_ANGLES[self as usize]
    }

    #[inline]
    #[must_use]
    /// Returns the angle in degrees of the given direction for *pointy* hexagons
    ///
    /// See [`Self::angle_flat`] for *flat* hexagons
    pub fn angle_flat_degrees(self) -> f32 {
        self.angle_pointy_degrees() + DIRECTION_ANGLE_OFFSET_DEGREES
    }

    #[inline]
    #[must_use]
    /// Returns the angle in degrees of the given direction for *pointy* hexagons
    ///
    /// See [`Self::angle_flat`] for *flat* hexagons
    pub const fn angle_pointy_degrees(self) -> f32 {
        Self::POINTY_ANGLES_DEGREES[self as usize]
    }

    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    /// Returns the direction from the given `angle` in degrees
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    ///
    /// let direction = Direction::from_flat_angle_degrees(35.0);
    /// assert_eq!(direction, Direction::TopRight);
    /// ```
    pub fn from_flat_angle_degrees(angle: f32) -> Self {
        Self::from_pointy_angle_degrees(angle - DIRECTION_ANGLE_OFFSET_DEGREES)
    }

    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    /// Returns the direction from the given `angle` in degrees
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    ///
    /// let direction = Direction::from_pointy_angle_degrees(35.0);
    /// assert_eq!(direction, Direction::Top);
    /// ```
    pub fn from_pointy_angle_degrees(angle: f32) -> Self {
        let angle = angle % 360.0;
        let angle = if angle < 0.0 { angle + 360.0 } else { angle };
        let half_sector = angle as i32 / 30;
        match half_sector {
            11 | 0 => Self::TopRight,
            1 | 2 => Self::Top,
            3 | 4 => Self::TopLeft,
            5 | 6 => Self::BottomLeft,
            7 | 8 => Self::Bottom,
            _ => Self::BottomRight,
        }
    }

    #[inline]
    #[must_use]
    /// Returns the angle in radians of the given direction in the given `orientation`
    pub fn angle(self, orientation: &HexOrientation) -> f32 {
        self.angle_pointy() - orientation.angle_offset
    }

    #[deprecated(since = "0.6.0", note = "Use Direction::diagonal_ccw")]
    #[inline]
    #[must_use]
    /// Computes the counter clockwise [`DiagonalDirection`] neighbor of self.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    /// let diagonal = Direction::Top.diagonal_left();
    /// assert_eq!(diagonal, DiagonalDirection::TopLeft);
    /// ```
    pub const fn diagonal_left(self) -> DiagonalDirection {
        self.diagonal_ccw()
    }

    #[inline]
    #[must_use]
    /// Computes the counter clockwise [`DiagonalDirection`] neighbor of self.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    /// let diagonal = Direction::Top.diagonal_ccw();
    /// assert_eq!(diagonal, DiagonalDirection::TopLeft);
    /// ```
    pub const fn diagonal_ccw(self) -> DiagonalDirection {
        match self {
            Self::TopRight => DiagonalDirection::TopRight,
            Self::Top => DiagonalDirection::TopLeft,
            Self::TopLeft => DiagonalDirection::Left,
            Self::BottomLeft => DiagonalDirection::BottomLeft,
            Self::Bottom => DiagonalDirection::BottomRight,
            Self::BottomRight => DiagonalDirection::Right,
        }
    }

    #[deprecated(since = "0.6.0", note = "Use Direction::diagonal_cw")]
    #[inline]
    #[must_use]
    /// Computes the clockwise [`DiagonalDirection`] neighbor of self.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    /// let diagonal = Direction::Top.diagonal_right();
    /// assert_eq!(diagonal, DiagonalDirection::TopRight);
    /// ```
    pub const fn diagonal_right(self) -> DiagonalDirection {
        self.diagonal_cw()
    }

    #[inline]
    #[must_use]
    /// Computes the clockwise [`DiagonalDirection`] neighbor of self.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    /// let diagonal = Direction::Top.diagonal_cw();
    /// assert_eq!(diagonal, DiagonalDirection::TopRight);
    /// ```
    pub const fn diagonal_cw(self) -> DiagonalDirection {
        match self {
            Self::TopRight => DiagonalDirection::Right,
            Self::Top => DiagonalDirection::TopRight,
            Self::TopLeft => DiagonalDirection::TopLeft,
            Self::BottomLeft => DiagonalDirection::Left,
            Self::Bottom => DiagonalDirection::BottomLeft,
            Self::BottomRight => DiagonalDirection::BottomRight,
        }
    }
}
