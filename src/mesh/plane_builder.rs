use super::{MeshInfo, BASE_FACING};
use crate::{Hex, HexLayout, UVOptions};
use glam::{Quat, Vec3};

/// Builder struct to customize hex plane mesh generation.
///
/// The mesh will be anchored at the center of the hexagon, use offsets to
/// cutomize anchor/pivot position.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub struct PlaneMeshBuilder<'l> {
    /// The hexagonal layout, used to compute vertex positions
    pub layout: &'l HexLayout,
    /// Custom hex position, will apply an offset if not [`Hex::ZERO`]
    pub pos: Hex,
    /// Optional custom offset for the mesh vertex positions
    pub offset: Option<Vec3>,
    /// Optional custom scale factor for the mesh vertex positions
    pub scale: Option<Vec3>,
    /// Optional custom facing direction, useful to have the mesh already
    /// rotated.
    ///
    /// Note that the `scale` factor will be applied before the rotation
    ///
    /// By default the mesh is *facing* up (**Y** axis)
    pub facing: Option<Vec3>,
    /// UV mapping options
    pub uv_options: UVOptions,
}

impl<'l> PlaneMeshBuilder<'l> {
    /// Setup a new builder using the given `layout`
    #[must_use]
    pub const fn new(layout: &'l HexLayout) -> Self {
        Self {
            layout,
            pos: Hex::ZERO,
            facing: None,
            offset: None,
            scale: None,
            uv_options: UVOptions::cap_default(),
        }
    }

    /// Specifies a custom `pos`, which will apply an offset to the whole mesh.
    ///
    /// ## Note
    ///
    /// It might be more optimal to generate only one mesh at [`Hex::ZERO`] and
    /// offset it later than have one mesh per hex position
    #[must_use]
    pub const fn at(mut self, pos: Hex) -> Self {
        self.pos = pos;
        self
    }

    /// Specify a custom *facing* direction for the mesh, by default the column
    /// is vertical (facing up)
    ///
    /// Note that the `scale` factor will be applied before the rotation
    #[must_use]
    pub const fn facing(mut self, facing: Vec3) -> Self {
        self.facing = Some(facing);
        self
    }

    /// Specify a custom offset for the whole mesh
    #[must_use]
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

    /// Specify custom UV mapping options
    #[must_use]
    pub const fn with_uv_options(mut self, uv_options: UVOptions) -> Self {
        self.uv_options = uv_options;
        self
    }

    /// Comsumes the builder to return the computed mesh data
    #[must_use]
    pub fn build(self) -> MeshInfo {
        // We compute the mesh at the origin to allow scaling
        let mut mesh = MeshInfo::hexagonal_plane(self.layout, Hex::ZERO);
        // We store the offset to match the `self.pos`
        let pos = self.layout.hex_to_world_pos(self.pos);
        let mut offset = Vec3::new(pos.x, 0.0, pos.y);
        // We apply optional scale
        if let Some(scale) = self.scale {
            mesh.vertices.iter_mut().for_each(|p| *p *= scale);
        }
        // We offset the vertex positions
        if let Some(custom_offset) = self.offset {
            offset += custom_offset;
        }
        mesh = mesh.with_offset(offset);
        if let Some(facing) = self.facing {
            let facing = facing.normalize();
            let rotation = Quat::from_rotation_arc(BASE_FACING, facing);
            mesh = mesh.rotated(rotation);
        }
        self.uv_options.alter_uvs(&mut mesh.uvs);
        mesh
    }
}
