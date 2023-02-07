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
    fn mul(self, rhs: f32) -> Self::Output {
        self.rounded_mul(rhs)
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
    fn div(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl Div<f32> for Hex {
    type Output = Self;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        self.rounded_div(rhs)
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
        Self {
            x: self.x % rhs.x,
            y: self.y % rhs.y,
        }
    }
}

impl Rem<i32> for Hex {
    type Output = Self;

    #[inline]
    fn rem(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x % rhs,
            y: self.y % rhs,
        }
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

/// Extension trait for iterators of [`Hex`] to compute a mean (average) value
pub trait MeanExt: Iterator {
    /// Method which takes an iterator and generates `Self` from the elements by finding the mean
    /// (average) value.
    ///
    /// This method will return [`Hex::ZERO`] on an empty iterator
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    /// let mean = Hex::ZERO.range(10).average();
    /// ```
    #[doc(alias = "mean")]
    fn average(self) -> Hex;
}

/// Extension trait for iterators to [`Hex`] to compute a center value
pub trait CenterExt: Iterator {
    /// Method which takes an iterator and generates `Self` from the elements by finding the center
    /// value.
    ///
    /// This method will return [`Hex::ZERO`] on an empty iterator
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    /// let center = Hex::ZERO.range(10).center();
    /// ```
    fn center(self) -> Hex;
}

impl<I: Iterator<Item = Hex>> MeanExt for I {
    fn average(self) -> Hex {
        let mut sum = Hex::ZERO;
        let mut count = 0;

        for hex in self {
            count += 1;
            sum += hex;
        }
        // Avoid division by zero
        sum / count.max(1)
    }
}

impl<I: Iterator<Item = Hex>> CenterExt for I {
    fn center(self) -> Hex {
        let mut x_min = 0;
        let mut x_max = 0;
        let mut y_min = 0;
        let mut y_max = 0;

        for hex in self {
            x_min = x_min.min(hex.x);
            x_max = x_max.max(hex.x);
            y_min = y_min.min(hex.y);
            y_max = y_max.max(hex.y);
        }

        let x = (x_min + x_max) / 2;
        let y = (y_min + y_max) / 2;

        Hex::new(x, y)
    }
}
