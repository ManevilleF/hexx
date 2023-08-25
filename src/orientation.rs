use std::ops::Deref;

use crate::{direction::angles::DIRECTION_ANGLE_OFFSET_RAD, Direction};

const SQRT_3: f32 = 1.732_050_8;

// TODO: make const
static POINTY_ORIENTATION: HexOrientationData = HexOrientationData {
    forward_matrix: [SQRT_3, SQRT_3 / 2.0, 0.0, 3.0 / 2.0],
    inverse_matrix: [SQRT_3 / 3.0, -1.0 / 3.0, 0.0, 2.0 / 3.0],
    angle_offset: DIRECTION_ANGLE_OFFSET_RAD, // 30 degrees
};

// TODO: make const
static FLAT_ORIENTATION: HexOrientationData = HexOrientationData {
    forward_matrix: [3.0 / 2.0, 0.0, SQRT_3 / 2.0, SQRT_3],
    inverse_matrix: [2.0 / 3.0, 0.0, -1.0 / 3.0, SQRT_3 / 3.0],
    angle_offset: 0.0, // 0 degrees
};

/// [`HexOrientation`] inner data, retrieved by
/// [`HexOrientation::orientation_data`].
///
/// This struct stored a forward and inverse matrix, for pixel/hex conversion
/// and an angle offset
///
/// See [this article](https://www.redblobgames.com/grids/hexagons/#hex-to-pixel-axial) for more information
///
/// # Usage
///
/// ```rust
/// # use hexx::*;
/// let flat = HexOrientation::Flat;
/// let pointy = HexOrientation::Pointy;
/// let flat_data = flat.orientation_data();
/// let pointy_data = pointy.orientation_data();
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HexOrientationData {
    /// Matrix used to compute hexagonal coordinates to world/pixel coordinates
    pub(crate) forward_matrix: [f32; 4],
    /// Matrix used to compute world/pixel coordinates to hexagonal coordinates
    pub(crate) inverse_matrix: [f32; 4],
    /// orientation offset in radians
    pub(crate) angle_offset: f32,
}

/// Hexagonal orientation, either *Pointy-Topped* or *Flat-Topped*
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub enum HexOrientation {
    /// *Pointy* orientation, means that the hexagons are *pointy-topped*
    Pointy,
    /// *Flat* orientation, means that the hexagons are *flat-topped*
    #[default]
    Flat,
}

impl HexOrientation {
    #[must_use]
    #[inline]
    /// Computes the angle in radians of the given `direction` in the current
    /// orientation
    pub fn direction_angle(self, direction: Direction) -> f32 {
        direction.angle(self)
    }

    #[must_use]
    #[inline]
    /// Returns the orientation inner data, rotation angle and matrices
    pub fn orientation_data(self) -> &'static HexOrientationData {
        match self {
            Self::Pointy => &POINTY_ORIENTATION,
            Self::Flat => &FLAT_ORIENTATION,
        }
    }
}

impl Deref for HexOrientation {
    type Target = HexOrientationData;

    fn deref(&self) -> &Self::Target {
        self.orientation_data()
    }
}
