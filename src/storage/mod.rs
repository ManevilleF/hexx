//! Optimized storage module inspired by [this article]
//!
//! According to our benchmarks:
//!
//! * [`HexagonalMap`] is up to ~15x faster than a hash map
//! * [`RombusMap`] is up to ~100x faster than a hash map
//!
//! [this article]: https://www.redblobgames.com/grids/hexagons/#map-storage
pub(crate) mod hexagonal;
pub(crate) mod hexmod;
pub(crate) mod rombus;

pub use hexagonal::HexagonalMap;
pub use hexmod::HexModMap;
pub use rombus::RombusMap;

macro_rules! storage_impl {
    ($ty:ty) => {
        impl<T> std::ops::Index<crate::Hex> for $ty {
            type Output = T;

            fn index(&self, index: crate::Hex) -> &Self::Output {
                self.get(index).unwrap()
            }
        }

        impl<T> std::ops::Index<&crate::Hex> for $ty {
            type Output = T;

            fn index(&self, index: &crate::Hex) -> &Self::Output {
                self.get(*index).unwrap()
            }
        }

        impl<T> std::ops::IndexMut<crate::Hex> for $ty {
            fn index_mut(&mut self, index: crate::Hex) -> &mut Self::Output {
                self.get_mut(index).unwrap()
            }
        }

        impl<T> std::ops::IndexMut<&crate::Hex> for $ty {
            fn index_mut(&mut self, index: &crate::Hex) -> &mut Self::Output {
                self.get_mut(*index).unwrap()
            }
        }
    };
}

storage_impl!(HexagonalMap<T>);
storage_impl!(RombusMap<T>);
storage_impl!(HexModMap<T>);

/// Trait grouping common features for hexagonal storage types.
///
/// Implemented for
/// - [`HexagonalMap<T>`](HexagonalMap)
/// - [`RombusMap<T>`](RombusMap)
#[cfg_attr(
    not(feature = "bevy_platform"),
    doc = "- [`HashMap<Hex, T>`](std::collections::HashMap)"
)]
#[cfg_attr(
    feature = "bevy_platform",
    doc = "- [`HashMap<Hex, T>`](bevy_platform::collections::HashMap)"
)]
pub trait HexStore<T> {
    /// Returns a reference the stored value associated with `idx`.
    /// Returns `None` if `idx` is out of bounds
    #[must_use]
    fn get(&self, hex: crate::Hex) -> Option<&T>;

    /// Returns a mutable reference the stored value associated with `idx`.
    /// Returns `None` if `idx` is out of bounds
    #[must_use]
    fn get_mut(&mut self, hex: crate::Hex) -> Option<&mut T>;

    /// An iterator visiting all values in arbitrary order.
    /// The iterator element type is `&'s T`.
    fn values<'s>(&'s self) -> impl ExactSizeIterator<Item = &'s T>
    where
        T: 's;

    /// An iterator visiting all values mutably in arbitrary order.
    /// The iterator element type is `&'s mut T`.
    fn values_mut<'s>(&'s mut self) -> impl ExactSizeIterator<Item = &'s mut T>
    where
        T: 's;

    /// An iterator visiting all key-value pairs in arbitrary order.
    /// The iterator element type is `(Hex, &'s T)`.
    fn iter<'s>(&'s self) -> impl ExactSizeIterator<Item = (crate::Hex, &'s T)>
    where
        T: 's;

    /// An iterator visiting all key-value pairs in arbitrary order with mutable
    /// references to the values.
    /// The iterator element type is `(Hex, &'s mut T)`.
    fn iter_mut<'s>(&'s mut self) -> impl ExactSizeIterator<Item = (crate::Hex, &'s mut T)>
    where
        T: 's;
}

impl<T, S: std::hash::BuildHasher> HexStore<T> for std::collections::HashMap<crate::Hex, T, S> {
    fn get(&self, hex: crate::Hex) -> Option<&T> {
        self.get(&hex)
    }

    fn get_mut(&mut self, hex: crate::Hex) -> Option<&mut T> {
        self.get_mut(&hex)
    }

    fn values<'s>(&'s self) -> impl ExactSizeIterator<Item = &'s T>
    where
        T: 's,
    {
        self.values()
    }

    fn values_mut<'s>(&'s mut self) -> impl ExactSizeIterator<Item = &'s mut T>
    where
        T: 's,
    {
        self.values_mut()
    }

    fn iter<'s>(&'s self) -> impl ExactSizeIterator<Item = (crate::Hex, &'s T)>
    where
        T: 's,
    {
        self.iter().map(|(k, v)| (*k, v))
    }

    fn iter_mut<'s>(&'s mut self) -> impl ExactSizeIterator<Item = (crate::Hex, &'s mut T)>
    where
        T: 's,
    {
        self.iter_mut().map(|(k, v)| (*k, v))
    }
}

#[cfg(feature = "bevy_platform")]
impl<T, S: core::hash::BuildHasher> HexStore<T>
    for bevy_platform::collections::HashMap<crate::Hex, T, S>
{
    fn get(&self, hex: crate::Hex) -> Option<&T> {
        self.get(&hex)
    }

    fn get_mut(&mut self, hex: crate::Hex) -> Option<&mut T> {
        self.get_mut(&hex)
    }

    fn values<'s>(&'s self) -> impl ExactSizeIterator<Item = &'s T>
    where
        T: 's,
    {
        self.values()
    }

    fn values_mut<'s>(&'s mut self) -> impl ExactSizeIterator<Item = &'s mut T>
    where
        T: 's,
    {
        self.values_mut()
    }

    fn iter<'s>(&'s self) -> impl ExactSizeIterator<Item = (crate::Hex, &'s T)>
    where
        T: 's,
    {
        self.iter().map(|(k, v)| (*k, v))
    }

    fn iter_mut<'s>(&'s mut self) -> impl ExactSizeIterator<Item = (crate::Hex, &'s mut T)>
    where
        T: 's,
    {
        self.iter_mut().map(|(k, v)| (*k, v))
    }
}
