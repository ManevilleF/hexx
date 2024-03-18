use crate::{
    angles::{
        DIRECTION_ANGLE_DEGREES, DIRECTION_ANGLE_OFFSET_DEGREES, DIRECTION_ANGLE_OFFSET_RAD,
        DIRECTION_ANGLE_RAD,
    },
    Hex, HexOrientation, VertexDirection,
};
use std::{f32::consts::TAU, fmt::Debug};

/// All 6 possible neighbor/edge directions in hexagonal space.
///
/// ```txt
///            x Axis
///            ___
///           /   \
///       +--+  1  +--+
///      /  2 \___/ 0  \
///      \    /   \    /
///       +--+     +--+
///      /  3 \___/ 5  \
///      \    /   \    /
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
/// let direction = EdgeDirection::FLAT_TOP;
/// assert_eq!(-direction, EdgeDirection::FLAT_BOTTOM);
/// assert_eq!(direction >> 1, EdgeDirection::FLAT_TOP_RIGHT);
/// assert_eq!(direction << 1, EdgeDirection::FLAT_TOP_LEFT);
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
#[repr(transparent)]
#[doc(alias = "Direction")]
pub struct EdgeDirection(pub(crate) u8);

impl EdgeDirection {
    /// Direction to (1, -1)
    ///
    /// Represents "Top right" edge in flat orientation
    pub const FLAT_TOP_RIGHT: Self = Self(0);
    /// Direction to (1, -1)
    ///
    /// Represents "North west" edge in flat orientation
    pub const FLAT_NORTH_WEST: Self = Self(0);
    /// Direction to (1, -1)
    ///
    /// Represents "Right" edge in pointy orientation
    pub const POINTY_RIGHT: Self = Self(0);
    /// Direction to (1, -1)
    ///
    /// Represents "North west" edge in pointy orientation
    pub const POINTY_WEST: Self = Self(0);

    /// Direction to (0, -1)
    ///
    /// Represents "Top" edge in flat orientation
    pub const FLAT_TOP: Self = Self(1);
    /// Direction to (0, -1)
    ///
    /// Represents "North" edge in flat orientation
    pub const FLAT_NORTH: Self = Self(1);
    /// Direction to (0, -1)
    ///
    /// Represents "Top Right" edge in pointy orientation
    pub const POINTY_TOP_RIGHT: Self = Self(1);
    /// Direction to (0, -1)
    ///
    /// Represents "North West" edge in pointy orientation
    pub const POINTY_NORTH_WEST: Self = Self(1);

    /// Direction to (-1, 0)
    ///
    /// Represents "Top Left" in flat orientation
    pub const FLAT_TOP_LEFT: Self = Self(2);
    /// Direction to (-1, 0)
    ///
    /// Represents "North East" in flat orientation
    pub const FLAT_NORTH_EAST: Self = Self(2);
    /// Direction to (-1, 0)
    ///
    /// Represents "Top Left" in pointy orientation
    pub const POINTY_TOP_LEFT: Self = Self(2);
    /// Direction to (-1, 0)
    ///
    /// Represents "North East" in pointy orientation
    pub const POINTY_NORTH_EAST: Self = Self(2);

    /// Direction to (-1, 1)
    ///
    /// Represents "Bottom Left" in flat orientation
    pub const FLAT_BOTTOM_LEFT: Self = Self(3);
    /// Direction to (-1, 1)
    ///
    /// Represents "South East" in flat orientation
    pub const FLAT_SOUTH_EAST: Self = Self(3);
    /// Direction to (-1, 1)
    ///
    /// Represents "Left" in pointy orientation
    pub const POINTY_LEFT: Self = Self(3);
    /// Direction to (-1, 1)
    ///
    /// Represents "East" in pointy orientation
    pub const POINTY_EAST: Self = Self(3);

