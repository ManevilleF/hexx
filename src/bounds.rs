use crate::Hex;

/// Hexagonal bounds utils, representer as a center and radius
#[derive(Debug, Clone)]
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
        let [x, y] = rhs.const_sub(self.center).to_array();
        x.unsigned_abs() <= self.radius && y.unsigned_abs() <= self.radius
    }

    #[must_use]
    #[inline]
    /// Returns the number of hexagons in bounds
    pub const fn hex_count(&self) -> usize {
        Hex::range_count(self.radius)
    }

    #[doc(alias = "all_items")]
    /// Returns an iterator with all the coordinates in bounds
    pub fn all_coords(&self) -> impl Iterator<Item = Hex> {
        self.center.range(self.radius)
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
}
