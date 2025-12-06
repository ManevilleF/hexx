use crate::{EdgeDirection, Hex};

/// Hexagonal bounds utils, represented as a center and radius.
/// This type can be defined manually or from a [`Hex`] iterator.
///
/// # Example
///
/// ```rust
/// # use hexx::*;
///
/// let iter = Hex::ZERO.line_to(Hex::new(123, -456));
/// // You can compute the bounds of `iter`
/// let bounds: HexBounds = iter.collect();
/// ```
/// Bounds have [wraparound] features, useful for seamless or repeating maps.
///
/// # Example
///
/// ```rust
/// # use hexx::*;
///
/// let bounds = HexBounds::new(hex(1, 2), 10);
/// // Define a coordinate, even ouside of bounds
/// let point = Hex::new(100, 100);
/// assert!(!bounds.is_in_bounds(point));
/// // Retrieve the wrapped position in the map
/// let wrapped_point = bounds.wrap(point);
/// assert!(bounds.is_in_bounds(wrapped_point));
/// ```
///
/// [wraparound]: https://www.redblobgames.com/grids/hexagons/#wraparound
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
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

    /// Instantiates new bounds from a `radius` at [`Hex::ZERO`]
    #[inline]
    #[must_use]
    pub const fn from_radius(radius: u32) -> Self {
        Self {
            center: Hex::ZERO,
            radius,
        }
    }

    /// Computes the bounds `min` and `max`
    #[inline]
    #[must_use]
    pub fn from_min_max(min: Hex, max: Hex) -> Self {
        let center = (min + max) / 2;
        let radius = center.unsigned_distance_to(max);
        Self { center, radius }
    }

    /// Computes the bounds for `radius` with all coordinates
    /// being positive.
    ///
    /// This can be used for efficient map storage in a 2D vector
    /// disallowing negative coordinates
    #[inline]
    #[must_use]
    #[expect(clippy::cast_possible_wrap)]
    pub const fn positive_radius(radius: u32) -> Self {
        let center = Hex::splat(radius as i32);
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
        Hex::range_count(self.radius) as usize
    }

    #[doc(alias = "all_items")]
    #[must_use]
    /// Returns an iterator with all the coordinates in bounds
    pub fn all_coords(self) -> impl ExactSizeIterator<Item = Hex> {
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

    /// Wraps `coord`, returning a new local coodinate inside of the bounds,
    /// relative to the `center`.
    ///
    /// > This allows for seamless *wraparound* hexagonal maps.
    /// > See this [article] for more information.
    ///
    ///  > See also [`Self::wrap`] for global wrapping
    ///
    /// [article]: https://www.redblobgames.com/grids/hexagons/#wraparound
    #[must_use]
    pub fn wrap_local(&self, coord: Hex) -> Hex {
        let coord = coord - self.center;
        coord.wrap_in_range(self.radius)
    }

    /// Wraps `coord`, returning a new coodinate inside of the bounds.
    ///
    /// > This allows for seamless *wraparound* hexagonal maps.
    /// > See this [article] for more information.
    ///
    ///  > See also [`Self::wrap_local`] for local wrapping
    ///
    /// [article]: https://www.redblobgames.com/grids/hexagons/#wraparound
    #[must_use]
    pub fn wrap(&self, coord: Hex) -> Hex {
        self.wrap_local(coord) + self.center
    }

    /// Returns the 6 corners of the hex bounds in [`EdgeDirection`] order
    #[must_use]
    #[expect(clippy::cast_possible_wrap)]
    pub fn corners(&self) -> [Hex; 6] {
        EdgeDirection::ALL_DIRECTIONS.map(|dir| self.center.const_add(dir * self.radius as i32))
    }
}

