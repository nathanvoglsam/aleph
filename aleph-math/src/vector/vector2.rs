//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use crate::traits::{DotProduct, Length, LengthSquared, Lerp, Normalize, NormalizeAssign, Real};
use crate::vector::TVec3;
use crate::vector::TVec4;
use core::fmt::{Display, Error, Formatter};
use core::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

//==================================================================================================
///
/// A generic 2 component vector
///
#[repr(align(16))]
#[derive(Copy, Clone, Debug)]
pub struct TVec2<T: Real> {
    pub(crate) data: [T; 2],
}

///
/// Const fn for constructing a TVec2 in a const context
///
pub const fn vector_2_f32(x: f32, y: f32) -> TVec2<f32> {
    TVec2::<f32> { data: [x, y] }
}

///
/// Const fn for constructing a TVec2 in a const context
///
pub const fn vector_2_f64(x: f64, y: f64) -> TVec2<f64> {
    TVec2::<f64> { data: [x, y] }
}

impl<T: Real> TVec2<T> {
    ///
    /// Construct a new Vec2
    ///
    #[inline]
    pub fn new(x: T, y: T) -> Self {
        TVec2 { data: [x, y] }
    }

    ///
    /// Return a vector with all zeroes
    ///
    #[inline]
    pub fn zero() -> Self {
        Self::new(T::zero(), T::zero())
    }

    ///
    /// Return a vector with all ones
    ///
    #[inline]
    pub fn one() -> Self {
        Self::new(T::one(), T::one())
    }

    ///
    /// Return a unit vector that points in the up direction
    ///
    #[inline]
    pub fn up() -> Self {
        Self::new(T::zero(), T::one())
    }

    ///
    /// Return a unit vector that points in the down direction
    ///
    #[inline]
    pub fn down() -> Self {
        Self::new(T::zero(), -T::one())
    }

    ///
    /// Return a unit vector that points in the right direction
    ///
    #[inline]
    pub fn right() -> Self {
        Self::new(T::one(), T::zero())
    }

    ///
    /// Return a unit vector that points in the left direction
    ///
    #[inline]
    pub fn left() -> Self {
        Self::new(-T::one(), T::zero())
    }
}

impl<T: Real> Display for TVec2<T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(
            f,
            "[ {:precision$} {:precision$} ]",
            self.data[0],
            self.data[1],
            precision = f.precision().unwrap_or_else(|| f.width().unwrap_or(4)),
        )
    }
}

impl<T: Real> From<T> for TVec2<T> {
    ///
    /// Take a value as the x component of a Vec2 and leave the y component as 0
    ///
    #[inline]
    fn from(other: T) -> Self {
        Self::new(other, T::zero())
    }
}

impl<T: Real> From<TVec3<T>> for TVec2<T> {
    ///
    /// Take the xy components of a Vec3 and produce a Vec2
    ///
    #[inline]
    fn from(other: TVec3<T>) -> Self {
        Self::new(other.data[0], other.data[1])
    }
}

impl<T: Real> From<TVec4<T>> for TVec2<T> {
    ///
    /// Take the xy components of a Vec4 and produce a Vec2
    ///
    #[inline]
    fn from(other: TVec4<T>) -> Self {
        Self::new(other.data[0], other.data[1])
    }
}

impl<T: Real> From<[T; 2]> for TVec2<T> {
    ///
    /// Take the array as a vector
    ///
    #[inline]
    fn from(other: [T; 2]) -> Self {
        Self { data: other }
    }
}

impl<T: Real> Into<[T; 2]> for TVec2<T> {
    #[inline]
    fn into(self) -> [T; 2] {
        self.data
    }
}

impl<T: Real> From<(T, T)> for TVec2<T> {
    ///
    /// Take the array as a vector
    ///
    #[inline]
    fn from(other: (T, T)) -> Self {
        Self {
            data: [other.0, other.1],
        }
    }
}

impl<T: Real> Into<(T, T)> for TVec2<T> {
    #[inline]
    fn into(self) -> (T, T) {
        (self.data[0], self.data[1])
    }
}

impl<T: Real> Index<usize> for TVec2<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T: Real> IndexMut<usize> for TVec2<T> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

//
// VEC2 <-> VEC2 MATH TRAIT IMPLS
//

impl<T: Real> Add<TVec2<T>> for TVec2<T> {
    type Output = TVec2<T>;

    #[inline]
    fn add(mut self, rhs: TVec2<T>) -> Self::Output {
        self.data[0] += rhs.data[0];
        self.data[1] += rhs.data[1];
        self
    }
}

impl<T: Real> AddAssign<TVec2<T>> for TVec2<T> {
    #[inline]
    fn add_assign(&mut self, rhs: TVec2<T>) {
        self.data[0] += rhs.data[0];
        self.data[1] += rhs.data[1];
    }
}

impl<T: Real> Sub<TVec2<T>> for TVec2<T> {
    type Output = TVec2<T>;

    #[inline]
    fn sub(mut self, rhs: TVec2<T>) -> Self::Output {
        self.data[0] -= rhs.data[0];
        self.data[1] -= rhs.data[1];
        self
    }
}