    /// Direction to (0, 1)
    ///
    /// Represents "Bottom" in flat orientation
    pub const FLAT_BOTTOM: Self = Self(4);
    /// Direction to (0, 1)
    ///
    /// Represents "South" in flat orientation
    pub const FLAT_SOUTH: Self = Self(4);
    /// Direction to (0, 1)
    ///
    /// Represents "Bottom left" in pointy orientation
    pub const POINTY_BOTTOM_LEFT: Self = Self(4);
    /// Direction to (0, 1)
    ///
    /// Represents "South east" in pointy orientation
    pub const POINTY_SOUTH_EAST: Self = Self(4);

    /// Direction to (1, 0)
    ///
    /// Represents "Bottom Right" in flat orientation
    pub const FLAT_BOTTOM_RIGHT: Self = Self(5);
    /// Direction to (1, 0)
    ///
    /// Represents "South West" in flat orientation
    pub const FLAT_SOUTH_WEST: Self = Self(5);
    /// Direction to (1, 0)
    ///
    /// Represents "Bottom Right" in pointy orientation
    pub const POINTY_BOTTOM_RIGHT: Self = Self(5);
    /// Direction to (1, 0)
    ///
    /// Represents "South West" in pointy orientation
    pub const POINTY_SOUTH_WEST: Self = Self(5);

    /// All 6 hexagonal directions matching
    /// [`Hex::NEIGHBORS_COORDS`](crate::Hex::NEIGHBORS_COORDS)
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
    pub const ALL_DIRECTIONS: [Self; 6] = [Self(0), Self(1), Self(2), Self(3), Self(4), Self(5)];

    /// Iterates through all directions in clockwise order
    #[must_use]
    pub fn iter() -> impl ExactSizeIterator<Item = Self> {
        Self::ALL_DIRECTIONS.into_iter()
    }

    /// Returns the inner index of the edge direction, from 0 to 5
    #[must_use]
    #[inline]
    pub const fn index(self) -> u8 {
        self.0
    }

    /// Converts the direction to a normalized hex coordinate
    #[must_use]
    #[inline]
    pub const fn into_inner(self) -> Hex {
        Hex::NEIGHBORS_COORDS[self.0 as usize]
    }

    /// Computes the opposite direction of `self`
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    /// assert_eq!(
    ///     EdgeDirection::FLAT_TOP.const_neg(),
    ///     EdgeDirection::FLAT_BOTTOM
    /// );
    /// ```
    #[must_use]
    #[inline]
    pub const fn const_neg(self) -> Self {
        Self((self.0 + 3) % 6)
    }

    /// Returns the next direction in clockwise order
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    /// assert_eq!(
    ///     EdgeDirection::FLAT_TOP.clockwise(),
    ///     EdgeDirection::FLAT_TOP_RIGHT
    /// );
    /// ```
    #[must_use]
    #[inline]
    #[doc(alias = "cw")]
    pub const fn clockwise(self) -> Self {
        Self((self.0 + 5) % 6)
    }

    /// Returns the next direction in counter clockwise order
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    /// assert_eq!(
    ///     EdgeDirection::FLAT_TOP.counter_clockwise(),
    ///     EdgeDirection::FLAT_TOP_LEFT
    /// );
    /// ```
    #[must_use]
    #[inline]
    #[doc(alias = "ccw")]
    pub const fn counter_clockwise(self) -> Self {
        Self((self.0 + 1) % 6)
    }

    #[must_use]
    #[inline]
    /// Rotates `self` counter clockwise by `offset` amount.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    /// assert_eq!(
    ///     EdgeDirection::FLAT_TOP,
    ///     EdgeDirection::FLAT_TOP.rotate_ccw(6)
    /// );
    /// ```
    pub const fn rotate_ccw(self, offset: u8) -> Self {
        Self((self.0 + (offset % 6)) % 6)
    }

    #[must_use]
    #[inline]
    /// Rotates `self` clockwise by `offset` amount.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    /// assert_eq!(
    ///     EdgeDirection::FLAT_TOP,
    ///     EdgeDirection::FLAT_TOP.rotate_cw(6)
    /// );
    /// ```
    pub const fn rotate_cw(self, offset: u8) -> Self {
        Self((self.0 + 6 - (offset % 6)) % 6)
    }

    #[must_use]
    #[inline]
    const fn steps_between(self, rhs: Self) -> u8 {
        (self.0 + 6 - rhs.0) % 6
    }

