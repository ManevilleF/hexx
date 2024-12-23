use std::ops::Deref;

use glam::{Mat2, Vec2};
pub(crate) const SQRT_3: f32 = 1.732_050_8;

/// Pointy orientation matrices and offset
const POINTY_ORIENTATION: HexOrientationData = HexOrientationData {
    forward_matrix: Mat2::from_cols_array(&[SQRT_3, 0.0, SQRT_3 / 2.0, 3.0 / 2.0]),
    inverse_matrix: Mat2::from_cols_array(&[SQRT_3 / 3.0, 0.0, -1.0 / 3.0, 2.0 / 3.0]),
};

/// Flat orientation matrices and offset
const FLAT_ORIENTATION: HexOrientationData = HexOrientationData {
    forward_matrix: Mat2::from_cols_array(&[3.0 / 2.0, SQRT_3 / 2.0, 0.0, SQRT_3]),
    inverse_matrix: Mat2::from_cols_array(&[2.0 / 3.0, -1.0 / 3.0, 0.0, SQRT_3 / 3.0]),
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
    pub(crate) forward_matrix: Mat2,
    /// Matrix used to compute world/pixel coordinates to hexagonal coordinates
    pub(crate) inverse_matrix: Mat2,
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
    /// Returns the orientation inner data, rotation angle and matrices
    pub const fn orientation_data(self) -> &'static HexOrientationData {
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

impl HexOrientationData {
    #[must_use]
    #[inline]
    /// Applies the orientation `forward_matrix` to a point `p`
    pub fn forward(&self, p: Vec2) -> Vec2 {
        self.forward_matrix.mul_vec2(p)
    }

    #[must_use]
    #[inline]
    /// Applies the orientation `inverse_matrix` to a point `p`
    pub fn inverse(&self, p: Vec2) -> Vec2 {
        self.inverse_matrix.mul_vec2(p)
    }
}
