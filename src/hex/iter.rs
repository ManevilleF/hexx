use crate::{Hex, HexBounds};

/// Extension trait for iterators of [`Hex`]
pub trait HexIterExt: Iterator {
    /// Method which takes an iterator and finds the mean (average) value.
    ///
    /// This method will return [`Hex::ZERO`] on an empty iterator
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    /// let mean = Hex::ZERO.range(10).average();
    /// assert_eq!(mean, Hex::ZERO);
    /// ```
    #[doc(alias = "mean")]
    fn average(self) -> Hex;

    /// Method which takes an iterator and finds the center (centroid) value.
    ///
    /// This method will return [`Hex::ZERO`] on an empty iterator
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    /// let center = Hex::ZERO.range(10).center();
    /// assert_eq!(center, Hex::ZERO);
    /// ```
    #[doc(alias = "centroid")]
    fn center(self) -> Hex;

    /// Method which takes an iterator and finds the bounds containing all
    /// elements.
    ///
    /// This method will return ([`Hex::ZERO`], 0) on an empty iterator
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    /// let bounds = Hex::ZERO.range(10).bounds();
    /// assert_eq!(bounds.center, Hex::ZERO);
    /// assert_eq!(bounds.radius, 10);
    /// ```
    fn bounds(self) -> HexBounds;
}

impl<I: Iterator<Item = Hex>> HexIterExt for I {
    fn average(self) -> Hex {
        let mut sum = Hex::ZERO;
        let mut count = 0;

        for hex in self {
            count += 1;
            sum += hex;
        }
        // Avoid division by zero
        sum / count.max(1)
    }

    fn center(self) -> Hex {
        self.bounds().center
    }

    fn bounds(self) -> HexBounds {
        self.collect()
    }
}

/// Private container for a [`Hex`] [`Iterator`] of known size
#[derive(Debug, Clone)]
pub struct ExactSizeHexIterator<I> {
    /// The inner iterator
    pub iter: I,
    /// The remaining iterator elements count
    pub count: usize,
}

impl<I> Iterator for ExactSizeHexIterator<I>
where
    I: Iterator<Item = Hex>,
{
    type Item = Hex;

    fn next(&mut self) -> Option<Self::Item> {
        self.count = self.count.saturating_sub(1);
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.count, Some(self.count))
    }
}

impl<I> ExactSizeIterator for ExactSizeHexIterator<I> where I: Iterator<Item = Hex> {}
