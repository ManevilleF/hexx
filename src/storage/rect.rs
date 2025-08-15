use std::fmt::Debug;

use glam::{IVec2, Vec2Swizzles};

use crate::{Hex, HexLayout, OffsetHexMode, storage::HexStore};

/// [`Vec`] Based storage for rectangular maps.
///
/// > See [this article](https://www.redblobgames.com/grids/hexagons/#map-storage)
///
/// [`RectMap`] is made for _rectangular_ large _dense_ maps, utilizing some
/// tricks to map [`Hex`] coordinate to a positive 1D array.
///
/// It can be used only if:
/// - The map is a rectanglar shape
/// - The map is _dense_
/// - No coordinate will be added or removed from the map
///
/// If your use case doesn't match all of the above, use a [`std::collections::HashMap`] instead
///
/// # Example
/// ```rust
/// use hexx::storage::{HexStore, RectMap, RectMetadata, WrapStrategy};
/// use hexx::*;
///
/// let layout = HexLayout::pointy()
///     .with_hex_size(30.0)
///     .with_origin(Vec2::ZERO);
/// let rect_map = RectMetadata::default()
///     .with_hex_layout(layout)
///     .with_half_size(IVec2 { x: 8, y: 4 })
///     .with_wrap_strategies([WrapStrategy::Cycle, WrapStrategy::Clamp])
///     .build_default::<i32>();
///
/// assert_eq!(rect_map.get(Hex::new(0, 0)), Some(&0));
/// ```
///
/// internally handle coordinate transform
/// - `hex`
/// - => `ij` offset coordinate
/// - => `rc` 2D view of `Vec`
/// - => `idx` `Vec` index
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
#[cfg_attr(
    feature = "bevy_ecs",
    derive(bevy_ecs::resource::Resource, bevy_ecs::component::Component)
)]
#[derive(Clone)]
pub struct RectMap<T> {
    inner: Vec<T>,
    meta: RectMetadata,
}

/// Metadata struct for [`RectMap`]
///
/// # Example
/// ```rust
/// use hexx::{Hex, HexLayout, OffsetHexMode};
/// use hexx::storage::{RectMap, HexStore, RectMetadata, WrapStrategy};
/// use hexx::IVec2;
///
/// let rect_hex_map = RectMetadata::default()
///     .with_hex_layout(HexLayout::pointy().with_hex_size(1.0))
///     .with_half_size(IVec2::new(8,12))
///     .with_offset_mode(OffsetHexMode::Odd)
///     .with_wrap_strategies([WrapStrategy::Cycle, WrapStrategy::Clamp])
///     .build_default::<i32>();
/// let hex = Hex::new(0,0);
///
/// assert_eq!(rect_hex_map.get(hex), Some(&0_i32));
/// assert_eq!(rect_hex_map.wrapped_get(hex), &0_i32);
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
#[cfg_attr(
    feature = "bevy_ecs",
    derive(bevy_ecs::resource::Resource, bevy_ecs::component::Component)
)]
pub struct RectMetadata {
    /// the hex layout of the map
    hex_layout: HexLayout,
    /// the offset mode of the map
    ///
    /// affect which way does the map zic zac.
    offset_mode: OffsetHexMode,
    /// the half size of the map
    half_size: IVec2,
    /// the wrapping strategy for indexing
    wrap_strategies: [WrapStrategy; 2],
}

/// Wrapping Strategy for when try to [`RectMap::wrapped_get`] and [`RectMap::wrapped_get_mut`]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub enum WrapStrategy {
    /// Clamp (offset) coordinate to `min` and `max - 1`
    ///
    /// useful for latitude/vertical coordinate
    Clamp,
    /// Cycle (offset) coordinate to range of`min` and `max - 1`
    ///
    /// useful for longitude/horizontal coordinate
    Cycle,
}

impl Default for RectMetadata {
    fn default() -> Self {
        Self {
            hex_layout: HexLayout::pointy(),
            offset_mode: OffsetHexMode::Odd,
            half_size: IVec2::new(32, 16),
            wrap_strategies: [WrapStrategy::Cycle, WrapStrategy::Clamp],
        }
    }
}

