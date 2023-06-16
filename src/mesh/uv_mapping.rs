use glam::Vec2;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "ser_de", derive(serde::Serialize, serde::Deserialize))]
/// Struct containing options for UV mapping.
///
/// # Usage
///
/// ```rust
/// # use hexx::*;
/// # use glam::*;
///
/// let options = UVOptions::new()
///     .with_offset(vec2(1.0, 0.5))
///     .flip_u()
///     .with_scale_factor(vec2(1.0, 2.0));
/// ```    
pub struct UVOptions {
    /// The scale factor for the UV coordinates.
    /// * the `x` value applies to `u`
    /// * the `y` value applies to `v`
    pub scale_factor: Vec2,
    /// Flag indicating whether to flip the UV coordinates along the U-axis.
    pub flip_u: bool,
    /// Flag indicating whether to flip the UV coordinates along the V-axis.
    pub flip_v: bool,
    /// The offset value of the UV coordinates.
    /// * the `x` value applies to `u`
    /// * the `y` value applies to `v`
    pub offset: Vec2,
}

impl UVOptions {
    /// Setup new uv options with default values
    #[must_use]
    pub const fn new() -> Self {
        Self {
            scale_factor: Vec2::ONE,
            flip_u: false,
            flip_v: false,
            offset: Vec2::ZERO,
        }
    }

    /// Defines custom UV scale factor.
    /// * the `x` value will scale `u`
    /// * the `y` value will scale `v`
    #[must_use]
    #[inline]
    pub const fn with_scale_factor(mut self, scale_factor: Vec2) -> Self {
        self.scale_factor = scale_factor;
        self
    }

    /// Defines custom UV offset
    /// * the `x` value will apply to `u`
    /// * the `y` value will apply to `v`
    #[must_use]
    #[inline]
    pub const fn with_offset(mut self, offset: Vec2) -> Self {
        self.offset = offset;
        self
    }

    /// The `u` value will be flipped
    #[must_use]
    #[inline]
    pub const fn flip_u(mut self) -> Self {
        self.flip_u = true;
        self
    }

    /// The `v` value will be flipped
    #[must_use]
    #[inline]
    pub const fn flip_v(mut self) -> Self {
        self.flip_v = true;
        self
    }

    /// Apply the options to `uv`, returning the new value as a [`Vec2`]
    #[must_use]
    pub fn alter_uv(&self, mut uv: Vec2) -> Vec2 {
        if self.flip_u {
            uv.x = 1.0 - uv.x;
        }
        if self.flip_v {
            uv.y = 1.0 - uv.y;
        }
        uv * self.scale_factor + self.offset
    }

    /// Apply the options to all UV coords in `uvs`
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
