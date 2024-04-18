use crate::Hex;
use glam::{IVec2, IVec3, Vec2};

impl From<(i32, i32)> for Hex {
    #[inline]
    fn from((x, y): (i32, i32)) -> Self {
        Self::new(x, y)
    }
}

impl From<[i32; 2]> for Hex {
    #[inline]
    fn from(a: [i32; 2]) -> Self {
        Self::from_array(a)
    }
}

impl From<(f32, f32)> for Hex {
    #[inline]
    fn from(v: (f32, f32)) -> Self {
        Self::round(v.into())
    }
}

impl From<[f32; 2]> for Hex {
    #[inline]
    fn from(v: [f32; 2]) -> Self {
        Self::round(v)
    }
}

impl From<Hex> for IVec2 {
    #[inline]
    fn from(hex: Hex) -> Self {
        hex.as_ivec2()
    }
}

impl From<Vec2> for Hex {
    #[inline]
    fn from(value: Vec2) -> Self {
        Self::from(value.to_array())
    }
}

impl From<Hex> for IVec3 {
    #[inline]
    fn from(hex: Hex) -> Self {
        hex.as_ivec3()
    }
}

impl From<IVec2> for Hex {
    #[inline]
    fn from(v: IVec2) -> Self {
        Self::new(v.x, v.y)
    }
}

impl Hex {
    /// Unpack from a [`u64`].
    /// [x][`Hex::x`] is read from the most signifigant 32 bits; [x][`Hex::y`] is read from the least signifigant 32 bits.
    /// Intended to be used with [`Hex::as_u64`].
    ///
    /// # Example
    ///
    /// ```
    /// # use hexx::*;
    /// let x: u64 = 0x000000AA_FFFFFF45;
    /// assert_eq!(Hex::from_u64(x), Hex::new(0xAA, -0xBB));
    /// ```
    #[inline]
    #[doc(alias = "unpack")]
    pub fn from_u64(value: u64) -> Self {
        let x = (value >> 32) as i32;
        let y = (value & 0xFFFF_FFFF) as i32;
        Self::new(x, y)
    }

    /// Pack into a [`u64`].
    /// [x][`Hex::x`] is placed in the most signifigant 32 bits; [y][`Hex::y`] is placed in the least signifigant 32 bits.
    /// Can be used as a sort key, or for saving in a binary format.
    /// Intended to be used with [`Hex::from_u64`].
    ///
    /// # Example
    ///
    /// ```
    /// # use hexx::*;
    /// let x = Hex::new(0xAA, -0xBB).as_u64();
    /// assert_eq!(x, 0x000000AA_FFFFFF45u64);
    /// ```
    #[inline]
    #[doc(alias = "pack")]
    pub fn as_u64(self) -> u64 {
        let high = (self.x as u32 as u64) << 32;
        let low = self.y as u32 as u64;
        high | low
    }
}
