use std::{fmt::Debug, ops::Deref};

use crate::{shapes::Rombus, Hex};

use super::HexStore;

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
/// ## Performance agains [`HashMap`]
///
/// This struct is uses less memory and the larger the map, the faster `get`
/// operations are agains a hashmap, approximately 10x to 100x faster
///
/// But for iterating this storage is *slightly less* performant than a hashmap
///
/// [`HashMap`]: std::collections::HashMap
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub struct RombusMap<T> {
    inner: Vec<T>,
    meta: RombusMetadata,
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
struct RombusMetadata(Rombus);

impl Deref for RombusMetadata {
    type Target = Rombus;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl RombusMetadata {
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

    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    fn idx_to_hex(&self, idx: usize) -> Hex {
        let idx = idx as u32;
        debug_assert!(
            idx < (self.columns * self.rows),
            "idx `{idx}` is out of bounds"
        );

        let x = (idx % self.columns) as i32;
        let y = (idx / self.columns) as i32;

        Hex { x, y } + self.origin
    }
}

impl<T> RombusMap<T> {
    /// Creates and fills a rombus shaped map
    ///
    /// # Arguments
    ///
    /// * `rombus` - The rombus parameters
    /// * `values` - Function called for each coordinate to fill the map
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::{*, storage::RombusMap, shapes::Rombus};
    ///
    /// let rombus = Rombus {
    ///   origin: Hex::ZERO,
    ///   rows: 5,
    ///   columns: 10
    /// };
    /// let map = RombusMap::new(rombus, |coord| coord.length());
    /// assert_eq!(map[hex(1, 0)], 1);
    /// ```
    #[must_use]
    #[allow(clippy::cast_possible_wrap)]
    pub fn new(rombus: Rombus, values: impl FnMut(Hex) -> T) -> Self {
        let inner = rombus.coords().map(values).collect();
        Self {
            inner,
            meta: RombusMetadata(rombus),
        }
    }

    #[must_use]
    #[inline]
    /// Map storage length, equals to `rows * columns`
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    #[must_use]
    #[inline]
    /// Returns `true` if `rows` or `columns` is zero
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    #[must_use]
    #[inline]
    /// Amount of rows
    pub fn rows(&self) -> u32 {
        self.meta.rows
    }

    #[must_use]
    #[inline]
    /// Amount of columns
    pub fn columns(&self) -> u32 {
        self.meta.columns
    }
}

impl<T> HexStore<T> for RombusMap<T> {
    fn get(&self, hex: crate::Hex) -> Option<&T> {
        let index = self.meta.hex_to_idx(hex)?;
        self.inner.get(index)
    }

    fn get_mut(&mut self, hex: crate::Hex) -> Option<&mut T> {
        let index = self.meta.hex_to_idx(hex)?;
        self.inner.get_mut(index)
    }

    fn values<'s>(&'s self) -> impl ExactSizeIterator<Item = &'s T>
    where
        T: 's,
    {
        self.inner.iter()
    }

    fn values_mut<'s>(&'s mut self) -> impl ExactSizeIterator<Item = &'s mut T>
    where
        T: 's,
    {
        self.inner.iter_mut()
    }

    fn iter<'s>(&'s self) -> impl ExactSizeIterator<Item = (crate::Hex, &'s T)>
    where
        T: 's,
    {
        self.values().enumerate().map(|(i, value)| {
            let hex = self.meta.idx_to_hex(i);
            (hex, value)
        })
    }

    fn iter_mut<'s>(&'s mut self) -> impl ExactSizeIterator<Item = (crate::Hex, &'s mut T)>
    where
        T: 's,
    {
        let meta = self.meta;
        self.values_mut().enumerate().map(move |(i, value)| {
            let hex = meta.idx_to_hex(i);
            (hex, value)
        })
    }
}

impl<T> Clone for RombusMap<T>
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

impl<T> Debug for RombusMap<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RombusMap")
            .field("inner", &self.inner)
            .field("meta", &self.meta)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn validity() {
        for origin in Hex::ZERO.range(20) {
            for rows in 0_u32..25 {
                for columns in 0_u32..25 {
                    let rombus = Rombus {
                        origin,
                        rows,
                        columns,
                    };
                    let expected: HashMap<Hex, usize> =
                        rombus.coords().enumerate().map(|(i, h)| (h, i)).collect();

                    let map = RombusMap::new(rombus, |h| expected[&h]);

                    assert_eq!(map.len(), (rows * columns) as usize);
                    for (k, v) in &expected {
                        assert_eq!(*v, map[k]);
                    }
                    for k in crate::shapes::rombus(origin - Hex::NEG_ONE, rows + 2, columns + 2) {
                        assert_eq!(expected.get(&k), map.get(k));
                    }
                }
            }
        }
    }

    #[test]
    fn iter() {
        for origin in Hex::ZERO.range(20) {
            for rows in 0_u32..25 {
                for columns in 0_u32..25 {
                    let rombus = Rombus {
                        origin,
                        rows,
                        columns,
                    };
                    let expected: HashMap<Hex, usize> =
                        rombus.coords().enumerate().map(|(i, h)| (h, i)).collect();

                    let map = RombusMap::new(rombus, |h| expected[&h]);

                    let iter: HashMap<Hex, usize> = map.iter().map(|(k, v)| (k, *v)).collect();
                    assert_eq!(expected, iter);
                }
            }
        }
    }
}
