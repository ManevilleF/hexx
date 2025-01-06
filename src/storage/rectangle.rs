use crate::{
    hex::ExactSizeHexIterator,
    shapes::{PointyRectangle, Rectangle},
    Hex,
};
use std::{fmt, ops::Deref};

use super::HexStore;

/// [`Vec`] Based storage for hexagonal maps.
///
/// > See [this article](https://www.redblobgames.com/grids/hexagons/#map-storage)
///
/// [`RectangleMap`] is made for _rectangular_ large _dense_ maps, utilizing some
/// tricks to map [`Hex`] coordinate to a positive 2D array.
///
/// It can be used only if:
/// - The map is an rectangular shape
/// - The map is _dense_
/// - No coordinate will be added or removed from the map
///
/// If your use case doesn't match all of the above, use a [`HashMap`] instead
///
/// ## Performance agains [`HashMap`]
///
/// This struct is uses less memory and the larger the map, the faster `get`
/// operations are agains a hashmap, approximately 10x faster
///
/// But for iterating this storage is *less* performant than a hashmap
/// approximately 3x slower
///
/// [`HashMap`]: std::collections::HashMap
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub struct RectangleMap<T> {
    inner: Vec<Vec<T>>,
    meta: RectangleMapMetadata,
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
struct RectangleMapMetadata(PointyRectangle);

impl Deref for RectangleMapMetadata {
    type Target = PointyRectangle;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl RectangleMapMetadata {
    fn offset(&self) -> Hex {
        Hex::new(self.left, self.top)
    }

    #[allow(clippy::cast_sign_loss)]
    fn hex_to_idx(&self, idx: Hex) -> [usize; 2] {
        let idx = idx - self.offset();
        [idx.y as usize, (idx.x + (idx.y >> 1)) as usize]
    }

    #[allow(clippy::cast_possible_wrap, clippy::cast_possible_truncation)]
    fn idx_to_hex(&self, [y, x]: [usize; 2]) -> Hex {
        let x = (x.saturating_sub(y >> 1)) as i32;
        let y = y as i32;

        Hex::new(x, y) + self.offset()
    }
}

impl<T> RectangleMap<T> {
    /// Creates and fills a rectangle shaped map
    ///
    /// # Arguments
    ///
    /// * `rectangle` - The rectangle shape parameters
    /// * `values` - Function called for each coordinate in the `radius` to fill
    ///   the map
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::{*, storage::RectangleMap};
    ///
    /// let map = RectangleMap::new(Hex::ZERO, 10, |coord| coord.length());
    /// assert_eq!(map[hex(1, 0)], 1);
    /// ```
    #[must_use]
    #[allow(clippy::cast_possible_wrap)]
    pub fn new(rectangle: Rectangle, mut values: impl FnMut(Hex) -> T) -> Self {
        let inner = (rectangle.top..=rectangle.bottom)
            .map(|y| {
                let y_offset = y >> 1;
                ((rectangle.left - y_offset)..=(rectangle.right - y_offset))
                    .map(|x| values(Hex::new(x, y)))
                    .collect::<Vec<_>>()
            })
            .collect();
        Self {
            inner,
            meta: RectangleMapMetadata(PointyRectangle(rectangle)),
        }
    }

    #[must_use]
    #[allow(clippy::cast_sign_loss)]
    /// Map storage length
    pub fn len(&self) -> usize {
        (self.meta.right.saturating_sub(self.meta.left) + 1) as usize
            * (self.meta.bottom.saturating_sub(self.meta.top) + 1) as usize
    }

    #[must_use]
    /// Returns `true` if `len` is zero
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl<T> HexStore<T> for RectangleMap<T> {
    fn get(&self, hex: crate::Hex) -> Option<&T> {
        let [y, x] = self.meta.hex_to_idx(hex);
        println!("{hex:?} -> [{y},{x}] (offset = {:?})", self.meta.offset(),);
        self.inner.get(y).and_then(|v| v.get(x))
    }

    fn get_mut(&mut self, hex: crate::Hex) -> Option<&mut T> {
        let [y, x] = self.meta.hex_to_idx(hex);
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

impl<T> fmt::Debug for RectangleMap<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RectangleMap")
            .field("inner", &self.inner)
            .field("meta", &self.meta)
            .finish()
    }
}

impl<T> Clone for RectangleMap<T>
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
        let rectangle = Rectangle {
            left: 0,
            right: 6,
            top: 0,
            bottom: 6,
        };
        let iter = rectangle.pointy_coords();
        let len = iter.len();
        let expected: HashMap<Hex, usize> = iter.enumerate().map(|(i, h)| (h, i)).collect();

        let map = RectangleMap::new(rectangle, |h| expected[&h]);

        println!("{rectangle:?}");
        assert_eq!(map.len(), len);
        for (k, v) in &expected {
            assert_eq!(*v, map[k]);
        }
        for rows in 0..25 {
            for columns in 0..25 {
                let rectangle = Rectangle {
                    left: -rows,
                    right: rows,
                    top: -columns,
                    bottom: columns,
                };
                let iter = rectangle.pointy_coords();
                let len = iter.len();
                let expected: HashMap<Hex, usize> = iter.enumerate().map(|(i, h)| (h, i)).collect();

                let map = RectangleMap::new(rectangle, |h| expected[&h]);

                println!("{rectangle:?}");
                assert_eq!(map.len(), len);
                for (k, v) in &expected {
                    assert_eq!(*v, map[k]);
                }
                for k in crate::shapes::pointy_rectangle([
                    -rows - 1,
                    rows + 1,
                    -columns - 1,
                    columns + 1,
                ]) {
                    assert_eq!(expected.get(&k), map.get(k));
                }
            }
        }
    }

    #[test]
    fn iter() {
        for rows in 0..25 {
            for columns in 0..25 {
                let rectangle = Rectangle {
                    left: -rows,
                    right: rows,
                    top: -columns,
                    bottom: columns,
                };
                let iter = rectangle.pointy_coords();
                let expected: HashMap<Hex, usize> = iter.enumerate().map(|(i, h)| (h, i)).collect();

                let map = RectangleMap::new(rectangle, |h| expected[&h]);

                let iter: HashMap<Hex, usize> = map.iter().map(|(k, v)| (k, *v)).collect();
                assert_eq!(expected, iter);
            }
        }
    }
}
