use std::ops::Neg;

use crate::{DiagonalDirection, Direction};

/// Describes a direction way, which can be a `Single` direction or a `Tie` betwen two directions.
pub enum DirectionWay<T> {
    /// Single direction
    Single(T),
    /// Tie between two directions
    Tie([T; 2]),
}

pub trait Way: Copy + Neg<Output = Self> {
    fn left(self) -> Self;
    fn right(self) -> Self;
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

    #[inline]
    #[must_use]
    pub(crate) const fn single(v: T) -> Self {
        Self::Single(v)
    }

    #[inline]
    #[must_use]
    pub(crate) const fn tie(v: [T; 2]) -> Self {
        Self::Tie(v)
    }
}

impl<T: Way> DirectionWay<T> {
    #[inline]
    pub(crate) fn way_from(is_neg: bool, eq_left: bool, eq_right: bool, dir: T) -> Self {
        match [is_neg, eq_left, eq_right] {
            [false, true, _] => Self::Tie([dir, dir.left()]),
            [true, true, _] => Self::Tie([-dir, (-dir).left()]),
            [false, _, true] => Self::Tie([dir, dir.right()]),
            [true, _, true] => Self::Tie([-dir, (-dir).right()]),
            [true, _, _] => Self::Single(-dir),
            _ => Self::Single(dir),
        }
    }
}

impl<T> From<T> for DirectionWay<T> {
    #[inline]
    fn from(value: T) -> Self {
        Self::single(value)
    }
}

impl<T> From<[T; 2]> for DirectionWay<T> {
    #[inline]
    fn from(value: [T; 2]) -> Self {
        Self::tie(value)
    }
}

impl Way for Direction {
    fn left(self) -> Self {
        self.left()
    }

    fn right(self) -> Self {
        self.right()
    }
}

impl Way for DiagonalDirection {
    fn left(self) -> Self {
        self.left()
    }

    fn right(self) -> Self {
        self.right()
    }
}
