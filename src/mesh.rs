use crate::{Hex, HexLayout};
use glam::{Quat, Vec2, Vec3};

const UP_VECTOR: [f32; 3] = [0.0, 1.0, 0.0];
const DOWN_VECTOR: [f32; 3] = [0.0, -1.0, 0.0];

#[derive(Debug, Clone)]
/// Mesh information. The `const LEN` attribute ensures that there is the same number of vertices, normals and uvs
pub struct MeshInfo<const LEN: usize> {
    /// All vertices information (`Vertex_Position` attribute)
    pub vertices: [[f32; 3]; LEN],
    /// Normals for each vertex (You might need to swap `y` and `z`) (`Vertex_Normal` attribute)
    pub normals: [[f32; 3]; LEN],
    /// UV coordinates of each vertex (`Vertex_Uv` attribute)
    pub uvs: [[f32; 2]; LEN],
    /// Vertex indices for triangles
    pub indices: Vec<u16>,
    /// Direction the mesh is facing
    facing: [f32; 3],
}

impl<const LEN: usize> MeshInfo<LEN> {
    /// Returns a new [`MeshInfo`] but rotated in order to face `facing` direction
    ///
    /// # Panics
    ///
    /// Will panic if `facing` is zero length
    #[inline]
    #[must_use]
    pub fn facing(self, facing: Vec3) -> Self {
        let current_facing = Vec3::from_array(self.facing);
        let facing = facing.normalize();
        let rotation = Quat::from_rotation_arc(current_facing, facing);

        Self {
            vertices: self
                .vertices
                .map(|v| rotation.mul_vec3(Vec3::from_array(v)).to_array()),
            normals: self
                .normals
                .map(|n| rotation.mul_vec3(Vec3::from_array(n)).to_array()),
            facing: facing.to_array(),
            ..self
        }
    }
}

impl MeshInfo<7> {
    /// Computes mesh data for an hexagonal plane facing `Vec3::Y`
    #[must_use]
    pub fn hexagonal_plane(layout: &HexLayout, hex: Hex) -> Self {
        let center = layout.hex_to_world_pos(hex);
        let center = [center.x, 0., center.y];
        let corners = layout.hex_corners(hex);
        let corners_arr = corners.map(|p| [p.x, 0., p.y]);
        let uv_delta = Vec2::splat(0.5);
        Self {
            vertices: [
                center,
                corners_arr[0],
                corners_arr[1],
                corners_arr[2],
                corners_arr[3],
                corners_arr[4],
                corners_arr[5],
            ],
            uvs: [
                uv_delta.to_array(),
                (corners[0] + uv_delta).to_array(),
                (corners[1] + uv_delta).to_array(),
                (corners[2] + uv_delta).to_array(),
                (corners[3] + uv_delta).to_array(),
                (corners[4] + uv_delta).to_array(),
                (corners[5] + uv_delta).to_array(),
            ],
            normals: [UP_VECTOR; 7],
            indices: vec![
                1, 0, 2, // 1
                2, 0, 3, // 2
                3, 0, 4, // 3
                4, 0, 5, // 4
                5, 0, 6, // 5
                6, 0, 1, // 6
            ],
            facing: UP_VECTOR,
        }
    }
}