    #[allow(clippy::cast_lossless)]
    #[must_use]
    #[inline]
    /// Computes the angle between `self` and `rhs` in radians.
    pub fn angle_between(self, rhs: Self) -> f32 {
        let steps = self.steps_between(rhs) as f32;
        steps * DIRECTION_ANGLE_RAD
    }

    #[allow(clippy::cast_lossless)]
    #[must_use]
    #[inline]
    /// Computes the angle between `self` and `rhs` in degrees.
    pub fn angle_degrees_between(self, rhs: Self) -> f32 {
        let steps = self.steps_between(rhs) as f32;
        steps * DIRECTION_ANGLE_DEGREES
    }

    #[inline]
    #[must_use]
    /// Returns the angle in radians of the given direction for *flat* hexagons
    ///
    /// See [`Self::angle_pointy`] for *pointy* hexagons
    pub fn angle_flat(self) -> f32 {
        self.angle_pointy() + DIRECTION_ANGLE_OFFSET_RAD
    }

    #[inline]
    #[must_use]
    /// Returns the angle in radians of the given direction for *pointy*
    /// hexagons
    ///
    /// See [`Self::angle_flat`] for *flat* hexagons
    pub fn angle_pointy(self) -> f32 {
        self.angle_between(Self::default())
    }

    #[inline]
    #[must_use]
    /// Returns the angle in radians of the given direction in the given
    /// `orientation`
    pub fn angle(self, orientation: HexOrientation) -> f32 {
        self.angle_pointy() - orientation.angle_offset
    }

    #[inline]
    #[must_use]
    /// Returns the angle in degrees of the given direction for *pointy*
    /// hexagons
    ///
    /// See [`Self::angle_flat`] for *flat* hexagons
    pub fn angle_flat_degrees(self) -> f32 {
        self.angle_pointy_degrees() + DIRECTION_ANGLE_OFFSET_DEGREES
    }

    #[inline]
    #[must_use]
    /// Returns the angle in degrees of the given direction for *pointy*
    /// hexagons
    ///
    /// See [`Self::angle_flat`] for *flat* hexagons
    pub fn angle_pointy_degrees(self) -> f32 {
        self.angle_degrees_between(Self::default())
    }

    #[inline]
    #[must_use]
    /// Returns the angle in degrees of the given direction according to its
    /// `orientation`
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
    /// let direction = EdgeDirection::from_pointy_angle_degrees(35.0);
    /// assert_eq!(direction, EdgeDirection::FLAT_TOP);
    /// ```
    pub fn from_pointy_angle_degrees(angle: f32) -> Self {
        Self::from_flat_angle_degrees(angle + DIRECTION_ANGLE_OFFSET_DEGREES)
    }

    #[must_use]
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    /// Returns the direction from the given `angle` in degrees
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    ///
    /// let direction = EdgeDirection::from_flat_angle_degrees(35.0);
    /// assert_eq!(direction, EdgeDirection::FLAT_TOP_RIGHT);
    /// ```
    pub fn from_flat_angle_degrees(angle: f32) -> Self {
        let angle = angle.rem_euclid(360.0);
        let sector = (angle / DIRECTION_ANGLE_DEGREES).trunc() as u8;
        Self(sector)
    }

    #[must_use]
    /// Returns the direction from the given `angle` in radians
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    ///
    /// let direction = EdgeDirection::from_pointy_angle(0.6);
    /// assert_eq!(direction, EdgeDirection::FLAT_TOP);
    /// ```
    pub fn from_pointy_angle(angle: f32) -> Self {
        Self::from_flat_angle(angle + DIRECTION_ANGLE_OFFSET_RAD)
    }

    #[must_use]
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    /// Returns the direction from the given `angle` in radians
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    ///
    /// let direction = EdgeDirection::from_flat_angle(0.6);
    /// assert_eq!(direction, EdgeDirection::FLAT_TOP_RIGHT);
    /// ```
    pub fn from_flat_angle(angle: f32) -> Self {
        let angle = angle.rem_euclid(TAU);
        let sector = (angle / DIRECTION_ANGLE_RAD) as u8;
        Self(sector)
    }

