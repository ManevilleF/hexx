use crate::{Hex, HexOrientation};

/// Layout mode for [doubled] coordinates conversion. See
/// [`Hex::to_doubled_coordinates`] and [`Hex::from_doubled_coordinates`].
///
/// [doubled]: https://www.redblobgames.com/grids/hexagons/#coordinates-doubled
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub enum DoubledHexMode {
    /// Doubles column values
    #[default]
    DoubledWidth,
    /// Doubles row values
    DoubledHeight,
}

/// Layout mode for [offset] coordinates conversion. See
/// [`Hex::to_offset_coordinates`] and [`Hex::from_offset_coordinates`].
///
/// [offset]: https://www.redblobgames.com/grids/hexagons/#coordinates-offset
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub enum OffsetHexMode {
    /// Depending on the orientation:
    ///
    /// * Flat: Vertical layout, shoves even columns down
    /// * Pointy: Horizontal layout, shoves even rows right
    Even,
    /// Depending on the orientation:
    ///
    /// * Flat: Vertical layout, shoves odd columns down
    /// * Pointy: Horizontal layout, shoves odd rows right
    Odd,
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

    /// Converts `self` to [offset] coordinates according to the given `mode`
    /// and `orientation`
    ///
    /// The coordinates are returned as `[COLUMN, ROW]`
    ///
    /// [offset]: https://www.redblobgames.com/grids/hexagons/#coordinates-offset
    #[inline]
    #[must_use]
    pub const fn to_offset_coordinates(
        self,
        mode: OffsetHexMode,
        orientation: HexOrientation,
    ) -> [i32; 2] {
        match (mode, orientation) {
            (OffsetHexMode::Even, HexOrientation::Flat) => {
                [self.x, self.y + i32::midpoint(self.x, self.x & 1)]
            }
            (OffsetHexMode::Even, HexOrientation::Pointy) => {
                [self.x + i32::midpoint(self.y, self.y & 1), self.y]
            }
            (OffsetHexMode::Odd, HexOrientation::Flat) => {
                [self.x, self.y + (self.x - (self.x & 1)) / 2]
            }
            (OffsetHexMode::Odd, HexOrientation::Pointy) => {
                [self.x + (self.y - (self.y & 1)) / 2, self.y]
            }
        }
    }

    /// Converts `self` to [hexmod] coordinates according to the given `range`
    ///
    /// [hexmod]: https://observablehq.com/@sanderevers/hexmod-representation
    #[inline]
    #[must_use]
    #[expect(clippy::cast_possible_wrap)]
    pub const fn to_hexmod_coordinates(self, range: u32) -> u32 {
        let area = Self::range_count(range) as i32;
        let shift = Self::shift(range) as i32;
        (self.y + shift * self.x).rem_euclid(area) as u32
    }

    /// Converts [hexmod] to [axial] coordinates according to the given `range`
    ///
    /// # Note
    ///
    /// The resulting coordinate will be wrong if `coord` is not a valid hexmod
    /// value in the given `range`.
    /// `coord` should be lesser or equal to `3 * range * (range + 1) + 1`
    ///
    /// [hexmod]: https://observablehq.com/@sanderevers/hexmod-representation
    /// [axial]: https://www.redblobgames.com/grids/hexagons/#coordinates-axial
    #[must_use]
    #[expect(clippy::cast_possible_wrap)]
    pub const fn from_hexmod_coordinates(coord: u32, range: u32) -> Self {
        let shift = Self::shift(range) as i32;
        let range = range as i32;
        let coord = coord as i32;
        let ms = (coord + range) / shift;
        let mcs = (coord + 2 * range) / (shift - 1);
        Self::new(
            ms * (range + 1) + mcs * -range,
            coord + ms * (-2 * range - 1) + mcs * (-range - 1),
        )
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

    /// Converts [offset] to [axial] coordinates according to the given `mode`
    /// and `orientation`
    ///
    /// [offset]: https://www.redblobgames.com/grids/hexagons/#coordinates-offset
    /// [axial]: https://www.redblobgames.com/grids/hexagons/#coordinates-axial
    #[inline]
    #[must_use]
    pub const fn from_offset_coordinates(
        [col, row]: [i32; 2],
        mode: OffsetHexMode,
        orientation: HexOrientation,
    ) -> Self {
        match (mode, orientation) {
            (OffsetHexMode::Even, HexOrientation::Flat) => {
                Self::new(col, row - i32::midpoint(col, col & 1))
            }
            (OffsetHexMode::Odd, HexOrientation::Flat) => {
                Self::new(col, row - (col - (col & 1)) / 2)
            }
            (OffsetHexMode::Even, HexOrientation::Pointy) => {
                Self::new(col - i32::midpoint(row, row & 1), row)
            }
            (OffsetHexMode::Odd, HexOrientation::Pointy) => {
                Self::new(col - (row - (row & 1)) / 2, row)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn doubled_coordinates() {
        for hex in Hex::ZERO.range(20) {
            for mode in [DoubledHexMode::DoubledWidth, DoubledHexMode::DoubledHeight] {
                let doubled = hex.to_doubled_coordinates(mode);
                let converted = Hex::from_doubled_coordinates(doubled, mode);
                assert_eq!(converted, hex);
            }
        }
    }

    #[test]
    fn offset_coordinates() {
        for hex in Hex::ZERO.range(20) {
            for mode in [OffsetHexMode::Even, OffsetHexMode::Odd] {
                for orientation in [HexOrientation::Flat, HexOrientation::Pointy] {
                    let offset = hex.to_offset_coordinates(mode, orientation);
                    let converted = Hex::from_offset_coordinates(offset, mode, orientation);
                    assert_eq!(converted, hex);
                }
            }
        }
    }

    #[test]
    fn hexmod_coordinates() {
        let range = 20;
        for hex in Hex::ZERO.range(range) {
            let hexmod = hex.to_hexmod_coordinates(range);
            let converted = Hex::from_hexmod_coordinates(hexmod, range);
            assert_eq!(converted, hex);
        }
    }
}
