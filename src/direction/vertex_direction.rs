use crate::{
    angles::{DIRECTION_ANGLE_DEGREES, DIRECTION_ANGLE_RAD},
    EdgeDirection, Hex,
};
use std::fmt::Debug;

/// All 6 possible diagonal/vertex directions in hexagonal space.
///
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
/// let direction = VertexDirection::FLAT_RIGHT;
/// assert_eq!(-direction, VertexDirection::FLAT_LEFT);
/// assert_eq!(direction >> 1, VertexDirection::FLAT_BOTTOM_RIGHT);
/// assert_eq!(direction << 1, VertexDirection::FLAT_TOP_RIGHT);
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
#[repr(transparent)]
#[doc(alias = "DiagonalDirection")]
pub struct VertexDirection(pub(crate) u8);

impl VertexDirection {
    /// Direction to (2, -1) or (2, -1, -1)
    ///
    /// Represents "Right" in flat orientation
    pub const FLAT_RIGHT: Self = Self(0);
    /// Direction to (2, -1) or (2, -1, -1)
    ///
    /// Represents "West" in flat orientation
    pub const FLAT_WEST: Self = Self(0);
    /// Direction to (2, -1) or (2, -1, -1)
    ///
    /// Represents "Bottom right" in pointy orientation
    pub const POINTY_BOTTOM_RIGHT: Self = Self(0);
    /// Direction to (2, -1) or (2, -1, -1)
    ///
    /// Represents "South West" in pointy orientation
    pub const POINTY_SOUTH_WEST: Self = Self(0);

    /// Direction to (1, -2) or (1, -2, 1)
    ///
    /// Represents "Top Right" in flat orientation
    pub const FLAT_TOP_RIGHT: Self = Self(1);
    /// Direction to (1, -2) or (1, -2, 1)
    ///
    /// Represents "North West" in flat orientation
    pub const FLAT_NORTH_WEST: Self = Self(1);
    /// Direction to (1, -2) or (1, -2, 1)
    ///
    /// Represents "Top Right" in pointy orientation
    pub const POINTY_TOP_RIGHT: Self = Self(1);
    /// Direction to (1, -2) or (1, -2, 1)
    ///
    /// Represents "North West" in pointy orientation
    pub const POINTY_NORTH_WEST: Self = Self(1);

    /// Direction to (-1, -1) or (-1, -1, 2)
    ///
    /// Represents "Top Left" in flat orientation
    pub const FLAT_TOP_LEFT: Self = Self(2);
    /// Direction to (-1, -1) or (-1, -1, 2)
    ///
    /// Represents "North east" in flat orientation
    pub const FLAT_NORTH_EAST: Self = Self(2);
    /// Direction to (-1, -1) or (-1, -1, 2)
    ///
    /// Represents "Top" in pointy orientation
    pub const POINTY_TOP: Self = Self(2);
    /// Direction to (-1, -1) or (-1, -1, 2)
    ///
    /// Represents "North" in pointy orientation
    pub const POINTY_NORTH: Self = Self(2);

    /// Direction to (-2, 1) or (-2, 1, 1)
    ///
    /// Represents "Left" in flat orientation
    pub const FLAT_LEFT: Self = Self(3);
    /// Direction to (-2, 1) or (-2, 1, 1)
    ///
    /// Represents "East" in flat orientation
    pub const FLAT_EAST: Self = Self(3);
    /// Direction to (-2, 1) or (-2, 1, 1)
    ///
    /// Represents "Top Left" in pointy orientation
    pub const POINTY_TOP_LEFT: Self = Self(3);
    /// Direction to (-2, 1) or (-2, 1, 1)
    ///
    /// Represents "North East" in pointy orientation
    pub const POINTY_NORTH_EAST: Self = Self(3);

    /// Direction to (-1, 2) or (-1, 2, -1)
    ///
    /// Represents "Bottom Left" in flat orientation
    pub const FLAT_BOTTOM_LEFT: Self = Self(4);
    /// Direction to (-1, 2) or (-1, 2, -1)
    ///
    /// Represents "South East" in flat orientation
    pub const FLAT_SOUTH_EAST: Self = Self(4);
    /// Direction to (-1, 2) or (-1, 2, -1)
    ///
    /// Represents "Bottom Left" in pointy orientation
    pub const POINTY_BOTTOM_LEFT: Self = Self(4);
    /// Direction to (-1, 2) or (-1, 2, -1)
    ///
    /// Represents "South Easth" in pointy orientation
    pub const POINTY_SOUTH_EAST: Self = Self(4);

