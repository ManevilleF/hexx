use crate::{EdgeDirection, Hex};
use std::ops::Neg;

use super::GridVertex;

/// Hexagonal grid orientated edge representation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(not(target_arch = "spirv"), derive(Hash))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub struct GridEdge {
    /// The coordinate of the edge
    pub origin: Hex,
    /// The direction the edge points towards
    pub direction: EdgeDirection,
}

impl GridEdge {
    /// Edges are equivalent if they have identical or flipped origin or direction
    #[must_use]
    pub fn equivalent(&self, other: &Self) -> bool {
        (self.origin == other.origin && self.direction == other.direction)
            || (self.origin == other.destination() && self.direction == other.direction.const_neg())
    }

    #[inline]
    #[must_use]
    /// Returns the coordinate the edge id pointing to
    pub const fn destination(&self) -> Hex {
        self.origin.add_dir(self.direction)
    }

    #[inline]
    #[must_use]
    /// Returns the two vertices making this edge in clockwise order
    pub const fn vertices(&self) -> [GridVertex; 2] {
        [
            GridVertex {
                origin: self.origin,
                direction: self.direction.diagonal_ccw(),
            },
            GridVertex {
                origin: self.origin,
                direction: self.direction.diagonal_cw(),
            },
        ]
    }

    #[inline]
    #[must_use]
    /// Flips the edge, changing its `origin` to be the `destination` and
    /// inverting its direction
    pub const fn flipped(self) -> Self {
        Self {
            origin: self.destination(),
            direction: self.direction.const_neg(),
        }
    }

    #[inline]
    #[must_use]
    /// Inverts the edge, now facing the opposite direction
    pub const fn const_neg(self) -> Self {
        Self {
            direction: self.direction.const_neg(),
            ..self
        }
    }

    #[inline]
    #[must_use]
    /// Returns the next edge in clockwise order
    pub const fn clockwise(self) -> Self {
        Self {
            direction: self.direction.clockwise(),
            ..self
        }
    }

    #[inline]
    #[must_use]
    /// Returns the next edge in counter clockwise order
    pub const fn counter_clockwise(self) -> Self {
        Self {
            direction: self.direction.counter_clockwise(),
            ..self
        }
    }

    #[inline]
    #[must_use]
    /// Rotates `self` clockwise by `offset` amount.
    pub const fn rotate_cw(self, offset: u8) -> Self {
        Self {
            direction: self.direction.rotate_cw(offset),
            ..self
        }
    }
    #[inline]
    #[must_use]
    /// Rotates `self` counter clockwise by `offset` amount.
    pub const fn rotate_ccw(self, offset: u8) -> Self {
        Self {
            direction: self.direction.rotate_ccw(offset),
            ..self
        }
    }
}

impl Hex {
    #[must_use]
    #[inline]
    /// Return all 6 edges of the coordinate
    pub fn all_edges(self) -> [GridEdge; 6] {
        EdgeDirection::ALL_DIRECTIONS.map(|direction| GridEdge {
            origin: self,
            direction,
        })
    }
}

impl Neg for GridEdge {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        self.const_neg()
    }
}

impl From<EdgeDirection> for GridEdge {
    fn from(direction: EdgeDirection) -> Self {
        Self {
            origin: Hex::ZERO,
            direction,
        }
    }
}
