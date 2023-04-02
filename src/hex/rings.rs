use super::{iter::ExactSizeHexIterator, Direction, Hex};
use crate::DiagonalDirection;

impl Hex {
    #[must_use]
    #[allow(clippy::cast_possible_wrap)]
    /// Retrieves one [`Hex`] ring around `self` in a given `range`.
    /// The returned coordinates start from `start_dir` and loop counter clockwise around `self`
    /// unless `clockwise` is set to `true`.
    ///
    /// > If you only need the coordinates see [`Self::ring`]
    ///
    /// # Note
    /// The returned iterator will have `6 * range` ([`Self::ring_count`]) items, unless `range` is
    /// 0 which will return `self`
    pub fn custom_ring(
        self,
        range: u32,
        start_dir: Direction,
        clockwise: bool,
    ) -> impl ExactSizeIterator<Item = Self> {
        let mut directions = Self::NEIGHBORS_COORDS;
        // TODO: improve code clarity
        directions.rotate_left(start_dir as usize);
        if clockwise {
            directions.reverse();
            directions.rotate_left(1);
        } else {
            directions.rotate_left(2);
        }

        let point = self + start_dir * range as i32;
        let iter = directions
            .into_iter()
            .flat_map(move |dir| std::iter::repeat(dir).take(range as usize))
            .scan(point, move |pos, dir| {
                let next = *pos + dir;
                if next == point {
                    None
                } else {
                    *pos = next;
                    Some(next)
                }
            });
        ExactSizeHexIterator {
            iter: std::iter::once(point).chain(iter),
            count: Self::ring_count(range),
        }
    }

    #[must_use]
    /// Retrieves one [`Hex`] ring around `self` in a given `range`.
    /// The returned coordinates start from [`Direction::TopRight`] and loop around `self` counter clockwise.
    ///
    /// > See [`Self::custom_ring`] for more options.
    ///
    /// # Note
    /// The returned iterator will have `6 * range` ([`Self::ring_count`]) items, unless `range` is
    /// 0 which will return `self`
    pub fn ring(self, range: u32) -> impl ExactSizeIterator<Item = Self> {
        self.custom_ring(range, Direction::TopRight, false)
    }

    /// Retrieves `range` [`Hex`] rings around `self` in a given `range`.
    /// The returned coordinates start from [`Direction::TopRight`] and loop around `self` counter clockwise.
    ///
    /// See [`Self::custom_rings`] for more options.
    /// If you only need the coordinates see [`Self::spiral_range`].
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    /// let rings: Vec<Vec<Hex>> = Hex::ZERO.rings(3..10).collect();
    /// assert_eq!(rings.len(), 7);
    /// ```
    pub fn rings(
        self,
        range: impl Iterator<Item = u32>,
    ) -> impl Iterator<Item = impl ExactSizeIterator<Item = Self>> {
        range.map(move |r| self.ring(r))
    }

    /// Retrieves `range` [`Hex`] rings around `self` in a given `range`.
    /// The returned coordinates start from `start_dir` and loop around `self` counter clockwise
    /// unless `clockwise` is set to true.
    ///
    /// If you only need the coordinates see [`Self::spiral_range`] or [`Self::rings`].
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    /// let rings: Vec<Vec<Hex>> = Hex::ZERO.custom_rings(3..10, Direction::Top, true).collect();
    /// assert_eq!(rings.len(), 7);
    /// ```
    pub fn custom_rings(
        self,
        range: impl Iterator<Item = u32>,
        start_dir: Direction,
        clockwise: bool,
    ) -> impl Iterator<Item = impl ExactSizeIterator<Item = Self>> {
        range.map(move |r| self.custom_ring(r, start_dir, clockwise))
    }

    #[must_use]
    #[allow(clippy::cast_possible_wrap)]
    /// Retrieves one [`Hex`] ring edge around `self` in a given `radius` and `direction`.
    /// The returned coordinates are sorted counter clockwise unless `clockwise` is set to `true`.
    ///
    /// If you only need the coordinates see [`Self::ring_edge`].
    ///
    /// # Note
    /// The returned vector will be of `radius + 1` length
    pub fn custom_ring_edge(
        self,
        radius: u32,
        direction: DiagonalDirection,
        clockwise: bool,
    ) -> impl ExactSizeIterator<Item = Self> {
        let [start_dir, end_dir] = if clockwise {
            let dir = direction.direction_left();
            [dir, dir >> 2]
        } else {
            let dir = direction.direction_right();
            [dir, dir << 2]
        };
        let p = self + start_dir * radius as i32;
        ExactSizeHexIterator {
            iter: (0..=radius).map(move |i| p + end_dir * i as i32),
            count: radius as usize + 1,
        }
    }

