use crate::Hex;

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
        let radius = center.unsigned_distance_to(max) / 2;
        Self { center, radius }
    }

    /// Computes the bounds for `radius` with all coordinates
    /// being positive.
    ///
    /// This can be used for efficient map storage in a 2D vector
    /// disallowing negative coordinates
    #[inline]
    #[must_use]
    #[allow(clippy::cast_possible_wrap)]
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
}

impl FromIterator<Hex> for HexBounds {
    fn from_iter<T: IntoIterator<Item = Hex>>(iter: T) -> Self {
        let mut minx = i32::MAX;
        let mut miny = i32::MAX;
        let mut minz = i32::MAX;
        let mut maxx = i32::MIN;
        let mut maxy = i32::MIN;
        let mut maxz = i32::MIN;

        let count = iter
            .into_iter()
            .map(|hex| {
                minx = minx.min(hex.x);
                miny = miny.min(hex.y);
                minz = minz.min(hex.z());
                maxx = maxx.max(hex.x);
                maxy = maxy.max(hex.y);
                maxz = maxz.max(hex.z());
            })
            .count();

        // This algorithm will malfunction if the iterator is empty or has only one element
        if count == 0 {
            // Exit early
            return Self::from_radius(0);
        }

        // Calculate the minimum size of the hexagon that can contain all the hexes
        let xsize = maxx - minx;
        let ysize = maxy - miny;
        let zsize = maxz - minz;

        let trisize1 = maxx + maxy + maxz;
        let trisize2 = -minx - miny - minz;

        let axissize = xsize.max(ysize).max(zsize);
        let trisize = trisize1.max(trisize2);

        // Convert the sizes to radii
        let axisradius = (axissize + 1) / 2;
        let triradius = (trisize + 2) / 3;

        // This is the minimum radius
        let radius = axisradius.max(triradius);

        // Now we need to calculate the center of the hexagon

        // The center must exist between these extremes.
        let cminx = maxx - radius;
        let cminy = maxy - radius;
        let cminz = maxz - radius;
        let cmaxx = minx + radius;
        let cmaxy = miny + radius;
        // We don't actually need to calculate cmaxz

        // How much wiggle room we have on these two axes
        let xrange = cmaxx - cminx;
        let yrange = cmaxy - cminy;

        // Position the center of the hexagon
        let mut x = cminx;
        let mut y = cminy;
        let z = cminz;

        // This sum needs to be 0, but it could be negative
        // This is *never* positive.
        let mut sum = x + y + z;

        // Can sum be fixed entirely by moving on the x axis?
        if -sum > xrange {
            // No, we need to move on the y axis too
            // Move on the x axis first
            sum += xrange;
            x += xrange;

            // Can we fix the sum by moving on the y axis?
            if -sum > yrange {
                // No, we need to move on the z axis too
                y += yrange;
                // z is guaranteed to be able to fix the sum.
                // We don't need to calculate it here.
            } else {
                // Yes, we can fix the sum by moving on the y axis
                // Move on the y axis
                y -= sum;
            }
        } else {
            // Yes, we can fix the sum by moving on the x axis
            // Move on the x axis
            x -= sum;
        }

        let center = Hex::new(x, y);
        let radius = radius as u32;
        HexBounds { center, radius }
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
            let reconstructed = HexBounds::from_iter(coords);
            assert_eq!(bounds, reconstructed);
        }

        for radius in 0..8 {
            let bounds = HexBounds::new(Hex::new(15, -19), radius);
            let coords = bounds.all_coords();
            let reconstructed = HexBounds::from_iter(coords);
            assert_eq!(bounds, reconstructed);
        }
    }

    #[test]
    fn bounds_rhombus() {
        for size in 1..10 {
            for rotation in 0..3 {
                let coords = (0..size * size)
                    .map(|i| Hex::new(i / size, i % size).rotate_cw(rotation))
                    .collect::<Vec<_>>();
                let reconstructed = HexBounds::from_iter(coords.iter().copied());
                for h in coords {
                    assert!(reconstructed.is_in_bounds(h));
                }
                assert_eq!(reconstructed.radius, size as u32 - 1);
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
                let reconstructed = HexBounds::from_iter(coords.iter().copied());
                for h in coords {
                    assert!(reconstructed.is_in_bounds(h));
                }
                assert_eq!(reconstructed.radius, (size as u32 + 1) / 2);
            }
        }
    }

    #[test]
    fn bounds_edge_cases() {
        let mut coords = vec![];
        let reconstructed = HexBounds::from_iter(coords.iter().copied());
        // Doesn't matter where it's placed.
        assert_eq!(reconstructed.radius, 0);

        coords.push(Hex::new(0, 0));
        let reconstructed = HexBounds::from_iter(coords.iter().copied());
        assert_eq!(reconstructed, HexBounds::from_radius(0));
    }

    #[test]
    fn bounds_wedge() {
        for size in 1..10 {
            let coords = Hex::new(0, 0)
                .full_wedge(size, crate::VertexDirection(1))
                .collect::<Vec<_>>();
            let reconstructed = HexBounds::from_iter(coords.iter().copied());
            for h in coords {
                assert!(reconstructed.is_in_bounds(h));
            }
            assert_eq!(reconstructed.radius, 2 * (size + 1) / 3);

            let coords = Hex::new(0, 0)
                .full_wedge(size, crate::VertexDirection(2))
                .collect::<Vec<_>>();
            let reconstructed = HexBounds::from_iter(coords.iter().copied());
            for h in coords {
                assert!(reconstructed.is_in_bounds(h));
            }
            assert_eq!(reconstructed.radius, 2 * (size + 1) / 3);
        }
    }
}
