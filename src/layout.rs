use crate::{Hex, HexOrientation};
use glam::Vec2;
use std::f32::consts::PI;

/// Hexagonal layout
#[derive(Debug, Clone)]
pub struct HexLayout {
    /// The hexagonal orientation of the layout (usually "flat" or "pointy")
    pub orientation: HexOrientation,
    /// The origin of the hexagonal representation, usually [`Hex::ZERO`]
    pub origin: Vec2,
    /// The size of the hexagons in world/pixel space. The size can be irregular
    pub hex_size: Vec2,
}

impl HexLayout {
    #[allow(clippy::cast_precision_loss)]
    #[must_use]
    /// Computes hexagonal coordinates `hex` into world/pixel coordinates
    pub fn hex_to_world_pos(&self, hex: Hex) -> Vec2 {
        let matrix = self.orientation.forward_matrix;
        Vec2::new(
            matrix[0].mul_add(hex.x() as f32, matrix[1] * hex.y() as f32),
            matrix[2].mul_add(hex.x() as f32, matrix[3] * hex.y() as f32),
        ) * self.hex_size
            + self.origin
    }

    #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
    #[must_use]
    /// Computes world/pixel coordinates `pos` into hexagonal coordinates
    pub fn world_pos_to_hex(&self, pos: Vec2) -> Hex {
        let matrix = self.orientation.inverse_matrix;
        let point = (pos - self.origin) / self.hex_size;
        Hex::round((
            matrix[0].mul_add(point.x, matrix[1] * point.y),
            matrix[2].mul_add(point.x, matrix[3] * point.y),
        ))
    }

    #[allow(clippy::cast_precision_loss)]
    #[must_use]
    /// Retrieves all 6 corner coordinates of the given hexagonal coordinates `hex`
    pub fn hex_corners(&self, hex: Hex) -> [Vec2; 6] {
        let center = self.hex_to_world_pos(hex);
        [0, 1, 2, 3, 4, 5].map(|corner| {
            let angle = PI * 2.0 * (self.orientation.start_rotation + corner as f32) / 6.;
            center + Vec2::new(self.hex_size.x * angle.cos(), self.hex_size.y * angle.sin())
        })
    }
}

impl Default for HexLayout {
    fn default() -> Self {
        Self {
            orientation: HexOrientation::flat(),
            origin: Vec2::ZERO,
            hex_size: Vec2::ONE,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn corners() {
        let hex = Hex::new(0, 0);
        let layout = HexLayout {
            orientation: HexOrientation::flat(),
            origin: Vec2::ZERO,
            hex_size: Vec2::new(10., 10.),
        };
        let corners = layout.hex_corners(hex).map(Vec2::round);
        assert_eq!(
            corners,
            [
                Vec2::new(10.0, 0.0),
                Vec2::new(5.0, 9.0),
                Vec2::new(-5.0, 9.0),
                Vec2::new(-10.0, -0.0),
                Vec2::new(-5.0, -9.0),
                Vec2::new(5.0, -9.0)
            ]
        );

        let layout = HexLayout {
            orientation: HexOrientation::pointy(),
            origin: Vec2::ZERO,
            hex_size: Vec2::new(10., 10.),
        };
        let corners = layout.hex_corners(hex).map(Vec2::round);
        assert_eq!(
            corners,
            [
                Vec2::new(9.0, 5.0),
                Vec2::new(-0.0, 10.0),
                Vec2::new(-9.0, 5.0),
                Vec2::new(-9.0, -5.0),
                Vec2::new(0.0, -10.0),
                Vec2::new(9.0, -5.0)
            ]
        );
    }
}
