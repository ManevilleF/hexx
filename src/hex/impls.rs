use crate::{EdgeDirection, Hex, VertexDirection};
use std::{
    iter::{Product, Sum},
    ops::{
        Add, AddAssign, BitAnd, BitOr, BitXor, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign,
        Shl, Shr, Sub, SubAssign,
    },
};

impl PartialEq<Hex> for &Hex {
    fn eq(&self, other: &Hex) -> bool {
        (*self).eq(other)
    }
}

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
            x: self.x.add(rhs),
            y: self.y.add(rhs),
        }
    }
}

impl Add<EdgeDirection> for Hex {
    type Output = Self;

    #[inline]
    fn add(self, rhs: EdgeDirection) -> Self::Output {
        self.add_dir(rhs)
    }
}

impl Add<VertexDirection> for Hex {
    type Output = Self;

    #[inline]
    fn add(self, rhs: VertexDirection) -> Self::Output {
        self.add_diag_dir(rhs)
    }
}

impl AddAssign for Hex {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = self.add(rhs);
    }
}

impl AddAssign<i32> for Hex {
    #[inline]
    fn add_assign(&mut self, rhs: i32) {
        *self = self.add(rhs);
    }
}

impl AddAssign<EdgeDirection> for Hex {
    #[inline]
    fn add_assign(&mut self, rhs: EdgeDirection) {
        *self = self.add(rhs);
    }
}

impl AddAssign<VertexDirection> for Hex {
    #[inline]
    fn add_assign(&mut self, rhs: VertexDirection) {
        *self = self.add(rhs);
    }
}

impl Sum for Hex {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, Self::const_add)
    }
}

impl<'a> Sum<&'a Self> for Hex {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |a, &b| Self::const_add(a, b))
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
            x: self.x.sub(rhs),
            y: self.y.sub(rhs),
        }
    }
}

impl Sub<EdgeDirection> for Hex {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: EdgeDirection) -> Self::Output {
        self.sub(Self::from(rhs))
    }
}

impl Sub<VertexDirection> for Hex {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: VertexDirection) -> Self::Output {
        self.sub(Self::from(rhs))
    }
}

impl SubAssign for Hex {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.sub(rhs);
    }
}

impl SubAssign<i32> for Hex {
    #[inline]
    fn sub_assign(&mut self, rhs: i32) {
        *self = self.sub(rhs);
    }
}

impl SubAssign<EdgeDirection> for Hex {
    #[inline]
    fn sub_assign(&mut self, rhs: EdgeDirection) {
        *self = self.sub(rhs);
    }
}

impl SubAssign<VertexDirection> for Hex {
    #[inline]
    fn sub_assign(&mut self, rhs: VertexDirection) {
        *self = self.sub(rhs);
    }
}

impl Mul<Self> for Hex {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.mul(rhs.x),
            y: self.y.mul(rhs.y),
        }
    }
}

impl Mul<i32> for Hex {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x.mul(rhs),
            y: self.y.mul(rhs),
        }
    }
}

impl Mul<f32> for Hex {
    type Output = Self;

    #[inline]
    #[expect(clippy::cast_precision_loss)]
    fn mul(self, rhs: f32) -> Self::Output {
        Self::round([self.x as f32 * rhs, self.y as f32 * rhs])
    }
}

impl MulAssign for Hex {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.mul(rhs);
    }
}

impl MulAssign<i32> for Hex {
    #[inline]
    fn mul_assign(&mut self, rhs: i32) {
        *self = self.mul(rhs);
    }
}

impl MulAssign<f32> for Hex {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        *self = self.mul(rhs);
    }
}

impl Product for Hex {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, Self::mul)
    }
}

impl<'a> Product<&'a Self> for Hex {
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, |a, &b| Self::mul(a, b))
    }
}

impl Div<Self> for Hex {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.div(rhs.x),
            y: self.y.div(rhs.y),
        }
    }
}

impl Div<i32> for Hex {
    type Output = Self;

    #[inline]
    #[expect(clippy::cast_precision_loss)]
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
    #[expect(clippy::cast_precision_loss)]
    #[expect(clippy::cast_possible_truncation)]
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
        *self = self.div(rhs);
    }
}

impl DivAssign<i32> for Hex {
    #[inline]
    fn div_assign(&mut self, rhs: i32) {
        *self = self.div(rhs);
    }
}

impl DivAssign<f32> for Hex {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        *self = self.div(rhs);
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
        *self = self.rem(rhs);
    }
}

impl RemAssign<i32> for Hex {
    #[inline]
    fn rem_assign(&mut self, rhs: i32) {
        *self = self.rem(rhs);
    }
}

impl Neg for Hex {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        self.const_neg()
    }
}

impl BitAnd for Hex {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.bitand(rhs.x),
            y: self.y.bitand(rhs.y),
        }
    }
}

impl BitOr for Hex {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.bitor(rhs.x),
            y: self.y.bitor(rhs.y),
        }
    }
}

impl BitXor for Hex {
    type Output = Self;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.bitxor(rhs.x),
            y: self.y.bitxor(rhs.y),
        }
    }
}

impl BitAnd<i32> for Hex {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x.bitand(rhs),
            y: self.y.bitand(rhs),
        }
    }
}

impl BitOr<i32> for Hex {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x.bitor(rhs),
            y: self.y.bitor(rhs),
        }
    }
}

impl BitXor<i32> for Hex {
    type Output = Self;
    #[inline]
    fn bitxor(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x.bitxor(rhs),
            y: self.y.bitxor(rhs),
        }
    }
}

impl Shl<i8> for Hex {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: i8) -> Self::Output {
        Self {
            x: self.x.shl(rhs),
            y: self.y.shl(rhs),
        }
    }
}

impl Shr<i8> for Hex {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: i8) -> Self::Output {
        Self {
            x: self.x.shr(rhs),
            y: self.y.shr(rhs),
        }
    }
}

impl Shl<i16> for Hex {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: i16) -> Self::Output {
        Self {
            x: self.x.shl(rhs),
            y: self.y.shl(rhs),
        }
    }
}

impl Shr<i16> for Hex {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: i16) -> Self::Output {
        Self {
            x: self.x.shr(rhs),
            y: self.y.shr(rhs),
        }
    }
}

impl Shl<i32> for Hex {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x.shl(rhs),
            y: self.y.shl(rhs),
        }
    }
}

impl Shr<i32> for Hex {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x.shr(rhs),
            y: self.y.shr(rhs),
        }
    }
}

impl Shl<u8> for Hex {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: u8) -> Self::Output {
        Self {
            x: self.x.shl(rhs),
            y: self.y.shl(rhs),
        }
    }
}

impl Shr<u8> for Hex {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: u8) -> Self::Output {
        Self {
            x: self.x.shr(rhs),
            y: self.y.shr(rhs),
        }
    }
}

impl Shl<u16> for Hex {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: u16) -> Self::Output {
        Self {
            x: self.x.shl(rhs),
            y: self.y.shl(rhs),
        }
    }
}

impl Shr<u16> for Hex {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: u16) -> Self::Output {
        Self {
            x: self.x.shr(rhs),
            y: self.y.shr(rhs),
        }
    }
}

impl Shl<u32> for Hex {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: u32) -> Self::Output {
        Self {
            x: self.x.shl(rhs),
            y: self.y.shl(rhs),
        }
    }
}

impl Shr<u32> for Hex {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: u32) -> Self::Output {
        Self {
            x: self.x.shr(rhs),
            y: self.y.shr(rhs),
        }
    }
}

impl Shl for Hex {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.shl(rhs.x),
            y: self.y.shl(rhs.y),
        }
    }
}
