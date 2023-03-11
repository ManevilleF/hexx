use crate::Hex;

/// Hexagonal bounds utils, representer as a center and radius.
/// This type can be defined manually or from a [`Hex`] iterator.
///
/// # Example
///
/// ```rust
/// # use hexx::*;
///
/// let iter = Hex::ZERO.line_to(Hexx:new(123, -456));
/// // You can compute the bounds of `iter`
/// let bounds: HexBounds = iter.collect();
/// ```
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "ser_de", derive(serde::Serialize, serde::Deserialize))]
pub struct HexBounds {
    /// bounds center
    pub center: Hex,
    /// bounds radius
    pub radius: u32,
}

impl HexBounds {
    /// Instantiates new bounds from a `center` and `radius`
    #[inline]
    #[must_use]
    pub const fn new(center: Hex, radius: u32) -> Self {
        Self { center, radius }
    }

    /// Computes the bounds `min` and `max`
    #[inline]
    #[must_use]
    pub fn from_min_max(min: Hex, max: Hex) -> Self {
        let center = (min + max) / 2;
        let radius = center.unsigned_distance_to(max) / 2;
        Self { center, radius }
    }

    #[inline]
    #[must_use]
    /// Checks if `rhs` is in bounds
    pub const fn is_in_bounds(&self, rhs: Hex) -> bool {
        self.center.unsigned_distance_to(rhs) <= self.radius
    }

    #[must_use]
    #[inline]
    #[doc(alias = "coords_count")]
    #[doc(alias = "len")]
    /// Returns the number of hexagons in bounds
    pub const fn hex_count(&self) -> usize {
        Hex::range_count(self.radius)
    }

    #[doc(alias = "all_items")]
    #[must_use]
    /// Returns an iterator with all the coordinates in bounds
    pub fn all_coords(&self) -> impl ExactSizeIterator<Item = Hex> {
        self.center.range(self.radius)
    }

    /// Computes all coordinates in the intersection between `self` and `rhs`
    pub fn intersecting_with(self, rhs: Self) -> impl Iterator<Item = Hex> {
        let [start, end] = if self.radius > rhs.radius {
            [rhs, self]
        } else {
            [self, rhs]
        };
        start.all_coords().filter(move |h| end.is_in_bounds(*h))
    }
}

impl FromIterator<Hex> for HexBounds {
    fn from_iter<T: IntoIterator<Item = Hex>>(iter: T) -> Self {
        let mut min = Hex::new(i32::MAX, i32::MAX);
        let mut max = Hex::new(i32::MIN, i32::MIN);

        for hex in iter {
            min.x = min.x.min(hex.x);
            max.x = max.x.max(hex.x);
            min.y = min.y.min(hex.y);
            max.y = max.y.max(hex.y);
        }
        Self::from_min_max(min, max)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn in_bounds_work() {
        let bounds = HexBounds::new(Hex::new(-4, 23), 34);
        for h in bounds.all_coords() {
            assert!(bounds.is_in_bounds(h));
        }
    }

    #[test]
    fn intersecting_with() {
        let ba = HexBounds::new(Hex::ZERO, 3);
        let bb = HexBounds::new(Hex::new(4, 0), 3);
        let intersection = ba.intersecting_with(bb);
        assert_eq!(intersection.count(), 9);
    }
}
