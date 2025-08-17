use std::fmt::Debug;

use glam::{IVec2, UVec2, Vec2Swizzles};

use crate::{Hex, HexOrientation, OffsetHexMode, storage::HexStore};

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
/// If your use case doesn't match all of the above, use a
/// [`std::collections::HashMap`] instead
///
/// # Example
/// ```rust
/// use hexx::{
///     storage::{HexStore, RectMap, RectMetadata, WrapStrategy},
///     *,
/// };
///
/// let rect_map = RectMetadata::from_half_size(UVec2 { x: 8, y: 4 })
///     .with_orientation(HexOrientation::Pointy)
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
/// use hexx::{
///     storage::{HexStore, RectMap, RectMetadata, WrapStrategy},
///     *,
/// };
///
/// let rect_hex_map = RectMetadata::from_half_size(UVec2::new(8, 12))
///     .with_orientation(HexOrientation::Pointy)
///     .with_offset_mode(OffsetHexMode::Odd)
///     .with_wrap_strategies([WrapStrategy::Cycle, WrapStrategy::Clamp])
///     .build_default::<i32>();
/// let hex = Hex::new(0, 0);
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
    orientation: HexOrientation,
    /// the offset mode of the map
    ///
    /// affect which way does the map zic zac along point axis
    offset_mode: OffsetHexMode,
    /// the offset coordinate for the start element of the flat vec
    ///
    /// it represent the minimum offset coordinate element-wise among whole map
    ///
    start: IVec2,
    /// dimension of the map
    dim: UVec2,
    /// the wrapping strategy for indexing
    ///
    /// this only affect result of [`RectMap::wrapped_get`] and
    /// [`RectMap::wrapped_get_mut`]
    wrap_strategies: [WrapStrategy; 2],
}

/// Wrapping Strategy for when try to [`RectMap::wrapped_get`] and
/// [`RectMap::wrapped_get_mut`]
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

impl RectMetadata {
    // ================================
    // Builder Pattern
    // ================================
    /// create a new [`RectMetadata`] with half size
    ///
    /// # Param
    /// * `half_size`: the half span of the map
    ///
    /// the resulting map will have:
    /// - columns: `-half_size[0]..(half_size[0] - 1)`, and
    /// - rows: `-half_size[1]..(half_size[1] - 1)`, and
    #[must_use]
    pub fn from_half_size(half_size: UVec2) -> Self {
        Self {
            start: -half_size.as_ivec2(),
            dim: 2 * half_size,
            orientation: HexOrientation::Pointy,
            offset_mode: OffsetHexMode::Odd,
            wrap_strategies: [WrapStrategy::Cycle, WrapStrategy::Clamp],
        }
    }
    /// create a new [`RectMetadata`] with start and end
    ///
    /// # Param
    /// * `start`: the element wise minimum offset coordinate in the map
    /// * `end`: the element wise maximum offset coordinate in the map
    ///
    /// the resulting map will have:
    /// - columns: `start[0]..max(start[0], end[0])`
    /// - rows: `start[1]..max(start[1], end[1])`
    #[must_use]
    pub fn from_start_end(start: IVec2, end: IVec2) -> Self {
        Self {
            start,
            dim: (end.max(start) - start).as_uvec2(),
            orientation: HexOrientation::Pointy,
            offset_mode: OffsetHexMode::Odd,
            wrap_strategies: [WrapStrategy::Cycle, WrapStrategy::Clamp],
        }
    }
    /// create a new [`RectMetadata`] with start and dimension
    ///
    /// # Param
    /// * `start`: the element wise minimum offset coordinate in the map
    /// * `dim`: the dimension of the map
    ///
    /// the resulting map will have:
    /// - columns: `start[0]..(start[0] + dim[0])`
    /// - rows: `start[1]..(start[1] + dim[1])`
    #[must_use]
    pub const fn from_start_dim(start: IVec2, dim: UVec2) -> Self {
        Self {
            start,
            dim,
            orientation: HexOrientation::Pointy,
            offset_mode: OffsetHexMode::Odd,
            wrap_strategies: [WrapStrategy::Cycle, WrapStrategy::Clamp],
        }
    }
    /// builder patter, set half size
    ///
    /// # Param
    /// * `half_size`: the half span of the map
    ///
    /// the resulting map will have:
    /// - columns: `-half_size[0]..(half_size[0] - 1)`, and
    /// - rows: `-half_size[1]..(half_size[1] - 1)`, and
    #[must_use]
    pub fn with_half_size(mut self, half_size: IVec2) -> Self {
        // self.half_size = half_size.abs();
        self.start = -half_size.abs();
        self.dim = (2 * half_size.abs()).as_uvec2();
        self
    }
    /// builder patter, set start and end
    ///
    /// # Param
    /// * `start`: the element wise minimum offset coordinate in the map
    /// * `end`: the element wise maximum offset coordinate in the map
    ///
    /// the resulting map will have:
    /// - columns: `start[0]..max(start[0], end[0])`
    /// - rows: `start[1]..max(start[1], end[1])`
    #[must_use]
    pub fn with_start_end(mut self, start: IVec2, end: IVec2) -> Self {
        // self.half_size = half_size.abs();
        self.start = start;
        self.dim = (end.max(start) - start).as_uvec2();
        self
    }
    /// builder pattern, set start and dimension
    ///
    /// # Param
    /// * `start`: the element wise minimum offset coordinate in the map
    /// * `dim`: the dimension of the map
    ///
    /// the resulting map will have:
    /// - columns: `start[0]..(start[0] + dim[0])`
    /// - rows: `start[1]..(start[1] + dim[1])`
    #[must_use]
    pub const fn with_start_dim(mut self, start: IVec2, dim: UVec2) -> Self {
        self.start = start;
        self.dim = dim;
        self
    }
    /// builder patter, set hex layout
    #[must_use]
    pub const fn with_orientation(mut self, orientation: HexOrientation) -> Self {
        self.orientation = orientation;
        self
    }
    /// builder pattern, set offset mode
    ///
    /// affect which way does the map zic zac along point axis
    #[must_use]
    pub const fn with_offset_mode(mut self, offset_mode: OffsetHexMode) -> Self {
        self.offset_mode = offset_mode;
        self
    }
    /// builder patter, set wrapping strategy
    ///
    /// this only affect result of [`RectMap::wrapped_get`] and
    /// [`RectMap::wrapped_get_mut`]
    #[must_use]
    pub const fn with_wrap_strategies(mut self, wrap_strategies: [WrapStrategy; 2]) -> Self {
        self.wrap_strategies = wrap_strategies;
        self
    }

