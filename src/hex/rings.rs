use super::{Direction, Hex};

impl Hex {
    #[must_use]
    #[allow(clippy::cast_possible_wrap)]
    /// Retrieves one [`Hex`] ring around `self` in a given `range`.
    /// The returned coordinates start from `start_dir` and loop counter clockwise around `self`
    /// unless `clockwise` is set to `true`.
    ///
    /// If you only need the coordinates see [`Self::ring`].
    ///
    /// # Note
    /// The returned vector will be of `6 * radius` ([`Self::ring_count`]) length
    pub fn custom_ring(self, range: u32, start_dir: Direction, clockwise: bool) -> Vec<Self> {
        if range == 0 {
            return vec![self];
        }
        let mut directions = Self::NEIGHBORS_COORDS;
        // TODO: improve code clarity
        directions.rotate_left(start_dir as usize);
        if clockwise {
            directions.reverse();
            directions.rotate_left(1);
        } else {
            directions.rotate_left(2);
        }

        let mut hex = self + Self::neighbor_coord(start_dir) * range as i32;
        let mut res = Vec::with_capacity(Self::ring_count(range));
        for dir in directions {
            (0..range).for_each(|_| {
                res.push(hex);
                hex += dir;
            });
        }
        res
    }

    #[must_use]
    /// Retrieves one [`Hex`] ring around `self` in a given `range`.
    /// The returned coordinates start from [`Direction::TopRight`] and loop around `self` counter clockwise.
    ///
    /// See [`Self::custom_ring`] for more options.
    ///
    /// # Note
    /// The returned vector will be of `6 * radius` ([`Self::ring_count`]) length
    pub fn ring(self, range: u32) -> Vec<Self> {
        self.custom_ring(range, Direction::TopRight, false)
    }

    /// Retrieves `range` [`Hex`] rings around `self` in a given `range`.
    /// The returned coordinates start from [`Direction::TopRight`] and loop around `self` counter clockwise.
    ///
    /// See [`Self::custom_rings`] for more options.
    /// If you only need the coordinates see [`Self::spiral_range`].
    ///
    ///
    /// # Note
    /// The returned iterator will be of `radius + 1` length
    pub fn rings(self, range: u32) -> impl Iterator<Item = Vec<Self>> {
        (0..=range).map(move |r| self.ring(r))
    }

    /// Retrieves `range` [`Hex`] rings around `self` in a given `range`.
    /// The returned coordinates start from [`start_dir`] and loop around `self` counter clockwise
    /// unless `clockwise` is set to true.
    ///
    /// If you only need the coordinates see [`Self::spiral_range`] or [`Self::rings`].
    ///
    ///
    /// # Note
    /// The returned iterator will be of `radius + 1` length
    pub fn custom_rings(
        self,
        range: u32,
        start_dir: Direction,
        clockwise: bool,
    ) -> impl Iterator<Item = Vec<Self>> {
        (0..=range).map(move |r| self.custom_ring(r, start_dir, clockwise))
    }

    #[must_use]
    #[allow(clippy::cast_possible_wrap)]
    /// Retrieves one [`Hex`] ring edge around `self` in a given `range`.
    /// The returned coordinates start from `start_dir` and move counter clockwise around `self`
    /// unless `clockwise` is set to `true`.
    ///
    /// If you only need the coordinates see [`Self::ring_edge`].
    ///
    /// # Note
    /// The returned vector will be of `radius + 1` length
    pub fn custom_ring_edge(self, range: u32, start_dir: Direction, clockwise: bool) -> Vec<Self> {
        if range == 0 {
            return vec![self];
        }
        let end_dir = Self::neighbor_coord(if clockwise {
            start_dir.rotate_right(2)
        } else {
            start_dir.rotate_left(2)
        });
        let hex = self + Self::neighbor_coord(start_dir) * range as i32;
        (0..=range).map(|i| hex + end_dir * i as i32).collect()
    }

    #[must_use]
    /// Retrieves one [`Hex`] ring edge around `self` in a given `range`.
    /// The returned coordinates start from [`Direction::TopRight`] and move counter clockwise around `self`.
    ///
    /// See [`Self::custom_ring_edge`] for more options.
    ///
    /// # Note
    /// The returned vector will be of `radius + 1` length
    pub fn ring_edge(self, range: u32, start_dir: Direction) -> Vec<Self> {
        self.custom_ring_edge(range, start_dir, false)
    }

