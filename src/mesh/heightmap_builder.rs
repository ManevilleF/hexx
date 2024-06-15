use super::{face::Quad, FaceOptions, InsetOptions, MeshInfo};
use crate::{EdgeDirection, Hex, HexLayout, PlaneMeshBuilder, UVOptions};
use glam::{Quat, Vec3};
use std::collections::HashMap;

pub struct HeightMapMeshBuilder<'l, 'm> {
    /// The hexagonal layout, used to compute vertex positions
    pub layout: &'l HexLayout,
    /// The column height on missing neighbor
    pub base_height: Option<f32>,
    pub map: &'m HashMap<Hex, f32>,
    /// Top hexagonal face builder
    pub top_face_options: Option<FaceOptions>,
    pub side_options: Option<FaceOptions>,
    pub center_aligned: bool,
    pub fill_holes: bool,
    /// Optional custom offset for the mesh vertex positions
    pub offset: Option<Vec3>,
    /// Optional custom scale factor for the mesh vertex positions
    pub scale: Option<Vec3>,
    /// Optional rotation quaternion, useful to have the mesh already
    /// rotated
    ///
    /// By default the mesh is *facing* up (**Y** axis)
    pub rotation: Option<Quat>,
}

impl<'l, 'm> HeightMapMeshBuilder<'l, 'm> {
    pub const fn new(layout: &'l HexLayout, map: &'m HashMap<Hex, f32>) -> Self {
        Self {
            layout,
            map,
            base_height: None,
            top_face_options: Some(FaceOptions::new()),
            side_options: Some(FaceOptions::new()),
            center_aligned: false,
            fill_holes: false,
            offset: None,
            scale: None,
            rotation: None,
        }
    }

    /// Specify a custom rotation for the whole mesh
    #[must_use]
    pub const fn with_rotation(mut self, rotation: Quat) -> Self {
        self.rotation = Some(rotation);
        self
    }

    /// Specify a cusom offset for the whole mesh. This can be used to customize
    /// the anchor/pivot of the mesh.
    #[must_use]
    #[inline]
    pub const fn with_offset(mut self, offset: Vec3) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Specify a custom scale factor for the whole mesh
    #[must_use]
    pub const fn with_scale(mut self, scale: Vec3) -> Self {
        self.scale = Some(scale);
        self
    }

    /// The mesh will not include a *top* hexagon face
    #[must_use]
    #[inline]
    pub const fn without_top_face(mut self) -> Self {
        self.top_face_options = None;
        self
    }

    #[must_use]
    #[inline]
    /// Specify custom face options for the top cap triangles
    pub const fn with_cap_options(mut self, options: FaceOptions) -> Self {
        self.top_face_options = Some(options);
        self
    }

    /// The mesh will not include a side faces
    #[must_use]
    #[inline]
    pub const fn without_sides(mut self) -> Self {
        self.side_options = None;
        self
    }
    #[must_use]
    #[inline]
    /// Specify custom face options for the column sides
    pub const fn with_side_options(mut self, options: FaceOptions) -> Self {
        self.side_options = Some(options);
        self
    }

    #[must_use]
    #[inline]
    /// Specify custom uv options for the top cap triangles
    ///
    /// Note:
    /// this won't have any effect if `top_face_options` is disabled
    pub const fn with_cap_uv_options(mut self, uv_options: UVOptions) -> Self {
        if let Some(opts) = &mut self.top_face_options {
            opts.uv = uv_options;
        }
        self
    }

    /// Specify inset option for the top cap face
    ///
    /// Note:
    /// this won't have any effect if `top_face_options` is disabled
    #[must_use]
    #[inline]
    pub const fn with_cap_inset_options(mut self, inset: InsetOptions) -> Self {
        if let Some(opts) = &mut self.top_face_options {
            opts.insetting = Some(inset)
        }
        self
    }

    #[must_use]
    #[inline]
    /// Generates sides in the case of missing neighbor
    pub const fn fill_holes(mut self) -> Self {
        self.fill_holes = true;
        self
    }

    #[must_use]
    #[inline]
    /// Ignores the [`HexLayout::origin`] offset, generating a mesh centered
    /// around `(0.0, 0.0)`.
    pub const fn center_aligned(mut self) -> Self {
        self.center_aligned = true;
        self
    }

    pub fn build(self) -> MeshInfo {
        // We create the final mesh
        let mut mesh = MeshInfo::default();

        let min = self.map.values().copied().reduce(f32::min).unwrap_or(0.0);
        let max = self.map.values().copied().reduce(f32::max).unwrap_or(0.0);

        for (&hex, &height) in self.map {
            if let Some(opts) = &self.top_face_options {
                let mut plane = PlaneMeshBuilder::new(self.layout)
                    .at(hex)
                    .with_offset(Vec3::Y * height)
                    .with_uv_options(opts.uv);
                if let Some(inset) = opts.insetting {
                    plane = plane.with_inset_options(inset);
                }
                mesh.merge_with(plane.build());
            }
            if let Some(side_opts) = &self.side_options {
                let corners = self.layout.hex_edge_corners(hex);
                let dir_heights = EdgeDirection::ALL_DIRECTIONS.map(|dir| {
                    (
                        dir,
                        self.map.get(&(hex + dir)).copied().or(self.base_height),
                    )
                });
                for (dir, opt_height) in dir_heights {
                    let points = corners[dir.index() as usize];
                    let Some(other_height) = opt_height else {
                        if self.fill_holes {
                            let quad = Quad::new_bounded(points, min, height, [min, max]);
                            mesh.merge_with(quad.apply_options(side_opts));
                        }
                        continue;
                    };
                    if other_height <= height {
                        continue;
                    }
                    let quad = Quad::new_bounded(points, other_height, height, [min, max]);
                    mesh.merge_with(quad.apply_options(side_opts));
                }
            }
        }
        // **S** - We apply optional scale
        if let Some(scale) = self.scale {
            mesh = mesh.with_scale(scale);
        }
        // **R** - We rotate the mesh to face the given direction
        if let Some(rotation) = self.rotation {
            mesh = mesh.rotated(rotation);
        }
        // **T** - We offset the vertex positions after scaling and rotating
        if let Some(offset) = self.offset {
            mesh = mesh.with_offset(offset)
        }
        mesh
    }
}
