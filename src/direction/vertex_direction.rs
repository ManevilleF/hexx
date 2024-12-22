use crate::{
    angles::{
        DIRECTION_ANGLE_DEGREES, DIRECTION_ANGLE_OFFSET_DEGREES, DIRECTION_ANGLE_OFFSET_RAD,
        DIRECTION_ANGLE_RAD,
    },
    EdgeDirection, Hex, HexLayout, HexOrientation,
};
use glam::Vec2;
use std::{f32::consts::TAU, fmt::Debug};

/// All 6 possible diagonal/vertex directions in hexagonal space.
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
///
/// ## Storage
///
/// Both [`EdgeDirection`] and [`VertexDirection`] store a u8 byte between 0 and
/// 5 as following:
///
/// ```txt
///           e4
///       v4_____ v5
///     e3 /     \ e5
///       /       \
///   v3 (         ) v0
///       \       /
///     e2 \_____/ e0
///      v2   e1  v1
/// ```
///
/// On pointy orientation the hexagon is shifted by 30 degrees clockwise
#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(not(target_arch = "spirv"), derive(Hash))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
#[repr(transparent)]
#[doc(alias = "DiagonalDirection")]
pub struct VertexDirection(pub(crate) u8);

impl VertexDirection {
    /// Direction towards `X, -Y, -Z`
    pub const X_NEG_Y_NEG_Z: Self = Self(0);
    /// Direction to (2, -1) or (2, -1, -1)
    pub const X: Self = Self(0);
    /// Direction to (2, -1) or (2, -1, -1)
    ///
    /// Represents "Right" in flat orientation
    pub const FLAT_RIGHT: Self = Self(0);
    /// Direction to (2, -1) or (2, -1, -1)
    ///
    /// Represents "East" in flat orientation
    pub const FLAT_EAST: Self = Self(0);
    /// Direction to (2, -1) or (2, -1, -1)
    ///
    /// Represents "Top right" in pointy orientation
    pub const POINTY_TOP_RIGHT: Self = Self(0);
    /// Direction to (2, -1) or (2, -1, -1)
    ///
    /// Represents "North East" in pointy orientation
    pub const POINTY_NORTH_EAST: Self = Self(0);

    /// Direction towards `X, -Y, Z`
    pub const X_NEG_Y_Z: Self = Self(5);
    /// Direction to (1, -2) or (1, -2, 1)
    pub const NEG_Y: Self = Self(5);
    /// Direction to (1, -2) or (1, -2, 1)
    ///
    /// Represents "Top Right" in flat orientation
    pub const FLAT_TOP_RIGHT: Self = Self(5);
    /// Direction to (1, -2) or (1, -2, 1)
    ///
    /// Represents "North East" in flat orientation
    pub const FLAT_NORTH_EAST: Self = Self(5);
    /// Direction to (1, -2) or (1, -2, 1)
    ///
    /// Represents "Top" in pointy orientation
    pub const POINTY_TOP: Self = Self(5);
    /// Direction to (1, -2) or (1, -2, 1)
    ///
    /// Represents "North" in pointy orientation
    pub const POINTY_NORTH: Self = Self(5);

    /// Direction towards `-X, -Y, Z`
    pub const NEG_X_NEG_Y: Self = Self(4);
    /// Direction to (-1, -1) or (-1, -1, 2)
    pub const Z: Self = Self(4);
    /// Direction to (-1, -1) or (-1, -1, 2)
    ///
    /// Represents "Top Left" in flat orientation
    pub const FLAT_TOP_LEFT: Self = Self(4);
    /// Direction to (-1, -1) or (-1, -1, 2)
    ///
    /// Represents "North West" in flat orientation
    pub const FLAT_NORTH_WEST: Self = Self(4);
    /// Direction to (-1, -1) or (-1, -1, 2)
    ///
    /// Represents "Top Left" in pointy orientation
    pub const POINTY_TOP_LEFT: Self = Self(4);
    /// Direction to (-1, -1) or (-1, -1, 2)
    ///
    /// Represents "North West" in pointy orientation
    pub const POINTY_NORTH_WEST: Self = Self(4);