impl MeshInfo<31> {
    /// Computes mesh data for an hexagonal column facing `Vec3::Y` without the bottom face
    #[must_use]
    #[allow(clippy::too_many_lines)]
    pub fn partial_hexagonal_column(layout: &HexLayout, hex: Hex, column_height: f32) -> Self {
        let center = layout.hex_to_world_pos(hex);
        let center_top = Vec3::new(center.x, column_height, center.y);
        let corners = layout.hex_corners(hex);
        let (top_corners, bot_corners) = (
            corners.map(|p| Vec3::new(p.x, column_height, p.y)),
            corners.map(|p| Vec3::new(p.x, 0., p.y)),
        );
        let quad_normals = [
            ((top_corners[0] - center_top) + (top_corners[1] - center_top)).to_array(),
            ((top_corners[1] - center_top) + (top_corners[2] - center_top)).to_array(),
            ((top_corners[2] - center_top) + (top_corners[3] - center_top)).to_array(),
            ((top_corners[3] - center_top) + (top_corners[4] - center_top)).to_array(),
            ((top_corners[4] - center_top) + (top_corners[5] - center_top)).to_array(),
            ((top_corners[5] - center_top) + (top_corners[0] - center_top)).to_array(),
        ];
        let indices = vec![
            // Top Face triangles
            1, 0, 2, // 1
            2, 0, 3, // 2
            3, 0, 4, // 3
            4, 0, 5, // 4
            5, 0, 6, // 5
            6, 0, 1, // 6
            // Side triangles
            7, 8, 10, 10, 9, 7, // Quad 1
            11, 12, 14, 14, 13, 11, // Quad 2
            15, 16, 18, 18, 17, 15, // Quad 3
            19, 20, 22, 22, 21, 19, // Quad 4
            23, 24, 26, 26, 25, 23, // Quad 5
            27, 28, 30, 30, 29, 27, // Quad 6
        ];
        let uv_delta = Vec2::splat(0.5);
        Self {
            vertices: [
                // Top face
                center_top.to_array(),     // 0
                top_corners[0].to_array(), // 1
                top_corners[1].to_array(), // 2
                top_corners[2].to_array(), // 3
                top_corners[3].to_array(), // 4
                top_corners[4].to_array(), // 5
                top_corners[5].to_array(), // 6
                // Quad 0
                top_corners[0].to_array(), // 7
                top_corners[1].to_array(), // 8
                bot_corners[0].to_array(), // 9
                bot_corners[1].to_array(), // 10
                // Quad 1
                top_corners[1].to_array(), // 11
                top_corners[2].to_array(), // 12
                bot_corners[1].to_array(), // 13
                bot_corners[2].to_array(), // 14
                // Quad 2
                top_corners[2].to_array(), // 15
                top_corners[3].to_array(), // 16
                bot_corners[2].to_array(), // 17
                bot_corners[3].to_array(), // 18
                // Quad 3
                top_corners[3].to_array(), // 19
                top_corners[4].to_array(), // 20
                bot_corners[3].to_array(), // 21
                bot_corners[4].to_array(), // 22
                // Quad 4
                top_corners[4].to_array(), // 23
                top_corners[5].to_array(), // 24
                bot_corners[4].to_array(), // 25
                bot_corners[5].to_array(), // 26
                // Quad 5
                top_corners[5].to_array(), // 27
                top_corners[0].to_array(), // 28
                bot_corners[5].to_array(), // 29
                bot_corners[0].to_array(), // 30
            ],
            uvs: [
                // Top face
                uv_delta.to_array(),
                (corners[0] + uv_delta).to_array(),
                (corners[1] + uv_delta).to_array(),
                (corners[2] + uv_delta).to_array(),
                (corners[3] + uv_delta).to_array(),
                (corners[4] + uv_delta).to_array(),
                (corners[5] + uv_delta).to_array(),
                // Quad 0
                [0., column_height],
                [1., column_height],
                [0., 1.],
                [1., 1.],
                // Quad 1
                [0., column_height],
                [1., column_height],
                [0., 1.],
                [1., 1.],
                // Quad 2
                [0., column_height],
                [1., column_height],
                [0., 1.],
                [1., 1.],
                // Quad 3
                [0., column_height],
                [1., column_height],
                [0., 1.],
                [1., 1.],
                // Quad 4
                [0., column_height],
                [1., column_height],
                [0., 1.],
                [1., 1.],
                // Quad 5
                [0., column_height],
                [1., column_height],
                [0., 1.],
                [1., 1.],
            ],
            normals: [
                // Top face
                UP_VECTOR,
                UP_VECTOR,
                UP_VECTOR,
                UP_VECTOR,
                UP_VECTOR,
                UP_VECTOR,
                UP_VECTOR,
                // Quad 0
                quad_normals[0],
                quad_normals[0],
                quad_normals[0],
                quad_normals[0],
                // Quad 1
                quad_normals[1],
                quad_normals[1],
                quad_normals[1],
                quad_normals[1],
                // Quad 2
                quad_normals[2],
                quad_normals[2],
                quad_normals[2],
                quad_normals[2],
                // Quad 3
                quad_normals[3],
                quad_normals[3],
                quad_normals[3],
                quad_normals[3],
                // Quad 4
                quad_normals[4],
                quad_normals[4],
                quad_normals[4],
                quad_normals[4],
                // Quad 5
                quad_normals[5],
                quad_normals[5],
                quad_normals[5],
                quad_normals[5],
            ],
            indices,
            facing: UP_VECTOR,
        }
    }
}

