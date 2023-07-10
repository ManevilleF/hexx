#[allow(clippy::wildcard_imports)]
use super::angles::*;
use crate::{Direction, HexOrientation};

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
/// let direction = DiagonalDirection::Right;
/// assert_eq!(-direction, DiagonalDirection::Left);
/// assert_eq!(direction >> 1, DiagonalDirection::BottomRight);
/// assert_eq!(direction << 1, DiagonalDirection::TopRight);
/// ```
#[repr(u8)]
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
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
    #[doc(alias = "East")]
    Right = 0,
    /// Direction to (1, -2)
    ///
    /// Angles:
    ///
    /// |orientation |radians|degrees|
    /// |------------|-------|-------|
    /// | Flat Top   |  π/3  |  60   |   
    /// | Pointy Top |  π/6  |  30   |
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
    #[doc(alias = "NorthEast")]
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
    #[doc(alias = "NorthWest")]
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
    #[doc(alias = "West")]
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
    #[doc(alias = "SouthWest")]
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
    #[doc(alias = "SouthEast")]
    BottomRight = 5,
}

impl DiagonalDirection {
    /// All 6 diagonal directions matching [`Hex::DIAGONAL_COORDS`](crate::Hex::DIAGONAL_COORDS)
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
    #[doc(alias = "cw")]
    /// Returns the next direction in clockwise order
    pub const fn clockwise(self) -> Self {
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
    #[doc(alias = "ccw")]
    /// Returns the next direction in counter clockwise order
    pub const fn counter_clockwise(self) -> Self {
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

    #[inline]
    #[must_use]
    /// Rotates `self` clockwise by `offset` amount.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    /// assert_eq!(Direction::Top, Direction::Top.rotate_ccw(6));
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
        self.angle_flat() - DIRECTION_ANGLE_OFFSET_RAD
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
    pub fn angle(self, orientation: HexOrientation) -> f32 {
        self.angle_pointy() - orientation.angle_offset
    }

    #[inline]
    #[must_use]
    /// Returns the angle in degrees of the given direction according to its `orientation`
    ///
    /// See [`Self::angle`] for radians angles
    pub fn angle_degrees(self, orientation: HexOrientation) -> f32 {
        match orientation {
            HexOrientation::Pointy => self.angle_pointy_degrees(),
            HexOrientation::Flat => self.angle_flat_degrees(),
        }
    }

    #[must_use]
    /// Returns the direction from the given `angle` in degrees
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    ///
    /// let direction = DiagonalDirection::from_flat_angle_degrees(15.0);
    /// assert_eq!(direction, DiagonalDirection::Right);
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
    /// let direction = DiagonalDirection::from_pointy_angle_degrees(15.0);
    /// assert_eq!(direction, DiagonalDirection::TopRight);
    /// ```
    pub fn from_pointy_angle_degrees(angle: f32) -> Self {
        let angle = angle.rem_euclid(360.0);
        let sector = (angle / DIRECTION_ANGLE_DEGREES).trunc() as i32;
        println!("{angle} - {sector}");
        match sector {
            0 => Self::TopRight,
            1 => Self::TopLeft,
            2 => Self::Left,
            3 => Self::BottomLeft,
            4 => Self::BottomRight,
            _ => Self::Right,
        }
    }

    #[must_use]
    /// Returns the direction from the given `angle` in radians
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    ///
    /// let direction = DiagonalDirection::from_flat_angle(0.26);
    /// assert_eq!(direction, DiagonalDirection::Right);
    /// ```
    pub fn from_flat_angle(angle: f32) -> Self {
        Self::from_pointy_angle(angle - DIRECTION_ANGLE_OFFSET_RAD)
    }

    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    /// Returns the direction from the given `angle` in radians
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    ///
    /// let direction = DiagonalDirection::from_pointy_angle(0.26);
    /// assert_eq!(direction, DiagonalDirection::TopRight);
    /// ```
    pub fn from_pointy_angle(angle: f32) -> Self {
        let angle = angle.rem_euclid(PI_2);
        let sector = (angle / DIRECTION_ANGLE_RAD) as i32;
        println!("{angle} - {sector}");
        match sector {
            0 => Self::TopRight,
            1 => Self::TopLeft,
            2 => Self::Left,
            3 => Self::BottomLeft,
            4 => Self::BottomRight,
            _ => Self::Right,
        }
    }

    #[must_use]
    /// Returns the direction from the given `angle` in degrees according the `orientation`
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    ///
    /// let angle = 15.0;
    /// assert_eq!(DiagonalDirection::from_angle_degrees(angle, HexOrientation::Flat), DiagonalDirection::Right);
    /// assert_eq!(DiagonalDirection::from_angle_degrees(angle, HexOrientation::Pointy), DiagonalDirection::TopRight);
    /// ```
    pub fn from_angle_degrees(angle: f32, orientation: HexOrientation) -> Self {
        match orientation {
            HexOrientation::Pointy => Self::from_pointy_angle_degrees(angle),
            HexOrientation::Flat => Self::from_flat_angle_degrees(angle),
        }
    }

    #[must_use]
    /// Returns the direction from the given `angle` in radians according the `orientation`
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    ///
    /// let angle = 0.26;
    /// assert_eq!(DiagonalDirection::from_angle(angle, HexOrientation::Flat), DiagonalDirection::Right);
    /// assert_eq!(DiagonalDirection::from_angle(angle, HexOrientation::Pointy), DiagonalDirection::TopRight);
    /// ```
    pub fn from_angle(angle: f32, orientation: HexOrientation) -> Self {
        match orientation {
            HexOrientation::Pointy => Self::from_pointy_angle(angle),
            HexOrientation::Flat => Self::from_flat_angle(angle),
        }
    }

    #[inline]
    #[must_use]
    /// Computes the counter clockwise [`Direction`] neighbor of `self`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    /// let dir = DiagonalDirection::Right.direction_ccw();
    /// assert_eq!(dir, Direction::TopRight);
    /// ```
    pub const fn direction_ccw(self) -> Direction {
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
    /// let dir = DiagonalDirection::Right.direction_cw();
    /// assert_eq!(dir, Direction::BottomRight);
    /// ```
    pub const fn direction_cw(self) -> Direction {
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