impl RectMetadata {
    // ================================
    // Builder Pattern
    // ================================
    /// builder patter, set hex layout
    #[must_use]
    pub const fn with_hex_layout(mut self, hex_layout: HexLayout) -> Self {
        self.hex_layout = hex_layout;
        self
    }
    /// builder patter, set half size
    #[must_use]
    pub fn with_half_size(mut self, half_size: IVec2) -> Self {
        self.half_size = half_size.abs();
        self
    }
    /// builder patter, set offset mode
    #[must_use]
    pub const fn with_offset_mode(mut self, offset_mode: OffsetHexMode) -> Self {
        self.offset_mode = offset_mode;
        self
    }
    /// builder patter, set wrapping strategy
    #[must_use]
    pub const fn with_wrap_strategies(mut self, wrap_strategies: [WrapStrategy; 2]) -> Self {
        self.wrap_strategies = wrap_strategies;
        self
    }
    /// builder patter, build map with function to eval value
    /// # Example
    /// ```rust
    /// use hexx::storage::{HexStore, RectMap, RectMetadata, WrapStrategy};
    /// use hexx::*;
    ///
    /// let layout = HexLayout::pointy()
    ///     .with_hex_size(30.0)
    ///     .with_origin(Vec2::ZERO);
    /// let rect_map = RectMetadata::default()
    ///     .with_hex_layout(layout)
    ///     .with_half_size(IVec2 { x: 8, y: 4 })
    ///     .with_wrap_strategies([WrapStrategy::Cycle, WrapStrategy::Clamp])
    ///     .build(|hex| hex.x + hex.y);
    ///
    /// assert_eq!(rect_map.get(Hex::new(0, 0)), Some(&0));
    /// ```
    pub fn build<T>(self, values: impl FnMut(Hex) -> T) -> RectMap<T> {
        RectMap::new(self, values)
    }
    /// builder patter, build map with default values
    /// # Example
    /// ```rust
    /// use hexx::storage::{HexStore, RectMap, RectMetadata, WrapStrategy};
    /// use hexx::*;
    ///
    /// let layout = HexLayout::pointy()
    ///     .with_hex_size(30.0)
    ///     .with_origin(Vec2::ZERO);
    /// let rect_map = RectMetadata::default()
    ///     .with_hex_layout(layout)
    ///     .with_half_size(IVec2 { x: 8, y: 4 })
    ///     .with_wrap_strategies([WrapStrategy::Cycle, WrapStrategy::Clamp])
    ///     .build_default::<i32>();
    ///
    /// assert_eq!(rect_map.get(Hex::new(0, 0)), Some(&0));
    /// ```
    #[must_use]
    pub fn build_default<T: Default>(self) -> RectMap<T> {
        RectMap::default_values(self)
    }

    // ================================
    // Access
    // ================================
    /// get the offset mode of the map
    #[must_use]
    pub const fn offset_mode(&self) -> OffsetHexMode {
        self.offset_mode
    }
    /// get the half size of the map
    #[must_use]
    pub const fn half_size(&self) -> IVec2 {
        self.half_size
    }
    /// get the wrap strategies of the map
    #[must_use]
    pub const fn wrap_strategies(&self) -> [WrapStrategy; 2] {
        self.wrap_strategies
    }

    // ================================
    // Forward Coordinate Conversion
    // ================================
    /// calculate hex coordinate from index
    ///
    /// infallible
    ///
    /// - `idx` `Vec` index  
    /// - => `rc` 2D view of `Vec`
    /// - => `ij` offset coordinate
    /// - => `hex`
    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    fn idx_to_hex(&self, idx: usize) -> Hex {
        let idx = idx as i32;
        let rc = IVec2::new(idx % (2 * self.half_size[0]), idx / (2 * self.half_size[0]));
        let ij = rc - self.half_size;
        self.ij_to_hex(ij)
    }

    fn ij_to_hex(&self, ij: IVec2) -> Hex {
        Hex::from_offset_coordinates(
            ij.xy().to_array(),
            self.offset_mode,
            self.hex_layout.orientation,
        )
    }