impl MeshInfo<13> {
    /// Copmputes cheap mesh data for an hexagonal column facing `Vec3::Y` without the bottom face.
    ///
    /// This mesh has only 13 vertices, as no vertex is duplicated. As a consequence the normals will behave strangely.
    /// The UV mapping will be incorrect
    ///
    /// Use this mesh if you don't care about lighting and texturing.
    #[must_use]
    pub fn cheap_hexagonal_column(layout: &HexLayout, hex: Hex, column_height: f32) -> Self {
        let center = layout.hex_to_world_pos(hex);
        let center_top = Vec3::new(center.x, column_height, center.y);
        let corners = layout.hex_corners(hex);
        let (top_corners, bot_corners) = (
            corners.map(|p| Vec3::new(p.x, column_height, p.y)),
            corners.map(|p| Vec3::new(p.x, 0., p.y)),
        );

        let quad_normals = [
            (top_corners[0] - center_top).to_array(),
            (top_corners[1] - center_top).to_array(),
            (top_corners[2] - center_top).to_array(),
            (top_corners[3] - center_top).to_array(),
            (top_corners[4] - center_top).to_array(),
            (top_corners[5] - center_top).to_array(),
        ];

        let vertices = [
            // Top Vertices
            center_top.to_array(),     // 0
            top_corners[0].to_array(), // 1
            top_corners[1].to_array(), // 2
            top_corners[2].to_array(), // 3
            top_corners[3].to_array(), // 4
            top_corners[4].to_array(), // 5
            top_corners[5].to_array(), // 6
            // Bottom Vertices
            bot_corners[0].to_array(), // 7
            bot_corners[1].to_array(), // 8
            bot_corners[2].to_array(), // 9
            bot_corners[3].to_array(), // 10
            bot_corners[4].to_array(), // 11
            bot_corners[5].to_array(), // 12
        ];
        let indices = vec![
            // Top Face triangles
            1, 0, 2, // 1
            2, 0, 3, // 2
            3, 0, 4, // 3
            4, 0, 5, // 4
            5, 0, 6, // 5
            6, 0, 1, // 6
            // Side triangles
            1, 2, 8, 8, 7, 1, // Quad 1
            2, 3, 9, 9, 8, 2, // Quad 2
            3, 4, 10, 10, 9, 3, // Quad 3
            4, 5, 11, 11, 10, 4, // Quad 4
            5, 6, 12, 12, 11, 5, // Quad 5
            6, 1, 7, 7, 12, 6, // Quad 6
        ];
        Self {
            vertices,
            normals: [
                UP_VECTOR,
                quad_normals[0],
                quad_normals[1],
                quad_normals[2],
                quad_normals[3],
                quad_normals[4],
                quad_normals[5],
                quad_normals[0],
                quad_normals[1],
                quad_normals[2],
                quad_normals[3],
                quad_normals[4],
                quad_normals[5],
            ],
            uvs: [[0., 1.]; 13], // TODO: Find decent UV mapping
            indices,
            facing: UP_VECTOR,
        }
    }
}

