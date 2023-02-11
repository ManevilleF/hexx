use std::ops::{Add, Neg, Sub};

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

impl Add<usize> for Direction {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        self.rotate_right(rhs)
    }
}

impl Add<usize> for DiagonalDirection {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        self.rotate_right(rhs)
    }
}

impl Sub<usize> for Direction {
    type Output = Self;

    fn sub(self, rhs: usize) -> Self::Output {
        self.rotate_left(rhs)
    }
}

impl Sub<usize> for DiagonalDirection {
    type Output = Self;

    fn sub(self, rhs: usize) -> Self::Output {
        self.rotate_left(rhs)
    }
}