    // ================================
    // Backward Coordinate Conversion
    // ================================
    /// `None` if outside map
    fn hex_to_idx(&self, hex: Hex) -> Option<usize> {
        let ij = self.hex_to_ij(hex);
        if self.contains_ij(ij) {
            Some(self.ij_to_idx(ij))
        } else {
            None
        }
    }

    /// Wrap if `hex` lie outside of layout
    fn wrapped_hex_to_idx(&self, hex: Hex) -> usize {
        let ij = self.hex_to_ij(hex);
        let ij = self.wrap_ij(ij);
        self.ij_to_idx(ij)
    }

    /// infallable
    fn hex_to_ij(&self, hex: Hex) -> IVec2 {
        let v: IVec2 = hex
            .to_offset_coordinates(self.offset_mode, self.hex_layout.orientation)
            .into();
        v.xy()
    }

    /// fallable input, internal
    ///
    /// - `ij` offset coordinate
    /// - => `rc` 2D view of `Vec`
    /// - => `idx` `Vec` index
    fn ij_to_idx(&self, ij: IVec2) -> usize {
        let rc = (ij + self.half_size).as_uvec2();
        (rc[0] + rc[1] * (2 * self.half_size.as_uvec2()[0])) as usize
    }

    /// infallable
    fn wrap_ij(&self, mut ij: IVec2) -> IVec2 {
        if self.wrap_strategies[0] == WrapStrategy::Cycle {
            while ij[0] < -self.half_size[0] {
                ij[0] += self.half_size[0] * 2;
            }
            while ij[0] >= self.half_size[0] {
                ij[0] -= self.half_size[0] * 2;
            }
        } else {
            ij[0] = ij[0].clamp(-self.half_size[0], self.half_size[0] - 1);
        }

        if self.wrap_strategies[1] == WrapStrategy::Cycle {
            while ij[1] < -self.half_size[1] {
                ij[1] += self.half_size[1] * 2;
            }
            while ij[1] >= self.half_size[1] {
                ij[1] -= self.half_size[1] * 2;
            }
        } else {
            ij[1] = ij[1].clamp(-self.half_size[1], self.half_size[1] - 1);
        }

        ij
    }

    /// Wrap a Hex to ensure it fall inside the map
    #[must_use]
    pub fn wrap_hex(&self, hex: Hex) -> Hex {
        let ij = self.hex_to_ij(hex);
        let ij = self.wrap_ij(ij);
        self.ij_to_hex(ij)
    }

    // ================================
    // Contains
    // ================================
    /// whether the map contains certain `Hex`
    #[must_use]
    pub fn contains_hex(&self, hex: Hex) -> bool {
        self.contains_ij(self.hex_to_ij(hex))
    }
    /// internally
    fn contains_ij(&self, ij: IVec2) -> bool {
        ij == ij.clamp(-self.half_size, self.half_size.wrapping_sub([1, 1].into()))
    }

    // ================================
    // Iteration
    // ================================
    /// total size of the map
    #[must_use]
    pub fn len(&self) -> usize {
        (self.half_size.as_uvec2().element_product() * 4) as usize
    }

    /// whether of not the map layout is empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// iterator over the hexagonal coordinates in the map.
    pub fn iter_hex(&self) -> impl Iterator<Item = Hex> + use<'_> {
        (0..self.len()).map(|idx| self.idx_to_hex(idx))
    }
}

impl std::ops::Deref for RectMetadata {
    type Target = HexLayout;
    fn deref(&self) -> &Self::Target {
        &self.hex_layout
    }
}

