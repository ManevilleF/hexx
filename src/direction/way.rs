use std::ops::Neg;

use crate::{DiagonalDirection, Direction};

/// Describes a direction way, which can be a `Single` direction or a `Tie` betwen two directions.
///
/// # Comparison
///
/// To compare it with its inner [`Direction`] or [`DiagonalDirection`] you can use
/// `Self::contains` or the [`PartialEq`] implementation:
///
/// ```rust
/// # use hexx::*;
/// let a = hex(1,4);
/// let b = hex(6,-2);
/// let way = a.way_to(b);
/// let diag_way = a.diagonal_way_to(b);
/// if way == Direction::Top {
///   // do something
/// }
/// if diag_way == DiagonalDirection::Left {
///   // do something
/// }
/// ```
///
/// You can also `unwrap` the way to safely retrieve a single direction, with potential inaccuracy
/// in case of a `Tie`
pub enum DirectionWay<T> {
    /// Single direction
    Single(T),
    /// Tie between two directions
    Tie([T; 2]),
}

pub trait Way: Copy + Neg<Output = Self> {
    #[deprecated = "Use ccw"]
    fn left(self) -> Self;
    fn ccw(self) -> Self;
    #[deprecated = "Use cw"]
    fn right(self) -> Self;
    fn cw(self) -> Self;
}

impl<T: PartialEq> PartialEq<T> for DirectionWay<T> {
    fn eq(&self, other: &T) -> bool {
        self.contains(other)
    }
}

impl<T> DirectionWay<T> {
    #[inline]
    #[must_use]
    #[allow(clippy::missing_const_for_fn)]
    /// Resolves the current way and returns the `Single` direction or the first of the two in `Tie`.
    pub fn unwrap(self) -> T {
        match self {
            Self::Single(v) | Self::Tie([v, _]) => v,
        }
    }

    #[inline]
    #[must_use]
    /// Does the way include the given `dir` ?
    pub fn contains(&self, dir: &T) -> bool
    where
        T: PartialEq,
    {
        match self {
            Self::Single(d) => d.eq(dir),
            Self::Tie([a, b]) => a.eq(dir) || b.eq(dir),
        }
    }
}

impl<T: Way> DirectionWay<T> {
    #[inline]
    pub(crate) fn way_from(is_neg: bool, eq_left: bool, eq_right: bool, dir: T) -> Self {
        match [is_neg, eq_left, eq_right] {
            [false, true, _] => Self::Tie([dir, dir.ccw()]),
            [true, true, _] => Self::Tie([-dir, (-dir).ccw()]),
            [false, _, true] => Self::Tie([dir, dir.cw()]),
            [true, _, true] => Self::Tie([-dir, (-dir).cw()]),
            [true, _, _] => Self::Single(-dir),
            _ => Self::Single(dir),
        }
    }
}

impl<T> From<T> for DirectionWay<T> {
    #[inline]
    fn from(value: T) -> Self {
        Self::Single(value)
    }
}

impl<T> From<[T; 2]> for DirectionWay<T> {
    #[inline]
    fn from(value: [T; 2]) -> Self {
        Self::Tie(value)
    }
}

impl Way for Direction {
    fn left(self) -> Self {
        self.counter_clockwise()
    }

    fn ccw(self) -> Self {
        self.counter_clockwise()
    }

    fn right(self) -> Self {
        self.clockwise()
    }

    fn cw(self) -> Self {
        self.clockwise()
    }
}

impl Way for DiagonalDirection {
    fn left(self) -> Self {
        self.counter_clockwise()
    }

    fn ccw(self) -> Self {
        self.counter_clockwise()
    }

    fn right(self) -> Self {
        self.clockwise()
    }

    fn cw(self) -> Self {
        self.clockwise()
    }
}