    #[must_use]
    /// Retrieves one [`Hex`] ring edge around `self` in a given `radius` and `direction`.
    /// The returned coordinates are sorted counter clockwise around `self`.
    ///
    /// See [`Self::custom_ring_edge`] for more options.
    ///
    /// # Note
    /// The returned vector will be of `radius + 1` length
    pub fn ring_edge(
        self,
        radius: u32,
        direction: DiagonalDirection,
    ) -> impl ExactSizeIterator<Item = Self> {
        self.custom_ring_edge(radius, direction, false)
    }

    /// Retrieves all successive [`Hex`] ring edges around `self` in given `ranges` and
    /// `direction`.
    /// The returned edges coordinates are sorted counter clockwise around `self`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    /// let edges: Vec<_> = Hex::ZERO.ring_edges(3..10, DiagonalDirection::Right).collect();
    /// assert_eq!(edges.len(), 7);
    /// ```
    ///
    /// See also [`Self::custom_ring_edges`]
    /// If you only need the coordinates see [`Self::custom_wedge`]
    pub fn ring_edges(
        self,
        ranges: impl Iterator<Item = u32>,
        direction: DiagonalDirection,
    ) -> impl Iterator<Item = impl ExactSizeIterator<Item = Self>> {
        ranges.map(move |r| self.ring_edge(r, direction))
    }

    /// Retrieves all successive [`Hex`] ring edges around `self` in given `ranges` and
    /// `direction`.
    /// The returned edges coordinates are sorted counter clockwise around `self` unless
    /// `clockwise` is set to `true`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    /// let edges: Vec<_> = Hex::ZERO.custom_ring_edges(3..10, DiagonalDirection::Right, true).collect();
    /// assert_eq!(edges.len(), 7);
    /// ```
    ///
    /// See also [`Self::ring_edges`]
    /// If you only need the coordinates see [`Self::wedge`]
    pub fn custom_ring_edges(
        self,
        ranges: impl Iterator<Item = u32>,
        direction: DiagonalDirection,
        clockwise: bool,
    ) -> impl Iterator<Item = impl ExactSizeIterator<Item = Self>> {
        ranges.map(move |r| self.custom_ring_edge(r, direction, clockwise))
    }

    /// Retrieves all successive [`Hex`] ring edges around `self` in given `ranges` and
    /// `direction`.
    /// The returned edges coordinates are sorted counter clockwise around `self` unless
    /// `clockwise` is set to `true`.
    ///
    /// See also [`Self::custom_ring_edges`]
    /// If you only need the coordinates see [`Self::wedge`]
    /// If you want a full wedge see [`Self::custom_full_wedge`]
    pub fn custom_wedge(
        self,
        ranges: impl Iterator<Item = u32>,
        direction: DiagonalDirection,
        clockwise: bool,
    ) -> impl Iterator<Item = Self> {
        self.custom_ring_edges(ranges, direction, clockwise)
            .flatten()
    }

    /// Retrieves all successive [`Hex`] ring edges from `self` to `rhs`
    /// The returned edges coordinates are sorted counter clockwise around `self` unless
    /// `clockwise` is set to `true`.
    ///
    /// See also [`Self::custom_ring_edges`] and [`Self::wedge_to`]
    #[must_use]
    pub fn custom_wedge_to(
        self,
        rhs: Self,
        clockwise: bool,
    ) -> impl ExactSizeIterator<Item = Self> {
        let range = self.unsigned_distance_to(rhs);
        let direction = self.diagonal_to(rhs);
        self.custom_full_wedge(range, direction, clockwise)
    }

    /// Retrieves all successive [`Hex`] ring edges around `self` in a given `range` and `direction`
    /// The returned edges coordinates are sorted counter clockwise around `self` unless
    /// `clockwise` is set to `true`.
    ///
    /// See also [`Self::custom_wedge`] and [`Self::full_wedge`]
    #[must_use]
    pub fn custom_full_wedge(
        self,
        range: u32,
        direction: DiagonalDirection,
        clockwise: bool,
    ) -> impl ExactSizeIterator<Item = Self> {
        ExactSizeHexIterator {
            iter: self.custom_wedge(0..=range, direction, clockwise),
            count: Self::wedge_count(range) as usize,
        }
    }

    /// Counts how many coordinates there are in a wedge of given `range`
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    /// let point = Hex::new(3, -6);
    /// let wedge: Vec<Hex> = point.wedge(0..=13, DiagonalDirection::Right).collect();
    /// assert_eq!(wedge.len(), Hex::wedge_count(13) as usize);
    /// ```
    #[inline]
    #[must_use]
    pub const fn wedge_count(range: u32) -> u32 {
        range * (range + 3) / 2 + 1
    }