    /// builder patter, build map with function to eval value
    /// # Example
    /// ```rust
    /// use hexx::{
    ///     storage::{HexStore, RectMap, RectMetadata, WrapStrategy},
    ///     *,
    /// };
    ///
    /// let rect_map = RectMetadata::from_half_size(UVec2 { x: 8, y: 4 })
    ///     .with_orientation(HexOrientation::Pointy)
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
    /// use hexx::{
    ///     storage::{HexStore, RectMap, RectMetadata, WrapStrategy},
    ///     *,
    /// };
    ///
    /// let rect_map = RectMetadata::from_half_size(UVec2 { x: 8, y: 4 })
    ///     .with_orientation(HexOrientation::Pointy)
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
    /// get the hex orientation of the map
    #[must_use]
    pub const fn orientation(&self) -> HexOrientation {
        self.orientation
    }
    /// get the offset mode of the map
    #[must_use]
    pub const fn offset_mode(&self) -> OffsetHexMode {
        self.offset_mode
    }
    /// get the dimension of the map
    #[must_use]
    pub const fn dim(&self) -> UVec2 {
        self.dim
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
        let idx = idx as u32;
        let rc = UVec2::new(idx % self.dim.x, idx / self.dim.x);
        // let rc = IVec2::new(idx % (2 * self.half_size.x), idx / (2 *
        // self.half_size.x)); let ij = rc - self.half_size;
        let ij = rc.as_ivec2() + self.start;
        self.ij_to_hex(ij)
    }

    fn ij_to_hex(&self, ij: IVec2) -> Hex {
        Hex::from_offset_coordinates(ij.xy().to_array(), self.offset_mode, self.orientation)
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
            .to_offset_coordinates(self.offset_mode, self.orientation)
            .into();
        v.xy()
    }

    /// fallable input, internal
    ///
    /// - `ij` offset coordinate
    /// - => `rc` 2D view of `Vec`
    /// - => `idx` `Vec` index
    fn ij_to_idx(&self, ij: IVec2) -> usize {
        let rc = (ij - self.start).as_uvec2();
        (rc.x + rc.y * self.dim.x) as usize
    }

