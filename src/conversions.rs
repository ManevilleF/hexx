use crate::Hex;

/// Layout mode for [doubled] coordinates conversion. See [`Hex::to_doubled_coordinates`] and
/// [`Hex::from_doubled_coordinates`].
///
/// [doubled]: https://www.redblobgames.com/grids/hexagons/#coordinates-doubled
#[derive(Debug, Clone, Copy, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DoubledHexMode {
    /// Doubles column values
    #[default]
    DoubledWidth,
    /// Doubles row values
    DoubledHeight,
}

/// Layout mode for [offset] coordinates conversion. See [`Hex::to_offset_coordinates`] and
/// [`Hex::from_offset_coordinates`].
///
/// [offset]: https://www.redblobgames.com/grids/hexagons/#coordinates-offset
#[derive(Debug, Clone, Copy, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum OffsetHexMode {
    /// Vertical layout, shoves even columns down
    EvenColumns,
    /// Vertical layout, shoves odd columns down
    OddColumns,
    /// Horizontal layout, shoves even rows right
    EvenRows,
    #[default]
    /// Horizontal layout, shoves odd rows right
    OddRows,
}

impl Hex {
    /// Converts `self` to [doubled] coordinates according to the given `mode`.
    ///
    /// The coordinates are returned as `[COLUMN, ROW]`
    ///
    /// [doubled]: https://www.redblobgames.com/grids/hexagons/#coordinates-doubled
    #[inline]
    #[must_use]
    pub const fn to_doubled_coordinates(self, mode: DoubledHexMode) -> [i32; 2] {
        match mode {
            DoubledHexMode::DoubledWidth => [2 * self.x + self.y, self.y],
            DoubledHexMode::DoubledHeight => [self.x, 2 * self.y + self.x],
        }
    }

    /// Converts `self` to [offset] coordinates according to the given `mode`.
    ///
    /// The coordinates are returned as `[COLUMN, ROW]`
    ///
    /// [offset]: https://www.redblobgames.com/grids/hexagons/#coordinates-offset
    #[inline]
    #[must_use]
    pub const fn to_offset_coordinates(self, mode: OffsetHexMode) -> [i32; 2] {
        match mode {
            OffsetHexMode::EvenColumns => [self.x, self.y + (self.x + (self.x & 1)) / 2],
            OffsetHexMode::OddColumns => [self.x, self.y + (self.x - (self.x & 1)) / 2],
            OffsetHexMode::EvenRows => [self.x + (self.y + (self.y & 1)) / 2, self.y],
            OffsetHexMode::OddRows => [self.x + (self.y - (self.y & 1)) / 2, self.y],
        }
    }

    /// Converts [doubled] to [axial] coordinates according to the given `mode`.
    ///
    /// [doubled]: https://www.redblobgames.com/grids/hexagons/#coordinates-doubled
    /// [axial]: https://www.redblobgames.com/grids/hexagons/#coordinates-axial
    #[inline]
    #[must_use]
    pub const fn from_doubled_coordinates([col, row]: [i32; 2], mode: DoubledHexMode) -> Self {
        match mode {
            DoubledHexMode::DoubledWidth => Self::new((col - row) / 2, row),
            DoubledHexMode::DoubledHeight => Self::new(col, (row - col) / 2),
        }
    }

    /// Converts [offset] to [axial] coordinates according to the given `mode`.
    ///
    /// [offset]: https://www.redblobgames.com/grids/hexagons/#coordinates-offset
    /// [axial]: https://www.redblobgames.com/grids/hexagons/#coordinates-axial
    #[inline]
    #[must_use]
    pub const fn from_offset_coordinates([col, row]: [i32; 2], mode: OffsetHexMode) -> Self {
        match mode {
            OffsetHexMode::EvenColumns => Self::new(col, row - (col + (col & 1)) / 2),
            OffsetHexMode::OddColumns => Self::new(col, row - (col - (col & 1)) / 2),
            OffsetHexMode::EvenRows => Self::new(col - (row + (row & 1)) / 2, row),
            OffsetHexMode::OddRows => Self::new(col - (row - (row & 1)) / 2, row),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn doubled_coordinates() {
        for hex in Hex::ZERO.range(10) {
            for mode in [DoubledHexMode::DoubledWidth, DoubledHexMode::DoubledHeight] {
                let doubled = hex.to_doubled_coordinates(mode);
                let converted = Hex::from_doubled_coordinates(doubled, mode);
                assert_eq!(converted, hex);
            }
        }
    }

    #[test]
    fn offset_coordinates() {
        for hex in Hex::ZERO.range(10) {
            for mode in [
                OffsetHexMode::OddRows,
                OffsetHexMode::OddColumns,
                OffsetHexMode::EvenColumns,
                OffsetHexMode::EvenRows,
            ] {
                let offset = hex.to_offset_coordinates(mode);
                let converted = Hex::from_offset_coordinates(offset, mode);
                assert_eq!(converted, hex);
            }
        }
    }
}