    /// Retrieves all successive [`Hex`] ring edges around `self` in a given `range`.
    /// The returned edges start from [`start_dir`] and move counter clockwise around `self`.
    ///
    /// See also [`Self::custom_ring_edges`]
    /// If you only need the coordinates see [`Self::custom_wedge`]
    pub fn ring_edges(self, range: u32, start_dir: Direction) -> impl Iterator<Item = Vec<Self>> {
        (0..=range).map(move |r| self.ring_edge(r, start_dir))
    }

    /// Retrieves all successive [`Hex`] ring edges around `self` in a given `range`.
    /// The returned edges start from [`start_dir`] and move counter clockwise around `self` unless
    /// `clockwise` is set to `true`.
    ///
    /// See also [`Self::ring_edges`]
    /// If you only need the coordinates see [`Self::wedge`]
    pub fn custom_ring_edges(
        self,
        range: u32,
        start_dir: Direction,
        clockwise: bool,
    ) -> impl Iterator<Item = Vec<Self>> {
        (0..=range).map(move |r| self.custom_ring_edge(r, start_dir, clockwise))
    }

    #[allow(clippy::cast_possible_truncation)]
    #[must_use]
    /// Retrieves all successive [`Hex`] ring edges around `self` in a given `RANGE` as an array of
    /// edges.
    /// The returned edges start from [`start_dir`] and move counter clockwise around `self` unless
    /// `clockwise` is set to `true`.
    ///
    /// See also [`Self::custom_cached_ring_edges`]
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
    /// let cache = Hex::ORIGIN.cached_custom_ring_edges::<10>(Direction::Top, true);
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
        start_dir: Direction,
        clockwise: bool,
    ) -> [Vec<Self>; RANGE] {
        std::array::from_fn(|r| self.custom_ring_edge(r as u32, start_dir, clockwise))
    }

    #[allow(clippy::cast_possible_truncation)]
    #[must_use]
    /// Retrieves all successive [`Hex`] ring edges around `self` in a given `RANGE` as an array of
    /// edges.
    /// The returned edges start from [`start_dir`] and move counter clockwise around `self`.
    ///
    /// See also [`Self::custom_cached_ring_edges`]
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
    /// let cache = Hex::ORIGIN.cached_ring_edges::<10>(Direction::Top);
    /// // We have our target center
    /// let target = Hex::new(11, 24);
    /// // We retrieve the ring of range 5 and offset it to match the target
    /// let five_ring = cache[5].iter().map(|h| *h + target);
    /// ```
    ///
    /// See this [article](https://www.redblobgames.com/grids/hexagons/directions.html) for more
    /// information
    pub fn cached_ring_edges<const RANGE: usize>(self, start_dir: Direction) -> [Vec<Self>; RANGE] {
        std::array::from_fn(|r| self.ring_edge(r as u32, start_dir))
    }

    #[allow(clippy::cast_possible_truncation)]
    #[must_use]
    /// Retrieves all successive [`Hex`] rings around `self` in a given `RANGE` as an array of
    /// rings.
    /// The returned rings start from [`Direction::TopRight`] and loop around `self` counter clockwise.
    ///
    /// See also [`Self::custom_cached_rings`]
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
        std::array::from_fn(|r| self.ring(r as u32))
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
        std::array::from_fn(|r| self.custom_ring(r as u32, start_dir, clockwise))
    }

    #[must_use]
    /// Retrieves all [`Hex`] around `self` in a given `range` but ordered as successive rings,
    /// starting from `start_dir` and looping counter clockwise unless `clockwise` is set to `true`, forming a spiral
    ///
    /// If you only need the coordinates see [`Self::spiral_range`].
    ///
    /// See this [article](https://www.redblobgames.com/grids/hexagons/#rings-spiral) for more
    /// information
    pub fn custom_spiral_range(
        self,
        range: u32,
        start_dir: Direction,
        clockwise: bool,
    ) -> Vec<Self> {
        let mut res = Vec::with_capacity(Self::range_count(range));
        for i in 0..=range {
            res.extend(self.custom_ring(i, start_dir, clockwise));
        }
        res
    }

    #[must_use]
    #[allow(clippy::cast_sign_loss)]
    /// Retrieves all [`Hex`] around `self` in a given `range` but ordered as successive rings,
    /// starting from [`Direction::TopRight`] and looping counter clockwise, forming a spiral.
    ///
    /// See [`Self::custom_spiral_range`] for more options
    ///
    /// See this [article](https://www.redblobgames.com/grids/hexagons/#rings-spiral) for more
    /// information
    pub fn spiral_range(self, range: u32) -> Vec<Self> {
        self.custom_spiral_range(range, Direction::TopRight, false)
    }

    #[inline]
    #[must_use]
    /// Counts how many coordinates there are in a ring at the given `range`
    pub const fn ring_count(range: u32) -> usize {
        6 * range as usize
    }
}
