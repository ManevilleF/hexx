use glam::{BVec2, Vec2};

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
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
///
/// # Order of operations
///
/// - [`Self::flip`]
/// - [`Self::scale_factor`]
/// - [`Self::offset`]
/// - [`Self::rect`]
pub struct UVOptions {
    /// The scale factor for the UV coordinates.
    /// * the `x` value applies to `u`
    /// * the `y` value applies to `v`
    pub scale_factor: Vec2,
    /// Flag indicating whether to flip the UV
    /// * the `x` value applies to `u`
    /// * the `y` value applies to `v`
    pub flip: BVec2,
    /// The offset value of the UV coordinates.
    /// * the `x` value applies to `u`
    /// * the `y` value applies to `v`
    pub offset: Vec2,
    /// Subsection of the texture the UV coordinates for remapping.
    ///
    /// Defaults to (0,0) -> (1, 1)
    pub rect: Rect,
}

/// 2D rect, with remapping utilities
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub struct Rect {
    /// minimum coordinate
    /// * the `x` value applies to `u`
    /// * the `y` value applies to `v`
    pub min: Vec2,
    /// maximum coordinate
    /// * the `x` value applies to `u`
    /// * the `y` value applies to `v`
    pub max: Vec2,
}

impl UVOptions {
    /// Setup new uv options with default values
    #[must_use]
    pub const fn new() -> Self {
        Self {
            scale_factor: Vec2::ONE,
            flip: BVec2::FALSE,
            offset: Vec2::ZERO,
            rect: Rect::new_uv(),
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

    /// Defines a custom section of the texture for remapping.
    /// The UV coordinate will be mapped between `min` and `max`.
    #[must_use]
    #[inline]
    pub const fn with_rect(mut self, min: Vec2, max: Vec2) -> Self {
        self.rect = Rect { min, max };
        self
    }

    /// The `u` value will be flipped
    #[must_use]
    #[inline]
    pub const fn flip_u(mut self) -> Self {
        self.flip.x = true;
        self
    }

    /// The `v` value will be flipped
    #[must_use]
    #[inline]
    pub const fn flip_v(mut self) -> Self {
        self.flip.y = true;
        self
    }

    /// Apply the options to `uv`, returning the new value as a [`Vec2`]
    ///
    /// Are applied in order:
    /// - [`Self::flip`]
    /// - [`Self::scale_factor`]
    /// - [`Self::offset`]
    /// - [`Self::rect`]
    #[must_use]
    pub fn alter_uv(&self, mut uv: Vec2) -> Vec2 {
        if self.flip.x {
            uv.x = 1.0 - uv.x;
        }
        if self.flip.y {
            uv.y = 1.0 - uv.y;
        }
        uv = uv * self.scale_factor + self.offset;
        uv = self.rect.remap(uv);
        uv
    }

    /// Apply the options to all UV coords in `uvs`
    pub fn alter_uvs(&self, uvs: &mut Vec<Vec2>) {
        for uv in uvs {
            *uv = self.alter_uv(*uv);
        }
    }

    /// Default values for hexagonal planes or column caps
    #[must_use]
    #[deprecated(since = "0.14.0", note = "Use `UVOptions::new` instead")]
    pub const fn cap_default() -> Self {
        Self::new()
    }

    /// Default values for quads
    #[must_use]
    #[deprecated(since = "0.14.0", note = "Use `UVOptions::new` instead")]
    pub const fn quad_default() -> Self {
        Self::new()
    }

    /// This function maps `p` to be normalized and between 0 and 1
    #[inline]
    pub(crate) fn wrap_uv(p: Vec2) -> Vec2 {
        let p = p.try_normalize().unwrap_or(p);
        (p / 2.0) + Vec2::splat(0.5)
    }
}

impl Default for UVOptions {
    fn default() -> Self {
        Self::new()
    }
}

/// Remaps `value` (0.0..1.0) to (min..max)
#[inline]
fn remap(value: f32, min: f32, max: f32) -> f32 {
    (max - min).mul_add(value, min)
}

impl Rect {
    #[inline]
    #[must_use]
    pub(crate) const fn new_uv() -> Self {
        Self {
            min: Vec2::ZERO,
            max: Vec2::ONE,
        }
    }
    /// Remaps `value` (0.0..1.0) to the rect
    #[inline]
    #[must_use]
    pub(crate) fn remap(&self, value: Vec2) -> Vec2 {
        Vec2::new(
            remap(value.x, self.min.x, self.max.x),
            remap(value.y, self.min.y, self.max.y),
        )
    }
}

impl Default for Rect {
    fn default() -> Self {
        Self::new_uv()
    }
}
