use std::{
    fmt::Debug,
    slice::{Iter, IterMut},
};

use crate::Hex;

/// [`Vec`] Based storage for rombus maps.
///
/// > See [this article](https://www.redblobgames.com/grids/hexagons/#map-storage)
///
/// [`RombusMap`] is made for _rombus_ large _dense_ maps, utilizing some
/// tricks to map [`Hex`] coordinate to a positive 1D array.
///
/// It can be used only if:
/// - The map is a rombus shape
/// - The map is _dense_
/// - No coordinate will be added or removed from the map
///
/// If your use case doesn't match all of the above, use a [`HashMap`] instead
///
/// [`HashMap`]: std::collections::HashMap
pub struct RombusMap<T> {
    inner: Vec<T>,
    origin: Hex,
    rows: u32,
    columns: u32,
}

impl<T> RombusMap<T> {
    /// Creates and fills a rombus shaped map
    ///
    /// # Arguments
    ///
    /// * `origin` - The smallest coordinate of the hexagon
    /// * `rows` - The amount of `y` values per column
    /// * `columns` - The amount of `x` values per row
    /// * `values` - Function called for each coordinate to fill the map
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::{*, storage::RombusMap};
    ///
    /// let map = RombusMap::new(Hex::ZERO, 5, 10, |coord| coord.length());
    /// assert_eq!(map[hex(1, 0)], 1);
    /// ```
    #[must_use]
    #[allow(clippy::cast_possible_wrap)]
    pub fn new(origin: Hex, rows: u32, columns: u32, mut values: impl FnMut(Hex) -> T) -> Self {
        let mut inner = Vec::with_capacity((rows * columns) as usize);
        for y in 0..rows {
            for x in 0..columns {
                let p = origin.const_add(Hex::new(x as i32, y as i32));
                inner.push(values(p));
            }
        }
        Self {
            inner,
            origin,
            rows,
            columns,
        }
    }

    fn hex_to_idx(&self, idx: Hex) -> Option<usize> {
        let hex = idx - self.origin;
        let x = u32::try_from(hex.x).ok()?;
        if x >= self.columns {
            return None;
        }
        let y = u32::try_from(hex.y).ok()?;
        if y >= self.rows {
            return None;
        }
        Some((y * self.columns + x) as usize)
    }

    #[must_use]
    /// Returns a reference the stored value associated with `idx`.
    /// Returns `None` if `idx` is out of bounds
    pub fn get(&self, hex: Hex) -> Option<&T> {
        let index = self.hex_to_idx(hex)?;
        self.inner.get(index)
    }

    #[must_use]
    /// Returns a mutable reference the stored value associated with `idx`.
    /// Returns `None` if `idx` is out of bounds
    pub fn get_mut(&mut self, hex: Hex) -> Option<&mut T> {
        let index = self.hex_to_idx(hex)?;
        self.inner.get_mut(index)
    }

    /// Returns an iterator over the storage, in `y` order
    pub fn iter(&self) -> Iter<T> {
        self.inner.iter()
    }

    /// Returns an iterator over the storage, in `y` order
    pub fn iter_mut(&mut self) -> IterMut<T> {
        self.inner.iter_mut()
    }

    #[must_use]
    /// Map storage length, equals to `rows * columns`
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    #[must_use]
    /// Returns `true` if `rows` or `columns` is zero
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    #[must_use]
    /// Amount of rows
    pub const fn rows(&self) -> u32 {
        self.rows
    }

    #[must_use]
    /// Amount of columns
    pub const fn columns(&self) -> u32 {
        self.columns
    }
}

impl<T> IntoIterator for RombusMap<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a RombusMap<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut RombusMap<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<T> Clone for RombusMap<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            origin: self.origin,
            rows: self.rows,
            columns: self.columns,
        }
    }
}

impl<T> Debug for RombusMap<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RombusMap")
            .field("inner", &self.inner)
            .field("origin", &self.origin)
            .field("rows", &self.rows)
            .field("columns", &self.columns)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use bevy::utils::HashMap;

    use crate::shapes::rombus;

    use super::*;

    #[test]
    fn validity() {
        for origin in Hex::ZERO.range(20) {
            for rows in 0_u32..25 {
                for columns in 0_u32..25 {
                    let expected: HashMap<Hex, usize> = rombus(origin, rows, columns)
                        .enumerate()
                        .map(|(i, h)| (h, i))
                        .collect();

                    let map = RombusMap::new(origin, rows, columns, |h| expected[&h]);

                    assert_eq!(map.len(), (rows * columns) as usize);
                    for (k, v) in &expected {
                        assert_eq!(*v, map[k]);
                    }
                }
            }
        }
    }
}