    /// Retrieves all successive [`Hex`] ring edges around `self` in a given `range` and
    /// `direction`.
    /// The returned edges coordinates are sorted counter clockwise around `self`.
    ///
    /// See also [`Self::custom_ring_edges`] and [`Self::custom_wedge`]
    pub fn wedge(
        self,
        range: impl Iterator<Item = u32>,
        direction: DiagonalDirection,
    ) -> impl Iterator<Item = Self> {
        self.ring_edges(range, direction).flatten()
    }

    /// Retrieves all successive [`Hex`] ring edges from `self` to `rhs`
    /// The returned edges coordinates are sorted counter clockwise.
    ///
    /// See also [`Self::custom_ring_edges`] and [`Self::custom_wedge_to`]
    #[must_use]
    pub fn wedge_to(self, rhs: Self) -> impl ExactSizeIterator<Item = Self> {
        self.custom_wedge_to(rhs, false)
    }

    /// Retrieves all successive [`Hex`] ring edges around `self` in a given `range` and `direction`
    /// The returned edges coordinates are sorted counter clockwise around `self`.
    ///
    /// See also [`Self::custom_full_wedge`] and [`Self::wedge`]
    #[must_use]
    pub fn full_wedge(
        self,
        range: u32,
        direction: DiagonalDirection,
    ) -> impl ExactSizeIterator<Item = Self> {
        self.custom_full_wedge(range, direction, false)
    }

    /// Retrieves all successive [`Hex`] half ring edges around `self` in a given `range` and
    /// `direction`.
    ///
    /// See also [`Self::corner_wedge_to`] and [`Self::wedge`]
    pub fn corner_wedge(
        self,
        range: impl Iterator<Item = u32> + Clone,
        direction: Direction,
    ) -> impl Iterator<Item = Self> {
        let left = self.wedge(range.clone(), direction.diagonal_left());
        let right = self.wedge(range, direction.diagonal_right());
        left.chain(right)
            .filter(move |h| self.direction_to(*h) == direction)
    }

    /// Retrieves all successive [`Hex`] half ring edges from `self` to `rhs`
    ///
    /// See also [`Self::corner_wedge_to`] and [`Self::wedge_to`]
    pub fn corner_wedge_to(self, rhs: Self) -> impl Iterator<Item = Self> {
        let range = self.unsigned_distance_to(rhs);
        self.corner_wedge(0..=range, self.direction_to(rhs))
    }

    #[allow(clippy::cast_possible_truncation)]
    #[must_use]
    /// Retrieves all successive [`Hex`] ring edges around `self` in a given `RANGE` and `direction`
    /// as an array of edges.
    /// The returned edges coordinates are sorted counter clockwise around `self` unless
    /// `clockwise` is set to `true`.
    ///
    /// See also [`Self::cached_ring_edges`]
    /// If you only need the coordinates see [`Self::ring_edges`] or [`Self::wedge`].
    ///
    /// # Usage
    ///
    /// This function's objective is to pre-compute edges around a coordinate, the returned array
    /// can be used as a cache to avoid extra computation.
    ///
    /// ## Example
    ///
    /// ```rust
    /// # use hexx::*;
    ///
    /// // We cache 10 rings around the origin
    /// let cache = Hex::ORIGIN.cached_custom_ring_edges::<10>(DiagonalDirection::Right, true);
    /// // We have our target center
    /// let target = Hex::new(11, 24);
    /// // We retrieve the ring of range 5 and offset it to match the target
    /// let five_ring = cache[5].iter().map(|h| *h + target);
    /// ```
    ///
    /// See this [article](https://www.redblobgames.com/grids/hexagons/directions.html) for more
    /// information
    pub fn cached_custom_ring_edges<const RANGE: usize>(
        self,
        direction: DiagonalDirection,
        clockwise: bool,
    ) -> [Vec<Self>; RANGE] {
        std::array::from_fn(|r| {
            self.custom_ring_edge(r as u32, direction, clockwise)
                .collect()
        })
    }

    #[allow(clippy::cast_possible_truncation)]
    #[must_use]
    /// Retrieves all successive [`Hex`] ring edges around `self` in a given `RANGE` and
    /// `direction` as an array of edges.
    /// The returned edges coordinates are sorted counter clockwise around `self`.
    ///
    /// See also [`Self::cached_custom_ring_edges`]
    /// If you only need the coordinates see [`Self::ring_edges`] or [`Self::wedge`].
    ///
    /// # Usage
    ///
    /// This function's objective is to pre-compute edges around a coordinate, the returned array
    /// can be used as a cache to avoid extra computation.
    ///
    /// ## Example
    ///
    /// ```rust
    /// # use hexx::*;
    ///
    /// // We cache 10 rings around the origin
    /// let cache = Hex::ORIGIN.cached_ring_edges::<10>(DiagonalDirection::Right);
    /// // We have our target center
    /// let target = Hex::new(11, 24);
    /// // We retrieve the ring of range 5 and offset it to match the target
    /// let five_ring = cache[5].iter().map(|h| *h + target);
    /// ```
    ///
    /// See this [article](https://www.redblobgames.com/grids/hexagons/directions.html) for more
    /// information
    pub fn cached_ring_edges<const RANGE: usize>(
        self,
        direction: DiagonalDirection,
    ) -> [Vec<Self>; RANGE] {
        std::array::from_fn(|r| self.ring_edge(r as u32, direction).collect())
    }

