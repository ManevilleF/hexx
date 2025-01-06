use super::{face::Quad, FaceOptions, InsetOptions, MeshInfo};
use crate::{storage::HexStore, EdgeDirection, HexLayout, PlaneMeshBuilder, UVOptions};
use glam::{Quat, Vec3};
use std::{collections::HashMap, sync::Arc};

type CapOptionsFn = dyn Fn(Hex) -> Option<FaceOptions>;
type SideOptionsFn = dyn Fn(Hex, Hex) -> Option<FaceOptions>;

/// Builder struct to customize hex column heightmap mesh generation.
///
/// # Example
///
/// ```rust
/// # use hexx::*;
/// # use std::collections::HashMap;
///
/// let map: HashMap<Hex, f32> = [
///     (hex(0, 0), 0.0),
///     (hex(1, 0), 2.0),
///     // ...
/// ]
/// .into();
/// let layout = HexLayout::default();
/// let mesh = HeightMapMeshBuilder::new(&layout, &map)
///     .with_offset(Vec3::new(1.2, 3.45, 6.7))
///     .without_top_face()
///     .build();
/// ```
///
/// To specify a default height, and enable side quads at the map edges
/// (No niehgbor found in a direction) use [`Self::with_default_height`]
///
/// # Note
///
/// Transform operations (Scale, Rotate, Translate) through the methods
///
/// - Scale: [`Self::with_scale`]
/// - Rotate: [`Self::with_rotation`]
/// - Translate: [`Self::with_offset`]
///
/// Are executed in that order, or **SRT**
pub struct HeightMapMeshBuilder<'l, 'm, HeightMap> {
    /// The hexagonal layout, used to compute vertex positions
    pub layout: &'l HexLayout,
    /// The column height on missing neighbor
    pub base_height: Option<f32>,
    /// Map between the coordinates and the associated height
    pub map: &'m HeightMap,
    /// Top hexagonal face options. If `None` no top faces will be generated
    pub top_face_options: Option<FaceOptions>,
    /// Side quad face options. If `None` no side quads will be generated
    pub side_options: Option<FaceOptions>,
    /// Specifies a default height for side quads to be generated at the border
    /// of the map or if holes are present in `map`.
    pub default_height: Option<f32>,
    /// Optional custom offset for the mesh vertex positions
    pub offset: Option<Vec3>,
    /// Optional custom scale factor for the mesh vertex positions
    pub scale: Option<Vec3>,
    /// Optional rotation quaternion, useful to have the mesh already
    /// rotated
    ///
    /// By default the mesh is *facing* up (**Y** axis)
    pub rotation: Option<Quat>,
    /// If set to `true`, the mesh will ignore [`HexLayout::origin`]
    pub center_aligned: bool,
    /// Optional function pointer to specify custom [`FaceOption`] for some
    /// top faces
    pub custom_caps_options: Option<Arc<CapOptionsFn>>,
    /// Optional function pointer to specify custom [`FaceOption`] for some
    /// side quads.
    pub custom_sides_options: Option<Arc<SideOptionsFn>>,
}