    #[must_use]
    /// Returns the direction from the given `angle` in degrees according the
    /// `orientation`
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    ///
    /// let angle = 35.0;
    /// assert_eq!(
    ///     EdgeDirection::from_angle_degrees(angle, HexOrientation::Flat),
    ///     EdgeDirection::FLAT_TOP_RIGHT
    /// );
    /// assert_eq!(
    ///     EdgeDirection::from_angle_degrees(angle, HexOrientation::Pointy),
    ///     EdgeDirection::FLAT_TOP
    /// );
    /// ```
    pub fn from_angle_degrees(angle: f32, orientation: HexOrientation) -> Self {
        match orientation {
            HexOrientation::Pointy => Self::from_pointy_angle_degrees(angle),
            HexOrientation::Flat => Self::from_flat_angle_degrees(angle),
        }
    }

    #[must_use]
    /// Returns the direction from the given `angle` in radians according the
    /// `orientation`
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    ///
    /// let angle = 0.6;
    /// assert_eq!(
    ///     EdgeDirection::from_angle(angle, HexOrientation::Flat),
    ///     EdgeDirection::FLAT_TOP_RIGHT
    /// );
    /// assert_eq!(
    ///     EdgeDirection::from_angle(angle, HexOrientation::Pointy),
    ///     EdgeDirection::FLAT_TOP
    /// );
    /// ```
    pub fn from_angle(angle: f32, orientation: HexOrientation) -> Self {
        match orientation {
            HexOrientation::Pointy => Self::from_pointy_angle(angle),
            HexOrientation::Flat => Self::from_flat_angle(angle),
        }
    }

    #[inline]
    #[must_use]
    /// Computes the counter clockwise [`VertexDirection`] neighbor of self.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    /// let diagonal = EdgeDirection::FLAT_TOP.diagonal_ccw();
    /// assert_eq!(diagonal, VertexDirection::FLAT_TOP_LEFT);
    /// ```
    pub const fn diagonal_ccw(self) -> VertexDirection {
        self.vertex_ccw()
    }

    #[inline]
    #[must_use]
    /// Computes the counter clockwise [`VertexDirection`] neighbor of self.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    /// let diagonal = EdgeDirection::FLAT_TOP.vertex_ccw();
    /// assert_eq!(diagonal, VertexDirection::FLAT_TOP_LEFT);
    /// ```
    pub const fn vertex_ccw(self) -> VertexDirection {
        VertexDirection(self.counter_clockwise().0)
    }

    #[inline]
    #[must_use]
    /// Computes the clockwise [`VertexDirection`] neighbor of self.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    /// let diagonal = EdgeDirection::FLAT_TOP.diagonal_cw();
    /// assert_eq!(diagonal, VertexDirection::FLAT_TOP_RIGHT);
    /// ```
    pub const fn diagonal_cw(self) -> VertexDirection {
        self.vertex_cw()
    }

    #[inline]
    #[must_use]
    /// Computes the clockwise [`VertexDirection`] neighbor of self.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    /// let diagonal = EdgeDirection::FLAT_TOP.vertex_cw();
    /// assert_eq!(diagonal, VertexDirection::FLAT_TOP_RIGHT);
    /// ```
    pub const fn vertex_cw(self) -> VertexDirection {
        VertexDirection(self.0)
    }

    #[inline]
    #[must_use]
    /// Computes the two adjacent [`VertexDirection`] in clockwise order
    pub const fn vertex_directions(self) -> [VertexDirection; 2] {
        [self.vertex_ccw(), self.vertex_cw()]
    }
}

impl From<EdgeDirection> for Hex {
    fn from(value: EdgeDirection) -> Self {
        value.into_inner()
    }
}

#[cfg(not(target_arch = "spirv"))]
impl Debug for EdgeDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = self.into_inner();
        f.debug_struct("EdgeDirection")
            .field("index", &self.0)
            .field("x", &c.x)
            .field("y", &c.y)
            .field("z", &c.z())
            .finish()
    }
}
