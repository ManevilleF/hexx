use crate::{hex::ExactSizeHexIterator, Hex, HexBounds};
use std::fmt;

use super::HexStore;

/// [`Vec`] Based storage for hexagonal maps using [hexmod] coordinates.
///
/// [`HexModMap`] is made for _hexagonal_ large _dense_ maps, utilizing [hexmod]
/// coordinates to map [`Hex`] coordinate to a positive 1D array.
///
/// This provides faster iteration performance compared to [`HexagonalMap`].
///
/// It can be used only if:
/// - The map is an hexagon shape
/// - The map is _dense_
/// - No coordinate will be added or removed from the map
///
/// If your use case doesn't match all of the above, use a [`HashMap`] instead
///
/// ## Performance agains [`HashMap`]
///
/// This struct is uses less memory and the larger the map, the faster `get`
/// operations are agains a hashmap, approximately 20x to 200x faster
///
/// But for iterating this storage is *less* performant than a hashmap
/// approximately 3x slower
///
/// [`HashMap`]: std::collections::HashMap
/// [hexmod]: https://observablehq.com/@sanderevers/hexmod-representation
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub struct HexModMap<T> {
    inner: Vec<T>,
    meta: HexModMapMetadata,
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
struct HexModMapMetadata {
    bounds: HexBounds,
}

impl HexModMapMetadata {
    #[inline]
    const fn new(bounds: HexBounds) -> Self {
        Self { bounds }
    }

    /// Converts from hex coordinates to a 1D array index
    #[inline]
    const fn hex_to_idx(&self, hex: Hex) -> Option<usize> {
        if !self.bounds.is_in_bounds(hex) {
            return None;
        }
        Some(
            hex.const_sub(self.bounds.center)
                .to_hexmod_coordinates(self.bounds.radius) as usize,
        )
    }

    /// Converts from a 1D array index back to hex coordinates
    #[inline]
    #[allow(clippy::cast_possible_truncation)]
    const fn idx_to_hex(&self, idx: usize) -> Hex {
        self.bounds
            .center
            .const_add(Hex::from_hexmod_coordinates(idx as u32, self.bounds.radius))
    }
}

impl<T> HexModMap<T> {
    /// Creates and fills a hexagon shaped map
    ///
    /// # Arguments
    ///
    /// * `center` - The center coordinate of the hexagon
    /// * `radius` - The radius of the map, around `center`
    /// * `values` - Function called for each coordinate in the `radius` to fill
    ///   the map
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::{*, storage::HexModMap};
    ///
    /// let map = HexModMap::new(Hex::ZERO, 10, |coord| coord.length());
    /// assert_eq!(map[hex(1, 0)], 1);
    /// ```
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn new(center: Hex, radius: u32, mut values: impl FnMut(Hex) -> T) -> Self {
        let bounds = HexBounds::new(center, radius);
        let meta = HexModMapMetadata::new(bounds);

        let hex_count = bounds.hex_count();

        // Iterate over all valid hexes in the hexagonal region and fill the map
        let inner: Vec<_> = (0..hex_count)
            .map(|coord| {
                let hex = center + Hex::from_hexmod_coordinates(coord as u32, bounds.radius);
                values(hex)
            })
            .collect();

        Self { inner, meta }
    }

    #[inline]
    #[must_use]
    /// Returns the associated coordinate bounds
    pub const fn bounds(&self) -> &HexBounds {
        &self.meta.bounds
    }

    #[must_use]
    /// Map storage length
    pub const fn len(&self) -> usize {
        self.meta.bounds.hex_count()
    }

    #[must_use]
    /// Returns `true` if `len` is zero
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl<T> HexStore<T> for HexModMap<T> {
    fn get(&self, hex: crate::Hex) -> Option<&T> {
        let idx = self.meta.hex_to_idx(hex)?;
        self.inner.get(idx)
    }

    fn get_mut(&mut self, hex: crate::Hex) -> Option<&mut T> {
        let idx = self.meta.hex_to_idx(hex)?;
        self.inner.get_mut(idx)
    }

    fn values<'s>(&'s self) -> impl ExactSizeIterator<Item = &'s T>
    where
        T: 's,
    {
        ExactSizeHexIterator {
            count: self.len(),
            iter: self.inner.iter(),
        }
    }

    fn values_mut<'s>(&'s mut self) -> impl ExactSizeIterator<Item = &'s mut T>
    where
        T: 's,
    {
        ExactSizeHexIterator {
            count: self.len(),
            iter: self.inner.iter_mut(),
        }
    }

    fn iter<'s>(&'s self) -> impl ExactSizeIterator<Item = (crate::Hex, &'s T)>
    where
        T: 's,
    {
        let count = self.len();
        let meta = self.meta;
        let iter = self.inner.iter().enumerate().map(move |(i, value)| {
            let hex = meta.idx_to_hex(i);
            (hex, value)
        });
        ExactSizeHexIterator { iter, count }
    }

    fn iter_mut<'s>(&'s mut self) -> impl ExactSizeIterator<Item = (crate::Hex, &'s mut T)>
    where
        T: 's,
    {
        let count = self.len();
        let meta = self.meta;
        let iter = self.inner.iter_mut().enumerate().map(move |(i, value)| {
            let hex = meta.idx_to_hex(i);
            (hex, value)
        });
        ExactSizeHexIterator { iter, count }
    }
}

impl<T> fmt::Debug for HexModMap<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HexModMap")
            .field("inner", &self.inner)
            .field("meta", &self.meta)
            .finish()
    }
}

impl<T> Clone for HexModMap<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            meta: self.meta,
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::platform::collections::HashSet;

    use super::*;
    use std::collections::HashMap;

    #[test]
    fn validity() {
        for center in Hex::ZERO.range(20) {
            for radius in 0_u32..30 {
                let expected: HashMap<Hex, usize> = center
                    .range(radius)
                    .enumerate()
                    .map(|(i, h)| (h, i))
                    .collect();

                let map = HexModMap::new(center, radius, |h| expected[&h]);

                for (k, v) in &expected {
                    assert_eq!(*v, *map.get(*k).unwrap());
                }
                for k in center.range(radius + 1) {
                    assert_eq!(expected.get(&k), map.get(k));
                }

                for k in map.bounds().all_coords() {
                    assert_eq!(map[k], expected[&k]);
                }
            }
        }
    }

    #[test]
    fn iter() {
        for center in Hex::ZERO.range(20) {
            for radius in 0_u32..30 {
                let expected: HashMap<Hex, usize> = center
                    .range(radius)
                    .enumerate()
                    .map(|(i, h)| (h, i))
                    .collect();

                let map = HexModMap::new(center, radius, |h| expected[&h]);

                let mut values: Vec<_> = map.values().copied().collect();
                let mut expected_values: Vec<_> = expected.values().copied().collect();

                values.sort_unstable();
                expected_values.sort_unstable();
                assert_eq!(values, expected_values);

                let keys: HashSet<_> = map.iter().map(|(k, _)| k).collect();
                let expected_keys: HashSet<_> = expected.keys().copied().collect();

                assert_eq!(keys, expected_keys);

                assert_eq!(map.values().len(), expected.values().len());
                assert_eq!(map.iter().len(), expected.iter().len());
            }
        }
    }
}
