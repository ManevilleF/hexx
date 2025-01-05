use crate::{hex::ExactSizeHexIterator, Hex, HexBounds};
use std::fmt;

use super::HexStore;

/// [`Vec`] Based storage for hexagonal maps.
///
/// > See [this article](https://www.redblobgames.com/grids/hexagons/#map-storage)
///
/// [`HexagonalMap`] is made for _hexagonal_ large _dense_ maps, utilizing some
/// tricks to map [`Hex`] coordinate to a positive 2D array.
///
/// It can be used only if:
/// - The map is an hexagon shape
/// - The map is _dense_
/// - No coordinate will be added or removed from the map
///
/// If your use case doesn't match all of the above, use a [`HashMap`] instead
///
/// [`HashMap`]: std::collections::HashMap
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub struct HexagonalMap<T> {
    inner: Vec<Vec<T>>,
    meta: HexagonalMapMetadata,
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
struct HexagonalMapMetadata {
    bounds: HexBounds,
}

impl HexagonalMapMetadata {
    #[allow(clippy::cast_possible_wrap)]
    const fn offset(&self) -> Hex {
        Hex::splat(self.bounds.radius as i32).const_sub(self.bounds.center)
    }
    fn hex_to_idx(&self, idx: Hex) -> Option<[usize; 2]> {
        let key = idx + self.offset();
        let x = u32::try_from(key.x).ok()?;
        let y = u32::try_from(key.y).ok()?;
        Some([
            y as usize,
            x.saturating_sub(self.bounds.radius.saturating_sub(y)) as usize,
        ])
    }

    #[allow(clippy::cast_possible_wrap, clippy::cast_possible_truncation)]
    fn idx_to_hex(&self, [y, x]: [usize; 2]) -> Hex {
        let x = (x as u32).saturating_add(self.bounds.radius.saturating_sub(y as u32)) as i32;
        let y = y as i32;

        Hex { x, y } - self.offset()
    }
}

impl<T> HexagonalMap<T> {
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
    /// # use hexx::{*, storage::HexagonalMap};
    ///
    /// let map = HexagonalMap::new(Hex::ZERO, 10, |coord| coord.length());
    /// assert_eq!(map[hex(1, 0)], 1);
    /// ```
    #[must_use]
    #[allow(clippy::cast_possible_wrap)]
    pub fn new(center: Hex, radius: u32, mut values: impl FnMut(Hex) -> T) -> Self {
        let bounds = HexBounds::new(center, radius);
        let range = radius as i32;
        let inner = (-range..=range)
            .map(|y| {
                let x_min = i32::max(-range, -y - range);
                let x_max = i32::min(range, range - y);
                (x_min..=x_max)
                    .map(|x| {
                        let coord = center.const_add(Hex::new(x, y));
                        values(coord)
                    })
                    .collect()
            })
            .collect();
        Self {
            inner,
            meta: HexagonalMapMetadata { bounds },
        }
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

impl<T> HexStore<T> for HexagonalMap<T> {
    fn get(&self, hex: crate::Hex) -> Option<&T> {
        let [y, x] = self.meta.hex_to_idx(hex)?;
        self.inner.get(y).and_then(|v| v.get(x))
    }

    fn get_mut(&mut self, hex: crate::Hex) -> Option<&mut T> {
        let [y, x] = self.meta.hex_to_idx(hex)?;
        self.inner.get_mut(y).and_then(|v| v.get_mut(x))
    }

    fn values<'s>(&'s self) -> impl ExactSizeIterator<Item = &'s T>
    where
        T: 's,
    {
        ExactSizeHexIterator {
            count: self.len(),
            iter: self.inner.iter().flatten(),
        }
    }

    fn values_mut<'s>(&'s mut self) -> impl ExactSizeIterator<Item = &'s mut T>
    where
        T: 's,
    {
        ExactSizeHexIterator {
            count: self.len(),
            iter: self.inner.iter_mut().flatten(),
        }
    }

    fn iter<'s>(&'s self) -> impl ExactSizeIterator<Item = (crate::Hex, &'s T)>
    where
        T: 's,
    {
        let count = self.len();
        let iter = self.inner.iter().enumerate().flat_map(move |(y, arr)| {
            arr.iter().enumerate().map(move |(x, value)| {
                let hex = self.meta.idx_to_hex([y, x]);
                (hex, value)
            })
        });
        ExactSizeHexIterator { iter, count }
    }

    fn iter_mut<'s>(&'s mut self) -> impl ExactSizeIterator<Item = (crate::Hex, &'s mut T)>
    where
        T: 's,
    {
        let count = self.len();
        let meta = self.meta;
        let iter = self.inner.iter_mut().enumerate().flat_map(move |(y, arr)| {
            arr.iter_mut().enumerate().map(move |(x, value)| {
                let hex = meta.idx_to_hex([y, x]);
                (hex, value)
            })
        });
        ExactSizeHexIterator { iter, count }
    }
}

impl<T> fmt::Debug for HexagonalMap<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HexagonalMap")
            .field("inner", &self.inner)
            .field("meta", &self.meta)
            .finish()
    }
}

impl<T> Clone for HexagonalMap<T>
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

                let map = HexagonalMap::new(center, radius, |h| expected[&h]);

                for (k, v) in &expected {
                    assert_eq!(*v, map[k]);
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

                let map = HexagonalMap::new(center, radius, |h| expected[&h]);
                let iter: HashMap<Hex, usize> = map.iter().map(|(k, v)| (k, *v)).collect();

                assert_eq!(iter, expected);
            }
        }
    }
}
