use crate::{orientation::SQRT_3, EdgeDirection, Hex, HexOrientation, VertexDirection};
use glam::Vec2;

/// Hexagonal layout. This type is the bridge between your *world*/*pixel*
/// coordinate system and the hexagonal coordinate system.
///
/// # Axis
///
/// By default, the [`Hex`] `y` axis is pointing up and the `x` axis is
/// pointing right but you have the option to invert them using `invert_x` and
/// `invert_y` This may be useful depending on the coordinate system of your
/// display.
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
///     // We define the world space scale of the hexagons
///     scale: Vec2::new(1.0, 1.0),
/// };
/// // You can now find the world positon (center) of any given hexagon
/// let world_pos = layout.hex_to_world_pos(Hex::ZERO);
/// // You can also find which hexagon is at a given world/screen position
/// let hex_pos = layout.world_pos_to_hex(Vec2::new(1.23, 45.678));
/// ```
///
/// # Builder
///
/// `HexLayout` provides a builder pattern:
///
/// ```rust
/// # use hexx::*;
///
/// let mut layout = HexLayout::flat()
///     .with_scale(Vec2::new(2.0, 3.0)) // Individual Hexagon size
///     .with_origin(Vec2::new(-1.0, 0.0)); // World origin
/// // Invert the x axis, which will now go left. Will change `scale.x` to `-2.0`
/// layout.invert_x();
/// // Invert the y axis, which will now go down. Will change `scale.y` to `-3.0`
/// layout.invert_y();
/// ```
///
/// ## Working with Sprites
///
/// If you intend to use the hexagonal grid to place images/sprites you may use
/// `HexLayout::with_rect_size` to make the hexagon scale fit the your sprite
/// dimensions.
///
/// You can also retrieve the matching rect size from any layout using
/// `HexLayout::rect_size()`
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub struct HexLayout {
    /// The hexagonal orientation of the layout (usually "flat" or "pointy")
    pub orientation: HexOrientation,
    /// The origin of the hexagonal representation in world/pixel space, usually
    /// [`Vec2::ZERO`]
    pub origin: Vec2,
    /// The size of individual hexagons in world/pixel space. The scale can be
    /// irregular or negative
    pub scale: Vec2,
}

impl HexLayout {
    /// Inverts the layout `X` axis
    pub fn invert_x(&mut self) {
        self.scale.x *= -1.0;
    }

    /// Inverts the layout `Y` axis
    pub fn invert_y(&mut self) {
        self.scale.y *= -1.0;
    }

    /// Transforms a local hex space vector to world space
    /// by applying the layout `scale` but NOT the origin
    #[must_use]
    #[inline]
    pub fn transform_vector(&self, vector: Vec2) -> Vec2 {
        vector * self.scale
    }

    /// Transforms a local hex point to world space
    /// by applying the layout `scale` and `origin`
    #[must_use]
    #[inline]
    pub fn transform_point(&self, point: Vec2) -> Vec2 {
        self.origin + self.transform_vector(point)
    }

    /// Transforms a world space vector to local hex space
    /// by applying the layout `scale` but NOT the origin
    #[must_use]
    #[inline]
    pub fn inverse_transform_vector(&self, vector: Vec2) -> Vec2 {
        vector / self.scale
    }

    /// Transforms a world pace point to local hex space
    /// by applying the layout `scale` and `origin`
    #[must_use]
    #[inline]
    pub fn inverse_transform_point(&self, point: Vec2) -> Vec2 {
        self.inverse_transform_vector(point - self.origin)
    }
}

impl HexLayout {
    #[must_use]
    #[inline]
    /// Computes hexagonal coordinates `hex` into world/pixel coordinates
    pub fn hex_to_world_pos(&self, hex: Hex) -> Vec2 {
        self.hex_to_center_aligned_world_pos(hex) + self.origin
    }

    #[must_use]
    #[inline]
    /// Computes hexagonal coordinates `hex` into world/pixel coordinates but
    /// ignoring [`HexLayout::origin`]
    pub(crate) fn hex_to_center_aligned_world_pos(&self, hex: Hex) -> Vec2 {
        let p = self.orientation.forward(hex.as_vec2());
        self.transform_vector(p)
    }

    #[must_use]
    #[inline]
    /// Computes fractional hexagonal coordinates `hex` into world/pixel
    /// coordinates
    pub fn fract_hex_to_world_pos(&self, hex: Vec2) -> Vec2 {
        let p = self.orientation.forward(hex);
        self.transform_point(p)
    }