impl<T> RectMap<T> {
    // ================================
    // Creation
    // ================================
    /// Creates and fills a rectangular shaped map
    ///
    /// # Arguments
    /// * `meta`: The meta data for the map to create.
    /// * `values` - Function called for each coordinate to fill the map
    ///
    /// # Example
    /// ```
    /// use hexx::storage::{HexStore, RectMap, RectMetadata, WrapStrategy};
    /// use hexx::*;
    ///
    /// let layout = HexLayout::pointy()
    ///     .with_hex_size(30.0)
    ///     .with_origin(Vec2::ZERO);
    /// let meta = RectMetadata::default()
    ///     .with_hex_layout(layout)
    ///     .with_half_size(IVec2 { x: 8, y: 4 })
    ///     .with_wrap_strategies([WrapStrategy::Cycle, WrapStrategy::Clamp]);
    /// let rect_map = RectMap::new(meta, |hex| hex.x + hex.y);
    ///
    /// assert_eq!(rect_map.get(Hex::new(0, 0)), Some(&0));
    /// ```
    #[must_use]
    #[allow(clippy::cast_possible_wrap)]
    pub fn new(meta: RectMetadata, mut values: impl FnMut(Hex) -> T) -> Self {
        let size = (meta.half_size.as_uvec2().element_product() * 4) as usize;
        let mut inner = Vec::with_capacity(size);
        for h in meta.iter_hex() {
            inner.push(values(h));
        }
        Self { inner, meta }
    }

    /// Creates and fills a rectangular shaped map
    ///
    /// # Arguments
    /// * `meta`: The meta data for the map to create.
    ///
    /// # Example
    /// ```
    /// use hexx::storage::{HexStore, RectMap, RectMetadata, WrapStrategy};
    /// use hexx::*;
    ///
    /// let layout = HexLayout::pointy()
    ///     .with_hex_size(30.0)
    ///     .with_origin(Vec2::ZERO);
    /// let meta = RectMetadata::default()
    ///     .with_hex_layout(layout)
    ///     .with_half_size(IVec2 { x: 8, y: 4 })
    ///     .with_wrap_strategies([WrapStrategy::Cycle, WrapStrategy::Clamp]);
    /// let rect_map = RectMap::default_values(meta);
    ///
    /// assert_eq!(rect_map.get(Hex::new(0, 0)), Some(&0));
    /// ```
    #[must_use]
    pub fn default_values(meta: RectMetadata) -> Self
    where
        T: Default,
    {
        Self::new(meta, |_| Default::default())
    }

    // ================================
    // Wrapped Index
    // ================================
    /// Wrap if `hex` lie outside of layout
    /// - Clamp for latitude/vertical coordinate
    /// - Wrap for longitude/horizontal coordinate
    #[must_use]
    pub fn wrapped_get(&self, hex: Hex) -> &T {
        let idx = self.meta.wrapped_hex_to_idx(hex);
        &self.inner[idx]
    }

    /// Wrap if `hex` lie outside of layout
    /// - Clamp for latitude/vertical coordinate
    /// - Wrap for longitude/horizontal coordinate
    pub fn wrapped_get_mut(&mut self, hex: Hex) -> &mut T {
        let idx = self.meta.wrapped_hex_to_idx(hex);
        &mut self.inner[idx]
    }
}

impl<T> std::ops::Deref for RectMap<T> {
    type Target = RectMetadata;
    fn deref(&self) -> &Self::Target {
        &self.meta
    }
}

impl<T: Default> Default for RectMap<T> {
    fn default() -> Self {
        RectMetadata::default().build_default()
    }
}

impl<T> HexStore<T> for RectMap<T> {
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
        let meta = self.meta.clone();
        self.values_mut().enumerate().map(move |(i, value)| {
            let hex = meta.idx_to_hex(i);
            (hex, value)
        })
    }
}