    /// Direction to (1, 1) or (1, 1, -2)
    ///
    /// Represents "Bottom Right" in flat orientation
    pub const FLAT_BOTTOM_RIGHT: Self = Self(5);
    /// Direction to (1, 1) or (1, 1, -2)
    ///
    /// Represents "South West" in flat orientation
    pub const FLAT_SOUTH_WEST: Self = Self(5);
    /// Direction to (1, 1) or (1, 1, -2)
    ///
    /// Represents "Bottom" in pointy orientation
    pub const POINTY_BOTTOM: Self = Self(5);
    /// Direction to (1, 1) or (1, 1, -2)
    ///
    /// Represents "South" in pointy orientation
    pub const POINTY_SOUTH: Self = Self(5);

    /// All 6 diagonal directions matching
    /// [`Hex::DIAGONAL_COORDS`](crate::Hex::DIAGONAL_COORDS)
    ///
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

    /// Converts the direction to a hex coordinate
    #[must_use]
    #[inline]
    pub const fn into_inner(self) -> Hex {
        Hex::DIAGONAL_COORDS[self.0 as usize]
    }

    /// Computes the opposite direction of `self`
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    /// assert_eq!(EdgeDirection::FLAT_TOP.const_neg(), Direction::FLAT_BOTTOM);
    /// ```
    #[must_use]
    #[inline]
    pub const fn const_neg(self) -> Self {
        Self((6 - self.0) % 6)
    }

    /// Returns the next direction in clockwise order
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    /// assert_eq!(EdgeDirection::FLAT_TOP.clockwise(), Direction::FLAT_TOP_RIGHT);
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
    /// assert_eq!(EdgeDirection::FLAT_TOP.counter_clockwise(), Direction::FLAT_TOP_LEFT);
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
    /// assert_eq!(EdgeDirection::FLAT_TOP, Direction::FLAT_TOP.rotate_ccw(6));
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
    /// assert_eq!(EdgeDirection::FLAT_TOP, Direction::FLAT_TOP.rotate_cw(6));
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
    /// Computes the counter clockwise [`EdgeDirection`] neighbor of self.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    /// let diagonal = VertexDirection::FLAT_RIGHT.edge_ccw();
    /// assert_eq!(diagonal, EdgeDirection::FLAT_TOP_RIGHT);
    /// ```
    pub const fn direction_ccw(self) -> EdgeDirection {
        self.edge_ccw()
    }

    #[inline]
    #[must_use]
    /// Computes the counter clockwise [`EdgeDirection`] neighbor of self.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    /// let diagonal = VertexDirection::FLAT_RIGHT.edge_ccw();
    /// assert_eq!(diagonal, EdgeDirection::FLAT_TOP_RIGHT);
    /// ```
    pub const fn edge_ccw(self) -> EdgeDirection {
        EdgeDirection(self.0)
    }

    #[inline]
    #[must_use]
    /// Computes the clockwise [`EdgeDirection`] neighbor of self.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    /// let diagonal = VertexDirection::FLAT_RIGHT.diagonal_cw();
    /// assert_eq!(diagonal, EdgeDirection::FLAT_BOTTOM_RIGHT);
    /// ```
    pub const fn direction_cw(self) -> EdgeDirection {
        self.edge_cw()
    }

    #[inline]
    #[must_use]
    /// Computes the clockwise [`EdgeDirection`] neighbor of self.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    /// let diagonal = VertexDirection::FLAT_RIGHT.diagonal_cw();
    /// assert_eq!(diagonal, EdgeDirection::FLAT_BOTTOM_RIGHT);
    // ```
    pub const fn edge_cw(self) -> EdgeDirection {
        EdgeDirection(self.clockwise().0)
    }

    #[inline]
    #[must_use]
    /// Computes the two adjacent [`EdgeDirection`] in clockwise order
    pub const fn edge_directions(self) -> [EdgeDirection; 2] {
        [self.edge_ccw(), self.edge_cw()]
    }
}

impl From<VertexDirection> for Hex {
    fn from(value: VertexDirection) -> Self {
        value.into_inner()
    }
}

impl Debug for VertexDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = self.into_inner();
        f.debug_struct("VertexDirection")
            .field("index", &self.0)
            .field("x", &c.x)
            .field("y", &c.y)
            .finish()
    }
}
