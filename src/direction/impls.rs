use std::ops::{Mul, Neg, Shl, Shr};

use crate::{DiagonalDirection, Direction, Hex};

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
        self.rotate_cw(rhs)
    }
}

impl Shr<usize> for DiagonalDirection {
    type Output = Self;

    fn shr(self, rhs: usize) -> Self::Output {
        self.rotate_cw(rhs)
    }
}

impl Shl<usize> for Direction {
    type Output = Self;

    fn shl(self, rhs: usize) -> Self::Output {
        self.rotate_ccw(rhs)
    }
}

impl Shl<usize> for DiagonalDirection {
    type Output = Self;

    fn shl(self, rhs: usize) -> Self::Output {
        self.rotate_ccw(rhs)
    }
}

impl Mul<i32> for Direction {
    type Output = Hex;

    fn mul(self, rhs: i32) -> Self::Output {
        Hex::from(self).mul(rhs)
    }
}

impl Mul<i32> for DiagonalDirection {
    type Output = Hex;

    fn mul(self, rhs: i32) -> Self::Output {
        Hex::from(self).mul(rhs)
    }
}
