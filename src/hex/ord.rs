use crate::Hex;
use std::cmp::Ordering;

/// [`Ordering`] wrapper around [`Hex`], comparing [`Hex::length`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OrdByLength(pub Hex);

/// [`Ordering`] wrapper around [`Hex`], comparing [`Hex::x`] then [`Hex::y`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OrdByXY(pub Hex);

/// [`Ordering`] wrapper around [`Hex`], comparing [`Hex::y`] then [`Hex::x`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OrdByYX(pub Hex);

impl From<Hex> for OrdByLength {
    fn from(value: Hex) -> Self {
        Self(value)
    }
}

impl From<Hex> for OrdByXY {
    fn from(value: Hex) -> Self {
        Self(value)
    }
}

impl From<Hex> for OrdByYX {
    fn from(value: Hex) -> Self {
        Self(value)
    }
}

impl PartialOrd for OrdByLength {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialOrd for OrdByXY {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialOrd for OrdByYX {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for OrdByLength {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.length().cmp(&other.0.length())
    }
}

impl Ord for OrdByXY {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.x.cmp(&other.0.x).then(self.0.y.cmp(&other.0.y))
    }
}

impl Ord for OrdByYX {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.y.cmp(&other.0.y).then(self.0.x.cmp(&other.0.x))
    }
}