impl MeshInfo<38> {
    /// Computes mesh data for an hexagonal column facing `Vec3::Y`
    #[must_use]
    #[allow(clippy::too_many_lines)]
    pub fn hexagonal_column(layout: &HexLayout, hex: Hex, column_height: f32) -> Self {
        let center = layout.hex_to_world_pos(hex);
        let center_top = Vec3::new(center.x, column_height, center.y);
        let center_bot = Vec3::new(center.x, 0., center.y);
        let corners = layout.hex_corners(hex);
        let (top_corners, bot_corners) = (
            corners.map(|p| Vec3::new(p.x, column_height, p.y)),
            corners.map(|p| Vec3::new(p.x, 0., p.y)),
        );
        let quad_normals = [
            ((top_corners[0] - center_top) + (top_corners[1] - center_top)).to_array(),
            ((top_corners[1] - center_top) + (top_corners[2] - center_top)).to_array(),
            ((top_corners[2] - center_top) + (top_corners[3] - center_top)).to_array(),
            ((top_corners[3] - center_top) + (top_corners[4] - center_top)).to_array(),
            ((top_corners[4] - center_top) + (top_corners[5] - center_top)).to_array(),
            ((top_corners[5] - center_top) + (top_corners[0] - center_top)).to_array(),
        ];
        let indices = vec![
            // Top Face triangles
            1, 0, 2, // 1
            2, 0, 3, // 2
            3, 0, 4, // 3
            4, 0, 5, // 4
            5, 0, 6, // 5
            6, 0, 1, // 6
            // Side triangles
            7, 8, 10, 10, 9, 7, // Quad 1
            11, 12, 14, 14, 13, 11, // Quad 2
            15, 16, 18, 18, 17, 15, // Quad 3
            19, 20, 22, 22, 21, 19, // Quad 4
            23, 24, 26, 26, 25, 23, // Quad 5
            27, 28, 30, 30, 29, 27, // Quad 6
            // Bottom Face triangles
            31, 32, 33, // 1
            31, 33, 34, // 2
            31, 34, 35, // 3
            31, 35, 36, // 4
            31, 36, 37, // 5
            31, 37, 32, // 6
        ];
        let uv_delta = Vec2::splat(0.5);
        Self {
            vertices: [
                // Top face
                center_top.to_array(),     // 0
                top_corners[0].to_array(), // 1
                top_corners[1].to_array(), // 2
                top_corners[2].to_array(), // 3
                top_corners[3].to_array(), // 4
                top_corners[4].to_array(), // 5
                top_corners[5].to_array(), // 6
                // Sides
                // Quad 1
                top_corners[0].to_array(), // 7
                top_corners[1].to_array(), // 8
                bot_corners[0].to_array(), // 9
                bot_corners[1].to_array(), // 10
                // Quad 2
                top_corners[1].to_array(), // 11
                top_corners[2].to_array(), // 12
                bot_corners[1].to_array(), // 13
                bot_corners[2].to_array(), // 14
                // Quad 3
                top_corners[2].to_array(), // 15
                top_corners[3].to_array(), // 16
                bot_corners[2].to_array(), // 17
                bot_corners[3].to_array(), // 18
                // Quad 4
                top_corners[3].to_array(), // 19
                top_corners[4].to_array(), // 20
                bot_corners[3].to_array(), // 21
                bot_corners[4].to_array(), // 22
                // Quad 5
                top_corners[4].to_array(), // 23
                top_corners[5].to_array(), // 24
                bot_corners[4].to_array(), // 25
                bot_corners[5].to_array(), // 26
                // Quad 6
                top_corners[5].to_array(), // 27
                top_corners[0].to_array(), // 28
                bot_corners[5].to_array(), // 29
                bot_corners[0].to_array(), // 30
                // Bottom face
                center_bot.to_array(),     // 31
                bot_corners[0].to_array(), // 32
                bot_corners[1].to_array(), // 33
                bot_corners[2].to_array(), // 34
                bot_corners[3].to_array(), // 35
                bot_corners[4].to_array(), // 36
                bot_corners[5].to_array(), // 37
            ],
            uvs: [
                // Top face
                uv_delta.to_array(),
                (corners[0] + uv_delta).to_array(),
                (corners[1] + uv_delta).to_array(),
                (corners[2] + uv_delta).to_array(),
                (corners[3] + uv_delta).to_array(),
                (corners[4] + uv_delta).to_array(),
                (corners[5] + uv_delta).to_array(),
                // Quad 0
                [0., column_height],
                [1., column_height],
                [0., 1.],
                [1., 1.],
                // Quad 1
                [0., column_height],
                [1., column_height],
                [0., 1.],
                [1., 1.],
                // Quad 2
                [0., column_height],
                [1., column_height],
                [0., 1.],
                [1., 1.],
                // Quad 3
                [0., column_height],
                [1., column_height],
                [0., 1.],
                [1., 1.],
                // Quad 4
                [0., column_height],
                [1., column_height],
                [0., 1.],
                [1., 1.],
                // Quad 5
                [0., column_height],
                [1., column_height],
                [0., 1.],
                [1., 1.],
                // Bottom face
                uv_delta.to_array(),
                (corners[0] + uv_delta).to_array(),
                (corners[1] + uv_delta).to_array(),
                (corners[2] + uv_delta).to_array(),
                (corners[3] + uv_delta).to_array(),
                (corners[4] + uv_delta).to_array(),
                (corners[5] + uv_delta).to_array(),
            ],
            normals: [
                // Top face
                UP_VECTOR,
                UP_VECTOR,
                UP_VECTOR,
                UP_VECTOR,
                UP_VECTOR,
                UP_VECTOR,
                UP_VECTOR,
                // Quad 0
                quad_normals[0],
                quad_normals[0],
                quad_normals[0],
                quad_normals[0],
                // Quad 1
                quad_normals[1],
                quad_normals[1],
                quad_normals[1],
                quad_normals[1],
                // Quad 2
                quad_normals[2],
                quad_normals[2],
                quad_normals[2],
                quad_normals[2],
                // Quad 3
                quad_normals[3],
                quad_normals[3],
                quad_normals[3],
                quad_normals[3],
                // Quad 4
                quad_normals[4],
                quad_normals[4],
                quad_normals[4],
                quad_normals[4],
                // Quad 5
                quad_normals[5],
                quad_normals[5],
                quad_normals[5],
                quad_normals[5],
                // Bottom face
                DOWN_VECTOR,
                DOWN_VECTOR,
                DOWN_VECTOR,
                DOWN_VECTOR,
                DOWN_VECTOR,
                DOWN_VECTOR,
                DOWN_VECTOR,
            ],
            indices,
            facing: UP_VECTOR,
        }
    }
}