    /// Direction towards `-X, Y, Z`
    pub const NEG_X_Y_Z: Self = Self(3);
    /// Direction to (-2, 1) or (-2, 1, 1)
    pub const NEG_X: Self = Self(3);
    /// Direction to (-2, 1) or (-2, 1, 1)
    ///
    /// Represents "Left" in flat orientation
    pub const FLAT_LEFT: Self = Self(3);
    /// Direction to (-2, 1) or (-2, 1, 1)
    ///
    /// Represents "West" in flat orientation
    pub const FLAT_WEST: Self = Self(3);
    /// Direction to (-2, 1) or (-2, 1, 1)
    ///
    /// Represents "Bottom Left" in pointy orientation
    pub const POINTY_BOTTOM_LEFT: Self = Self(3);
    /// Direction to (-2, 1) or (-2, 1, 1)
    ///
    /// Represents "South West" in pointy orientation
    pub const POINTY_SOUTH_WEST: Self = Self(3);

    /// Direction towards `-X, Y, -Z`
    pub const NEG_X_Y_NEG_Z: Self = Self(2);
    /// Direction to (-1, 2) or (-1, 2, -1)
    pub const Y: Self = Self(2);
    /// Direction to (-1, 2) or (-1, 2, -1)
    ///
    /// Represents "Bottom Left" in flat orientation
    pub const FLAT_BOTTOM_LEFT: Self = Self(2);
    /// Direction to (-1, 2) or (-1, 2, -1)
    ///
    /// Represents "South West" in flat orientation
    pub const FLAT_SOUTH_WEST: Self = Self(2);
    /// Direction to (-1, 2) or (-1, 2, -1)
    ///
    /// Represents "Bottom " in pointy orientation
    pub const POINTY_BOTTOM: Self = Self(2);
    /// Direction to (-1, 2) or (-1, 2, -1)
    ///
    /// Represents "South" in pointy orientation
    pub const POINTY_SOUTH: Self = Self(2);

    /// Direction towards `X, Y, -Z`
    pub const X_Y: Self = Self(1);
    /// Direction to (1, 1) or (1, 1, -2)
    pub const NEG_Z: Self = Self(1);
    /// Direction to (1, 1) or (1, 1, -2)
    ///
    /// Represents "Bottom Right" in flat orientation
    pub const FLAT_BOTTOM_RIGHT: Self = Self(1);
    /// Direction to (1, 1) or (1, 1, -2)
    ///
    /// Represents "South East" in flat orientation
    pub const FLAT_SOUTH_EAST: Self = Self(1);
    /// Direction to (1, 1) or (1, 1, -2)
    ///
    /// Represents "Bottom Right" in pointy orientation
    pub const POINTY_BOTTOM_RIGHT: Self = Self(1);
    /// Direction to (1, 1) or (1, 1, -2)
    ///
    /// Represents "South East" in pointy orientation
    pub const POINTY_SOUTH_EAST: Self = Self(1);

    /// All 6 diagonal directions matching
    /// [`Hex::DIAGONAL_COORDS`](crate::Hex::DIAGONAL_COORDS)
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
    pub const fn into_hex(self) -> Hex {
        Hex::DIAGONAL_COORDS[self.0 as usize]
    }

    /// Computes the opposite direction of `self`
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    /// assert_eq!(VertexDirection::X.const_neg(), VertexDirection::NEG_X);
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
    /// assert_eq!(VertexDirection::X.clockwise(), VertexDirection::NEG_Z);
    /// ```
    #[must_use]
    #[inline]
    #[doc(alias = "cw")]
    pub const fn clockwise(self) -> Self {
        Self((self.0 + 1) % 6)
    }

    /// Returns the next direction in counter clockwise order
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    /// assert_eq!(
    ///     VertexDirection::X.counter_clockwise(),
    ///     VertexDirection::NEG_Y
    /// );
    /// ```
    #[must_use]
    #[inline]
    #[doc(alias = "ccw")]
    pub const fn counter_clockwise(self) -> Self {
        Self((self.0 + 5) % 6)
    }