impl<T: Real> SubAssign<TVec2<T>> for TVec2<T> {
    #[inline]
    fn sub_assign(&mut self, rhs: TVec2<T>) {
        self.data[0] -= rhs.data[0];
        self.data[1] -= rhs.data[1];
    }
}

impl<T: Real> Mul<TVec2<T>> for TVec2<T> {
    type Output = TVec2<T>;

    #[inline]
    fn mul(mut self, rhs: TVec2<T>) -> Self::Output {
        self.data[0] *= rhs.data[0];
        self.data[1] *= rhs.data[1];
        self
    }
}

impl<T: Real> MulAssign<TVec2<T>> for TVec2<T> {
    #[inline]
    fn mul_assign(&mut self, rhs: TVec2<T>) {
        self.data[0] *= rhs.data[0];
        self.data[1] *= rhs.data[1];
    }
}

impl<T: Real> Div<TVec2<T>> for TVec2<T> {
    type Output = TVec2<T>;

    #[inline]
    fn div(mut self, rhs: TVec2<T>) -> Self::Output {
        self.data[0] /= rhs.data[0];
        self.data[1] /= rhs.data[1];
        self
    }
}

impl<T: Real> DivAssign<TVec2<T>> for TVec2<T> {
    #[inline]
    fn div_assign(&mut self, rhs: TVec2<T>) {
        self.data[0] /= rhs.data[0];
        self.data[1] /= rhs.data[1];
    }
}

//
// VEC2 <-> FLOAT MATH TRAITS IMPLS
//

impl<T: Real> Add<T> for TVec2<T> {
    type Output = TVec2<T>;

    #[inline]
    fn add(mut self, rhs: T) -> Self::Output {
        self.data[0] += rhs;
        self.data[1] += rhs;
        self
    }
}

impl<T: Real> AddAssign<T> for TVec2<T> {
    #[inline]
    fn add_assign(&mut self, rhs: T) {
        self.data[0] += rhs;
        self.data[1] += rhs;
    }
}

impl<T: Real> Sub<T> for TVec2<T> {
    type Output = TVec2<T>;

    #[inline]
    fn sub(mut self, rhs: T) -> Self::Output {
        self.data[0] -= rhs;
        self.data[1] -= rhs;
        self
    }
}

impl<T: Real> SubAssign<T> for TVec2<T> {
    #[inline]
    fn sub_assign(&mut self, rhs: T) {
        self.data[0] -= rhs;
        self.data[1] -= rhs;
    }
}

impl<T: Real> Mul<T> for TVec2<T> {
    type Output = TVec2<T>;

    #[inline]
    fn mul(mut self, rhs: T) -> Self::Output {
        self.data[0] *= rhs;
        self.data[1] *= rhs;
        self
    }
}

impl<T: Real> MulAssign<T> for TVec2<T> {
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        self.data[0] *= rhs;
        self.data[1] *= rhs;
    }
}

impl<T: Real> Div<T> for TVec2<T> {
    type Output = TVec2<T>;

    #[inline]
    fn div(mut self, rhs: T) -> Self::Output {
        self.data[0] /= rhs;
        self.data[1] /= rhs;
        self
    }
}

impl<T: Real> DivAssign<T> for TVec2<T> {
    #[inline]
    fn div_assign(&mut self, rhs: T) {
        self.data[0] /= rhs;
        self.data[1] /= rhs;
    }
}

impl<T: Real> Neg for TVec2<T> {
    type Output = Self;

    #[inline]
    fn neg(mut self) -> Self::Output {
        self.data[0] = -self.data[0];
        self.data[1] = -self.data[1];
        self
    }
}

impl<T: Real> Lerp<T> for TVec2<T> {
    #[inline]
    fn lerp(&self, b: &Self, factor: T) -> Self {
        *self + ((*self - *b) * factor)
    }
}

impl<T: Real> Length for TVec2<T> {
    type Output = T;

    #[inline]
    fn length(&self) -> Self::Output {
        self.length_squared().sqrt()
    }
}

impl<T: Real> LengthSquared for TVec2<T> {
    type Output = T;

    #[inline]
    fn length_squared(&self) -> Self::Output {
        self.dot(self)
    }
}

impl<T: Real> Normalize for TVec2<T> {
    #[inline]
    fn normalize(mut self) -> Self {
        self.normalize_assign();
        self
    }
}

impl<T: Real> NormalizeAssign for TVec2<T> {
    #[inline]
    fn normalize_assign(&mut self) {
        let len = self.length();
        *self *= T::one() / len;
    }
}

impl<T: Real> DotProduct<T> for TVec2<T> {
    #[inline]
    fn dot(&self, rhs: &Self) -> T {
        (self.data[0] * rhs.data[0]) + (self.data[1] * rhs.data[1])
    }
}

impl<T: Real> PartialEq<TVec2<T>> for TVec2<T> {
    #[inline]
    fn eq(&self, other: &TVec2<T>) -> bool {
        self.data[0] == other.data[0] && self.data[1] == other.data[1]
    }

    #[allow(clippy::partialeq_ne_impl)]
    #[inline]
    fn ne(&self, other: &TVec2<T>) -> bool {
        self.data[0] != other.data[0] || self.data[1] != other.data[1]
    }
}