    #[allow(clippy::cast_possible_truncation)]
    #[must_use]
    /// Retrieves all successive [`Hex`] rings around `self` in a given `RANGE` as an array of
    /// rings.
    /// The returned rings start from [`Direction::TopRight`] and loop around `self` counter clockwise.
    ///
    /// See also [`Self::cached_custom_rings`]
    /// If you only need the coordinates see [`Self::range`] or [`Self::spiral_range`].
    ///
    /// # Usage
    ///
    /// This function's objective is to pre-compute rings around a coordinate, the returned array
    /// can be used as a cache to avoid extra computation.
    ///
    /// ## Example
    ///
    /// ```rust
    /// # use hexx::*;
    ///
    /// // We cache 10 rings around the origin
    /// let cache = Hex::ORIGIN.cached_rings::<10>();
    /// // We have our target center
    /// let target = Hex::new(11, 24);
    /// // We retrieve the ring of range 5 and offset it to match the target
    /// let five_ring = cache[5].iter().map(|h| *h + target);
    /// ```
    ///
    /// See this [article](https://www.redblobgames.com/grids/hexagons/#rings-spiral) for more
    /// information
    pub fn cached_rings<const RANGE: usize>(self) -> [Vec<Self>; RANGE] {
        std::array::from_fn(|r| self.ring(r as u32).collect())
    }

    #[allow(clippy::cast_possible_truncation)]
    #[must_use]
    /// Retrieves all successive [`Hex`] rings around `self` in a given `RANGE` as an array of
    /// rings.
    /// The returned rings start from `start_dir`] and loop around `self` counter clockwise unless
    /// `clockwise` is set to `true`.
    ///
    /// See also [`Self::cached_rings`]
    /// If you only need the coordinates see [`Self::range`] or [`Self::custom_spiral_range`].
    ///
    /// # Usage
    ///
    /// This function's objective is to pre-compute rings around a coordinate, the returned array
    /// can be used as a cache to avoid extra computation.
    ///
    /// ## Example
    ///
    /// ```rust
    /// # use hexx::*;
    ///
    /// // We cache 10 rings around the origin
    /// let cache = Hex::ORIGIN.cached_custom_rings::<10>(Direction::Top, true);
    /// // We have our target center
    /// let target = Hex::new(11, 24);
    /// // We retrieve the ring of range 5 and offset it to match the target
    /// let five_ring = cache[5].iter().map(|h| *h + target);
    /// ```
    ///
    /// See this [article](https://www.redblobgames.com/grids/hexagons/#rings-spiral) for more
    /// information
    pub fn cached_custom_rings<const RANGE: usize>(
        self,
        start_dir: Direction,
        clockwise: bool,
    ) -> [Vec<Self>; RANGE] {
        std::array::from_fn(|r| self.custom_ring(r as u32, start_dir, clockwise).collect())
    }

    /// Retrieves all [`Hex`] around `self` in a given `range` but ordered as successive rings,
    /// starting from `start_dir` and looping counter clockwise unless `clockwise` is set to `true`, forming a spiral
    ///
    /// If you only need the coordinates see [`Self::spiral_range`].
    ///
    /// See this [article](https://www.redblobgames.com/grids/hexagons/#rings-spiral) for more
    /// information
    pub fn custom_spiral_range(
        self,
        range: impl Iterator<Item = u32>,
        start_dir: Direction,
        clockwise: bool,
    ) -> impl Iterator<Item = Self> {
        self.custom_rings(range, start_dir, clockwise).flatten()
    }

    /// Retrieves all [`Hex`] around `self` in a given `range` but ordered as successive rings,
    /// starting from [`Direction::TopRight`] and looping counter clockwise, forming a spiral.
    ///
    /// See [`Self::custom_spiral_range`] for more options
    ///
    /// See this [article](https://www.redblobgames.com/grids/hexagons/#rings-spiral) for more
    /// information
    pub fn spiral_range(self, range: impl Iterator<Item = u32>) -> impl Iterator<Item = Self> {
        self.custom_spiral_range(range, Direction::TopRight, false)
    }

    #[inline]
    #[must_use]
    /// Counts how many coordinates there are in a ring at the given `range`
    pub const fn ring_count(range: u32) -> usize {
        if range == 0 {
            1
        } else {
            6 * range as usize
        }
    }
}