    #[must_use]
    #[inline]
    /// Rotates `self` counter clockwise by `offset` amount.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    /// assert_eq!(VertexDirection::X, VertexDirection::X.rotate_ccw(6));
    /// ```
    pub const fn rotate_ccw(self, offset: u8) -> Self {
        Self((self.0 + 6 - (offset % 6)) % 6)
    }

    #[must_use]
    #[inline]
    /// Rotates `self` clockwise by `offset` amount.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    /// assert_eq!(VertexDirection::X, VertexDirection::X.rotate_cw(6));
    /// ```
    pub const fn rotate_cw(self, offset: u8) -> Self {
        Self((self.0 + (offset % 6)) % 6)
    }

    #[must_use]
    #[inline]
    const fn steps_between(self, rhs: Self) -> u8 {
        (self.0 + 6 - rhs.0) % 6
    }

    /// Computes the angle between `a` and `b` in radians.
    #[must_use]
    #[inline]
    pub fn angle_between(a: Self, b: Self) -> f32 {
        a.angle_to(b)
    }

    /// Computes the angle between `a` and `b` in degrees.
    #[must_use]
    #[inline]
    pub fn angle_degrees_between(a: Self, b: Self) -> f32 {
        a.angle_degrees_to(b)
    }

    #[allow(clippy::cast_lossless)]
    #[must_use]
    #[inline]
    /// Computes the angle between `self` and `rhs` in radians.
    pub fn angle_to(self, rhs: Self) -> f32 {
        let steps = self.steps_between(rhs) as f32;
        steps * DIRECTION_ANGLE_RAD
    }

    #[allow(clippy::cast_lossless)]
    #[must_use]
    #[inline]
    /// Computes the angle between `self` and `rhs` in degrees.
    pub fn angle_degrees_to(self, rhs: Self) -> f32 {
        let steps = self.steps_between(rhs) as f32;
        steps * DIRECTION_ANGLE_DEGREES
    }

    #[inline]
    #[must_use]
    /// Returns the angle in radians of the given direction for *flat* hexagons
    ///
    /// See [`Self::angle_pointy`] for *pointy* hexagons
    pub fn angle_flat(self) -> f32 {
        self.angle(HexOrientation::Flat)
    }

    #[inline]
    #[must_use]
    /// Returns the angle in radians of the given direction for *pointy*
    /// hexagons
    ///
    /// See [`Self::angle_flat`] for *flat* hexagons
    pub fn angle_pointy(self) -> f32 {
        self.angle(HexOrientation::Pointy)
    }

    #[inline]
    #[must_use]
    /// Returns the angle in radians of the given direction in the given
    /// `orientation`
    pub fn angle(self, orientation: HexOrientation) -> f32 {
        let base = self.angle_to(Self(0));
        match orientation {
            HexOrientation::Pointy => (base - DIRECTION_ANGLE_OFFSET_RAD).rem_euclid(TAU),
            HexOrientation::Flat => base,
        }
    }

    #[inline]
    #[must_use]
    /// Returns the unit vector of the direction in the given `orientation`
    ///
    /// The vector is normalized and in local hex space. To use within a
    /// [`HexLayout`] use [`HexLayout::transform_vector`] or [`world_unit_vector`]
    pub fn unit_vector(self, orientation: HexOrientation) -> Vec2 {
        let angle = self.angle(orientation);
        Vec2::new(angle.cos(), angle.sin())
    }

    #[inline]
    #[must_use]
    /// Returns the unit vector of the direction in the given `layout`.
    ///
    /// The vector is provided in pixel/workd space. To use in local hex
    /// space use [`unit_vector`]
    pub fn world_unit_vector(self, layout: &HexLayout) -> Vec2 {
        let vector = self.unit_vector(layout.orientation);
        layout.transform_vector(vector)
    }

    #[inline]
    #[must_use]
    /// Returns the angle in degrees of the given direction for *pointy*
    /// hexagons
    ///
    /// See [`Self::angle_pointy_degrees`] for *flat* hexagons
    pub fn angle_flat_degrees(self) -> f32 {
        self.angle_degrees(HexOrientation::Flat)
    }

    #[inline]
    #[must_use]
    /// Returns the angle in degrees of the given direction for *pointy*
    /// hexagons
    ///
    /// See [`Self::angle_flat_degrees`] for *flat* hexagons
    pub fn angle_pointy_degrees(self) -> f32 {
        self.angle_degrees(HexOrientation::Pointy)
    }

