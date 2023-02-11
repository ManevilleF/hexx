use std::ops::Neg;

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
