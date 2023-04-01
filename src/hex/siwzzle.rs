use super::Hex;

impl Hex {
    #[inline]
    #[must_use]
    #[doc(alias = "qq")]
    /// Returns a new [`Hex`] with both `x` and `y` set to the current `x` value
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    ///
    /// let point = Hex::new(1, 2);
    /// assert_eq!(point.xx(), Hex::new(1, 1));
    /// ```
    pub const fn xx(self) -> Self {
        Self::splat(self.x)
    }

    #[inline]
    #[must_use]
    #[doc(alias = "rr")]
    /// Returns a new [`Hex`] with both `x` and `y` set to the current `y` value
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    ///
    /// let point = Hex::new(1, 2);
    /// assert_eq!(point.yy(), Hex::new(2, 2));
    /// ```
    pub const fn yy(self) -> Self {
        Self::splat(self.y)
    }

    #[inline]
    #[must_use]
    #[doc(alias = "ss")]
    /// Returns a new [`Hex`] with both `x` and `y` set to the current `z` value
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    ///
    /// let point = Hex::new(1, 2);
    /// assert_eq!(point.zz(), Hex::new(-3, -3));
    /// ```
    pub const fn zz(self) -> Self {
        Self::splat(self.z())
    }

    #[inline]
    #[must_use]
    #[doc(alias = "rq")]
    /// Returns a new [`Hex`] with invertex `x` and `y` values
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    ///
    /// let point = Hex::new(1, 2);
    /// assert_eq!(point.yx(), Hex::new(2, 1));
    /// ```
    pub const fn yx(self) -> Self {
        Self {
            x: self.y,
            y: self.x,
        }
    }

    #[inline]
    #[must_use]
    #[doc(alias = "rs")]
    /// Returns a new [`Hex`] with its `y` valye as `x` and its `z` value as `y`
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    ///
    /// let point = Hex::new(1, 2);
    /// assert_eq!(point.yz(), Hex::new(2, -3));
    /// ```
    pub const fn yz(self) -> Self {
        Self {
            x: self.y,
            y: self.z(),
        }
    }

    #[inline]
    #[must_use]
    #[doc(alias = "qs")]
    /// Returns a new [`Hex`] with its `z` value as `y`
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    ///
    /// let point = Hex::new(1, 2);
    /// assert_eq!(point.xz(), Hex::new(1, -3));
    /// ```
    pub const fn xz(self) -> Self {
        Self {
            x: self.x,
            y: self.z(),
        }
    }

    #[inline]
    #[must_use]
    #[doc(alias = "sq")]
    /// Returns a new [`Hex`] with its `z` value as `x` and its `x` value as `y`
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    ///
    /// let point = Hex::new(1, 2);
    /// assert_eq!(point.zx(), Hex::new(-3, 1));
    /// ```
    pub const fn zx(self) -> Self {
        Self {
            x: self.z(),
            y: self.x,
        }
    }

    #[inline]
    #[must_use]
    #[doc(alias = "sr")]
    /// Returns a new [`Hex`] with its `z` value as `x`
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    ///
    /// let point = Hex::new(1, 2);
    /// assert_eq!(point.zy(), Hex::new(-3, 2));
    /// ```
    pub const fn zy(self) -> Self {
        Self {
            x: self.z(),
            y: self.y,
        }
    }
}
