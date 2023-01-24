use crate::{hexagon, Hex};

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
        hex.wrap_with(self.radius, &self.mirrors)
    }

    /// Returns an iterator with all the coordinates in the map bounds
    pub fn all_coords(&self) -> impl Iterator<Item = Hex> + '_ {
        hexagon(self.radius).map(|h| h + self.center)
    }
}
