use crate::Hex;
use std::{
    cmp::Ordering,
    ops::{Deref, DerefMut},
};

/// [`Ordering`] wrapper around [`Hex`], comparing [`Hex::length`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OrdByLength(pub Hex);

/// [`Ordering`] wrapper around [`Hex`], comparing [`Hex::x`] then [`Hex::y`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OrdByXY(pub Hex);

/// [`Ordering`] wrapper around [`Hex`], comparing [`Hex::y`] then [`Hex::x`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OrdByYX(pub Hex);

impl Ord for OrdByLength {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.length().cmp(&other.0.length())
    }
}

impl Ord for OrdByXY {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.x.cmp(&other.0.x).then(self.0.y.cmp(&other.0.y))
    }
}

impl Ord for OrdByYX {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.y.cmp(&other.0.y).then(self.0.x.cmp(&other.0.x))
    }
}

macro_rules! impl_ord_boilerplate {
    ($ty:ty) => {
        impl From<Hex> for $ty {
            #[inline]
            fn from(value: Hex) -> Self {
                Self(value)
            }
        }

        impl PartialOrd for $ty {
            #[inline]
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Deref for $ty {
            type Target = Hex;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl DerefMut for $ty {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}

impl_ord_boilerplate!(OrdByLength);
impl_ord_boilerplate!(OrdByXY);
impl_ord_boilerplate!(OrdByYX);
