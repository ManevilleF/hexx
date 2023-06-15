use glam::Vec2;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "ser_de", derive(serde::Serialize, serde::Deserialize))]
/// Struct containing options for UV mapping.
pub struct UVOptions {
    /// The scale factor for the UV coordinates.
    pub scale_factor: Vec2,
    /// Flag indicating whether to flip the UV coordinates along the U-axis.
    pub flip_u: bool,
    /// Flag indicating whether to flip the UV coordinates along the V-axis.
    pub flip_v: bool,
    /// The offset value of the UV coordinates.
    pub offset: Vec2,
}

impl UVOptions {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            scale_factor: Vec2::ONE,
            flip_u: false,
            flip_v: false,
            offset: Vec2::ZERO,
        }
    }

    #[must_use]
    #[inline]
    pub const fn with_scale_factor(mut self, scale_factor: Vec2) -> Self {
        self.scale_factor = scale_factor;
        self
    }

    #[must_use]
    #[inline]
    pub const fn with_offset(mut self, offset: Vec2) -> Self {
        self.offset = offset;
        self
    }

    #[must_use]
    #[inline]
    pub const fn flip_u(mut self) -> Self {
        self.flip_u = true;
        self
    }

    #[must_use]
    #[inline]
    pub const fn flip_v(mut self) -> Self {
        self.flip_v = true;
        self
    }

    #[must_use]
    pub fn alter_uv(&self, mut uv: Vec2) -> Vec2 {
        if self.flip_u {
            uv.x = 1.0 - uv.x;
        }
        if self.flip_v {
            uv.y = 1.0 - uv.y;
        }
        (uv + self.offset) * self.scale_factor
    }

    pub fn alter_uvs(&self, uvs: &mut Vec<Vec2>) {
        for uv in uvs {
            *uv = self.alter_uv(*uv);
        }
    }
}

impl Default for UVOptions {
    fn default() -> Self {
        Self::new()
    }
}
