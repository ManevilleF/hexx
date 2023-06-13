use std::ops::Deref;

use crate::direction::angles::DIRECTION_ANGLE_OFFSET;
use crate::Direction;

const SQRT_3: f32 = 1.732_050_8;

// TODO: make const
static POINTY_ORIENTATION: HexOrientationData = HexOrientationData {
    forward_matrix: [SQRT_3, SQRT_3 / 2.0, 0.0, 3.0 / 2.0],
    inverse_matrix: [SQRT_3 / 3.0, -1.0 / 3.0, 0.0, 2.0 / 3.0],
    angle_offset: DIRECTION_ANGLE_OFFSET, // 30 degrees
};

// TODO: make const
static FLAT_ORIENTATION: HexOrientationData = HexOrientationData {
    forward_matrix: [3.0 / 2.0, 0.0, SQRT_3 / 2.0, SQRT_3],
    inverse_matrix: [2.0 / 3.0, 0.0, -1.0 / 3.0, SQRT_3 / 3.0],
    angle_offset: 0.0, // 0 degrees
};

/// Hexagonal orientation, either `pointy` or `flat`
///
/// # Usage
///
/// ```rust
/// # use hexx::*;
/// let flat = HexOrientation::flat();
/// let pointy = HexOrientation::pointy();
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "ser_de", derive(serde::Serialize, serde::Deserialize))]
pub struct HexOrientationData {
    /// Matrix used to compute hexagonal coordinates to world/pixel coordinates
    pub(crate) forward_matrix: [f32; 4],
    /// Matrix used to compute world/pixel coordinates to hexagonal coordinates
    pub(crate) inverse_matrix: [f32; 4],
    /// orientation offset in radians
    pub(crate) angle_offset: f32,
}

/// Hexagonal orientation, either [`Pointy`] or [`Flat`]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "ser_de", derive(serde::Serialize, serde::Deserialize))]
pub enum HexOrientation {
    /// *Pointy* orientation, means that the hexagons are *pointy-topped*
    Pointy,
    /// *Flat* orientation, means that the hexagons are *flat-topped*
    #[default]
    Flat,
}

impl HexOrientation {
    #[inline]
    #[must_use]
    #[deprecated(since = "0.7.0", note = "Use HexOrientation::Pointy instead")]
    /// "Pointy top" hexagonal orientation⬢
    pub const fn pointy() -> Self {
        Self::Pointy
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "0.7.0", note = "Use HexOrientation::Flat instead")]
    /// "Flat top" hexagonal orientation
    pub const fn flat() -> Self {
        Self::Flat
    }

    #[must_use]
    #[inline]
    /// Computes the angle in radians of the given `direction` in the current orientation
    pub fn direction_angle(&self, direction: Direction) -> f32 {
        direction.angle(self)
    }

    #[must_use]
    #[inline]
    /// Returns the orientation inner data, rotation angle and matrices
    pub fn orientation_data(&self) -> &'static HexOrientationData {
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
