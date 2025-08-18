use super::Hex;

impl Hex {
    /// Computes squared euclidean distance from `self` to `origin` as an
    /// integer number in the Cartesian coordinate system.
    /// Euclidean distance can vary for coordinates in the same range, and
    /// can be used for operations outside of the hexagonal space, like
    /// checking if coordinates are in a circular range instead of an
    /// hexagonal range
    ///
    /// Note: For most cases you should use an [`crate::HexLayout`]
    ///
    /// > Source:
    /// > Xiangguo Li's 2013 [Paper]. ([DOI]) gives a formula for Euclidean
    /// > distance
    ///
    /// [Paper]: https://scholar.google.com/scholar?q=Storage+and+addressing+scheme+for+practical+hexagonal+image+processing
    /// [DOI]: https://doi.org/10.1117/1.JEI.22.1.010502
    #[must_use]
    pub const fn squared_euclidean_length(self) -> i32 {
        self.x.pow(2) + self.y.pow(2) + (self.x * self.y)
    }

    /// Computes euclidean distance from `self` to `origin` as an floating point
    /// number in the Cartesian coordinate system.
    /// Euclidean distance can vary for coordinates in the same range, and
    /// can be used for operations outside of the hexagonal space, like
    /// checking if coordinates are in a circular range instead of an
    /// hexagonal range
    ///
    /// Note: For most cases you should use an [`crate::HexLayout`]
    ///
    /// > Source:
    /// > Xiangguo Li's 2013 [Paper]. ([DOI]) gives a formula for Euclidean
    /// > distance
    ///
    /// [Paper]: https://scholar.google.com/scholar?q=Storage+and+addressing+scheme+for+practical+hexagonal+image+processing
    /// [DOI]: https://doi.org/10.1117/1.JEI.22.1.010502
    #[expect(clippy::cast_precision_loss)]
    #[must_use]
    pub fn euclidean_length(self) -> f32 {
        (self.squared_euclidean_length() as f32).sqrt()
    }

    /// Computes squared euclidean distance from `self` to `rhs` as an integer
    /// number in the Cartesian coordinate system.
    /// Euclidean distance can vary for coordinates in the same range, and
    /// can be used for operations outside of the hexagonal space, like
    /// checking if coordinates are in a circular range instead of an
    /// hexagonal range
    ///
    /// Note: For most cases you should use an [`crate::HexLayout`]
    ///
    /// Also check:
    /// - [`Hex::euclidean_distance_to`]
    /// - [`Hex::circular_range`]
    ///
    /// > Source:
    /// > Xiangguo Li's 2013 [Paper]. ([DOI]) gives a formula for Euclidean
    /// > distance
    ///
    /// [Paper]: https://scholar.google.com/scholar?q=Storage+and+addressing+scheme+for+practical+hexagonal+image+processing
    /// [DOI]: https://doi.org/10.1117/1.JEI.22.1.010502
    #[must_use]
    pub const fn squared_euclidean_distance_to(self, rhs: Self) -> i32 {
        rhs.const_sub(self).squared_euclidean_length()
    }

    /// Computes euclidean distance from `self` to `rhs` as a floating point
    /// number in the Cartesian coordinate system.
    /// Euclidean distance can vary for coordinates in the same range, and
    /// can be used for operations outside of the hexagonal space, like
    /// checking if coordinates are in a circular range instead of an
    /// hexagonal range
    ///
    /// Note: For most cases you should use an [`crate::HexLayout`]
    ///
    /// Also check:
    /// - [`Hex::squared_euclidean_distance_to`]
    /// - [`Hex::circular_range`]
    ///
    /// > Source:
    /// > Xiangguo Li's 2013 [Paper]. ([DOI]) gives a formula for Euclidean
    /// > distance
    ///
    /// [Paper]: https://scholar.google.com/scholar?q=Storage+and+addressing+scheme+for+practical+hexagonal+image+processing
    /// [DOI]: https://doi.org/10.1117/1.JEI.22.1.010502
    #[must_use]
    #[expect(clippy::cast_precision_loss)]
    pub fn euclidean_distance_to(self, rhs: Self) -> f32 {
        (self.squared_euclidean_distance_to(rhs) as f32).sqrt()
    }

    /// Retrieves all [`Hex`] around `self` in a given circular `range`.
    ///
    /// > See also [`Hex::range`] for hexagonal ranges
    ///
    /// Note that this implementation is very naive and inefficient.
    /// Use sparingly
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    /// let coord = hex(12, 34);
    /// assert_eq!(coord.circular_range(0.0).count(), 1);
    /// assert_eq!(coord.circular_range(1.0).count(), 7);
    /// ```
    #[expect(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    pub fn circular_range(self, range: f32) -> impl Iterator<Item = Self> {
        let radius = range.ceil() as u32;
        // TODO: Improve this computation to have the smallest hexagon
        // which fits the circle
        let hex_range = radius + (range / 6.0).floor() as u32;
        self.range(hex_range)
            .filter(move |h| self.euclidean_distance_to(*h) <= range)
    }
}
