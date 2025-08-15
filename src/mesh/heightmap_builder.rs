use super::{FaceOptions, InsetOptions, MeshInfo, face::Quad};
use crate::{EdgeDirection, Hex, HexLayout, PlaneMeshBuilder, UVOptions, storage::HexStore};
use glam::{Quat, Vec3};
use std::{ops::RangeInclusive, sync::Arc};

type MapFringeHeightFn = dyn Fn(Hex) -> f32;
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
///     .with_height_range(0.0..=5.0)
///     .without_top_face()
///     .build();
/// ```
///
/// # Filling holes
///
/// The builder will iterate through the entire `map`, and check for neighboring
/// heights to construct the vertical side quads. Meaning that there will always
/// be some missing neighbors:
/// * Inside the map if it is *sparse*
/// * On the "edge" of the map if it is *dense*
///
/// By default the builder will simply not generate the quads in those cases,
/// leading to vertical holes (See the `heightmap_builder` example]).
/// To fix this you have two options:
/// * Either define a "default height" ([`Self::with_default_height`]) which the
///   builder will consider to be the height of all those missing neighbors,
///   usually `0.0` or the miminim of your `height_range`.
/// * Provide the real height of those missing neighbors
///   ([`Self::with_fringe_heights`]) which the builder will use. This is
///   typically in the case of a large heightmap which is divided in smaller
///   meshes and you wish for all those meshes to connect seamlessly
///
/// # Notes
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
    /// Optional custom range. Otherwise computed from `map` values
    pub height_range: Option<RangeInclusive<f32>>,
    /// Map between the coordinates and the associated height
    pub map: &'m HeightMap,
    /// Top hexagonal face options. If `None` no top faces will be generated
    pub top_face_options: Option<FaceOptions>,
    /// Side quad face options. If `None` no side quads will be generated
    pub side_options: Option<FaceOptions>,
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
    /// Specifies the height for side quads to be generated at the fringe
    /// of the `map` (Map edge and potential holes in sparse maps).
    ///
    /// This function will be called on direct neighbors of map coordinates
    /// if that neighbor has no associated height in `map`
    pub fringe_heights: Option<Arc<MapFringeHeightFn>>,
    /// Optional function pointer to specify custom [`FaceOptions`] for some
    /// top faces
    pub custom_caps_options: Option<Arc<CapOptionsFn>>,
    /// Optional function pointer to specify custom [`FaceOptions`] for some
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
            height_range: None,
            top_face_options: Some(FaceOptions::new()),
            side_options: Some(FaceOptions::new()),
            offset: None,
            scale: None,
            rotation: None,
            center_aligned: false,
            fringe_heights: None,
            custom_caps_options: None,
            custom_sides_options: None,
        }
    }

    /// Specify a custom height range for the map. This is to be used only
    /// if you are generating only part of the global hashmap and don't want
    /// to rely on the automatic min/max calculation based on `map` values.
    ///
    /// These values are used to remap UV coordinates of side quads.
    ///
    /// # Notes
    /// * It is *heavily* recommended to specify a height range to avoid
    ///   inconsistent UVs
    /// * The range will *not* be checked, if out of range heights are found it
    ///   will have unexpected behaviour on UV calculations
    /// * The range should cover the full height map. If you intend for this
    ///   mesh to be part of a larger heightmap (See
    ///   [`Self::with_fringe_heights`]) then the range should include all
    ///   possible heights in the full map
    #[must_use]
    pub const fn with_height_range(mut self, range: RangeInclusive<f32>) -> Self {
        self.height_range = Some(range);
        self
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
    /// Notes:
    /// * This won't have any effect if [`Self::top_face_options`] is disabled
    ///   by [`Self::without_top_face`]
    /// * This method will override or be overidden by
    ///   [`Self::with_cap_options`]
    pub const fn with_cap_uv_options(mut self, uv_options: UVOptions) -> Self {
        if let Some(opts) = &mut self.top_face_options {
            opts.uv = uv_options;
        }
        self
    }

    /// Specify global insetting option for the top cap face
    ///
    /// Notes:
    /// * This won't have any effect if [`Self::top_face_options`] is disabled
    ///   by [`Self::without_top_face`]
    /// * This method will override or be overidden by
    ///   [`Self::with_cap_options`]
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
    /// * This won't have any effect if [`Self::top_face_options`] is disabled
    ///   by [`Self::without_top_face`]
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
    /// * This won't have any effect if [`Self::side_options`] is disabled by
    ///   [`Self::without_sides`]
    /// * Each hexagonal pair will be called *twice* but applied only *once*,
    ///   including coordinates at the fringe of the map (See
    ///   [`Self::with_fringe_heights`])
    #[must_use]
    #[inline]
    pub fn with_custom_sides_options(
        mut self,
        func: impl Fn(Hex, Hex) -> Option<FaceOptions> + 'static,
    ) -> Self {
        self.custom_sides_options = Some(Arc::new(func));
        self
    }

    /// Specifies a custom height for *out of bounds* or *missing* columns in
    /// order to generate side quads for columns on the edge of the map or
    /// to fill holes in the heightmap
    ///
    /// This is useful if you have multiple heightmaps which should connect to
    /// each other seamlessly
    ///
    /// # Notes
    ///
    /// * It is *recommended* to also call [`Self::with_height_range`] in this
    ///   case. As the returned fringe heights won't be used in UV remapping,
    ///   leading to inconsistent results
    /// * This method will override or be overidden by
    ///   [`Self::with_default_height`]
    #[must_use]
    #[inline]
    pub fn with_fringe_heights(mut self, func: impl Fn(Hex) -> f32 + 'static) -> Self {
        self.fringe_heights = Some(Arc::new(func));
        self
    }

    /// Specifies a global "default" height for *out of bounds* or missing
    /// columns in order to generate side quads for columns at the fringe of
    /// the map or to fill holes in the heightmap
    ///
    /// This is useful if you have multiple heightmaps which should connect to
    /// each other seamlessly
    ///
    /// # Notes
    ///
    /// * It is *recommended* to also call [`Self::with_height_range`] in this
    ///   case. As this `default_height` won't be used in UV remapping, leading
    ///   to inconsistent results
    /// * This method will override or be overidden by
    ///   [`Self::with_fringe_heights`]
    #[must_use]
    #[inline]
    pub fn with_default_height(mut self, default_height: f32) -> Self {
        self.fringe_heights = Some(Arc::new(move |_| default_height));
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

        let [min, max] = match self.height_range {
            Some(r) => [*r.start(), *r.end()],
            None => [
                self.map.values().copied().reduce(f32::min).unwrap_or(0.0),
                self.map.values().copied().reduce(f32::max).unwrap_or(0.0),
            ],
        };

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
                for dir in EdgeDirection::ALL_DIRECTIONS {
                    let neighbor = hex + dir;
                    let opt_height = self.map.get(hex + dir).copied();
                    // Maybe custom options
                    let side_opts = self
                        .custom_sides_options
                        .as_ref()
                        .and_then(|f| f(hex, neighbor))
                        .unwrap_or(side_opts);

                    let [a, b] = corners[dir.index() as usize];
                    let Some(neighbor_height) =
                        opt_height.or_else(|| self.fringe_heights.as_ref().map(|f| f(neighbor)))
                    else {
                        continue;
                    };
                    if neighbor_height >= height {
                        continue;
                    }
                    let quad = Quad::new_bounded([a, b], neighbor_height, height, [min, max]);
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
