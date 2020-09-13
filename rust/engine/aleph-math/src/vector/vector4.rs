//
//
// This file is a part of Aleph
//
// https://github.com/nathanvoglsam/aleph
//
// MIT License
//
// Copyright (c) 2020 Aleph Engine
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

use crate::traits::{
    DotProduct, IntoDegrees, IntoRadians, Length, LengthSquared, Lerp, Normalize, NormalizeAssign,
    Real,
};
use crate::vector::TVec2;
use crate::vector::TVec3;
use core::fmt::{Display, Error, Formatter};
use core::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

//==================================================================================================
///
/// A generic 4 component vector
///
#[repr(align(16))]
#[derive(Copy, Clone, Debug)]
pub struct TVec4<T: Real> {
    pub(crate) data: [T; 4],
}

///
/// Const fn for constructing a TVec2 in a const context
///
pub const fn vector_4_f32(x: f32, y: f32, z: f32, w: f32) -> TVec4<f32> {
    TVec4::<f32> { data: [x, y, z, w] }
}

///
/// Const fn for constructing a TVec2 in a const context
///
pub const fn vector_4_f64(x: f64, y: f64, z: f64, w: f64) -> TVec4<f64> {
    TVec4::<f64> { data: [x, y, z, w] }
}

impl<T: Real> TVec4<T> {
    ///
    /// Constructs a new Vec4
    ///
    #[inline]
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        TVec4 { data: [x, y, z, w] }
    }

    ///
    ///
    ///
    #[inline]
    pub fn zero() -> Self {
        Self::new(T::zero(), T::zero(), T::zero(), T::zero())
    }
}

impl<T: Real> Display for TVec4<T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(
            f,
            "[ {:precision$} {:precision$} {:precision$} {:precision$} ]",
            self.data[0],
            self.data[1],
            self.data[2],
            self.data[3],
            precision = f.precision().unwrap_or_else(|| f.width().unwrap_or(4)),
        )
    }
}

impl<T: Real> From<T> for TVec4<T> {
    #[inline]
    fn from(other: T) -> Self {
        Self::new(other, T::zero(), T::zero(), T::zero())
    }
}

impl<T: Real> From<TVec2<T>> for TVec4<T> {
    #[inline]
    fn from(other: TVec2<T>) -> Self {
        Self::new(other.data[0], other.data[1], T::zero(), T::zero())
    }
}

impl<T: Real> From<TVec3<T>> for TVec4<T> {
    #[inline]
    fn from(other: TVec3<T>) -> Self {
        Self::new(other.data[0], other.data[1], other.data[2], T::zero())
    }
}

impl<T: Real> From<[T; 4]> for TVec4<T> {
    ///
    /// Take the array as a vector
    ///
    #[inline]
    fn from(other: [T; 4]) -> Self {
        Self { data: other }
    }
}

impl<T: Real> Into<[T; 4]> for TVec4<T> {
    #[inline]
    fn into(self) -> [T; 4] {
        self.data
    }
}

impl<T: Real> Index<usize> for TVec4<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T: Real> IndexMut<usize> for TVec4<T> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

//
// VEC4 <-> VEC4 MATH TRAIT IMPLS
//

impl<T: Real> Add<TVec4<T>> for TVec4<T> {
    type Output = TVec4<T>;

    #[inline]
    fn add(mut self, rhs: TVec4<T>) -> Self::Output {
        self.data[0] += rhs.data[0];
        self.data[1] += rhs.data[1];
        self.data[2] += rhs.data[2];
        self.data[3] += rhs.data[3];
        self
    }
}

impl<T: Real> AddAssign<TVec4<T>> for TVec4<T> {
    #[inline]
    fn add_assign(&mut self, rhs: TVec4<T>) {
        self.data[0] += rhs.data[0];
        self.data[1] += rhs.data[1];
        self.data[2] += rhs.data[2];
        self.data[3] += rhs.data[3];
    }
}

impl<T: Real> Sub<TVec4<T>> for TVec4<T> {
    type Output = TVec4<T>;

    #[inline]
    fn sub(mut self, rhs: TVec4<T>) -> Self::Output {
        self.data[0] -= rhs.data[0];
        self.data[1] -= rhs.data[1];
        self.data[2] -= rhs.data[2];
        self.data[3] -= rhs.data[3];
        self
    }
}

impl<T: Real> SubAssign<TVec4<T>> for TVec4<T> {
    #[inline]
    fn sub_assign(&mut self, rhs: TVec4<T>) {
        self.data[0] -= rhs.data[0];
        self.data[1] -= rhs.data[1];
        self.data[2] -= rhs.data[2];
        self.data[3] -= rhs.data[3];
    }
}

impl<T: Real> Mul<TVec4<T>> for TVec4<T> {
    type Output = TVec4<T>;

    #[inline]
    fn mul(mut self, rhs: TVec4<T>) -> Self::Output {
        self.data[0] *= rhs.data[0];
        self.data[1] *= rhs.data[1];
        self.data[2] *= rhs.data[2];
        self.data[3] *= rhs.data[3];
        self
    }
}

impl<T: Real> MulAssign<TVec4<T>> for TVec4<T> {
    #[inline]
    fn mul_assign(&mut self, rhs: TVec4<T>) {
        self.data[0] *= rhs.data[0];
        self.data[1] *= rhs.data[1];
        self.data[2] *= rhs.data[2];
        self.data[3] *= rhs.data[3];
    }
}

impl<T: Real> Div<TVec4<T>> for TVec4<T> {
    type Output = TVec4<T>;

    #[inline]
    fn div(mut self, rhs: TVec4<T>) -> Self::Output {
        self.data[0] /= rhs.data[0];
        self.data[1] /= rhs.data[1];
        self.data[2] /= rhs.data[2];
        self.data[3] /= rhs.data[3];
        self
    }
}

