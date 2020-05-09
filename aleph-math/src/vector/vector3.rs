//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use crate::traits::{
    CrossProduct, DotProduct, IntoDegrees, IntoRadians, Length, LengthSquared, Lerp, Normalize,
    NormalizeAssign, Real,
};
use crate::vector::TVec2;
use crate::vector::TVec4;
use core::fmt::{Display, Error, Formatter};
use core::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

//==================================================================================================
///
/// A generic 3 component vector
///
#[repr(align(16))]
#[derive(Copy, Clone, Debug)]
pub struct TVec3<T: Real> {
    pub(crate) data: [T; 4],
}

///
/// Const fn for constructing a TVec3 in a const context
///
pub const fn vector_3_f32(x: f32, y: f32, z: f32) -> TVec3<f32> {
    TVec3::<f32> {
        data: [x, y, z, 0.0],
    }
}

///
/// Const fn for constructing a TVec3 in a const context
///
pub const fn vector_3_f64(x: f64, y: f64, z: f64) -> TVec3<f64> {
    TVec3::<f64> {
        data: [x, y, z, 0.0],
    }
}

impl<T: Real> TVec3<T> {
    ///
    /// Constructs a new Vec3
    ///
    #[inline]
    pub fn new(x: T, y: T, z: T) -> Self {
        TVec3 {
            data: [x, y, z, T::zero()],
        }
    }

    ///
    /// Return a vector with all zeroes
    ///
    #[inline]
    pub fn zero() -> Self {
        Self::new(T::zero(), T::zero(), T::zero())
    }

    ///
    /// Return a vector with all ones
    ///
    #[inline]
    pub fn one() -> Self {
        Self::new(T::one(), T::one(), T::one())
    }

    ///
    /// Return a unit vector that points in the up direction
    ///
    #[inline]
    pub fn up() -> Self {
        Self::new(T::zero(), T::one(), T::zero())
    }

    ///
    /// Return a unit vector that points in the down direction
    ///
    #[inline]
    pub fn down() -> Self {
        Self::new(T::zero(), -T::one(), T::zero())
    }

    ///
    /// Return a unit vector that points in the right direction
    ///
    #[inline]
    pub fn right() -> Self {
        Self::new(T::one(), T::zero(), T::zero())
    }

    ///
    /// Return a unit vector that points in the left direction
    ///
    #[inline]
    pub fn left() -> Self {
        Self::new(-T::one(), T::zero(), T::zero())
    }

    ///
    /// Return a unit vector that points in the forward direction
    ///
    #[inline]
    pub fn forward() -> Self {
        Self::new(T::zero(), T::zero(), T::one())
    }

    ///
    /// Return a unit vector that points in the back direction
    ///
    #[inline]
    pub fn back() -> Self {
        Self::new(T::zero(), T::zero(), -T::one())
    }
}

impl<T: Real> Display for TVec3<T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(
            f,
            "[ {:precision$} {:precision$} {:precision$} ]",
            self.data[0],
            self.data[1],
            self.data[2],
            precision = f.precision().unwrap_or(f.width().unwrap_or(4)),
        )
    }
}

impl<T: Real> From<T> for TVec3<T> {
    #[inline]
    fn from(other: T) -> Self {
        Self::new(other, T::zero(), T::zero())
    }
}

impl<T: Real> From<TVec2<T>> for TVec3<T> {
    #[inline]
    fn from(other: TVec2<T>) -> Self {
        Self::new(other.data[0], other.data[1], T::zero())
    }
}

impl<T: Real> From<TVec4<T>> for TVec3<T> {
    #[inline]
    fn from(other: TVec4<T>) -> Self {
        Self::new(other.data[0], other.data[1], other.data[2])
    }
}

impl<T: Real> From<[T; 3]> for TVec3<T> {
    ///
    /// Take the array as a vector
    ///
    #[inline]
    fn from(other: [T; 3]) -> Self {
        Self {
            data: [other[0], other[1], other[2], T::zero()],
        }
    }
}

impl<T: Real> Into<[T; 3]> for TVec3<T> {
    #[inline]
    fn into(self) -> [T; 3] {
        [self.data[0], self.data[1], self.data[2]]
    }
}

