mod column_builder;
mod plane_builder;
mod uv_mapping;

pub use column_builder::ColumnMeshBuilder;
pub use plane_builder::PlaneMeshBuilder;
pub use uv_mapping::UVOptions;

use glam::{Quat, Vec2, Vec3};

use crate::{Hex, HexLayout};

pub(crate) const BASE_FACING: Vec3 = Vec3::Y;

#[derive(Debug, Clone, Default)]
/// Mesh information. The `const LEN` attribute ensures that there is the same number of vertices, normals and uvs
pub struct MeshInfo {
    /// All vertices positions information (`Vertex_Position` attribute)
    pub vertices: Vec<Vec3>,
    /// Normals for each vertex (You might need to swap `y` and `z`) (`Vertex_Normal` attribute)
    pub normals: Vec<Vec3>,
    /// UV coordinates of each vertex (`Vertex_Uv` attribute)
    pub uvs: Vec<Vec2>,
    /// Vertex indices for triangles
    pub indices: Vec<u16>,
}

impl MeshInfo {
    /// Returns a new [`MeshInfo`] but with vertex positions and normals rotated
    #[inline]
    #[must_use]
    pub fn rotated(self, rotation: Quat) -> Self {
        Self {
            vertices: self
                .vertices
                .into_iter()
                .map(|v| rotation.mul_vec3(v))
                .collect(),
            normals: self
                .normals
                .into_iter()
                .map(|n| rotation.mul_vec3(n))
                .collect(),
            ..self
        }
    }

    /// Returns a new [`MeshInfo`] but with `offset` applied to vertex positions
    #[inline]
    #[must_use]
    pub fn with_offset(self, offset: Vec3) -> Self {
        Self {
            vertices: self.vertices.into_iter().map(|p| p + offset).collect(),
            ..self
        }
    }

    /// Merges `rhs` into `self`.
    /// All vertices, normals and uvs are appended to `self` and indices are offsetted to maintain
    /// triangle data.
    ///
    /// # Note
    ///
    /// This method doesn't merge vertices or tries to remove invisible faces.
    ///
    /// # Panics
    ///
    /// Will panic if there are more vertices than [`u16::MAX`]
    pub fn merge_with(&mut self, rhs: Self) {
        let indices_offset =
            u16::try_from(self.vertices.len()).expect("MeshInfo has too many vertices");
        self.vertices.extend(rhs.vertices);
        self.normals.extend(rhs.normals);
        self.uvs.extend(rhs.uvs);
        self.indices
            .extend(rhs.indices.into_iter().map(|i| i + indices_offset));
    }

    fn quad([left, right]: [Vec3; 2], normal: Vec3, height: f32) -> Self {
        let offset = BASE_FACING * height;
        Self {
            vertices: vec![left, left + offset, right + offset, right],
            normals: [normal; 4].to_vec(),
            uvs: vec![Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y],
            indices: vec![
                1, 2, 3, // Tri 1
                3, 0, 1, // Tri 2
            ],
        }
    }

    /// Computes mesh data for an hexagonal plane facing `Vec3::Y`
    ///
    /// # Note
    ///
    /// Prefer using [`PlaneMeshBuilder`] for additional customization like:
    /// * UV options
    /// * Offsets
    /// * rotation
    /// * etc
    #[must_use]
    pub fn hexagonal_plane(layout: &HexLayout, hex: Hex) -> Self {
        let center = layout.hex_to_world_pos(hex);
        let corners = layout.hex_corners(hex);
        let corners_arr = corners.map(|p| Vec3::new(p.x, 0., p.y));
        Self {
            vertices: vec![
                Vec3::new(center.x, 0., center.y),
                corners_arr[0],
                corners_arr[1],
                corners_arr[2],
                corners_arr[3],
                corners_arr[4],
                corners_arr[5],
            ],
            uvs: vec![
                center, corners[0], corners[1], corners[2], corners[3], corners[4], corners[5],
            ],
            normals: [Vec3::Y; 7].to_vec(),
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

    /// Computes mesh data for an hexagonal column facing `Vec3::Y` without the bottom face
    #[must_use]
    #[deprecated(since = "0.6.0", note = "Use ColumnMeshBuilder instead")]
    pub fn partial_hexagonal_column(layout: &HexLayout, hex: Hex, column_height: f32) -> Self {
        ColumnMeshBuilder::new(layout, column_height)
            .at(hex)
            .without_bottom_face()
            .build()
    }

    /// Computes mesh data for an hexagonal column facing `Vec3::Y`
    #[must_use]
    #[deprecated(since = "0.6.0", note = "Use ColumnMeshBuilder instead")]
    pub fn hexagonal_column(layout: &HexLayout, hex: Hex, column_height: f32) -> Self {
        ColumnMeshBuilder::new(layout, column_height)
            .at(hex)
            .build()
    }

    /// Computes cheap mesh data for an hexagonal column facing `Vec3::Y` without the bottom face.
    ///
    /// This mesh has only 13 vertices, as no vertex is duplicated. As a consequence the normals will behave strangely
    /// and the UV mapping will be incorrect.
    ///
    /// Use this mesh if you don't care about lighting and texturing, for example for collision
    /// shapes.
    #[must_use]
    pub fn cheap_hexagonal_column(layout: &HexLayout, hex: Hex, column_height: f32) -> Self {
        let center = layout.hex_to_world_pos(hex);
        let center_top = Vec3::new(center.x, column_height, center.y);
        let corners = layout.hex_corners(hex);
        let (top_corners, bot_corners) = (
            corners.map(|p| Vec3::new(p.x, column_height, p.y)),
            corners.map(|p| Vec3::new(p.x, 0., p.y)),
        );

        let quad_normals = vec![
            (top_corners[0] - center_top),
            (top_corners[1] - center_top),
            (top_corners[2] - center_top),
            (top_corners[3] - center_top),
            (top_corners[4] - center_top),
            (top_corners[5] - center_top),
        ];

        let vertices = vec![
            // Top Vertices
            center_top,     // 0
            top_corners[0], // 1
            top_corners[1], // 2
            top_corners[2], // 3
            top_corners[3], // 4
            top_corners[4], // 5
            top_corners[5], // 6
            // Bottom Vertices
            bot_corners[0], // 7
            bot_corners[1], // 8
            bot_corners[2], // 9
            bot_corners[3], // 10
            bot_corners[4], // 11
            bot_corners[5], // 12
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
            normals: vec![
                BASE_FACING,
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
            uvs: [Vec2::Y; 13].to_vec(), // TODO: Find decent UV mapping
            indices,
        }
    }
}
