use crate::{BASE_FACING, HexLayout, InsetScaleMode, MeshInfo, UVOptions};
use glam::{Vec2, Vec3};

use super::FaceOptions;

type VertexIdx = u16;

/// Structure storing three vertex indices
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub struct Tri(pub [VertexIdx; 3]);

/// Representation of a primitive face, with a fixed amount of vertices and
/// triangles
#[derive(Debug, Clone)]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub struct Face<const VERTS: usize, const TRIS: usize> {
    /// Vertex positions
    pub positions: [Vec3; VERTS],
    /// Vertex normals
    pub normals: [Vec3; VERTS],
    /// Vertex uvs
    pub uvs: [Vec2; VERTS],
    /// Triangle indices
    pub triangles: [Tri; TRIS],
}

/// A Quad face made of 4 vertices and 2 triangles
pub type Quad = Face<4, 2>;
/// An hexagonal face made of 6 vertices and 4 triangles
pub type Hexagon = Face<6, 4>;

impl Tri {
    /// Flips the vertex indices order, effectively making the triangle face
    /// the other way
    pub const fn flip(&mut self) {
        let [a, b, c] = self.0;
        self.0 = [c, b, a];
    }
}

impl Quad {
    /// Construct a regular quad from two [`left`, `right`] 2d positions for x
    /// and z and a `bottom_height` and `top_height` for y
    ///
    /// # Arguments
    ///
    /// * `[left, right]` - the two bottom 2d vertex positions
    /// * `bottom_height` - the bottom vertices Y value
    /// * `top_height` - the top vertices Y value
    #[must_use]
    pub fn new([left, right]: [Vec2; 2], bottom_height: f32, top_height: f32) -> Self {
        let normal = (left + right).normalize();
        let normal = Vec3::new(normal.x, 0.0, normal.y);
        let positions = [
            Vec3::new(right.x, bottom_height, right.y),
            Vec3::new(right.x, top_height, right.y),
            Vec3::new(left.x, top_height, left.y),
            Vec3::new(left.x, bottom_height, left.y),
        ];
        Self {
            positions,
            normals: [normal; 4],
            uvs: [Vec2::X, Vec2::ONE, Vec2::Y, Vec2::ZERO],
            // 2 - 1
            // | \ |
            // 3 - 0
            triangles: [
                Tri([2, 1, 0]), // Tri 1
                Tri([0, 3, 2]), // Tri 2
            ],
        }
    }

    /// Same as [`Quad::new`] but the UV `y` values are bounded based on
    /// [`min_height`, `max_height`]
    #[must_use]
    pub(crate) fn new_bounded(
        sides: [Vec2; 2],
        bottom_height: f32,
        top_height: f32,
        [min_height, max_height]: [f32; 2],
    ) -> Self {
        let delta = max_height - min_height;
        let mut quad = Self::new(sides, bottom_height, top_height);
        let bottom_v = (bottom_height - min_height) / delta;
        let top_v = (top_height - min_height) / delta;
        quad.uvs[0][1] = bottom_v;
        quad.uvs[1][1] = top_v;
        quad.uvs[2][1] = top_v;
        quad.uvs[3][1] = bottom_v;
        quad
    }
}

impl Hexagon {
    /// Constructs a _center aligned_ (no offset) hexagon face from the given
    /// `layout`
    #[must_use]
    pub fn center_aligned(layout: &HexLayout) -> Self {
        let corners = layout.center_aligned_hex_corners();
        let uvs = corners.map(UVOptions::wrap_uv);
        let positions = corners.map(|p| Vec3::new(p.x, 0., p.y));
        Self {
            positions,
            uvs,
            normals: [BASE_FACING; 6],
            triangles: [
                Tri([0, 2, 1]), // Top tri
                Tri([3, 5, 4]), // Bot tri
                Tri([0, 5, 3]), // Mid Quad
                Tri([3, 2, 0]), // Mid Quad
            ],
        }
    }
}

impl<const VERTS: usize, const TRIS: usize> Face<VERTS, TRIS> {
    /// Computes the centroid of the face positions
    #[inline]
    #[must_use]
    #[expect(clippy::cast_precision_loss)]
    pub fn centroid(&self) -> Vec3 {
        self.positions.iter().sum::<Vec3>() / VERTS as f32
    }

