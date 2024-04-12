use crate::{Hex, HexBounds};
use std::{
    fmt,
    slice::{Iter, IterMut},
};

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
pub struct HexagonalMap<T> {
    inner: Vec<Vec<T>>,
    bounds: HexBounds,
}

impl<T> HexagonalMap<T> {
    /// Creates and fills a hexagon shaped map
    ///
    /// # Arguments
    ///
    /// * `center` - The center coordinate of the hexagon
    /// * `radius` - The radius of the map, around `center`
    /// * `values` - Function called for each coordinate in the `radius` to fill the map
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
        Self { inner, bounds }
    }

    #[inline]
    #[must_use]
    /// Returns the associated coordinate bounds
    pub const fn bounds(&self) -> &HexBounds {
        &self.bounds
    }

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

    #[must_use]
    /// Returns a reference the stored value associated with `idx`.
    /// Returns `None` if `idx` is out of bounds
    pub fn get(&self, idx: Hex) -> Option<&T> {
        let [y, x] = self.hex_to_idx(idx)?;
        self.inner.get(y).and_then(|v| v.get(x))
    }

    /// Returns a mutable reference the stored value associated with `idx`.
    /// Returns `None` if `idx` is out of bounds
    #[must_use]
    pub fn get_mut(&mut self, idx: Hex) -> Option<&mut T> {
        let [y, x] = self.hex_to_idx(idx)?;
        self.inner.get_mut(y).and_then(|v| v.get_mut(x))
    }

    /// Returns an iterator over the storage, in `y` order
    pub fn iter(&self) -> Iter<Vec<T>> {
        self.inner.iter()
    }

    /// Returns an iterator over the storage, in `y` order
    pub fn iter_mut(&mut self) -> IterMut<Vec<T>> {
        self.inner.iter_mut()
    }
}

impl<T> IntoIterator for HexagonalMap<T> {
    type Item = Vec<T>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a HexagonalMap<T> {
    type Item = &'a Vec<T>;
    type IntoIter = Iter<'a, Vec<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut HexagonalMap<T> {
    type Item = &'a mut Vec<T>;
    type IntoIter = IterMut<'a, Vec<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<T> fmt::Debug for HexagonalMap<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HexagonalMap")
            .field("bounds", &self.bounds)
            .field("inner", &self.inner)
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
            bounds: self.bounds,
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::utils::HashMap;

    use super::*;

    #[test]
    fn validity() {
        for center in Hex::ZERO.range(20) {
            for radius in 0_u32..25 {
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
}