impl<'l, 'm, HeightMap: HexStore<f32>> HeightMapMeshBuilder<'l, 'm, HeightMap> {
    /// Setup a new builder using the given `layout` and height `map`.
    ///
    /// # Arguments
    ///
    /// * `layout` - the associated hexagonal horizontal layout
    /// * `map` - The heightmap values.
    ///
    /// Accepted values for `map` are:
    /// - [`HexagonalMap<f32>`](crate::storage::HexagonalMap)
    /// - [`RombusMap<f32>`](crate::storage::RombusMap)
    /// - [`HashMap<Hex, f32>`](std::collections::HashMap)
    #[must_use]
    pub const fn new(layout: &'l HexLayout, map: &'m HeightMap) -> Self {
        Self {
            layout,
            map,
            base_height: None,
            top_face_options: Some(FaceOptions::new()),
            side_options: Some(FaceOptions::new()),
            default_height: None,
            offset: None,
            scale: None,
            rotation: None,
            center_aligned: false,
            custom_caps_options: None,
            custom_sides_options: None,
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
    /// Specify global face options for the top cap triangles
    pub const fn with_cap_options(mut self, options: FaceOptions) -> Self {
        self.top_face_options = Some(options);
        self
    }

    #[must_use]
    #[inline]
    /// Specify global uv options for the top cap triangles
    ///
    /// Note:
    /// this won't have any effect if `top_face_options` is disabled
    pub const fn with_cap_uv_options(mut self, uv_options: UVOptions) -> Self {
        if let Some(opts) = &mut self.top_face_options {
            opts.uv = uv_options;
        }
        self
    }

    /// Specify global insetting option for the top cap face
    ///
    /// Note:
    /// this won't have any effect if `top_face_options` is disabled
    #[must_use]
    #[inline]
    pub const fn with_cap_inset_options(mut self, inset: InsetOptions) -> Self {
        if let Some(opts) = &mut self.top_face_options {
            opts.insetting = Some(inset);
        }
        self
    }

    /// Specify custom face options for the top cap faces to override the global
    /// `top_face_options` parameters.
    ///
    /// For each coordinate in the heightmap the function will be called. If it
    /// returns a `Some(opts)` then `opts` will be used for that face, otherwise
    /// the global `top_face_options` will be used
    ///
    /// Notes:
    /// * this won't have any effect if `top_face_options` is disabled
    #[must_use]
    #[inline]
    pub fn with_custom_cap_options(
        mut self,
        func: impl Fn(Hex) -> Option<FaceOptions> + 'static,
    ) -> Self {
        self.custom_caps_options = Some(Arc::new(func));
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
    /// Specify global face options for the column sides
    pub const fn with_side_options(mut self, options: FaceOptions) -> Self {
        self.side_options = Some(options);
        self
    }

    /// Specify custom sides options for the side faces to override the global
    /// `side_options` parameters.
    ///
    /// For each neighboring coordinate pair in the heightmap the function will
    /// be called. If it returns a `Some(opts)` then `opts` will be used for
    /// that face, otherwise the global `side_option` will be used
    ///
    /// Notes:
    /// * this won't have any effect if `side_options` is disabled
    /// * Each hexagonal pair will be called *twice* but applied only *once*.
    #[must_use]
    #[inline]
    pub fn with_custom_sides_options(
        mut self,
        func: impl Fn(Hex, Hex) -> Option<FaceOptions> + 'static,
    ) -> Self {
        self.custom_sides_options = Some(Arc::new(func));
        self
    }

    /// Specifies a default height for side quads to be generated at the border
    /// of the map or if holes are present in the height map
    #[must_use]
    #[inline]
    pub const fn with_default_height(mut self, default_height: f32) -> Self {
        self.default_height = Some(default_height);
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

    /// Comsumes the builder to return the computed mesh data
    pub fn build(self) -> MeshInfo {
        // We create the final mesh
        let mut mesh = MeshInfo::default();

        let mut min = self.map.values().copied().reduce(f32::min).unwrap_or(0.0);
        if let Some(default_height) = self.default_height {
            min = min.min(default_height);
        }
        let max = self.map.values().copied().reduce(f32::max).unwrap_or(0.0);

        for (hex, &height) in self.map.iter() {
            if let Some(opts) = self.top_face_options {
                // Maybe custom options
                let opts = self
                    .custom_caps_options
                    .as_ref()
                    .and_then(|f| f(hex))
                    .unwrap_or(opts);

                let mut plane = PlaneMeshBuilder::new(self.layout)
                    .at(hex)
                    .center_aligned()
                    .with_offset(Vec3::Y * height)
                    .with_uv_options(opts.uv);
                if let Some(inset) = opts.insetting {
                    plane = plane.with_inset_options(inset);
                }
                mesh.merge_with(plane.build());
            }
            if let Some(side_opts) = self.side_options {
                let corners = self.layout.hex_edge_corners(hex);
                let dir_heights = EdgeDirection::ALL_DIRECTIONS
                    .map(|dir| (dir, self.map.get(hex + dir).copied().or(self.base_height)));
                for (dir, opt_height) in dir_heights {
                    // Maybe custom options
                    let side_opts = self
                        .custom_sides_options
                        .as_ref()
                        .and_then(|f| f(hex, hex + dir))
                        .unwrap_or(side_opts);

                    let points = corners[dir.index() as usize];
                    let Some(other_height) = opt_height else {
                        if let Some(default_height) = self.default_height {
                            let quad =
                                Quad::new_bounded(points, default_height, height, [min, max]);
                            mesh.merge_with(quad.apply_options(&side_opts));
                        }
                        continue;
                    };
                    if other_height <= height {
                        continue;
                    }
                    let quad = Quad::new_bounded(points, other_height, height, [min, max]);
                    mesh.merge_with(quad.apply_options(&side_opts));
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
            mesh = mesh.with_offset(offset);
        }
        if !self.center_aligned {
            mesh = mesh.with_offset(Vec3::new(self.layout.origin.x, 0.0, self.layout.origin.y));
        }
        mesh
    }
}