impl<T: Real> From<(T, T, T)> for TVec3<T> {
    ///
    /// Take the array as a vector
    ///
    #[inline]
    fn from(other: (T, T, T)) -> Self {
        Self {
            data: [other.0, other.1, other.2, T::zero()],
        }
    }
}

impl<T: Real> Into<(T, T, T)> for TVec3<T> {
    #[inline]
    fn into(self) -> (T, T, T) {
        (self.data[0], self.data[1], self.data[2])
    }
}

impl<T: Real> Index<usize> for TVec3<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T: Real> IndexMut<usize> for TVec3<T> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

//
// VEC3 <-> VEC3 MATH TRAIT IMPLS
//

impl<T: Real> Add<TVec3<T>> for TVec3<T> {
    type Output = TVec3<T>;

    #[inline]
    fn add(mut self, rhs: TVec3<T>) -> Self::Output {
        self.data[0] += rhs.data[0];
        self.data[1] += rhs.data[1];
        self.data[2] += rhs.data[2];
        self
    }
}

impl<T: Real> AddAssign<TVec3<T>> for TVec3<T> {
    #[inline]
    fn add_assign(&mut self, rhs: TVec3<T>) {
        self.data[0] += rhs.data[0];
        self.data[1] += rhs.data[1];
        self.data[2] += rhs.data[2];
    }
}

impl<T: Real> Sub<TVec3<T>> for TVec3<T> {
    type Output = TVec3<T>;

    #[inline]
    fn sub(mut self, rhs: TVec3<T>) -> Self::Output {
        self.data[0] -= rhs.data[0];
        self.data[1] -= rhs.data[1];
        self.data[2] -= rhs.data[2];
        self
    }
}

impl<T: Real> SubAssign<TVec3<T>> for TVec3<T> {
    #[inline]
    fn sub_assign(&mut self, rhs: TVec3<T>) {
        self.data[0] -= rhs.data[0];
        self.data[1] -= rhs.data[1];
        self.data[2] -= rhs.data[2];
    }
}

impl<T: Real> Mul<TVec3<T>> for TVec3<T> {
    type Output = TVec3<T>;

    #[inline]
    fn mul(mut self, rhs: TVec3<T>) -> Self::Output {
        self.data[0] *= rhs.data[0];
        self.data[1] *= rhs.data[1];
        self.data[2] *= rhs.data[2];
        self
    }
}

impl<T: Real> MulAssign<TVec3<T>> for TVec3<T> {
    #[inline]
    fn mul_assign(&mut self, rhs: TVec3<T>) {
        self.data[0] *= rhs.data[0];
        self.data[1] *= rhs.data[1];
        self.data[2] *= rhs.data[2];
    }
}

impl<T: Real> Div<TVec3<T>> for TVec3<T> {
    type Output = TVec3<T>;

    #[inline]
    fn div(mut self, rhs: TVec3<T>) -> Self::Output {
        self.data[0] /= rhs.data[0];
        self.data[1] /= rhs.data[1];
        self.data[2] /= rhs.data[2];
        self
    }
}

impl<T: Real> DivAssign<TVec3<T>> for TVec3<T> {
    #[inline]
    fn div_assign(&mut self, rhs: TVec3<T>) {
        self.data[0] /= rhs.data[0];
        self.data[1] /= rhs.data[1];
        self.data[2] /= rhs.data[2];
    }
}

//
// VEC3 <-> FLOAT MATH TRAITS IMPLS
//

impl<T: Real> Add<T> for TVec3<T> {
    type Output = TVec3<T>;

    #[inline]
    fn add(mut self, rhs: T) -> Self::Output {
        self.data[0] += rhs;
        self.data[1] += rhs;
        self.data[2] += rhs;
        self
    }
}

impl<T: Real> AddAssign<T> for TVec3<T> {
    #[inline]
    fn add_assign(&mut self, rhs: T) {
        self.data[0] += rhs;
        self.data[1] += rhs;
        self.data[2] += rhs;
    }
}

impl<T: Real> Sub<T> for TVec3<T> {
    type Output = TVec3<T>;

    #[inline]
    fn sub(mut self, rhs: T) -> Self::Output {
        self.data[0] -= rhs;
        self.data[1] -= rhs;
        self.data[2] -= rhs;
        self
    }
}

