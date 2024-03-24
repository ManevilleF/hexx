mod column_builder;
mod plane_builder;
#[cfg(test)]
mod tests;
/// Utility module for mesh construction
pub mod utils;
mod uv_mapping;

pub use column_builder::ColumnMeshBuilder;
pub use plane_builder::PlaneMeshBuilder;
pub use uv_mapping::{Rect, UVOptions};

use glam::{Quat, Vec2, Vec3};

use crate::{Hex, HexLayout};

pub(crate) const BASE_FACING: Vec3 = Vec3::Y;

/// Insetting options for [`PlaneMeshBuilder`] and [`ColumnMeshBuilder`]
/// used to create an insetted face on either hexagonal planes or quads
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub struct InsetOptions {
    /// If set to `true` the original downscaled face will be kept
    pub keep_inner_face: bool,
    /// Scale factor
    pub scale: f32,
    /// Inset mode
    pub mode: InsetScaleMode,
}

/// [`InsetOptions`] mode, defining the inset scaling behaviour
#[derive(Debug, Copy, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub enum InsetScaleMode {
    #[default]
    /// Each inset vertex position will be at a scale of the original one
    /// relative to the centroid of the face
    Centroid,
    /// Each inset vertex position will be at a proportional scale of the
    /// original one relative to its smallest edge
    SmallestEdge,
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
/// Hexagonal mesh information.
///
/// # Usage
///
/// Use:
/// * [`ColumnMeshBuilder`] for 3d hexagonal column meshes
/// * [`PlaneMeshBuilder`] for hexagonal plane meshes
///
/// The mesh info has some customization options:
///
/// ```rust
/// # use hexx::*;
///
/// let layout = HexLayout::default();
/// // Build the mesh info
/// let info: MeshInfo = ColumnMeshBuilder::new(&layout, 2.0).build();
/// // Customize the generated mesh
/// let info = info
///     .rotated(Quat::IDENTITY)
///     .with_offset(Vec3::new(12.0, 34.2, -43.54));
/// ```
///
/// ## Merging
///
/// `MeshInfo` can be merged with other meshes using `Self::merge_with`.
/// Don't forget to offset the meshes in the mesh builder using:
/// * [`ColumnMeshBuilder::at`]
/// * [`PlaneMeshBuilder::at`]
/// * [`Self::with_offset`] for a custom offset
///
/// Otherwise you might end up with meshes at the same coordinates
pub struct MeshInfo {
    /// All vertices positions information (`Vertex_Position` attribute)
    pub vertices: Vec<Vec3>,
    /// Normals for each vertex (You might need to swap `y` and `z`)
    /// (`Vertex_Normal` attribute)
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
    pub fn rotated(mut self, rotation: Quat) -> Self {
        self.vertices
            .iter_mut()
            .for_each(|v| *v = rotation.mul_vec3(*v));
        self.normals
            .iter_mut()
            .for_each(|n| *n = rotation.mul_vec3(*n));
        self
    }

    /// Returns a new [`MeshInfo`] but with `offset` applied to vertex positions
    #[inline]
    #[must_use]
    pub fn with_offset(mut self, offset: Vec3) -> Self {
        self.vertices.iter_mut().for_each(|v| *v += offset);
        self
    }

    /// Returns a new [`MeshInfo`] but with `scale` applied to vertex positions
    #[inline]
    #[must_use]
    pub fn with_scale(mut self, scale: Vec3) -> Self {
        self.vertices.iter_mut().for_each(|p| *p *= scale);
        self
    }

    /// Returns a new [`MeshInfo`] but with `scale` applied to vertex uvs
    #[inline]
    #[must_use]
    pub fn with_uv_scale(mut self, scale: Vec2) -> Self {
        self.uvs.iter_mut().for_each(|c| *c *= scale);
        self
    }

    /// Computes the centroid of the mesh vertices
    #[inline]
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn centroid(&self) -> Vec3 {
        let len = self.vertices.len() as f32;
        self.vertices.iter().sum::<Vec3>() / len
    }

    /// Computes the centroid of the mesh uvs
    #[inline]
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn uv_centroid(&self) -> Vec2 {
        let len = self.uvs.len() as f32;
        self.uvs.iter().sum::<Vec2>() / len
    }

    /// Merges `rhs` into `self`.
    /// All vertices, normals and uvs are appended to `self` and indices are
    /// offsetted to maintain triangle data.
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

    /// Computes cheap mesh data for an hexagonal column facing `Vec3::Y`
    /// without the bottom face.
    ///
    /// This mesh has only 12 vertices, as no vertex is duplicated. As a
    /// consequence the normals will behave strangely and the UV mapping
    /// will be *extremely* simplistic and stretched on the sides.
    ///
    /// Use this mesh if you don't care about lighting and texturing, like
    /// for *convex hull* collision shapes.
    ///
    /// Prefer using [`ColumnMeshBuilder`] in most cases
    #[must_use]
    pub fn cheap_hexagonal_column(layout: &HexLayout, hex: Hex, column_height: f32) -> Self {
        let center = layout.hex_to_world_pos(hex);
        let center_top = Vec3::new(center.x, column_height, center.y);
        let corners = layout.hex_corners(hex);
        let uvs = corners.map(UVOptions::wrap_uv);
        let (top_corners, bot_corners) = (
            corners.map(|p| Vec3::new(p.x, column_height, p.y)),
            corners.map(|p| Vec3::new(p.x, 0., p.y)),
        );

        let quad_normals = [
            (top_corners[0] - center_top),
            (top_corners[1] - center_top),
            (top_corners[2] - center_top),
            (top_corners[3] - center_top),
            (top_corners[4] - center_top),
            (top_corners[5] - center_top),
        ];

        let vertices = vec![
            // Top Vertices
            top_corners[0], // 0
            top_corners[1], // 1
            top_corners[2], // 2
            top_corners[3], // 3
            top_corners[4], // 4
            top_corners[5], // 5
            // Bottom Vertices
            bot_corners[0], // 6
            bot_corners[1], // 7
            bot_corners[2], // 8
            bot_corners[3], // 9
            bot_corners[4], // 10
            bot_corners[5], // 11
        ];
        let indices = vec![
            // Top Face triangles
            0, 2, 1, // 0
            3, 5, 4, // 1
            0, 5, 3, // 2
            3, 2, 0, // 3
            // Side triangles
            0, 1, 7, 7, 6, 0, // Quad 0
            1, 2, 8, 8, 7, 1, // Quad 1
            2, 3, 9, 9, 8, 2, // Quad 2
            3, 4, 10, 10, 9, 3, // Quad 3
            4, 5, 11, 11, 10, 4, // Quad 4
            5, 0, 6, 6, 11, 5, // Quad 5
        ];
        Self {
            vertices,
            normals: vec![
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
            uvs: [
                uvs[0], uvs[1], uvs[2], uvs[3], uvs[4], uvs[5], uvs[0], uvs[1], uvs[2], uvs[3],
                uvs[4], uvs[5],
            ]
            .to_vec(),
            indices,
        }
    }
}
