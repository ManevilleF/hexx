use crate::{Direction, DIRECTION_ANGLE_OFFSET};

const SQRT_3: f32 = 1.732_050_8;

/// Hexagonal orientation
#[derive(Debug, Copy, Clone)]
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
    // TODO: make const
    pub fn pointy() -> Self {
        Self {
            forward_matrix: [SQRT_3, SQRT_3 / 2.0, 0.0, 3.0 / 2.0],
            inverse_matrix: [SQRT_3 / 3.0, -1.0 / 3.0, 0.0, 2.0 / 3.0],
            angle_offset: DIRECTION_ANGLE_OFFSET, // 30 degrees
        }
    }

    #[inline]
    #[must_use]
    /// "Flat top" hexagonal orientation
    // TODO: make const
    pub fn flat() -> Self {
        Self {
            forward_matrix: [3.0 / 2.0, 0.0, SQRT_3 / 2.0, SQRT_3],
            inverse_matrix: [2.0 / 3.0, 0.0, -1.0 / 3.0, SQRT_3 / 3.0],
            angle_offset: 0.0, // 0 degrees
        }
    }

    #[must_use]
    #[inline]
    /// Computes the angle in radians of the given `direction` in the current orientation
    pub fn direction_angle(&self, direction: Direction) -> f32 {
        direction.angle(self)
    }
}