    /// Computes the centroid of the face uvs
    #[inline]
    #[must_use]
    #[expect(clippy::cast_precision_loss)]
    pub fn uv_centroid(&self) -> Vec2 {
        self.uvs.iter().sum::<Vec2>() / VERTS as f32
    }

    /// Applies the face options to the face and returns a mesh
    #[must_use]
    pub fn apply_options(mut self, opts: &FaceOptions) -> MeshInfo {
        opts.uv.alter_uvs(&mut self.uvs);
        match opts.insetting {
            None => self.into(),
            Some(inset) => self.inset(inset.mode, inset.scale, inset.keep_inner_face),
        }
    }

    /// Performs an _inset_ operition on the mesh, assuming the mesh is a
    /// _looping face_, either a quad, triangle or hexagonal face.
    ///
    /// # Arguments
    ///
    /// * `mode` - the insetting behaviour mode
    /// * `keep_inner_face` - If set to true the insetted face will be kept,
    ///   otherwise it will be removed
    #[expect(clippy::cast_possible_truncation)]
    #[must_use]
    pub fn inset(self, mode: InsetScaleMode, scale: f32, keep_inner_face: bool) -> MeshInfo {
        // We compute the inset mesh, identical to the original face
        let mut inset_face = self.clone();
        // We downscale the inset face vertices and uvs along its plane
        match mode {
            InsetScaleMode::Centroid => {
                // vertices
                let centroid = inset_face.centroid();
                inset_face.positions.iter_mut().for_each(|v| {
                    *v = *v + ((centroid - *v) * scale);
                });
                // uvs
                let uv_centroid = inset_face.uv_centroid();
                inset_face.uvs.iter_mut().for_each(|uv| {
                    *uv = *uv + ((uv_centroid - *uv) * scale);
                });
            }
            InsetScaleMode::SmallestEdge => {
                let mut new_positions = inset_face.positions;
                let mut new_uvs = inset_face.uvs;
                for idx in 0..VERTS {
                    let [prev_idx, next_idx] = [(idx + VERTS - 1) % VERTS, (idx + 1) % VERTS];
                    // vertices
                    let [pos, prev, next] =
                        [idx, prev_idx, next_idx].map(|i| inset_face.positions[i]);
                    let [dir_prev, dir_next] = [(prev - pos), (next - pos)];
                    let [prev_len, next_len] = [dir_prev.length(), dir_next.length()];
                    let dist = prev_len.min(next_len) * scale;
                    new_positions[idx] =
                        pos + dir_next.normalize() * dist + dir_prev.normalize() * dist;
                    // uvs
                    let [disp_prev, disp_next] = [dist / prev_len, dist / next_len];
                    let [pos, prev, next] = [idx, prev_idx, next_idx].map(|i| inset_face.uvs[i]);
                    let [dir_prev, dir_next] = [(prev - pos), (next - pos)];
                    new_uvs[idx] = pos + dir_next * disp_next + dir_prev * disp_prev;
                }
                inset_face.positions = new_positions;
                inset_face.uvs = new_uvs;
            }
        }
        let mut inset_face = MeshInfo::from(inset_face);
        if !keep_inner_face {
            inset_face.indices.clear();
        }
        let mut mesh = MeshInfo::from(self);
        mesh.indices.clear();
        let vertex_count = VERTS as u16;
        let connection_indices = (0..vertex_count).flat_map(|v_idx| {
            let next_v_idx = (v_idx + 1) % vertex_count;
            let inset_v_idx = v_idx + vertex_count;
            let next_inset_v_idx = next_v_idx + vertex_count;

            let [mut a, mut b] = [
                Tri([next_inset_v_idx, next_v_idx, v_idx]),
                Tri([v_idx, inset_v_idx, next_inset_v_idx]),
            ];
            if scale < 0.0 {
                a.flip();
                b.flip();
            }
            a.0.into_iter().chain(b.0)
        });
        mesh.indices.extend(connection_indices);
        mesh.merge_with(inset_face);
        mesh
    }
}

impl<const VERTS: usize, const TRIS: usize> From<Face<VERTS, TRIS>> for MeshInfo {
    fn from(face: Face<VERTS, TRIS>) -> Self {
        Self {
            vertices: face.positions.to_vec(),
            normals: face.normals.to_vec(),
            uvs: face.uvs.to_vec(),
            indices: face.triangles.into_iter().flat_map(|t| t.0).collect(),
        }
    }
}