    /// infallable
    fn wrap_ij(&self, mut ij: IVec2) -> IVec2 {
        let dim = self.dim.as_ivec2();
        let end = self.start + dim;
        if self.wrap_strategies[0] == WrapStrategy::Cycle {
            while ij.x < -self.start.x {
                ij.x += dim.x;
            }
            while ij.x >= end.x {
                ij.x -= dim.x;
            }
        } else {
            ij.x = ij.x.clamp(self.start.x, end.x - 1);
        }

        if self.wrap_strategies[1] == WrapStrategy::Cycle {
            while ij.y < self.start.y {
                ij.y += dim.y;
            }
            while ij.y >= end.y {
                ij.y -= dim.y;
            }
        } else {
            ij.y = ij.y.clamp(self.start.y, end.y - 1);
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
        ij == ij.clamp(self.start, self.start + self.dim.as_ivec2() - IVec2::ONE)
            && !self.is_empty()
    }

    // ================================
    // Iteration
    // ================================
    /// total size of the map
    #[must_use]
    pub fn len(&self) -> usize {
        self.dim.element_product() as usize
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
    /// use hexx::{
    ///     storage::{HexStore, RectMap, RectMetadata, WrapStrategy},
    ///     *,
    /// };
    ///
    /// let meta = RectMetadata::from_half_size(UVec2 { x: 8, y: 4 })
    ///     .with_orientation(HexOrientation::Pointy)
    ///     .with_wrap_strategies([WrapStrategy::Cycle, WrapStrategy::Clamp]);
    /// let rect_map = RectMap::new(meta, |hex| hex.x + hex.y);
    ///
    /// assert_eq!(rect_map.get(Hex::new(0, 0)), Some(&0));
    /// ```
    #[must_use]
    #[allow(clippy::cast_possible_wrap)]
    pub fn new(meta: RectMetadata, mut values: impl FnMut(Hex) -> T) -> Self {
        let size = (meta.dim.element_product() * 4) as usize;
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
    /// use hexx::{
    ///     storage::{HexStore, RectMap, RectMetadata, WrapStrategy},
    ///     *,
    /// };
    ///
    /// let meta = RectMetadata::from_half_size(UVec2 { x: 8, y: 4 })
    ///     .with_orientation(HexOrientation::Pointy)
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
mod half_size_test {
    use super::*;

    // Test dimensions for creating RectHexMap instances.
    const HALF_SIZES: &[[u32; 2]] = &[
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
            let rect_map = RectMetadata::from_half_size(UVec2::from_array(*dim))
                .with_orientation(HexOrientation::Pointy)
                .with_offset_mode(OffsetHexMode::Odd)
                .with_wrap_strategies([WrapStrategy::Cycle, WrapStrategy::Clamp])
                .build_default::<i32>();

            for idx in 0..rect_map.len() {
                let hex = rect_map.idx_to_hex(idx);
                assert!(rect_map.contains_hex(hex));
                let _idx = rect_map.hex_to_idx(hex);
                assert_eq!(Some(idx), _idx);
            }
        }
    }

    /// Tests the containment of ij coordinates within the map.
    #[test]
    fn contains_test() {
        for dim in HALF_SIZES {
            let rect_map = RectMetadata::from_half_size(UVec2::from_array(*dim))
                .with_orientation(HexOrientation::Pointy)
                .with_offset_mode(OffsetHexMode::Odd)
                .with_wrap_strategies([WrapStrategy::Cycle, WrapStrategy::Clamp])
                .build_default::<i32>();

            let dim = [dim[0] as i32, dim[1] as i32];

            for a in -2..2 {
                for b in -2..2 {
                    for i in (2 * a * dim[0])..((2 * a + 1) * dim[0]) {
                        for j in (2 * b * dim[1])..((2 * b + 1) * dim[1]) {
                            assert_eq!(a == 0 && b == 0, rect_map.contains_ij(IVec2::new(i, j)));
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
                let rect_map = RectMetadata::from_half_size(UVec2::from_array(*dim))
                    .with_orientation(HexOrientation::Pointy)
                    .with_offset_mode(OffsetHexMode::Odd)
                    .with_wrap_strategies(*wrap_strategies)
                    .build_default::<i32>();

                let dim = [dim[0] as i32, dim[1] as i32];

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

                                let wij_a = rect_map.wrap_ij(ij_a);
                                let wij_b = rect_map.wrap_ij(ij_b);

                                assert_eq!(wij_a, wij_b);
                            }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod start_end_test {
    // ================================
    // Start End Test
    // ================================
    use super::*;

    const START_END: &[([i32; 2], [i32; 2])] = &[
        ([0, 0], [0, 0]),
        ([0, 0], [1, 1]),
        ([-1, -1], [1, 1]),
        ([-10, -10], [10, 10]),
        ([-10, -10], [15, 15]),
        ([0, 0], [15, 15]),
        ([0, 0], [-30, -30]),
        ([-17, -13], [31, 41]),
        ([-64, -64], [64, 64]),
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
        for (start, end) in START_END {
            let rect_map =
                RectMetadata::from_start_end((*start).into(), (*end).into()).build_default::<i32>();

            for idx in 0..rect_map.len() {
                let hex = rect_map.idx_to_hex(idx);
                assert!(rect_map.contains_hex(hex));
                let ij = rect_map.hex_to_ij(hex);
                assert!(rect_map.contains_ij(ij));
                assert_eq!(Some(idx), rect_map.hex_to_idx(hex));
            }
        }
    }

    #[test]
    fn contains_test() {
        for (start, end) in START_END {
            let rect_map =
                RectMetadata::from_start_end((*start).into(), (*end).into()).build_default::<i32>();

            for i in (start[0] - 10)..(end[0] + 10) {
                for j in (start[1] - 10)..(end[1] + 10) {
                    let contain = (i >= start[0] && i < end[0]) && (j >= start[1] && j < end[1]);

                    println!(
                        "{:?} || {:?} - {:?} | {}, {} | {}",
                        rect_map.dim, start, end, i, j, contain
                    );

                    assert_eq!(rect_map.contains_ij([i, j].into()), contain);
                }
            }
        }
    }

    #[test]
    fn wrap_test() {
        for (start, end) in START_END {
            for wrap_strategies in WRAP_STRATEGIES {
                let rect_map = RectMetadata::from_start_end((*start).into(), (*end).into())
                    .with_wrap_strategies(*wrap_strategies)
                    .build_default::<i32>();

                let dim = [(end[0] - start[0]).max(0), (end[1] - start[1]).max(0)];

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

                                let wij_a = rect_map.wrap_ij(ij_a);
                                let wij_b = rect_map.wrap_ij(ij_b);

                                assert_eq!(wij_a, wij_b);
                            }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod start_dim_test {
    // ================================
    // Start End Test
    // ================================
    use super::*;

    const START_END: &[([i32; 2], [u32; 2])] = &[
        ([0, 0], [0, 0]),
        ([0, 0], [1, 1]),
        ([-1, -1], [1, 1]),
        ([-10, -10], [10, 10]),
        ([-10, -10], [15, 15]),
        ([0, 0], [15, 15]),
        ([0, 0], [30, 30]),
        ([-17, -13], [31, 41]),
        ([-64, -64], [64, 64]),
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
        for (start, dim) in START_END {
            let rect_map =
                RectMetadata::from_start_dim((*start).into(), (*dim).into()).build_default::<i32>();

            for idx in 0..rect_map.len() {
                let hex = rect_map.idx_to_hex(idx);
                assert!(rect_map.contains_hex(hex));
                let ij = rect_map.hex_to_ij(hex);
                assert!(rect_map.contains_ij(ij));
                assert_eq!(Some(idx), rect_map.hex_to_idx(hex));
            }
        }
    }

    #[test]
    fn contains_test() {
        for (start, dim) in START_END {
            let rect_map =
                RectMetadata::from_start_dim((*start).into(), (*dim).into()).build_default::<i32>();

            for i in (start[0] - 10)..(start[0] + dim[0] as i32 + 10) {
                for j in (start[1] - 10)..(start[1] + dim[1] as i32 + 10) {
                    let contain = (i >= start[0] && i < start[0] + dim[0] as i32)
                        && (j >= start[1] && j < start[1] + dim[1] as i32);
                    assert_eq!(rect_map.contains_ij([i, j].into()), contain);
                }
            }
        }
    }

    #[test]
    fn wrap_test() {
        for (start, dim) in START_END {
            for wrap_strategies in WRAP_STRATEGIES {
                let rect_map = RectMetadata::from_start_dim((*start).into(), (*dim).into())
                    .with_wrap_strategies(*wrap_strategies)
                    .build_default::<i32>();

                let dim = [dim[0] as i32, dim[1] as i32];

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

                                let wij_a = rect_map.wrap_ij(ij_a);
                                let wij_b = rect_map.wrap_ij(ij_b);

                                assert_eq!(wij_a, wij_b);
                            }
                        }
                    }
                }
            }
        }
    }
}
