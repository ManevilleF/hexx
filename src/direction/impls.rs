use std::ops::{Neg, Shl, Shr};

use crate::{DiagonalDirection, Direction};

impl Neg for DiagonalDirection {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.const_neg()
    }
}

impl Neg for Direction {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.const_neg()
    }
}

impl Shr<usize> for Direction {
    type Output = Self;

    fn shr(self, rhs: usize) -> Self::Output {
        self.rotate_right(rhs)
    }
}

impl Shr<usize> for DiagonalDirection {
    type Output = Self;

    fn shr(self, rhs: usize) -> Self::Output {
        self.rotate_right(rhs)
    }
}

impl Shl<usize> for Direction {
    type Output = Self;

    fn shl(self, rhs: usize) -> Self::Output {
        self.rotate_left(rhs)
    }
}

impl Shl<usize> for DiagonalDirection {
    type Output = Self;

    fn shl(self, rhs: usize) -> Self::Output {
        self.rotate_left(rhs)
    }
}
