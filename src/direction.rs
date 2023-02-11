use crate::HexOrientation;
use std::f32::consts::PI;

/// Angle in radian between *flat* and *pointy* top orientations.
/// Equivalent to 30 degrees
pub const DIRECTION_ANGLE_OFFSET: f32 = PI / 6.0;
/// Angle in radian between *flat* and *pointy* top orientations.
/// Equivalent to π / 6 in radians
pub const DIRECTION_ANGLE_OFFSET_DEGREES: f32 = 30.0;
/// Angle in radian between two adjacent directions counter clockwise.
/// Equivalent to 60 degrees
pub const DIRECTION_ANGLE_RAD: f32 = PI / 3.0;
/// Angle in degrees between two adjacent directions counter clockwise.
/// Equivalent to π / 3 in radians
pub const DIRECTION_ANGLE_DEGREES: f32 = 60.0;

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
    #[doc(alias = "clockwise")]
    /// Returns the next direction in clockwise order
    pub const fn right(self) -> Self {
        match self {
            Self::TopRight => Self::BottomRight,
            Self::Top => Self::TopRight,
            Self::TopLeft => Self::Top,
            Self::BottomLeft => Self::TopLeft,
            Self::Bottom => Self::BottomLeft,
            Self::BottomRight => Self::Bottom,
        }
    }

    #[inline]
    #[must_use]
    #[doc(alias = "counterclockwise")]
    /// Returns the next direction in counter clockwise order
    pub const fn left(self) -> Self {
        match self {
            Self::TopRight => Self::Top,
            Self::Top => Self::TopLeft,
            Self::TopLeft => Self::BottomLeft,
            Self::BottomLeft => Self::Bottom,
            Self::Bottom => Self::BottomRight,
            Self::BottomRight => Self::TopRight,
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
    pub fn rotate_left(self, offset: usize) -> Self {
        let mut dirs = Self::ALL_DIRECTIONS;
        dirs.rotate_left(offset % 6);
        dirs[self as usize]
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
    pub fn rotate_right(self, offset: usize) -> Self {
        let mut dirs = Self::ALL_DIRECTIONS;
        dirs.rotate_right(offset % 6);
        dirs[self as usize]
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

    #[inline]
    #[must_use]
    /// Returns the angle in radians of the given direction in the given `orientation`
    pub fn angle(self, orientation: &HexOrientation) -> f32 {
        self.angle_pointy() - orientation.angle_offset
    }
}

#[cfg(test)]
#[allow(clippy::enum_glob_use)]
mod test {
    use super::Direction::*;
    use super::*;
    use std::f32::EPSILON;

    #[test]
    fn rotate_left_right() {
        for direction in Direction::ALL_DIRECTIONS {
            assert_eq!(direction, direction.rotate_right(6));
            assert_eq!(direction, direction.rotate_right(12));
            assert_eq!(direction, direction.rotate_right(1).rotate_left(1));
            assert_eq!(direction, direction.rotate_left(1).rotate_right(1));
            assert_eq!(direction.left(), direction.rotate_left(1));
            assert_eq!(direction.left().left(), direction.rotate_left(2));
            assert_eq!(direction.right(), direction.rotate_right(1));
            assert_eq!(direction.right().right(), direction.rotate_right(2));
        }
    }

    #[test]
    fn rotations_reverse_each_other() {
        for direction in Direction::ALL_DIRECTIONS {
            assert_eq!(direction, direction.left().right());
            assert_eq!(direction, direction.right().left());
        }
    }

    #[test]
    fn six_rotations_comes_home() {
        for direction in Direction::ALL_DIRECTIONS {
            let mut clockwise_dir = direction;
            let mut counter_clockwise_dir = direction;

            for _ in 0..6 {
                clockwise_dir = clockwise_dir.left();
                counter_clockwise_dir = counter_clockwise_dir.right();
            }

            assert_eq!(direction, clockwise_dir);
            assert_eq!(direction, counter_clockwise_dir);
        }
    }

    #[test]
    fn flat_angles_degrees() {
        let expected = [
            (BottomRight, 330.0),
            (TopRight, 30.0),
            (Top, 90.0),
            (TopLeft, 150.0),
            (BottomLeft, 210.0),
            (Bottom, 270.0),
        ];
        for (dir, angle) in expected {
            assert!(dir.angle_flat_degrees() - angle <= EPSILON);
        }
    }

    #[test]
    fn flat_angles_rad() {
        let expected = [
            (BottomRight, 11.0 * PI / 6.0),
            (TopRight, PI / 6.0),
            (Top, PI / 2.0),
            (TopLeft, 5.0 * PI / 6.0),
            (BottomLeft, 7.0 * PI / 6.0),
            (Bottom, 3.0 * PI / 2.0),
        ];
        let orientation = HexOrientation::flat();
        for (dir, angle) in expected {
            assert!(dir.angle_flat() - angle <= EPSILON);
            assert!(dir.angle(&orientation) - angle <= EPSILON);
        }
    }

    #[test]
    fn pointy_angles_degrees() {
        let expected = [
            (BottomRight, 300.0),
            (TopRight, 0.0),
            (Top, 60.0),
            (TopLeft, 120.0),
            (BottomLeft, 180.0),
            (Bottom, 240.0),
        ];
        for (dir, angle) in expected {
            assert!(dir.angle_pointy_degrees() - angle <= EPSILON);
        }
    }

    #[test]
    fn pointy_angles_rad() {
        let expected = [
            (BottomRight, 5.0 * PI / 3.0),
            (TopRight, 0.0),
            (Top, PI / 3.0),
            (TopLeft, 2.0 * PI / 3.0),
            (BottomLeft, PI),
            (Bottom, 4.0 * PI / 3.0),
        ];
        let orientation = HexOrientation::pointy();
        for (dir, angle) in expected {
            assert!(dir.angle_pointy() - angle <= EPSILON);
            assert!(dir.angle(&orientation) - angle <= EPSILON);
        }
    }
}