impl<T: Real> SubAssign<T> for TVec3<T> {
    #[inline]
    fn sub_assign(&mut self, rhs: T) {
        self.data[0] -= rhs;
        self.data[1] -= rhs;
        self.data[2] -= rhs;
    }
}

impl<T: Real> Mul<T> for TVec3<T> {
    type Output = TVec3<T>;

    #[inline]
    fn mul(mut self, rhs: T) -> Self::Output {
        self.data[0] *= rhs;
        self.data[1] *= rhs;
        self.data[2] *= rhs;
        self
    }
}

impl<T: Real> MulAssign<T> for TVec3<T> {
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        self.data[0] *= rhs;
        self.data[1] *= rhs;
        self.data[2] *= rhs;
    }
}

impl<T: Real> Div<T> for TVec3<T> {
    type Output = TVec3<T>;

    #[inline]
    fn div(mut self, rhs: T) -> Self::Output {
        self.data[0] /= rhs;
        self.data[1] /= rhs;
        self.data[2] /= rhs;
        self
    }
}

impl<T: Real> DivAssign<T> for TVec3<T> {
    #[inline]
    fn div_assign(&mut self, rhs: T) {
        self.data[0] /= rhs;
        self.data[1] /= rhs;
        self.data[2] /= rhs;
    }
}

impl<T: Real> Neg for TVec3<T> {
    type Output = Self;

    #[inline]
    fn neg(mut self) -> Self::Output {
        self.data[0] = -self.data[0];
        self.data[1] = -self.data[1];
        self.data[2] = -self.data[2];
        self
    }
}

impl<T: Real> Lerp<T> for TVec3<T> {
    #[inline]
    fn lerp(&self, b: &Self, factor: T) -> Self {
        *self + ((*self - *b) * factor)
    }
}

impl<T: Real> Length for TVec3<T> {
    type Output = T;

    #[inline]
    fn length(&self) -> Self::Output {
        self.length_squared().sqrt()
    }
}

impl<T: Real> LengthSquared for TVec3<T> {
    type Output = T;

    #[inline]
    fn length_squared(&self) -> Self::Output {
        self.dot(self)
    }
}

impl<T: Real> Normalize for TVec3<T> {
    #[inline]
    fn normalize(mut self) -> Self {
        self.normalize_assign();
        self
    }
}

impl<T: Real> NormalizeAssign for TVec3<T> {
    #[inline]
    fn normalize_assign(&mut self) {
        let len = self.length();
        *self *= T::one() / len;
    }
}

impl<T: Real> DotProduct<T> for TVec3<T> {
    #[inline]
    fn dot(&self, rhs: &Self) -> T {
        (self.data[0] * rhs.data[0]) + (self.data[1] * rhs.data[1]) + (self.data[2] * rhs.data[2])
    }
}

impl<T: Real> IntoDegrees for TVec3<T> {
    #[inline]
    fn into_degrees(mut self) -> Self {
        self.data[0] = self.data[0].into_degrees();
        self.data[1] = self.data[1].into_degrees();
        self.data[2] = self.data[2].into_degrees();
        self
    }
}

impl<T: Real> IntoRadians for TVec3<T> {
    #[inline]
    fn into_radians(mut self) -> Self {
        self.data[0] = self.data[0].into_radians();
        self.data[1] = self.data[1].into_radians();
        self.data[2] = self.data[2].into_radians();
        self
    }
}

impl<T: Real> CrossProduct for TVec3<T> {
    #[inline]
    fn cross(&self, rhs: &Self) -> Self {
        let x = (self.data[1] * rhs.data[2]) - (self.data[2] * rhs.data[1]);
        let y = (self.data[2] * rhs.data[0]) - (self.data[0] * rhs.data[2]);
        let z = (self.data[0] * rhs.data[1]) - (self.data[1] * rhs.data[0]);
        Self::new(x, y, z)
    }
}

impl<T: Real> PartialEq<TVec3<T>> for TVec3<T> {
    #[inline]
    fn eq(&self, other: &TVec3<T>) -> bool {
        self.data[0] == other.data[0]
            && self.data[1] == other.data[1]
            && self.data[2] == other.data[2]
    }

    #[inline]
    fn ne(&self, other: &TVec3<T>) -> bool {
        self.data[0] != other.data[0]
            || self.data[1] != other.data[1]
            || self.data[2] != other.data[2]
    }
}
