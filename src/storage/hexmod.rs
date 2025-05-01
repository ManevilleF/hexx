use crate::{hex::ExactSizeHexIterator, Hex, HexBounds};
use std::fmt;

use super::HexStore;

/// [`Vec`] Based storage for hexagonal maps using HexMod coordinates.
///
/// > See [this article](https://www.redblobgames.com/grids/hexagons/#map-storage)
///
/// [`HexModMap`] is made for _hexagonal_ large _dense_ maps, utilizing HexMod
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
/// ## Performance
///
/// This struct uses less memory than a [`HashMap`] and provides fast `get` operations
/// comparable to [`HexagonalMap`], while significantly improving the iteration
/// performance to be closer to that of a [`HashMap`].
///
/// [`HashMap`]: std::collections::HashMap
/// [`HexagonalMap`]: super::HexagonalMap
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
    side_length: i32,
}

impl HexModMapMetadata {
    fn new(bounds: HexBounds) -> Self {
        Self { bounds, side_length: bounds.radius as i32 }
    }

    fn hex_to_idx(&self, hex: Hex) -> Option<usize> {
        if !self.bounds.is_in_bounds(hex) {
            return None;
        }
        Some(hex.to_hexmod_coordinates(self.bounds.radius) as usize)
    }

    fn idx_to_hex(&self, idx: usize) -> Hex {
        Hex::from_hexmod_coordinates(idx as u32, self.bounds.radius)
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
    pub fn new(center: Hex, radius: u32, mut values: impl FnMut(Hex) -> T) -> Self {
        let bounds = HexBounds::new(center, radius);
        let meta = HexModMapMetadata::new(bounds);
        
        let side_length = radius as i32;
        let hex_count = bounds.hex_count();
        let mut inner = Vec::with_capacity(hex_count);

        // Iterate over all valid hexes in the hexagonal region and fill the map
        for coord in center.range(radius) {
            let value = values(coord);
            let idx = meta.hex_to_idx(coord).unwrap();
            
            // Ensure the vector is large enough
            if idx >= inner.len() {
                inner.resize_with(idx + 1, || panic!("Attempted to access an invalid index"));
            }
            
            inner[idx] = value;
        }
        
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
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn validity() {
        for center in Hex::ZERO.range(20) {
            for radius in 0_u32..30 {
                let expected: std::collections::HashMap<Hex, usize> = center
                    .range(radius)
                    .enumerate()
                    .map(|(i, h)| (h, i))
                    .collect();

                let map = HexModMap::new(center, radius, |h| expected[&h]);

                for (k, v) in &expected {
                    assert_eq!(*v, map[*k]);
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
                let expected: std::collections::HashMap<Hex, usize> = center
                    .range(radius)
                    .enumerate()
                    .map(|(i, h)| (h, i))
                    .collect();

                let map = HexModMap::new(center, radius, |h| expected[&h]);

                let values: Vec<_> = map.values().copied().collect();
                let expected_values: Vec<_> = expected.values().copied().collect();
                assert_eq!(values.len(), expected_values.len());
                assert!(values.iter().all(|v| expected_values.contains(v)));

                let keys: HashSet<_> = map.iter().map(|(k, _)| k).collect();
                let expected_keys: HashSet<_> = expected.keys().copied().collect();
                assert_eq!(keys, expected_keys);

                assert_eq!(map.values().len(), expected.values().len());
                assert_eq!(map.iter().len(), expected.iter().len());
            }
        }
    }
} 