impl FromIterator<Hex> for HexBounds {
    fn from_iter<T: IntoIterator<Item = Hex>>(iter: T) -> Self {
        let mut iter = iter.into_iter();

        let Some(first) = iter.next() else {
            // Exit early with a zero radius bounds at the origin
            return Self::from_radius(0);
        };

        // This algorithm is broken into two parts:
        // 1. Calculate the minimum size of the hexagon that can contain all the hexes
        // 2. Position the center of the hexagon

        // Step 1: Calculate the size of the hexagon that can contain all the hexes
        // We need to find the min and max for each axis

        // We can use the first hex as a starting point
        let mut max = first.as_ivec3();
        let mut min = max;

        for hex in iter {
            let hex = hex.as_ivec3();
            max = max.max(hex);
            min = min.min(hex);
        }

        // There are 5 ways the size of the hexagon can be bounded:
        // We need to calculate all of them and take the largest one.

        // 3 of them are calculated from opposite edges of the hexagon,
        // calculated here by taking the diference between the min and max.
        // We only need to convert the largest one to a radius,
        // so we'll find the max element right away.
        let duo_size = (max - min).max_element();

        // The other 2 are from triplets of edges.
        // They account for shapes that are more triangular.
        // Again, we only need to convert the largest one to a radius,
        // so we'll find the maximum of the two right away.
        let trio_size = max.element_sum().max(-min.element_sum());

        // Convert the sizes to radii

        // These steps are like integer division,
        // but we need to always round up
        // instead of truncating.
        let duo_radius = (duo_size + 1) / 2;
        let trio_radius = (trio_size + 2) / 3;

        // This is the minimum radius
        let radius = duo_radius.max(trio_radius);

        // This has completed the first step of the algorithm.
        // Now we need to do the second step:
        // Position the center of the hexagon

        // The center must exist between these extremes.
        let center_min = max - radius;
        let center_max = min + radius;

        // How much wiggle room do we have on these axes
        let range = center_max - center_min;

        // Start with the center at the minimum
        let mut center = center_min;

        // This sum needs to be 0, but it could be negative
        // This is *never* positive.
        let mut sum = center.element_sum();

        // This part of the algorithm considers each axis one step at a time.
        // We ask the question:
        // "Can we fix the sum by moving on just this axis?"
        // If the answer is no, we move as close as we can on this axis,
        // and move on to the next axis.
        // If the answer is yes, we move on this axis and stop.
        //
        // We can actually skip the last axis (z),
        // because we know that `center_max.element_sum() >= 0`
        // and that if the algorithm ran all the way to needing the z axis,
        // we would end up at `center_max` anyway.

        // Can sum be fixed entirely by moving on the x axis?
        if -sum > range.x {
            // No, we need to move on the y axis too
            // Move on the x axis first
            sum += range.x;
            center.x += range.x;

            // Can we fix the sum by moving on the y axis?
            if -sum > range.y {
                // No, we need to move on the z axis too
                center.y += range.y;
                // z is guaranteed to be able to fix the sum.
                // We don't need to calculate it here.
            } else {
                // Yes, we can fix the sum by moving on the y axis
                // Move on the y axis
                center.y -= sum;
            }
        } else {
            // Yes, we can fix the sum by moving on the x axis
            // Move on the x axis
            center.x -= sum;
        }

        // Step 2 is now complete.

        // Convert the `IVec3` back to a `Hex`
        let center = Hex::new(center.x, center.y);
        #[expect(clippy::cast_sign_loss)]
        let radius = radius as u32;
        Self { center, radius }
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

    #[test]
    fn wrapping_works() {
        let map = HexBounds::from_radius(3);

        assert_eq!(map.wrap(Hex::new(0, 4)), Hex::new(-3, 0));
        assert_eq!(map.wrap(Hex::new(4, 0)), Hex::new(-3, 3));
        assert_eq!(map.wrap(Hex::new(4, -4)), Hex::new(0, 3));
    }

    #[test]
    fn wrapping_outside_works() {
        let map = HexBounds::from_radius(2);

        assert_eq!(map.wrap(Hex::new(3, 0)), Hex::new(-2, 2));
        assert_eq!(map.wrap(Hex::new(5, 0)), Hex::new(0, 2));
        assert_eq!(map.wrap(Hex::new(6, 0)), Hex::new(-1, -1));

        assert_eq!(map.wrap(Hex::new(2, 3)), Hex::new(0, 0)); // mirror
        assert_eq!(map.wrap(Hex::new(4, 6)), Hex::new(0, 0));
    }

    #[test]
    fn positive_radius() {
        for radius in 0..100_u32 {
            let bounds = HexBounds::positive_radius(radius);
            let coords = bounds.all_coords();
            let fails: Vec<_> = coords.filter(|c| c.x < 0 || c.y < 0).collect();
            println!("{fails:#?}");
            assert!(fails.is_empty());
        }
    }

    #[test]
    fn bounds_hexagon() {
        for radius in 0..8 {
            let bounds = HexBounds::new(Hex::ZERO, radius);
            let coords = bounds.all_coords();
            let reconstructed: HexBounds = coords.collect();
            assert_eq!(bounds, reconstructed);
            let corners = bounds.corners();
            let reconstructed: HexBounds = corners.into_iter().collect();
            assert_eq!(bounds, reconstructed);
        }

        for radius in 0..8 {
            let bounds = HexBounds::new(Hex::new(15, -19), radius);
            let coords = bounds.all_coords();
            let reconstructed: HexBounds = coords.collect();
            assert_eq!(bounds, reconstructed);
            let corners = bounds.corners();
            let reconstructed: HexBounds = corners.into_iter().collect();
            assert_eq!(bounds, reconstructed);
        }
    }

    #[test]
    fn range_works() {
        let coords: Vec<_> = Hex::ZERO.range(10).collect();
        let bounds = HexBounds::from_iter(coords.clone());
        assert_eq!(bounds.center, Hex::ZERO);
        assert_eq!(bounds.radius, 10);
        for h in coords {
            assert!(bounds.is_in_bounds(h));
        }
    }

    #[test]
    fn bounds_rhombus() {
        for size in 1..10 {
            for rotation in 0..3 {
                let coords = (0..size * size)
                    .map(|i| Hex::new(i / size, i % size).rotate_cw(rotation))
                    .collect::<Vec<_>>();
                let reconstructed: HexBounds = coords.iter().copied().collect();
                for h in coords {
                    assert!(reconstructed.is_in_bounds(h));
                }
                let radius = size as u32 - 1;
                assert_eq!(reconstructed.radius, radius);
            }
        }
    }

    #[test]
    fn bounds_line() {
        for direction in 0..6 {
            for size in 1..10 {
                let coords = Hex::new(0, 0)
                    .line_to(Hex::new(size, 0))
                    .map(|h| h.rotate_cw(direction))
                    .collect::<Vec<_>>();
                let reconstructed: HexBounds = coords.iter().copied().collect();
                for h in coords {
                    assert!(reconstructed.is_in_bounds(h));
                }
                let radius = (size as u32).div_ceil(2);
                assert_eq!(reconstructed.radius, radius);
            }
        }
    }

    #[test]
    fn bounds_edge_cases() {
        let mut coords = vec![];
        let reconstructed: HexBounds = coords.iter().copied().collect();
        // Doesn't matter where it's placed.
        assert_eq!(reconstructed.radius, 0);

        coords.push(Hex::new(0, 0));
        let reconstructed: HexBounds = coords.iter().copied().collect();
        assert_eq!(reconstructed, HexBounds::from_radius(0));
    }

    #[test]
    fn bounds_wedge() {
        for size in 1..10 {
            let coords = Hex::new(0, 0)
                .full_wedge(size, crate::VertexDirection(1))
                .collect::<Vec<_>>();
            let reconstructed: HexBounds = coords.iter().copied().collect();
            for h in coords {
                assert!(reconstructed.is_in_bounds(h));
            }
            assert_eq!(reconstructed.radius, 2 * (size + 1) / 3);

            let coords = Hex::new(0, 0)
                .full_wedge(size, crate::VertexDirection(2))
                .collect::<Vec<_>>();
            let reconstructed: HexBounds = coords.iter().copied().collect();
            for h in coords {
                assert!(reconstructed.is_in_bounds(h));
            }
            assert_eq!(reconstructed.radius, 2 * (size + 1) / 3);
        }
    }
}
