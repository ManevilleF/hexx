use std::f32::consts::PI;

use crate::HexOrientation;

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
///       +--+  2  +--+
///      / 3  \___/  1 \
///      \    /   \    /
///       +--+     +--+
///      /    \___/    \
///      \ 4  /   \  0 /
///       +--+  5  +--+   y Axis
///           \___/
/// ```
///
/// See [`Hex::NEIGHBORS_COORDS`](crate::Hex::NEIGHBORS_COORDS)
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "ser_de", derive(serde::Serialize, serde::Deserialize))]
pub enum Direction {
    /// (1, 0)
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
    BottomRight = 0,
    /// (1, -1)
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
    TopRight = 1,
    /// (0, -1)
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
    Top = 2,
    /// (-1, 0)
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
    TopLeft = 3,
    /// (-1, 1)
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
    BottomLeft = 4,
    /// (0, 1)
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
    Bottom = 5,
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
        Self::BottomRight,
        Self::TopRight,
        Self::Top,
        Self::TopLeft,
        Self::BottomLeft,
        Self::Bottom,
    ];

    /// Iterates through all enum variant in order
    pub fn iter() -> impl Iterator<Item = Self> {
        Self::ALL_DIRECTIONS.into_iter()
    }

    #[inline]
    #[must_use]
    /// Returns the angle in radians of the given direction for *flat* hexagons
    ///
    /// See [`Self::angle_pointy`] for *pointy* hexagons
    pub fn angle_flat(self) -> f32 {
        self.angle_pointy() + DIRECTION_ANGLE_RAD / 2.0
    }

    #[inline]
    #[must_use]
    /// Returns the angle in radians of the given direction for *pointy* hexagons
    ///
    /// See [`Self::angle_flat`] for *flat* hexagons
    pub fn angle_pointy(self) -> f32 {
        DIRECTION_ANGLE_RAD
            * match self {
                Self::BottomRight => 5.0,
                Self::TopRight => 0.0,
                Self::Top => 1.0,
                Self::TopLeft => 2.0,
                Self::BottomLeft => 3.0,
                Self::Bottom => 4.0,
            }
    }

    #[inline]
    #[must_use]
    /// Returns the angle in degrees of the given direction for *pointy* hexagons
    ///
    /// See [`Self::angle_flat`] for *flat* hexagons
    pub fn angle_flat_degrees(self) -> f32 {
        self.angle_pointy_degrees() + DIRECTION_ANGLE_DEGREES / 2.0
    }

    #[inline]
    #[must_use]
    /// Returns the angle in degrees of the given direction for *pointy* hexagons
    ///
    /// See [`Self::angle_flat`] for *flat* hexagons
    pub fn angle_pointy_degrees(self) -> f32 {
        DIRECTION_ANGLE_DEGREES
            * match self {
                Self::BottomRight => 5.0,
                Self::TopRight => 0.0,
                Self::Top => 1.0,
                Self::TopLeft => 2.0,
                Self::BottomLeft => 3.0,
                Self::Bottom => 4.0,
            }
    }

    #[inline]
    #[must_use]
    /// Returns the angle in radians of the given direction in the given `orientation`
    pub fn angle(self, orientation: &HexOrientation) -> f32 {
        self.angle_pointy() - orientation.start_rotation
    }
}

#[cfg(test)]
#[allow(clippy::enum_glob_use)]
mod test {
    use std::f32::EPSILON;

    use super::Direction::*;
    use super::*;

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
