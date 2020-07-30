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

use core::ops::{Index, IndexMut, Mul, MulAssign};

use crate::matrix::TMat4x4;
use crate::traits::CrossProduct;
use crate::traits::DotProduct;
use crate::traits::Inverse;
use crate::traits::InverseAssign;
use crate::traits::Length;
use crate::traits::Normalize;
use crate::traits::NormalizeAssign;
use crate::traits::Real;
use crate::vector::TVec3;

#[repr(align(16))]
#[derive(Copy, Clone, Debug)]
pub struct TQuat<T: Real> {
    data: [T; 4],
}

impl<T: Real> TQuat<T> {
    ///
    ///
    ///
    #[inline]
    pub fn new(scalar: T, vector: TVec3<T>) -> Self {
        let data = [vector[0], vector[1], vector[2], scalar];
        TQuat { data }
    }

    ///
    ///
    ///
    #[inline]
    pub fn identity() -> Self {
        Self::new(T::one(), [T::zero(), T::zero(), T::zero()].into())
    }

    ///
    ///
    ///
    #[inline]
    pub fn from_euler_angles(angles: TVec3<T>) -> Self {
        let two = T::one() + T::one();
        let half_angles: TVec3<T> = angles / two;
        let sin_x = half_angles[0].sin();
        let cos_x = half_angles[0].cos();
        let sin_y = half_angles[1].sin();
        let cos_y = half_angles[1].cos();
        let sin_z = half_angles[2].sin();
        let cos_z = half_angles[2].cos();
        let vector = TVec3::<T>::new(
            sin_x * cos_y * cos_z - cos_x * sin_y * sin_z,
            cos_x * sin_y * cos_z + sin_x * cos_y * sin_z,
            cos_x * cos_y * sin_z - sin_x * sin_y * cos_z,
        );
        let scalar = cos_x * cos_y * cos_z + sin_x * sin_y * sin_z;
        Self::new(scalar, vector)
    }

    #[inline]
    pub fn from_angle_axis(angle: T, axis: TVec3<T>) -> TQuat<T> {
        let two = T::one() + T::one();

        let half_angle: T = angle / two;

        TQuat::<T>::new(half_angle.cos(), axis.normalize() * half_angle.sin())
    }

    ///
    ///
    ///
    #[inline]
    pub fn as_angle_axis(&self) -> (T, TVec3<T>) {
        let axis = if self.scalar() > T::zero() {
            self.vector()
        } else {
            -self.vector()
        };
        let axis = axis.normalize();
        let two = T::one() + T::one();
        let angle = two
            * T::atan2(
                axis.length(),
                if self.scalar() > T::zero() {
                    self.scalar()
                } else {
                    -self.scalar()
                },
            );
        (angle, axis)
    }

    ///
    ///
    ///
    pub fn conjugate(&self) -> TQuat<T> {
        TQuat::<T>::new(self.scalar(), -self.vector())
    }

    ///
    ///
    ///
    #[inline]
    pub fn scalar(&self) -> T {
        self.data[3]
    }

    ///
    ///
    ///
    #[inline]
    pub fn scalar_mut(&mut self) -> &mut T {
        &mut self.data[3]
    }

    ///
    ///
    ///
    #[inline]
    pub fn vector(&self) -> TVec3<T> {
        TVec3::<T>::new(self.data[0], self.data[1], self.data[2])
    }
}

impl<T: Real> Index<usize> for TQuat<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T: Real> IndexMut<usize> for TQuat<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T: Real> Mul<TQuat<T>> for TQuat<T> {
    type Output = TQuat<T>;

    #[inline]
    fn mul(mut self, rhs: TQuat<T>) -> Self::Output {
        self.mul_assign(rhs);
        self
    }
}

impl<T: Real> MulAssign<TQuat<T>> for TQuat<T> {
    #[inline]
    fn mul_assign(&mut self, rhs: TQuat<T>) {
        let lhs_scalar = self.scalar();
        let rhs_scalar = rhs.scalar();
        let lhs_vector = self.vector();
        let rhs_vector = rhs.vector();
        self.data[3] = (rhs_scalar * lhs_scalar) - (lhs_vector.dot(&rhs_vector));
        let vector =
            (rhs_vector * lhs_scalar) + (lhs_vector * rhs_scalar) + lhs_vector.cross(&rhs_vector);
        self.data[0] = vector[0];
        self.data[1] = vector[1];
        self.data[2] = vector[2];
    }
}

impl<T: Real> Mul<TVec3<T>> for TQuat<T> {
    type Output = TVec3<T>;

    #[inline]
    #[allow(clippy::suspicious_arithmetic_impl)]
    fn mul(self, rhs: TVec3<T>) -> Self::Output {
        let two = T::one() + T::one();
        let t = self.vector().cross(&rhs) * two;
        rhs + (t * self.scalar()) + self.vector().cross(&t)
    }
}

impl<T: Real> Inverse for TQuat<T> {
    #[inline]
    fn inverse(mut self) -> Self {
        self.inverse_assign();
        self
    }
}

impl<T: Real> InverseAssign for TQuat<T> {
    #[inline]
    fn inverse_assign(&mut self) {
        self.data[0] = -self.data[0];
        self.data[1] = -self.data[1];
        self.data[2] = -self.data[2];
    }
}

impl<T: Real> Normalize for TQuat<T> {
    #[inline]
    fn normalize(mut self) -> Self {
        self.normalize_assign();
        self
    }
}

impl<T: Real> NormalizeAssign for TQuat<T> {
    #[inline]
    fn normalize_assign(&mut self) {
        let len: T = (self.scalar() * self.scalar() + self.vector().dot(&self.vector())).sqrt();
        let len: T = T::one() / len;
        *self.scalar_mut() *= len;
        let vector = self.vector() * len;
        self.data[0] = vector[0];
        self.data[1] = vector[1];
        self.data[2] = vector[2];
    }
}

impl<T: Real> Into<TMat4x4<T>> for TQuat<T> {
    fn into(self) -> TMat4x4<T> {
        let mut result = TMat4x4::<T>::identity();

        let two = T::one() + T::one();

        let qxx = self.data[0] * self.data[0];
        let qyy = self.data[1] * self.data[1];
        let qzz = self.data[2] * self.data[2];
        let qxz = self.data[0] * self.data[2];
        let qxy = self.data[0] * self.data[1];
        let qyz = self.data[1] * self.data[2];
        let qwx = self.data[3] * self.data[0];
        let qwy = self.data[3] * self.data[1];
        let qwz = self.data[3] * self.data[2];

        result[(0, 0)] = T::one() - two * (qyy + qzz);
        result[(1, 0)] = two * (qxy + qwz);
        result[(2, 0)] = two * (qxz - qwy);

        result[(0, 1)] = two * (qxy - qwz);
        result[(1, 1)] = T::one() - two * (qxx + qzz);
        result[(2, 1)] = two * (qyz + qwx);

        result[(0, 2)] = two * (qxz + qwy);
        result[(1, 2)] = two * (qyz - qwx);
        result[(2, 2)] = T::one() - two * (qxx + qyy);

        result
    }
}
