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
        // Calculate the side length of the hexagon (distance from center to any corner)
        let side_length = bounds.radius as i32;
        Self { bounds, side_length }
    }

    const fn offset(&self) -> Hex {
        Hex::splat(self.side_length).const_sub(self.bounds.center)
    }

    /// Converts from hex coordinates to a 1D array index
    fn hex_to_idx(&self, hex: Hex) -> Option<usize> {
        if !self.bounds.is_in_bounds(hex) {
            return None;
        }
        
        // Apply offset to bring hex into positive coordinate range relative to bounds
        let hex = hex + self.offset();
        
        // Calculate index in the 1D array using the HexMod mapping formula
        // This formula converts from 2D hex coordinates to a 1D array index
        // for a hexagonal shape centered at the origin
        let q = hex.x;
        let r = hex.y;
        let s = -q - r;
        
        let diameter = 2 * self.side_length + 1;
        let qt = q + self.side_length;
        let rt = r + self.side_length;
        let st = s + self.side_length;
        
        // Validate coordinates are in range
        if qt < 0 || qt >= diameter || rt < 0 || rt >= diameter || st < 0 || st >= diameter {
            return None;
        }
        
        // Calculate the 1D index
        let index = qt + rt * diameter - (rt * (rt + 1)) / 2;
        
        Some(index as usize)
    }

    /// Converts from a 1D array index back to hex coordinates
    fn idx_to_hex(&self, idx: usize) -> Hex {
        // Calculate the row (axial y-coordinate + offset)
        let diameter = 2 * self.side_length + 1;
        let mut rt = 0;
        let mut row_size = diameter;
        let mut remaining = idx;
        
        // Find the row by subtracting row sizes
        while remaining >= row_size {
            remaining -= row_size;
            rt += 1;
            row_size -= 1;
        }
        
        // Calculate qt (axial x-coordinate + offset)
        let qt = remaining;
        
        // Convert back to original coordinate system
        let hex = Hex::new(qt as i32, rt as i32) - self.offset();
        
        hex
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
                    assert_eq!(*v, map[k]);
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

                let mut keys: Vec<_> = map.iter().map(|(k, _)| k).collect();
                let mut expected_keys: Vec<_> = expected.keys().copied().collect();

                keys.sort_unstable();
                expected_keys.sort_unstable();
                assert_eq!(keys, expected_keys);

                assert_eq!(map.values().len(), expected.values().len());
                assert_eq!(map.iter().len(), expected.iter().len());
            }
        }
    }
} 