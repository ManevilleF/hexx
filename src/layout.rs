use crate::{Direction, Hex, HexOrientation};
use glam::Vec2;

/// Hexagonal layout. This type is the bridge between your *world*/*pixel* coordinate system
/// and the hexagonal coordinate system.
///
/// # Example
///
/// ```rust
/// # use hexx::*;
///
/// let layout = HexLayout {
///     // We want flat topped hexagons
///     orientation: HexOrientation::Flat,
///     // We define the world space origin equivalent of `Hex::ZERO` in hex space
///     origin: Vec2::new(1.0, 2.0),
///     // We define the world space size of the hexagons
///     hex_size: Vec2::new(1.0, 1.0)
/// };
/// // You can now find the world positon (center) of any given hexagon
/// let world_pos = layout.hex_to_world_pos(Hex::ZERO);
/// // You can also find which hexagon is at a given world/screen position
/// let hex_pos = layout.world_pos_to_hex(Vec2::new(1.23, 45.678));
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "ser_de", derive(serde::Serialize, serde::Deserialize))]
pub struct HexLayout {
    /// The hexagonal orientation of the layout (usually "flat" or "pointy")
    pub orientation: HexOrientation,
    /// The origin of the hexagonal representation in world/pixel space, usually [`Vec2::ZERO`]
    pub origin: Vec2,
    /// The size of individual hexagons in world/pixel space. The size can be irregular
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
        Direction::ALL_DIRECTIONS.map(|dir| {
            let angle = dir.angle_pointy() + self.orientation.angle_offset;
            center + Vec2::new(self.hex_size.x * angle.cos(), self.hex_size.y * angle.sin())
        })
    }
}

impl Default for HexLayout {
    fn default() -> Self {
        Self {
            orientation: HexOrientation::default(),
            origin: Vec2::ZERO,
            hex_size: Vec2::ONE,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flat_corners() {
        let point = Hex::new(0, 0);
        let layout = HexLayout {
            orientation: HexOrientation::Flat,
            origin: Vec2::ZERO,
            hex_size: Vec2::new(10., 10.),
        };
        let corners = layout.hex_corners(point).map(Vec2::round);
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
    }

    #[test]
    fn pointy_corners() {
        let point = Hex::new(0, 0);
        let layout = HexLayout {
            orientation: HexOrientation::Pointy,
            origin: Vec2::ZERO,
            hex_size: Vec2::new(10., 10.),
        };
        let corners = layout.hex_corners(point).map(Vec2::round);
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