    #[inline]
    #[must_use]
    /// Returns the angle in degrees of the given direction according to its
    /// `orientation`
    ///
    /// See [`Self::angle`] for radians angles
    pub fn angle_degrees(self, orientation: HexOrientation) -> f32 {
        let base = self.angle_degrees_to(Self(0));
        match orientation {
            HexOrientation::Pointy => (base - DIRECTION_ANGLE_OFFSET_DEGREES).rem_euclid(360.0),
            HexOrientation::Flat => base,
        }
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
    /// let direction = VertexDirection::from_flat_angle_degrees(15.0);
    /// assert_eq!(direction, VertexDirection::X);
    /// ```
    pub fn from_flat_angle_degrees(angle: f32) -> Self {
        Self::from_pointy_angle_degrees(angle - DIRECTION_ANGLE_OFFSET_DEGREES)
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
    /// let direction = VertexDirection::from_pointy_angle_degrees(15.0);
    /// assert_eq!(direction, VertexDirection::FLAT_BOTTOM_RIGHT);
    /// ```
    pub fn from_pointy_angle_degrees(angle: f32) -> Self {
        let angle = angle.rem_euclid(360.0);
        let sector = (angle / DIRECTION_ANGLE_DEGREES).trunc() as u8;
        Self((sector + 1) % 6)
    }

    #[must_use]
    /// Returns the direction from the given `angle` in radians
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    ///
    /// let direction = VertexDirection::from_flat_angle(0.26);
    /// assert_eq!(direction, VertexDirection::FLAT_RIGHT);
    /// ```
    pub fn from_flat_angle(angle: f32) -> Self {
        Self::from_pointy_angle(angle - DIRECTION_ANGLE_OFFSET_RAD)
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
    /// let direction = VertexDirection::from_pointy_angle(0.26);
    /// assert_eq!(direction, VertexDirection::FLAT_BOTTOM_RIGHT);
    /// ```
    pub fn from_pointy_angle(angle: f32) -> Self {
        let angle = angle.rem_euclid(TAU);
        let sector = (angle / DIRECTION_ANGLE_RAD) as u8;
        Self((sector + 1) % 6)
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
    /// let angle = 15.0;
    /// assert_eq!(
    ///     VertexDirection::from_angle_degrees(angle, HexOrientation::Flat),
    ///     VertexDirection::FLAT_RIGHT
    /// );
    /// assert_eq!(
    ///     VertexDirection::from_angle_degrees(angle, HexOrientation::Pointy),
    ///     VertexDirection::FLAT_BOTTOM_RIGHT
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
    /// let angle = 0.26;
    /// assert_eq!(
    ///     VertexDirection::from_angle(angle, HexOrientation::Flat),
    ///     VertexDirection::FLAT_RIGHT
    /// );
    /// assert_eq!(
    ///     VertexDirection::from_angle(angle, HexOrientation::Pointy),
    ///     VertexDirection::FLAT_BOTTOM_RIGHT
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
    /// Computes the counter clockwise [`EdgeDirection`] neighbor of self.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    /// let diagonal = VertexDirection::FLAT_RIGHT.direction_ccw();
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
        EdgeDirection(self.counter_clockwise().0)
    }

    #[inline]
    #[must_use]
    /// Computes the clockwise [`EdgeDirection`] neighbor of self.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    /// let diagonal = VertexDirection::FLAT_RIGHT.direction_cw();
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
    /// let diagonal = VertexDirection::FLAT_RIGHT.edge_cw();
    /// assert_eq!(diagonal, EdgeDirection::FLAT_BOTTOM_RIGHT);
    // ```
    pub const fn edge_cw(self) -> EdgeDirection {
        EdgeDirection(self.0)
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
        value.into_hex()
    }
}

#[cfg(not(target_arch = "spirv"))]
impl Debug for VertexDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = self.into_hex();
        f.debug_struct("VertexDirection")
            .field("index", &self.0)
            .field("x", &c.x)
            .field("y", &c.y)
            .field("z", &c.z())
            .finish()
    }
}
