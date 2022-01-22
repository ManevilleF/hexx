use crate::{Hex, HexLayout};
use glam::{Vec2, Vec3};

#[derive(Debug, Clone)]
/// Mesh data of an hexagon
pub struct HexMeshInfo<const LEN: usize> {
    pub vertices: [Vec3; LEN],
    pub uvs: [Vec2; LEN],
    pub normals: [Vec3; LEN],
    pub indices: Vec<i32>,
}

impl HexMeshInfo<7> {
    /// Gets mesh data for an hexagonal plane facing `Vec3::Y`
    #[must_use]
    pub fn plane_mesh(layout: &HexLayout, hex: Hex) -> Self {
        let center = layout.hex_to_world_pos(hex);
        let center = Vec3::new(center.x, 0., center.y);
        let corners = layout.hex_corners(hex).map(|p| Vec3::new(p.x, 0., p.y));
        Self {
            vertices: [
                center, corners[0], corners[1], corners[2], corners[3], corners[4], corners[5],
            ],
            uvs: [Vec2::new(0., 1.); 7], // TODO: Fix the uvs
            normals: [Vec3::Y; 7],
            indices: vec![
                1, 0, 2, // 1
                2, 0, 3, // 2
                3, 0, 4, // 3
                4, 0, 5, // 4
                5, 0, 6, // 5
                6, 0, 1, // 6
            ],
        }
    }
}
