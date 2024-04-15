use std::ops::Neg;

use crate::{EdgeDirection, VertexDirection};

/// Describes a direction way, which can be a `Single` direction or a `Tie`
/// betwen two directions.
///
/// # Comparison
///
/// To compare it with its inner [`EdgeDirection`] or [`VertexDirection`] you
/// can use `Self::contains` or the [`PartialEq`] implementation:
///
/// ```rust
/// # use hexx::*;
/// let a = hex(1, 4);
/// let b = hex(6, -2);
/// let way = a.way_to(b);
/// let diag_way = a.diagonal_way_to(b);
/// if way == EdgeDirection::FLAT_TOP {
///     // do something
/// }
/// if diag_way == VertexDirection::FLAT_LEFT {
///     // do something
/// }
/// ```
///
/// You can also `unwrap` the way to safely retrieve a single direction, with
/// potential inaccuracy in case of a `Tie`
#[derive(Debug)]
pub enum DirectionWay<T> {
    /// Single direction
    Single(T),
    /// Tie between two directions
    Tie([T; 2]),
}

pub trait Way: Copy + Neg<Output = Self> {
    fn ccw(self) -> Self;
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
    /// Resolves the current way and returns the `Single` direction or the first
    /// of the two in `Tie`.
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
    /// Returns `DirectionWay::<U>` with `func` applied to the items
    pub fn map<U>(self, mut func: impl FnMut(T) -> U) -> DirectionWay<U> {
        match self {
            Self::Single(v) => DirectionWay::Single(func(v)),
            Self::Tie(v) => DirectionWay::Tie(v.map(func)),
        }
    }
}

impl<T: Way> DirectionWay<T> {
    #[inline]
    pub(crate) fn way_from(is_neg: bool, eq_left: bool, eq_right: bool, dir: T) -> Self {
        let dir = if is_neg { -dir } else { dir };
        match [eq_left, eq_right] {
            [true, _] => Self::Tie([dir, dir.ccw()]),
            [_, true] => Self::Tie([dir, dir.cw()]),
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

impl Way for EdgeDirection {
    #[inline]
    fn ccw(self) -> Self {
        self.counter_clockwise()
    }

    #[inline]
    fn cw(self) -> Self {
        self.clockwise()
    }
}

impl Way for VertexDirection {
    #[inline]
    fn ccw(self) -> Self {
        self.counter_clockwise()
    }

    #[inline]
    fn cw(self) -> Self {
        self.clockwise()
    }
}
