use glam::{Quat, Vec3};

use super::{utils::Quad, MeshInfo, BASE_FACING};
use crate::{EdgeDirection, Hex, HexLayout, InsetOptions, PlaneMeshBuilder, UVOptions};

/// Builder struct to customize hex column mesh generation.
///
/// By default this builder will create a full hexagonal column with all faces
/// and no side subdivisions.
/// The mesh will be anchored at the center of the *bottom face*, use offsets to
/// cutomize anchor/pivot position.
///
/// # Example
///
/// ```rust
/// # use hexx::*;
///
/// let layout = HexLayout::default();
/// let mesh = ColumnMeshBuilder::new(&layout, 15.0)
///     .at(hex(2, 3))
///     .facing(Vec3::Z)
///     .with_subdivisions(5)
///     .with_offset(Vec3::new(1.2, 3.45, 6.7))
///     .without_bottom_face()
///     .without_top_face()
///     .build();
/// ```
///
/// # Note
///
/// Transform operations (Scale, Rotate, Translate) through the methods
///
/// - Scale: [`Self::with_scale`]
/// - Rotate: [`Self::with_rotation`], [`Self::facing`]
/// - Translate: [`Self::with_offset`], [`Self::at`]
///
/// Are executed in that order, or **SRT**
#[derive(Debug, Clone)]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub struct ColumnMeshBuilder<'l> {
    /// The hexagonal layout, used to compute vertex positions
    pub layout: &'l HexLayout,
    /// The column height
    pub height: f32,
    /// Custom hex position, will apply an offset if not [`Hex::ZERO`]
    pub pos: Hex,
    /// Optional custom offset for the mesh vertex positions
    pub offset: Option<Vec3>,
    /// Optional custom scale factor for the mesh vertex positions
    pub scale: Option<Vec3>,
    /// Optional rotation quaternion, useful to have the mesh already
    /// rotated
    ///
    /// By default the mesh is *facing* up (**Y** axis)
    pub rotation: Option<Quat>,
    /// Amount of quads to be generated on the sides of the column
    pub subdivisions: Option<usize>,
    /// Top hexagonal face builder
    pub top_face: Option<PlaneMeshBuilder<'l>>,
    /// Bottom hexagonal face builder
    pub bottom_face: Option<PlaneMeshBuilder<'l>>,
    /// Options for the column side quads
    pub sides_options: [Option<SideOptions>; 6],
    /// If set to `true`, the mesh will ignore [`HexLayout::origin`]
    pub center_aligned: bool,
}

/// Column Quad options
#[derive(Debug, Copy, Clone, Default)]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub struct SideOptions {
    /// UV mapping options
    pub uv: UVOptions,
    /// Insetting options
    pub insetting: Option<InsetOptions>,
}

impl SideOptions {
    /// Generates default quad options
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self {
            uv: UVOptions::new(),
            insetting: None,
        }
    }
}

impl<'l> ColumnMeshBuilder<'l> {
    /// Setup a new builder using the given `layout` and `height`
    #[must_use]
    pub const fn new(layout: &'l HexLayout, height: f32) -> Self {
        Self {
            layout,
            height,
            pos: Hex::ZERO,
            rotation: None,
            subdivisions: None,
            offset: None,
            scale: None,
            top_face: Some(PlaneMeshBuilder::new(layout)),
            bottom_face: Some(PlaneMeshBuilder::new(layout)),
            sides_options: [Some(SideOptions::new()); 6],
            center_aligned: false,
        }
    }

    /// Specifies a custom `pos`, which will apply an offset to the whole mesh.
    ///
    /// ## Note
    ///
    /// It might be more optimal to generate only one mesh at [`Hex::ZERO`] and
    /// offset it later than have one mesh per hex position
    #[must_use]
    #[inline]
    pub const fn at(mut self, pos: Hex) -> Self {
        self.pos = pos;
        self
    }

