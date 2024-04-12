//! Optimized storage module inspired by [this article]
//!
//! According to our benchmarks:
//!
//! * [`HexagonalMap`] is ~15x faster than a hash map
//! * [`RombusMap`] is ~100x faster than a hash map
//!
//! [this article]: https://www.redblobgames.com/grids/hexagons/#map-storage
mod hexagonal;
mod rombus;

pub use hexagonal::HexagonalMap;
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