    #[must_use]
    #[inline]
    /// Computes world/pixel coordinates `pos` into hexagonal coordinates
    pub fn world_pos_to_hex(&self, pos: Vec2) -> Hex {
        let p = self.world_pos_to_fract_hex(pos).to_array();
        Hex::round(p)
    }

    #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
    #[must_use]
    /// Computes world/pixel coordinates `pos` into fractional hexagonal
    /// coordinates
    pub fn world_pos_to_fract_hex(&self, pos: Vec2) -> Vec2 {
        let point = self.inverse_transform_point(pos);
        self.orientation.inverse(point)
    }

    #[must_use]
    /// Retrieves all 6 corner coordinates of the given hexagonal coordinates
    /// `hex`
    pub fn hex_corners(&self, hex: Hex) -> [Vec2; 6] {
        let center = self.hex_to_world_pos(hex);
        self.center_aligned_hex_corners().map(|c| c + center)
    }

    /// Retrieves all 6 edge corner pair coordinates of the given hexagonal
    /// coordinates `hex`
    #[must_use]
    pub fn hex_edge_corners(&self, hex: Hex) -> [[Vec2; 2]; 6] {
        let center = self.hex_to_world_pos(hex);
        self.center_aligned_edge_corners()
            .map(|p| p.map(|c| c + center))
    }

    #[must_use]
    /// Retrieves all 6 edge corner pair coordinates of the given hexagonal
    /// coordinates `hex` without offsetting at the origin
    pub fn center_aligned_hex_corners(&self) -> [Vec2; 6] {
        VertexDirection::ALL_DIRECTIONS.map(|dir| dir.world_unit_vector(self))
    }

    #[must_use]
    /// Non offsetted hex edges
    pub(crate) fn center_aligned_edge_corners(&self) -> [[Vec2; 2]; 6] {
        EdgeDirection::ALL_DIRECTIONS
            .map(|dir| dir.vertex_directions().map(|v| v.world_unit_vector(self)))
    }

    #[inline]
    #[must_use]
    /// Returns the size of the bounding box/rect of an hexagon
    /// This uses both the `hex_size` and `orientation` of the layout.
    pub fn rect_size(&self) -> Vec2 {
        const FLAT_RECT: Vec2 = Vec2::new(2.0, SQRT_3);
        const POINTY_RECT: Vec2 = Vec2::new(SQRT_3, 2.0);

        self.scale
            * match self.orientation {
                HexOrientation::Pointy => POINTY_RECT,
                HexOrientation::Flat => FLAT_RECT,
            }
    }
}

#[cfg(feature = "grid")]
impl HexLayout {
    /// Returns the  world coordinate of the two edge vertices in clockwise
    /// order
    #[must_use]
    pub fn edge_coordinates(&self, edge: crate::GridEdge) -> [Vec2; 2] {
        let origin = self.hex_to_world_pos(edge.origin);
        edge.vertices()
            .map(|v| self.__vertex_coordinates(v) + origin)
    }

    /// Returns the  world coordinate of all edge vertex pairs in clockwise
    /// order
    #[must_use]
    pub fn all_edge_coordinates(&self, coord: Hex) -> [[Vec2; 2]; 6] {
        let origin = self.hex_to_world_pos(coord);
        coord.all_edges().map(|edge| {
            edge.vertices()
                .map(|v| self.__vertex_coordinates(v) + origin)
        })
    }

    /// Returns the world coordinate of the vertex
    #[must_use]
    pub fn vertex_coordinates(&self, vertex: crate::GridVertex) -> Vec2 {
        let origin = self.hex_to_world_pos(vertex.origin);
        self.__vertex_coordinates(vertex) + origin
    }

    fn __vertex_coordinates(&self, vertex: crate::GridVertex) -> Vec2 {
        vertex.direction.world_unit_vector(self)
    }
}

// Builder pattern
impl HexLayout {
    #[must_use]
    #[inline]
    /// Constructs a new layout with the given `orientation` and default
    /// values
    pub const fn new(orientation: HexOrientation) -> Self {
        Self {
            orientation,
            origin: Vec2::ZERO,
            scale: Vec2::ONE,
        }
    }

    #[must_use]
    #[inline]
    /// Constructs a new flat layout with default
    /// values
    pub const fn flat() -> Self {
        Self::new(HexOrientation::Flat)
    }

    #[must_use]
    #[inline]
    /// Constructs a new pointylayout with default
    /// values
    pub const fn pointy() -> Self {
        Self::new(HexOrientation::Pointy)
    }