    /// Specify a custom *facing* direction for the mesh, by default the column
    /// is vertical (facing up)
    ///
    /// # Panics
    ///
    /// Will panic if `facing` is zero length
    #[must_use]
    #[inline]
    pub fn facing(mut self, facing: Vec3) -> Self {
        self.rotation = Some(Quat::from_rotation_arc(BASE_FACING, facing));
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
    ///
    /// # Example
    ///
    /// To center the pivot you can offset the mesh by half its height:
    ///
    /// ```rust
    /// # use hexx::*;
    ///
    /// let layout = HexLayout::default();
    /// let height = 10.0;
    /// let mesh = ColumnMeshBuilder::new(&layout, height)
    ///     .with_offset(Vec3::new(0.0, -height / 2.0, 0.0))
    ///     .build();
    /// ```
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

    /// Defines the column side quads amount
    #[must_use]
    #[inline]
    pub const fn with_subdivisions(mut self, subdivisions: usize) -> Self {
        self.subdivisions = Some(subdivisions);
        self
    }

    /// The mesh will not include a *bottom* hexagon face
    #[must_use]
    #[inline]
    pub const fn without_bottom_face(mut self) -> Self {
        self.bottom_face = None;
        self
    }

    /// The mesh will not include a *top* hexagon face
    #[must_use]
    #[inline]
    pub const fn without_top_face(mut self) -> Self {
        self.top_face = None;
        self
    }

    #[must_use]
    #[inline]
    /// Specify custom uv options for the top/bottom caps triangles
    ///
    /// Note:
    /// this won't have any effect if `top_cap` and `bottom_cap` are disabled
    pub const fn with_caps_uv_options(mut self, uv_options: UVOptions) -> Self {
        if let Some(builder) = self.top_face {
            self.top_face = Some(builder.with_uv_options(uv_options));
        }
        if let Some(builder) = self.bottom_face {
            self.bottom_face = Some(builder.with_uv_options(uv_options));
        }
        self
    }

    /// Specify inset option for the top/bottom caps faces
    ///
    /// Note:
    /// this won't have any effect if `top_cap` and `bottom_cap` are disabled
    #[must_use]
    #[inline]
    pub const fn with_caps_inset_options(mut self, opts: InsetOptions) -> Self {
        if let Some(builder) = self.top_face {
            self.top_face = Some(builder.with_inset_options(opts));
        }
        if let Some(builder) = self.bottom_face {
            self.bottom_face = Some(builder.with_inset_options(opts));
        }
        self
    }

    #[must_use]
    #[inline]
    /// Specify custom global options for the side quad triangles.
    ///
    /// To customize each side quad, prefer
    /// [`Self::with_multi_sides_options`]
    pub const fn with_sides_options(mut self, options: SideOptions) -> Self {
        self.sides_options = [Some(options); 6];
        self
    }

    #[must_use]
    #[inline]
    #[allow(clippy::cast_possible_truncation)]
    /// Specify custom options for each of the side quad triangles.
    ///
    /// For a global setting prefer [`Self::with_sides_options`]
    pub fn with_sides_options_fn(
        mut self,
        options: impl Fn(EdgeDirection) -> Option<SideOptions>,
    ) -> Self {
        self.sides_options = std::array::from_fn(|i| options(EdgeDirection(i as u8)));
        self
    }

    #[must_use]
    #[inline]
    /// Specify options for each of the side quad triangles.
    ///
    /// For a global setting prefer [`Self::with_sides_options`]
    pub fn with_multi_sides_options(mut self, options: [SideOptions; 6]) -> Self {
        self.sides_options = options.map(Some);
        self
    }

    #[must_use]
    #[inline]
    /// Specify custom options for each of the side quad triangles.
    ///
    /// For a global setting prefer [`Self::with_sides_options`]
    pub const fn with_multi_custom_sides_options(
        mut self,
        options: [Option<SideOptions>; 6],
    ) -> Self {
        self.sides_options = options;
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

    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    #[allow(clippy::many_single_char_names)]
    /// Comsumes the builder to return the computed mesh data
    pub fn build(self) -> MeshInfo {
        // We store the offset to match the `self.pos`
        let pos = if self.center_aligned {
            self.layout.hex_to_center_aligned_world_pos(self.pos)
        } else {
            self.layout.hex_to_world_pos(self.pos)
        };
        let mut offset = Vec3::new(pos.x, 0.0, pos.y);
        // We create the final mesh
        let mut mesh = MeshInfo::default();
        // Column sides
        let subidivisions = self.subdivisions.unwrap_or(0).max(1);
        let delta = self.height / subidivisions as f32;
        let [a, b, c, d, e, f] = self.layout.center_aligned_hex_corners();
        let corners = [[a, b], [b, c], [c, d], [d, e], [e, f], [f, a]];
        (0..6).for_each(|side| {
            let [left, right] = corners[side];
            let Some(options) = self.sides_options[side] else {
                return;
            };
            let normal = (left + right).normalize();
            for div in 0..subidivisions {
                let height = delta * div as f32;
                let left = Vec3::new(left.x, height, left.y);
                let right = Vec3::new(right.x, height, right.y);
                let mut quad =
                    Quad::from_bottom([left, right], Vec3::new(normal.x, 0.0, normal.y), delta);
                options.uv.alter_uvs(&mut quad.uvs);
                let quad = if let Some(opts) = options.insetting {
                    quad.inset(opts.mode, opts.scale, opts.keep_inner_face)
                } else {
                    quad.into()
                };
                mesh.merge_with(quad);
            }
        });
        // Hexagon top face
        if let Some(builder) = self.top_face {
            mesh.merge_with(
                builder
                    .center_aligned()
                    .with_offset(Vec3::Y * self.height)
                    .build(),
            );
        }
        // Hexagon bottom face
        if let Some(builder) = self.bottom_face {
            let rotation = Quat::from_rotation_arc(BASE_FACING, -BASE_FACING);
            let bottom_face = builder.center_aligned().build().rotated(rotation);
            mesh.merge_with(bottom_face);
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
        if let Some(custom_offset) = self.offset {
            offset += custom_offset;
        }
        mesh = mesh.with_offset(offset);
        mesh
    }
}
