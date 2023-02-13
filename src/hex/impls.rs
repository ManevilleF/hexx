use super::Hex;
use std::{
    iter::{Product, Sum},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign},
};

impl Add<Self> for Hex {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        self.const_add(rhs)
    }
}

impl Add<i32> for Hex {
    type Output = Self;

    #[inline]
    fn add(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl AddAssign for Hex {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl AddAssign<i32> for Hex {
    #[inline]
    fn add_assign(&mut self, rhs: i32) {
        *self = *self + rhs;
    }
}

impl Sum for Hex {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |a, b| a + b)
    }
}

impl<'a> Sum<&'a Self> for Hex {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.copied().sum()
    }
}

impl Sub<Self> for Hex {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        self.const_sub(rhs)
    }
}

impl Sub<i32> for Hex {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x - rhs,
            y: self.y - rhs,
        }
    }
}

impl SubAssign for Hex {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl SubAssign<i32> for Hex {
    #[inline]
    fn sub_assign(&mut self, rhs: i32) {
        *self = *self - rhs;
    }
}

impl Mul<Self> for Hex {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl Mul<i32> for Hex {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Mul<f32> for Hex {
    type Output = Self;

    #[inline]
    #[allow(clippy::cast_precision_loss)]
    fn mul(self, rhs: f32) -> Self::Output {
        Self::round((self.x as f32 * rhs, self.y as f32 * rhs))
    }
}

impl MulAssign for Hex {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl MulAssign<i32> for Hex {
    #[inline]
    fn mul_assign(&mut self, rhs: i32) {
        *self = *self * rhs;
    }
}

impl MulAssign<f32> for Hex {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl Product for Hex {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |a, b| a * b)
    }
}

impl<'a> Product<&'a Self> for Hex {
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.copied().product()
    }
}

impl Div<Self> for Hex {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl Div<i32> for Hex {
    type Output = Self;

    #[inline]
    #[allow(clippy::cast_precision_loss)]
    fn div(self, rhs: i32) -> Self::Output {
        let length = self.length();
        let new_length = length / rhs;
        let lerp = new_length as f32 / length as f32;
        Self::ZERO.lerp(self, lerp)
    }
}

impl Div<f32> for Hex {
    type Output = Self;

    #[inline]
    #[allow(clippy::cast_precision_loss)]
    #[allow(clippy::cast_possible_truncation)]
    fn div(self, rhs: f32) -> Self::Output {
        let length = self.length();
        let new_length = (length as f32 / rhs) as i32;
        let lerp = new_length as f32 / length as f32;
        Self::ZERO.lerp(self, lerp)
    }
}

impl DivAssign for Hex {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl DivAssign<i32> for Hex {
    #[inline]
    fn div_assign(&mut self, rhs: i32) {
        *self = *self / rhs;
    }
}

impl DivAssign<f32> for Hex {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}

impl Rem<Self> for Hex {
    type Output = Self;

    #[inline]
    fn rem(self, rhs: Self) -> Self::Output {
        self - (self / rhs) * rhs
    }
}

impl Rem<i32> for Hex {
    type Output = Self;

    #[inline]
    fn rem(self, rhs: i32) -> Self::Output {
        self - (self / rhs) * rhs
    }
}

impl RemAssign for Hex {
    #[inline]
    fn rem_assign(&mut self, rhs: Self) {
        *self = *self % rhs;
    }
}

impl RemAssign<i32> for Hex {
    #[inline]
    fn rem_assign(&mut self, rhs: i32) {
        *self = *self % rhs;
    }
}

impl Neg for Hex {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        self.const_neg()
    }
}