impl<T: Real> DivAssign<TVec4<T>> for TVec4<T> {
    #[inline]
    fn div_assign(&mut self, rhs: TVec4<T>) {
        self.data[0] /= rhs.data[0];
        self.data[1] /= rhs.data[1];
        self.data[2] /= rhs.data[2];
        self.data[3] /= rhs.data[3];
    }
}

//
// VEC4 <-> FLOAT MATH TRAITS IMPLS
//

impl<T: Real> Add<T> for TVec4<T> {
    type Output = TVec4<T>;

    #[inline]
    fn add(mut self, rhs: T) -> Self::Output {
        self.data[0] += rhs;
        self.data[1] += rhs;
        self.data[2] += rhs;
        self.data[3] += rhs;
        self
    }
}

impl<T: Real> AddAssign<T> for TVec4<T> {
    #[inline]
    fn add_assign(&mut self, rhs: T) {
        self.data[0] += rhs;
        self.data[1] += rhs;
        self.data[2] += rhs;
        self.data[3] += rhs;
    }
}

impl<T: Real> Sub<T> for TVec4<T> {
    type Output = TVec4<T>;

    #[inline]
    fn sub(mut self, rhs: T) -> Self::Output {
        self.data[0] -= rhs;
        self.data[1] -= rhs;
        self.data[2] -= rhs;
        self.data[3] -= rhs;
        self
    }
}

impl<T: Real> SubAssign<T> for TVec4<T> {
    #[inline]
    fn sub_assign(&mut self, rhs: T) {
        self.data[0] -= rhs;
        self.data[1] -= rhs;
        self.data[2] -= rhs;
        self.data[3] -= rhs;
    }
}

impl<T: Real> Mul<T> for TVec4<T> {
    type Output = TVec4<T>;

    #[inline]
    fn mul(mut self, rhs: T) -> Self::Output {
        self.data[0] *= rhs;
        self.data[1] *= rhs;
        self.data[2] *= rhs;
        self.data[3] *= rhs;
        self
    }
}

impl<T: Real> MulAssign<T> for TVec4<T> {
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        self.data[0] *= rhs;
        self.data[1] *= rhs;
        self.data[2] *= rhs;
        self.data[3] *= rhs;
    }
}

impl<T: Real> Div<T> for TVec4<T> {
    type Output = TVec4<T>;

    #[inline]
    fn div(mut self, rhs: T) -> Self::Output {
        self.data[0] /= rhs;
        self.data[1] /= rhs;
        self.data[2] /= rhs;
        self.data[3] /= rhs;
        self
    }
}

impl<T: Real> DivAssign<T> for TVec4<T> {
    #[inline]
    fn div_assign(&mut self, rhs: T) {
        self.data[0] /= rhs;
        self.data[1] /= rhs;
        self.data[2] /= rhs;
        self.data[3] /= rhs;
    }
}

impl<T: Real> Neg for TVec4<T> {
    type Output = Self;

    #[inline]
    fn neg(mut self) -> Self::Output {
        self.data[0] = -self.data[0];
        self.data[1] = -self.data[1];
        self.data[2] = -self.data[2];
        self.data[3] = -self.data[3];
        self
    }
}

impl<T: Real> Lerp<T> for TVec4<T> {
    #[inline]
    fn lerp(&self, b: &Self, factor: T) -> Self {
        *self + ((*self - *b) * factor)
    }
}

impl<T: Real> Length for TVec4<T> {
    type Output = T;

    fn length(&self) -> Self::Output {
        self.length_squared().sqrt()
    }
}

impl<T: Real> LengthSquared for TVec4<T> {
    type Output = T;

    fn length_squared(&self) -> Self::Output {
        self.dot(self)
    }
}

impl<T: Real> Normalize for TVec4<T> {
    fn normalize(mut self) -> Self {
        self.normalize_assign();
        self
    }
}

impl<T: Real> NormalizeAssign for TVec4<T> {
    fn normalize_assign(&mut self) {
        let len = self.length();
        *self *= T::one() / len;
    }
}

impl<T: Real> IntoDegrees for TVec4<T> {
    #[inline]
    fn into_degrees(mut self) -> Self {
        self.data[0] = self.data[0].into_degrees();
        self.data[1] = self.data[1].into_degrees();
        self.data[2] = self.data[2].into_degrees();
        self.data[3] = self.data[3].into_degrees();
        self
    }
}

impl<T: Real> IntoRadians for TVec4<T> {
    #[inline]
    fn into_radians(mut self) -> Self {
        self.data[0] = self.data[0].into_radians();
        self.data[1] = self.data[1].into_radians();
        self.data[2] = self.data[2].into_radians();
        self.data[3] = self.data[3].into_radians();
        self
    }
}

impl<T: Real> DotProduct<T> for TVec4<T> {
    #[inline]
    fn dot(&self, rhs: &Self) -> T {
        (self.data[0] * rhs.data[0])
            + (self.data[1] * rhs.data[1])
            + (self.data[2] * rhs.data[2])
            + (self.data[3] * rhs.data[3])
    }
}

impl<T: Real> PartialEq<TVec4<T>> for TVec4<T> {
    #[inline]
    fn eq(&self, other: &TVec4<T>) -> bool {
        self.data[0] == other.data[0]
            && self.data[1] == other.data[1]
            && self.data[2] == other.data[2]
            && self.data[3] == other.data[3]
    }

    #[allow(clippy::partialeq_ne_impl)]
    #[inline]
    fn ne(&self, other: &TVec4<T>) -> bool {
        self.data[0] != other.data[0]
            || self.data[1] != other.data[1]
            || self.data[2] != other.data[2]
            || self.data[3] != other.data[3]
    }
}
