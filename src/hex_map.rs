use crate::Hex;

/// Hexagon shaped map with [wraparound] utils.
///
/// [wraparound]: https://www.redblobgames.com/grids/hexagons/#wraparound
#[derive(Debug, Clone)]
#[cfg_attr(feature = "ser_de", derive(serde::Serialize, serde::Deserialize))]
pub struct HexMap {
    /// The map radius
    radius: i32,
    /// The map center
    center: Hex,
    /// The 6 mirror centers, used to wrap coordinates
    mirrors: [Hex; 6],
}

impl HexMap {
    #[inline]
    #[must_use]
    /// Creates a new hexagonal map of the given `radius` with [`Hex::ZERO`] as its center
    pub const fn new(radius: i32) -> Self {
        Self {
            radius,
            center: Hex::ZERO,
            mirrors: Hex::wraparound_mirrors(radius),
        }
    }

    #[inline]
    #[must_use]
    /// Returns the map with a custom `center`
    ///
    /// # Usage
    ///
    /// ```rust
    /// # use hexx::*;
    /// let map = HexMap::new(10).with_center(Hex::new(5, -5));
    /// ```
    pub fn with_center(self, center: Hex) -> Self {
        Self {
            radius: self.radius,
            center,
            mirrors: self.mirrors.map(|h| h + center),
        }
    }

    #[inline]
    #[must_use]
    /// Returns the map center coordinates
    pub const fn center(&self) -> Hex {
        self.center
    }

    #[inline]
    #[must_use]
    /// Returns the map radius
    pub const fn radius(&self) -> i32 {
        self.radius
    }

    /// Wraps `hex` in the given map radius.
    /// this allows for seamless *wraparound* hexagonal maps.
    /// See this [article] for more information.
    ///
    /// [article]: https://www.redblobgames.com/grids/hexagons/#wraparound
    #[must_use]
    pub fn wrapped_hex(&self, hex: Hex) -> Hex {
        let pos = hex - self.center;
        let pos = pos.wrap_with(self.radius, &self.mirrors);
        pos + self.center
    }

    /// Computes the neighbors of `hex` wrapped in the map bounds.
    /// See [`Self::wrapped_hex`]
    #[must_use]
    #[inline]
    pub fn wrapped_neighbors(&self, hex: Hex) -> [Hex; 6] {
        hex.all_neighbors().map(|h| self.wrapped_hex(h))
    }

    #[must_use]
    #[inline]
    /// Returns the number of hexagons in the map
    pub const fn hex_count(&self) -> i32 {
        Hex::range_count(self.radius)
    }

    /// Returns an iterator with all the coordinates in the map bounds
    pub fn all_coords(&self) -> impl Iterator<Item = Hex> {
        self.center.range(self.radius)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrapping_works() {
        let map = HexMap::new(3);

        assert_eq!(map.wrapped_hex(Hex::new(0, 4)), Hex::new(-3, 0));
        assert_eq!(map.wrapped_hex(Hex::new(4, 0)), Hex::new(-3, 3));
        assert_eq!(map.wrapped_hex(Hex::new(4, -4)), Hex::new(0, 3));
    }

    #[test]
    fn wrapping_outside_works() {
        let map = HexMap::new(2);

        assert_eq!(map.wrapped_hex(Hex::new(3, 0)), Hex::new(-2, 2));
        assert_eq!(map.wrapped_hex(Hex::new(5, 0)), Hex::new(0, 2));
        assert_eq!(map.wrapped_hex(Hex::new(6, 0)), Hex::new(-1, -1));

        assert_eq!(map.wrapped_hex(Hex::new(2, 3)), Hex::new(0, 0)); // mirror
        assert_eq!(map.wrapped_hex(Hex::new(4, 6)), Hex::new(0, 0));
    }
}
