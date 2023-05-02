use std::ops::Add;

use crate::{Hex, HexLayout};
use glam::{Quat, Vec2, Vec3};

const BASE_FACING: Vec3 = Vec3::Y;
const UV_DELTA: Vec2 = Vec2::splat(0.5);

/// Builder struct to customize hex column mesh generation
#[derive(Debug, Clone)]
pub struct ColumnMeshBuilder<'l> {
    /// The hexagonal layout, used to compute vertex positions
    layout: &'l HexLayout,
    /// Custom hex position, will apply an offset if not [`Hex::ZERO`]
    pos: Hex,
    /// Optional custom offset for the mesh vertex positions
    offset: Option<Vec3>,
    /// Optional custom facing direction, useful to have the mesh already rotated
    ///
    /// By default the mesh is *facing* up (**Y** axis)
    facing: Option<Vec3>,
    /// The column height
    height: Option<f32>,
    /// Amount of quads to be generated on the sides of the column
    subdivisions: Option<usize>,
    /// Should the top hexagonal face be present
    top_face: bool,
    /// Should the bottom hexagonal face be present
    bottom_face: bool,
}

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

impl<'l> ColumnMeshBuilder<'l> {
    /// Setup a new builder using the given `layout`
    #[must_use]
    pub const fn new(layout: &'l HexLayout) -> Self {
        Self {
            layout,
            pos: Hex::ZERO,
            facing: None,
            height: None,
            subdivisions: None,
            offset: None,
            top_face: true,
            bottom_face: false,
        }
    }

    /// Specifies a custom `pos`, which will apply an offset to the whole mesh.
    ///
    /// ## Note
    ///
    /// It might be more optimal to generate only one mesh at [`Hex::ZERO`] and offset it later
    /// than have one mesh per hex position
    #[must_use]
    pub const fn at(mut self, pos: Hex) -> Self {
        self.pos = pos;
        self
    }

    /// Specify a custom *facing* direction for the mesh, by default the column is vertical (facing
    /// up)
    #[must_use]
    pub const fn facing(mut self, facing: Vec3) -> Self {
        self.facing = Some(facing);
        self
    }

    /// Specify a cusom offset for the whole mesh
    #[must_use]
    pub const fn with_offset(mut self, offset: Vec3) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Defines the column height
    #[must_use]
    pub const fn with_height(mut self, height: f32) -> Self {
        self.height = Some(height);
        self
    }

    /// Defines the column side quads amount
    #[must_use]
    pub const fn with_subdivisions(mut self, subdivisions: usize) -> Self {
        self.subdivisions = Some(subdivisions);
        self
    }

    /// The mesh will include a *bottom* hexagon face
    #[must_use]
    pub const fn with_bottom_face(mut self) -> Self {
        self.bottom_face = true;
        self
    }

    /// The mesh will include a *top* hexagon face
    #[must_use]
    pub const fn with_top_face(mut self) -> Self {
        self.top_face = true;
        self
    }

    /// The mesh will not include a *bottom* hexagon face
    #[must_use]
    pub const fn without_bottom_face(mut self) -> Self {
        self.bottom_face = false;
        self
    }

    /// The mesh will not include a *top* hexagon face
    #[must_use]
    pub const fn without_top_face(mut self) -> Self {
        self.top_face = false;
        self
    }

    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    #[allow(clippy::many_single_char_names)]
    /// Comsumes the builder to return the computed mesh data
    pub fn build(self) -> MeshInfo {
        let plane = MeshInfo::hexagonal_plane(self.layout, self.pos);
        let mut mesh = MeshInfo::default();
        let half_height = self.height.unwrap_or(0.0) / 2.0;
        if let Some(height) = self.height {
            let subidivisions = self.subdivisions.unwrap_or(0).max(1);
            let delta = height / subidivisions as f32;
            let center = self.layout.hex_to_world_pos(self.pos);
            let [a, b, c, d, e, f] = self.layout.hex_corners(self.pos);
            let corners = [[a, b], [b, c], [c, d], [d, e], [e, f], [f, a]];
            for div in 0..subidivisions {
                let height = delta.mul_add(div as f32, -half_height);
                for [left, right] in corners {
                    let normal = left - center + right - center;
                    let left = Vec3::new(left.x, height, left.y);
                    let right = Vec3::new(right.x, height, right.y);
                    let quad =
                        MeshInfo::quad([left, right], Vec3::new(normal.x, 0.0, normal.y), delta);
                    mesh = mesh + quad;
                }
            }
        };
        if self.top_face {
            mesh = mesh + plane.clone().with_offset(Vec3::Y * half_height);
        }
        if self.bottom_face {
            let rotation = Quat::from_rotation_arc(BASE_FACING, -BASE_FACING);
            let bottom_face = plane.with_offset(Vec3::Y * half_height).rotated(rotation);
            mesh = mesh + bottom_face;
        }
        if let Some(offset) = self.offset {
            mesh = mesh.with_offset(offset);
        }
        if let Some(facing) = self.facing {
            let facing = facing.normalize();
            let rotation = Quat::from_rotation_arc(BASE_FACING, facing);
            mesh = mesh.rotated(rotation);
        }
        mesh
    }
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
    #[must_use]
    pub fn hexagonal_plane(layout: &HexLayout, hex: Hex) -> Self {
        let center = layout.hex_to_world_pos(hex);
        let center = Vec3::new(center.x, 0., center.y);
        let corners = layout.hex_corners(hex);
        let corners_arr = corners.map(|p| Vec3::new(p.x, 0., p.y));
        Self {
            vertices: vec![
                center,
                corners_arr[0],
                corners_arr[1],
                corners_arr[2],
                corners_arr[3],
                corners_arr[4],
                corners_arr[5],
            ],
            uvs: vec![
                UV_DELTA,
                corners[0] + UV_DELTA,
                corners[1] + UV_DELTA,
                corners[2] + UV_DELTA,
                corners[3] + UV_DELTA,
                corners[4] + UV_DELTA,
                corners[5] + UV_DELTA,
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
    pub fn partial_hexagonal_column(layout: &HexLayout, hex: Hex, column_height: f32) -> Self {
        ColumnMeshBuilder::new(layout)
            .at(hex)
            .with_height(column_height)
            .build()
    }

    /// Computes mesh data for an hexagonal column facing `Vec3::Y`
    #[must_use]
    pub fn hexagonal_column(layout: &HexLayout, hex: Hex, column_height: f32) -> Self {
        ColumnMeshBuilder::new(layout)
            .at(hex)
            .with_height(column_height)
            .with_bottom_face()
            .build()
    }

    /// Computes cheap mesh data for an hexagonal column facing `Vec3::Y` without the bottom face.
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

impl Add for MeshInfo {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        let indices_offset =
            u16::try_from(self.vertices.len()).expect("MeshInfo has too many vertices");
        self.vertices.extend(rhs.vertices);
        self.normals.extend(rhs.normals);
        self.uvs.extend(rhs.uvs);
        self.indices
            .extend(rhs.indices.into_iter().map(|i| i + indices_offset));
        self
    }
}
