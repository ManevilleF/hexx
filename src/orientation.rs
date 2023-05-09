use crate::direction::angles::DIRECTION_ANGLE_OFFSET;
use crate::Direction;

const SQRT_3: f32 = 1.732_050_8;

// TODO: make const
static POINTY_ORIENTATION: HexOrientation = HexOrientation {
    forward_matrix: [SQRT_3, SQRT_3 / 2.0, 0.0, 3.0 / 2.0],
    inverse_matrix: [SQRT_3 / 3.0, -1.0 / 3.0, 0.0, 2.0 / 3.0],
    angle_offset: DIRECTION_ANGLE_OFFSET, // 30 degrees
};

// TODO: make const
static FLAT_ORIENTATION: HexOrientation = HexOrientation {
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
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "ser_de", derive(serde::Serialize, serde::Deserialize))]
pub struct HexOrientation {
    /// Matrix used to compute hexagonal coordinates to world/pixel coordinates
    pub(crate) forward_matrix: [f32; 4],
    /// Matrix used to compute world/pixel coordinates to hexagonal coordinates
    pub(crate) inverse_matrix: [f32; 4],
    /// orientation offset in radians
    pub(crate) angle_offset: f32,
}

impl HexOrientation {
    #[inline]
    #[must_use]
    /// "Pointy top" hexagonal orientation⬢
    pub fn pointy() -> Self {
        POINTY_ORIENTATION
    }

    #[inline]
    #[must_use]
    /// "Flat top" hexagonal orientation
    pub fn flat() -> Self {
        FLAT_ORIENTATION
    }

    #[must_use]
    #[inline]
    /// Computes the angle in radians of the given `direction` in the current orientation
    pub fn direction_angle(&self, direction: Direction) -> f32 {
        direction.angle(self)
    }
}

impl Default for HexOrientation {
    fn default() -> Self {
        Self::flat()
    }
}