impl<T: Debug> Debug for RectMap<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RectMap")
            .field("inner", &self.inner)
            .field("meta", &self.meta)
            .finish()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // Test dimensions for creating RectHexMap instances.
    const HALF_SIZES: &[[i32; 2]] = &[
        [0, 0],
        [0, 1],
        [1, 1],
        [2, 2],
        [8, 4],
        [16, 8],
        [28, 8],
        [64, 128],
        [127, 201],
        [256, 512],
        [-40, 3],
    ];

    const WRAP_STRATEGIES: &[[WrapStrategy; 2]] = &[
        [WrapStrategy::Clamp, WrapStrategy::Clamp],
        [WrapStrategy::Clamp, WrapStrategy::Cycle],
        [WrapStrategy::Cycle, WrapStrategy::Clamp],
        [WrapStrategy::Cycle, WrapStrategy::Cycle],
    ];

    /// Tests the conversion between index and hexagonal coordinates.
    #[test]
    fn idx_hex_test() {
        for dim in HALF_SIZES {
            let rect_hex_map = RectMetadata::default()
                .with_hex_layout(HexLayout::pointy().with_hex_size(1.0))
                .with_half_size(IVec2::from_array(*dim))
                .with_offset_mode(OffsetHexMode::Odd)
                .with_wrap_strategies([WrapStrategy::Cycle, WrapStrategy::Clamp])
                .build_default::<i32>();

            for idx in 0..rect_hex_map.len() {
                let hex = rect_hex_map.idx_to_hex(idx);
                assert!(rect_hex_map.contains_hex(hex));
                let _idx = rect_hex_map.hex_to_idx(hex);
                assert_eq!(Some(idx), _idx);
            }
        }
    }

    /// Tests the containment of ij coordinates within the map.
    #[test]
    fn contains_test() {
        for dim in HALF_SIZES {
            let rect_hex_map = RectMetadata::default()
                .with_hex_layout(HexLayout::pointy().with_hex_size(1.0))
                .with_offset_mode(OffsetHexMode::Odd)
                .with_half_size(IVec2::from_array(*dim))
                .with_wrap_strategies([WrapStrategy::Cycle, WrapStrategy::Clamp])
                .build_default::<i32>();

            for a in -2..2 {
                for b in -2..2 {
                    for i in (2 * a * dim[0])..((2 * a + 1) * dim[0]) {
                        for j in (2 * b * dim[1])..((2 * b + 1) * dim[1]) {
                            assert_eq!(
                                a == 0 && b == 0,
                                rect_hex_map.contains_ij(IVec2::new(i, j))
                            );
                        }
                    }
                }
            }
        }
    }

    /// Tests the wrapping functionality of ij coordinates.
    #[test]
    fn wrap_test() {
        for dim in HALF_SIZES {
            for wrap_strategies in WRAP_STRATEGIES {
                let rect_hex_map = RectMetadata::default()
                    .with_hex_layout(HexLayout::pointy().with_hex_size(1.0))
                    .with_offset_mode(OffsetHexMode::Odd)
                    .with_half_size(IVec2::from_array(*dim))
                    .with_wrap_strategies(*wrap_strategies)
                    .build_default::<i32>();

                for a in -2..2 {
                    for b in -2..2 {
                        let i_iter_a = (2 * a * dim[0] - dim[0])..((2 * a) * dim[0] + dim[0]);
                        let i_iter_b: Box<dyn Iterator<Item = i32>> =
                            if wrap_strategies[0] == WrapStrategy::Cycle {
                                Box::new(-dim[0]..dim[0])
                            } else {
                                Box::new(i_iter_a.clone().map(|i| i.clamp(-dim[0], dim[0] - 1)))
                            };

                        for (ia, ib) in i_iter_a.zip(i_iter_b) {
                            let j_iter_a = (2 * b * dim[1] - dim[1])..((2 * b) * dim[1] + dim[1]);
                            let j_iter_b: Box<dyn Iterator<Item = i32>> =
                                if wrap_strategies[1] == WrapStrategy::Cycle {
                                    Box::new(-dim[1]..dim[1])
                                } else {
                                    Box::new(j_iter_a.clone().map(|j| j.clamp(-dim[1], dim[1] - 1)))
                                };

                            for (ja, jb) in j_iter_a.zip(j_iter_b) {
                                let ij_a = IVec2::new(ia, ja);
                                let ij_b = IVec2::new(ib, jb);

                                let wij_a = rect_hex_map.wrap_ij(ij_a);
                                let wij_b = rect_hex_map.wrap_ij(ij_b);

                                assert_eq!(wij_a, wij_b);
                            }
                        }
                    }
                }
            }
        }
    }
}
