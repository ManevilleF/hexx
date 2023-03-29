use super::Hex;

const Y_OPERAND: u64 = 32;
const X_MASK: u64 = 0xFFFF_FFFF;

/// Bit packed axial coordinates
/// This struct provides 1-1 mapping with [`Hex`] through [`Hex::packed`] and [`PackedHex::unpacked`]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "ser_de", derive(serde::Serialize, serde::Deserialize))]
pub struct PackedHex(u64);

impl Hex {
    #[inline]
    #[must_use]
    /// Packs the coordinates in a single `u64`
    pub const fn packed(self) -> PackedHex {
        PackedHex::new(self.x, self.y)
    }
}

#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
impl PackedHex {
    #[inline]
    #[must_use]
    /// Instantiates a new hexagon with packed axial coordinates
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    /// let coord = PackedHex::new(3, 5);
    /// assert_eq!(coord.x(), 3);
    /// assert_eq!(coord.y(), 5);
    /// ```
    pub const fn new(x: i32, y: i32) -> Self {
        Self((x as u64) & X_MASK | ((y as u64) << Y_OPERAND))
    }

    #[inline]
    #[must_use]
    /// Unpacks and returns the `x` axial coordinate.
    pub const fn x(self) -> i32 {
        self.0 as i32
    }

    #[inline]
    #[must_use]
    /// Unpacks and returns the `x` axial coordinate.
    pub const fn y(self) -> i32 {
        (self.0 >> Y_OPERAND) as i32
    }

    #[inline]
    #[must_use]
    /// Unpackss the coordinates
    pub const fn unpacked(self) -> Hex {
        Hex {
            x: self.x(),
            y: self.y(),
        }
    }

    #[inline]
    /// Sets the `x` axial coordinate.
    pub fn set_x(&mut self, x: i32) {
        let y = self.y();
        self.0 = (x as u64) & X_MASK | ((y as u64) << Y_OPERAND);
    }

    #[inline]
    /// Sets the `y` axial coordinate.
    pub fn set_y(&mut self, y: i32) {
        let x = self.x();
        self.0 = ((x as u64) & X_MASK) | ((y as u64) << Y_OPERAND);
    }
}

impl From<Hex> for PackedHex {
    #[inline]
    fn from(value: Hex) -> Self {
        value.packed()
    }
}

impl From<PackedHex> for Hex {
    #[inline]
    fn from(value: PackedHex) -> Self {
        value.unpacked()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
        for x in -50..=50 {
            for y in -50..=50 {
                let hex = Hex::new(x, y);
                let packed = hex.packed();
                assert_eq!(packed.unpacked(), hex);
            }
        }
    }

    #[test]
    fn works_with_big_values() {
        let min = i32::MIN;
        let max = i32::MAX;

        let hexes = [
            Hex::splat(min),
            Hex::splat(max),
            Hex::new(min, max),
            Hex::new(max, min),
        ];
        for hex in hexes {
            let packed = hex.packed();
            assert_eq!(packed.unpacked(), hex);
        }
    }

    #[test]
    fn setter_works() {
        let mut packed = PackedHex::new(-1234, 2345);
        assert_eq!(packed.x(), -1234);
        assert_eq!(packed.y(), 2345);
        packed.set_x(654_321);
        assert_eq!(packed.x(), 654_321);
        assert_eq!(packed.y(), 2345);
        packed.set_y(-998_877);
        assert_eq!(packed.x(), 654_321);
        assert_eq!(packed.y(), -998_877);
    }
}