    #[must_use]
    #[inline]
    /// Specifies the world/pixel origin of the layout
    pub const fn with_origin(mut self, origin: Vec2) -> Self {
        self.origin = origin;
        self
    }

    #[must_use]
    #[inline]
    /// Specifies the world/pixel regular size of individual hexagons
    pub const fn with_hex_size(mut self, size: f32) -> Self {
        self.scale = Vec2::splat(size);
        self
    }

    #[inline]
    #[must_use]
    /// Specifies the world/pixel size of individual hexagons to match
    /// the given `rect_size`. This is useful if you want hexagons
    /// to match a sprite size
    pub fn with_rect_size(self, rect_size: Vec2) -> Self {
        const FLAT_RECT: Vec2 = Vec2::new(0.5, 1.0 / SQRT_3);
        const POINTY_RECT: Vec2 = Vec2::new(1.0 / SQRT_3, 0.5);

        let scale = rect_size
            * match self.orientation {
                HexOrientation::Pointy => POINTY_RECT,
                HexOrientation::Flat => FLAT_RECT,
            };
        self.with_scale(scale)
    }

    #[must_use]
    #[inline]
    /// Specifies the world/pixel scale of individual hexagons.
    ///
    /// # Note
    ///
    /// For most use cases prefer [`Self::with_hex_size`] instead.
    pub const fn with_scale(mut self, scale: Vec2) -> Self {
        self.scale = scale;
        self
    }
}

impl Default for HexLayout {
    #[inline]
    fn default() -> Self {
        Self::new(HexOrientation::default())
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn flat_corners() {
        let point = Hex::new(0, 0);
        let mut layout = HexLayout::new(HexOrientation::Flat).with_scale(Vec2::new(10., 10.));
        let corners = layout.hex_corners(point).map(Vec2::round);
        assert_eq!(
            corners,
            [
                Vec2::new(10.0, 0.0),
                Vec2::new(5.0, 9.0),
                Vec2::new(-5.0, 9.0),
                Vec2::new(-10.0, 0.0),
                Vec2::new(-5.0, -9.0),
                Vec2::new(5.0, -9.0),
            ]
        );
        layout.invert_y();
        let corners = layout.hex_corners(point).map(Vec2::round);
        assert_eq!(
            corners,
            [
                Vec2::new(10.0, 0.0),
                Vec2::new(5.0, -9.0),
                Vec2::new(-5.0, -9.0),
                Vec2::new(-10.0, 0.0),
                Vec2::new(-5.0, 9.0),
                Vec2::new(5.0, 9.0),
            ]
        );
    }

    #[test]
    fn pointy_corners() {
        let point = Hex::new(0, 0);
        let mut layout = HexLayout::new(HexOrientation::Pointy).with_scale(Vec2::new(10., 10.));
        let corners = layout.hex_corners(point).map(Vec2::round);
        assert_eq!(
            corners,
            [
                Vec2::new(9.0, -5.0),
                Vec2::new(9.0, 5.0),
                Vec2::new(-0.0, 10.0),
                Vec2::new(-9.0, 5.0),
                Vec2::new(-9.0, -5.0),
                Vec2::new(0.0, -10.0),
            ]
        );
        layout.invert_y();
        let corners = layout.hex_corners(point).map(Vec2::round);
        assert_eq!(
            corners,
            [
                Vec2::new(9.0, 5.0),
                Vec2::new(9.0, -5.0),
                Vec2::new(-0.0, -10.0),
                Vec2::new(-9.0, -5.0),
                Vec2::new(-9.0, 5.0),
                Vec2::new(0.0, 10.0),
            ]
        );
    }

    #[test]
    fn rect_size() {
        let sizes = [
            Vec2::ZERO,
            Vec2::ONE,
            Vec2::X,
            Vec2::Y,
            Vec2::NEG_ONE,
            Vec2::NEG_X,
            Vec2::NEG_Y,
            Vec2::new(10.0, 5.0),
            Vec2::new(-10.0, 31.1),
            Vec2::new(110.0, 25.0),
            Vec2::new(-210.54, -54.0),
        ];
        for size in sizes {
            for orientation in [HexOrientation::Flat, HexOrientation::Pointy] {
                let layout = HexLayout::new(orientation).with_rect_size(size);
                let rect = layout.rect_size();
                assert_relative_eq!(rect.x, size.x, epsilon = 0.00001);
                assert_relative_eq!(rect.y, size.y, epsilon = 0.00001);
            }
        }
    }
}
