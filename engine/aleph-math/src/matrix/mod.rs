//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod simd_x86;

use core::fmt::Display;
use core::fmt::Error;
use core::fmt::Formatter;
use core::ops::Add;
use core::ops::AddAssign;
use core::ops::Div;
use core::ops::DivAssign;
use core::ops::Index;
use core::ops::IndexMut;
use core::ops::Mul;
use core::ops::MulAssign;
use core::ops::Sub;
use core::ops::SubAssign;
use core::ptr;

use crate::traits::Column;
use crate::traits::ColumnRef;
use crate::traits::ColumnRefMut;
use crate::traits::DotProduct;
use crate::traits::Inverse;
use crate::traits::InverseAssign;
use crate::traits::Normalize;
use crate::traits::Real;
use crate::traits::Row;
use crate::traits::Transpose;
use crate::traits::TransposeAssign;
use crate::vector::TVec3;
use crate::vector::TVec4;

///
/// A column major 4x4 floating point matrix
///
#[repr(C)]
#[repr(align(16))]
#[derive(Clone, Debug)]
pub struct TMat4x4<T: Real> {
    data: [T; 16],
}

impl<T: Real> TMat4x4<T> {
    ///
    /// Construct a new 4x4 matrix from a flat array of 16 elements
    ///
    #[inline]
    pub fn new(input: [T; 16]) -> TMat4x4<T> {
        TMat4x4 { data: input }
    }

    ///
    /// Construct a new 4x4 matrix of entirely zeroes
    ///
    #[inline]
    pub fn zero() -> TMat4x4<T> {
        TMat4x4 {
            data: [T::zero(); 16],
        }
    }

    ///
    /// Construct a new 4x4 identity matrix
    ///
    pub fn identity() -> TMat4x4<T> {
        let data: [T; 16] = [
            T::one(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::one(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::one(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::one(),
        ];
        TMat4x4 { data }
    }

    ///
    /// Construct a new Vulkan compatible perspective transform matrix
    ///
    pub fn perspective(aspect: T, fov: T, near: T, far: T) -> TMat4x4<T> {
        let mut mat = TMat4x4::<T>::zero();

        let one = T::one();
        let two = one + one;

        let tan_half_fov = T::one() / (fov / two).tan();

        mat[(0, 0)] = tan_half_fov / aspect;
        mat[(1, 1)] = -tan_half_fov;
        mat[(2, 2)] = far / (far - near);
        mat[(2, 3)] = -(far * near) / (far - near);
        mat[(3, 2)] = one;

        mat
    }

    ///
    /// Construct a new translation matrix with the given translation
    ///
    pub fn translation(translation: TVec3<T>) -> TMat4x4<T> {
        let mut mat = TMat4x4::identity();
        mat[(0, 3)] = translation[0];
        mat[(1, 3)] = translation[1];
        mat[(2, 3)] = translation[2];

        mat
    }

    ///
    /// Apply a translation to this matrix
    ///
    pub fn translate(&mut self, translation: TVec3<T>) {
        let col_0 = self.get_column(0);
        let col_1 = self.get_column(1);
        let col_2 = self.get_column(2);
        let col_3 = self.get_column(3);

        let col_0 = col_0 * translation[0];
        let col_1 = col_1 * translation[1];
        let col_2 = col_2 * translation[2];

        let col_ref = self.get_column_ref_mut(3);
        *col_ref = col_0 + col_1 + col_2 + col_3;
    }

    ///
    /// Get an angle axis rotation matrix
    ///
    pub fn rotation(angle: T, axis: TVec3<T>) -> TMat4x4<T> {
        let mut out = TMat4x4::<T>::identity();
        out.rotate(angle, axis);
        out
    }

    ///
    /// Apply an angle axis rotation to this matrix
    ///
    pub fn rotate(&mut self, angle: T, in_axis: TVec3<T>) {
        let sin_angle = angle.sin();
        let cos_angle = angle.cos();

        let axis = in_axis.normalize();
        let temp: TVec3<T> = axis * (T::one() - cos_angle);

        let mut rot = TMat4x4::<T>::identity();

        rot[(0, 0)] = cos_angle + temp[0] * axis[0];
        rot[(0, 1)] = temp[0] * axis[1] + sin_angle * axis[2];
        rot[(0, 2)] = temp[0] * axis[2] - sin_angle * axis[1];

        rot[(1, 0)] = temp[1] * axis[0] - sin_angle * axis[2];
        rot[(1, 1)] = cos_angle + temp[1] * axis[1];
        rot[(1, 2)] = temp[1] * axis[2] + sin_angle * axis[0];

        rot[(2, 0)] = temp[2] * axis[0] + sin_angle * axis[1];
        rot[(2, 1)] = temp[2] * axis[1] - sin_angle * axis[0];
        rot[(2, 2)] = cos_angle + temp[2] * axis[2];

        let col_0 = self.get_column(0);
        let col_1 = self.get_column(1);
        let col_2 = self.get_column(2);

        let col_ref = self.get_column_ref_mut(0);
        *col_ref = col_0 * rot[(0, 0)] + col_1 * rot[(0, 1)] + col_2 * rot[(0, 2)];

        let col_ref = self.get_column_ref_mut(1);
        *col_ref = col_0 * rot[(1, 0)] + col_1 * rot[(1, 1)] + col_2 * rot[(1, 2)];

        let col_ref = self.get_column_ref_mut(2);
        *col_ref = col_0 * rot[(2, 0)] + col_1 * rot[(2, 1)] + col_2 * rot[(2, 2)];
    }

    ///
    /// Get a scaling matrix
    ///
    pub fn scaling(scale: TVec3<T>) -> TMat4x4<T> {
        let mut result = Self::identity();

        let col = result.get_column_ref_mut(0);
        *col *= scale[0];

        let col = result.get_column_ref_mut(1);
        *col *= scale[1];

        let col = result.get_column_ref_mut(2);
        *col *= scale[2];

        result
    }

    ///
    /// Apply scaling to the matrix
    ///
    pub fn scale(&mut self, scale: TVec3<T>) {
        let col = self.get_column_ref_mut(0);
        *col *= scale[0];

        let col = self.get_column_ref_mut(1);
        *col *= scale[1];

        let col = self.get_column_ref_mut(2);
        *col *= scale[2];
    }

    ///
    /// Construct a new matrix from the four columns passed into the function
    ///
    pub fn from_columns(col0: [T; 4], col1: [T; 4], col2: [T; 4], col3: [T; 4]) -> Self {
        let mut data = [T::zero(); 16];
        data[0..4].copy_from_slice(&col0);
        data[4..8].copy_from_slice(&col1);
        data[8..12].copy_from_slice(&col2);
        data[12..16].copy_from_slice(&col3);
        Self::new(data)
    }

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    #[inline]
    fn internal_apply(&self, vec: TVec4<T>) -> TVec4<T> {
        if T::is_f32() {
            simd_x86::simd_f32_apply(self, vec)
        } else {
            self.internal_apply_scalar(vec)
        }
    }

    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
    #[inline]
    fn internal_apply(&self, vec: TVec4<T>) -> TVec4<T> {
        self.internal_apply_scalar(vec)
    }

    #[inline]
    fn internal_apply_scalar(&self, vec: TVec4<T>) -> TVec4<T> {
        let row0 = self.get_row(0);
        let row1 = self.get_row(1);
        let row2 = self.get_row(2);
        let row3 = self.get_row(3);

        let mut out = TVec4::<T>::zero();

        out[0] = row0.dot(&vec);
        out[1] = row1.dot(&vec);
        out[2] = row2.dot(&vec);
        out[3] = row3.dot(&vec);

        out
    }

    ///
    /// Apply this matrix to a vector, creating a new one in the process
    ///
    pub fn apply(&self, vec: TVec4<T>) -> TVec4<T> {
        self.internal_apply(vec)
    }

    ///
    /// Apply a matrix to the given vector
    ///
    pub fn apply_to(&self, rhs: &mut TVec4<T>) {
        *rhs = self.apply(*rhs)
    }

    ///
    /// Multiply the given matrix onto this matrix
    ///
    pub fn multiply(&mut self, rhs: &TMat4x4<T>) {
        internal_mat4_mul_assign(self, rhs);
    }

    ///
    /// Gets a slice reference to the underlying data array
    ///
    pub fn as_slice(&self) -> &[T; 16] {
        &self.data
    }

    ///
    /// Gets a slice mutable reference to the underlying data array
    ///
    pub fn as_slice_mut(&mut self) -> &[T; 16] {
        &mut self.data
    }

    ///
    /// Returns column refs as TVec4<T>s
    ///
    #[inline]
    fn as_columns_ref(&self) -> (&TVec4<T>, &TVec4<T>, &TVec4<T>, &TVec4<T>) {
        unsafe {
            (
                &*(&self.data[0] as *const T as *const TVec4<T>),
                &*(&self.data[4] as *const T as *const TVec4<T>),
                &*(&self.data[8] as *const T as *const TVec4<T>),
                &*(&self.data[12] as *const T as *const TVec4<T>),
            )
        }
    }

    ///
    /// Returns column mut refs as TVec4<T>s
    ///
    #[inline]
    fn as_columns_mut(&mut self) -> (&mut TVec4<T>, &mut TVec4<T>, &mut TVec4<T>, &mut TVec4<T>) {
        unsafe {
            (
                &mut *(&mut self.data[0] as *mut T as *mut TVec4<T>),
                &mut *(&mut self.data[4] as *mut T as *mut TVec4<T>),
                &mut *(&mut self.data[8] as *mut T as *mut TVec4<T>),
                &mut *(&mut self.data[12] as *mut T as *mut TVec4<T>),
            )
        }
    }
}

impl<T: Real> Column for TMat4x4<T> {
    type Output = TVec4<T>;

    #[inline]
    fn get_column(&self, col: usize) -> Self::Output {
        TVec4::new(
            self.data[(col * 4)],
            self.data[(col * 4) + 1],
            self.data[(col * 4) + 2],
            self.data[(col * 4) + 3],
        )
    }
}

impl<T: Real> ColumnRef for TMat4x4<T> {
    #[inline]
    fn get_column_ref(&self, col: usize) -> &Self::Output {
        unsafe { &*(&self.data[(col * 4)] as *const T as *const TVec4<T>) }
    }
}

impl<T: Real> ColumnRefMut for TMat4x4<T> {
    #[inline]
    fn get_column_ref_mut(&mut self, col: usize) -> &mut Self::Output {
        unsafe { &mut *(&mut self.data[(col * 4)] as *mut T as *mut TVec4<T>) }
    }
}

impl<T: Real> Row for TMat4x4<T> {
    type Output = TVec4<T>;

    #[inline]
    fn get_row(&self, row: usize) -> Self::Output {
        TVec4::new(
            self.data[row],
            self.data[row + 4],
            self.data[row + 8],
            self.data[row + 12],
        )
    }
}

impl<T: Real> Display for TMat4x4<T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let precision = f.precision().unwrap_or_else(|| f.width().unwrap_or(4));
        for row in 0..4 {
            let row = self.get_row(row);
            let row_x = row[0];
            let row_y = row[1];
            let row_z = row[2];
            let row_w = row[3];
            writeln!(
                f,
                "    │ {:^width$} {:^width$} {:^width$} {:^width$} │",
                row_x,
                row_y,
                row_z,
                row_w,
                width = precision,
            )?;
        }

        Ok(())
    }
}

impl<T: Real> From<&[T; 16]> for TMat4x4<T> {
    #[inline]
    fn from(other: &[T; 16]) -> Self {
        TMat4x4::new(*other)
    }
}

impl<T: Real> Into<[T; 16]> for TMat4x4<T> {
    #[inline]
    fn into(self) -> [T; 16] {
        self.data
    }
}

impl<T: Real> Index<usize> for TMat4x4<T> {
    type Output = T;

    ///
    /// Index the matrix linearly
    ///
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T: Real> Index<(usize, usize)> for TMat4x4<T> {
    type Output = T;

    ///
    /// Index the matrix. (row, column)
    ///
    #[inline]
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let row = index.0;
        let col = index.1;
        &self.data[(col * 4) + row]
    }
}

impl<T: Real> IndexMut<usize> for TMat4x4<T> {
    ///
    /// Index the matrix linearly
    ///
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T: Real> IndexMut<(usize, usize)> for TMat4x4<T> {
    ///
    /// Index the matrix. (row, column)
    ///
    #[inline]
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let row = index.0;
        let col = index.1;
        &mut self.data[(col * 4) + row]
    }
}

impl<T: Real> Add<TMat4x4<T>> for TMat4x4<T> {
    type Output = Self;

    fn add(mut self, rhs: TMat4x4<T>) -> Self::Output {
        Self::add_assign(&mut self, rhs);
        self
    }
}

impl<T: Real> AddAssign<TMat4x4<T>> for TMat4x4<T> {
    fn add_assign(&mut self, rhs: TMat4x4<T>) {
        let (sv1, sv2, sv3, sv4) = self.as_columns_mut();
        let (rv1, rv2, rv3, rv4) = rhs.as_columns_ref();

        TVec4::add_assign(sv1, *rv1);
        TVec4::add_assign(sv2, *rv2);
        TVec4::add_assign(sv3, *rv3);
        TVec4::add_assign(sv4, *rv4);
    }
}

impl<T: Real> Sub<TMat4x4<T>> for TMat4x4<T> {
    type Output = Self;

    fn sub(mut self, rhs: TMat4x4<T>) -> Self::Output {
        Self::sub_assign(&mut self, rhs);
        self
    }
}

impl<T: Real> SubAssign<TMat4x4<T>> for TMat4x4<T> {
    fn sub_assign(&mut self, rhs: TMat4x4<T>) {
        let (sv1, sv2, sv3, sv4) = self.as_columns_mut();
        let (rv1, rv2, rv3, rv4) = rhs.as_columns_ref();

        TVec4::sub_assign(sv1, *rv1);
        TVec4::sub_assign(sv2, *rv2);
        TVec4::sub_assign(sv3, *rv3);
        TVec4::sub_assign(sv4, *rv4);
    }
}

impl<T: Real> Div<TMat4x4<T>> for TMat4x4<T> {
    type Output = Self;

    fn div(mut self, rhs: TMat4x4<T>) -> Self::Output {
        Self::div_assign(&mut self, rhs);
        self
    }
}

impl<T: Real> DivAssign<TMat4x4<T>> for TMat4x4<T> {
    fn div_assign(&mut self, rhs: TMat4x4<T>) {
        let (sv1, sv2, sv3, sv4) = self.as_columns_mut();
        let (rv1, rv2, rv3, rv4) = rhs.as_columns_ref();

        TVec4::div_assign(sv1, *rv1);
        TVec4::div_assign(sv2, *rv2);
        TVec4::div_assign(sv3, *rv3);
        TVec4::div_assign(sv4, *rv4);
    }
}

impl<T: Real> Mul<TMat4x4<T>> for TMat4x4<T> {
    type Output = Self;

    fn mul(mut self, rhs: TMat4x4<T>) -> Self::Output {
        Self::mul_assign(&mut self, rhs);
        self
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[inline]
fn internal_mat4_mul_assign<T: Real>(lhs: &mut TMat4x4<T>, rhs: &TMat4x4<T>) {
    if T::is_f32() {
        simd_x86::simd_f32_mul_assign(lhs, rhs);
    } else {
        internal_mat4_mul_assign_scalar(lhs, rhs);
    }
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
#[inline]
fn internal_mat4_mul_assign<T: Real>(lhs: &mut TMat4x4<T>, rhs: TMat4x4<T>) {
    internal_mat4_mul_assign_scalar(lhs, rhs);
}

#[inline]
fn internal_mat4_mul_assign_scalar<T: Real>(lhs: &mut TMat4x4<T>, rhs: &TMat4x4<T>) {
    let row0 = lhs.get_row(0);
    let row1 = lhs.get_row(1);
    let row2 = lhs.get_row(2);
    let row3 = lhs.get_row(3);
    let col0 = rhs.get_column(0);
    let col1 = rhs.get_column(1);
    let col2 = rhs.get_column(2);
    let col3 = rhs.get_column(3);

    {
        lhs[(0, 0)] = col0.dot(&row0);
        lhs[(0, 1)] = col1.dot(&row0);
        lhs[(0, 2)] = col2.dot(&row0);
        lhs[(0, 3)] = col3.dot(&row0);
    }
    {
        lhs[(1, 0)] = col0.dot(&row1);
        lhs[(1, 1)] = col1.dot(&row1);
        lhs[(1, 2)] = col2.dot(&row1);
        lhs[(1, 3)] = col3.dot(&row1);
    }
    {
        lhs[(2, 0)] = col0.dot(&row2);
        lhs[(2, 1)] = col1.dot(&row2);
        lhs[(2, 2)] = col2.dot(&row2);
        lhs[(2, 3)] = col3.dot(&row2);
    }
    {
        lhs[(3, 0)] = col0.dot(&row3);
        lhs[(3, 1)] = col1.dot(&row3);
        lhs[(3, 2)] = col2.dot(&row3);
        lhs[(3, 3)] = col3.dot(&row3);
    }
}

impl<T: Real> MulAssign<TMat4x4<T>> for TMat4x4<T> {
    fn mul_assign(&mut self, rhs: TMat4x4<T>) {
        internal_mat4_mul_assign(self, &rhs)
    }
}

impl<T: Real> Mul<TVec4<T>> for TMat4x4<T> {
    type Output = TVec4<T>;

    fn mul(self, rhs: TVec4<T>) -> Self::Output {
        self.apply(rhs)
    }
}

impl<T: Real> MulAssign<T> for TMat4x4<T> {
    fn mul_assign(&mut self, rhs: T) {
        *self.get_column_ref_mut(0) *= rhs;
        *self.get_column_ref_mut(1) *= rhs;
        *self.get_column_ref_mut(2) *= rhs;
        *self.get_column_ref_mut(3) *= rhs;
    }
}

impl<T: Real> Mul<T> for TMat4x4<T> {
    type Output = TMat4x4<T>;

    fn mul(mut self, rhs: T) -> Self::Output {
        *self.get_column_ref_mut(0) *= rhs;
        *self.get_column_ref_mut(1) *= rhs;
        *self.get_column_ref_mut(2) *= rhs;
        *self.get_column_ref_mut(3) *= rhs;
        self
    }
}

impl<T: Real> PartialEq<TMat4x4<T>> for TMat4x4<T> {
    fn eq(&self, other: &TMat4x4<T>) -> bool {
        self.get_column_ref(0) == other.get_column_ref(0)
            && self.get_column_ref(1) == other.get_column_ref(1)
            && self.get_column_ref(2) == other.get_column_ref(2)
            && self.get_column_ref(3) == other.get_column_ref(3)
    }

    #[allow(clippy::partialeq_ne_impl)]
    fn ne(&self, other: &TMat4x4<T>) -> bool {
        self.get_column_ref(0) != other.get_column_ref(0)
            || self.get_column_ref(1) != other.get_column_ref(1)
            || self.get_column_ref(2) != other.get_column_ref(2)
            || self.get_column_ref(3) != other.get_column_ref(3)
    }
}

impl<T: Real> Transpose for TMat4x4<T> {
    #[inline]
    fn transpose(mut self) -> Self {
        self.transpose_assign();
        self
    }
}

impl<T: Real> TransposeAssign for TMat4x4<T> {
    #[inline]
    fn transpose_assign(&mut self) {
        unsafe {
            ptr::swap(&mut self[(1, 0)], &mut self[(0, 1)]);
            ptr::swap(&mut self[(2, 1)], &mut self[(1, 2)]);
            ptr::swap(&mut self[(3, 2)], &mut self[(2, 3)]);

            ptr::swap(&mut self[(2, 0)], &mut self[(0, 2)]);
            ptr::swap(&mut self[(3, 0)], &mut self[(0, 3)]);
            ptr::swap(&mut self[(3, 1)], &mut self[(1, 3)]);
        }
    }
}

impl<T: Real> Inverse for TMat4x4<T> {
    fn inverse(self) -> Self {
        let mut m = self;

        let m_col0 = m.get_column(0);

        let coef00: T = m[(2, 2)] * m[(3, 3)] - m[(2, 3)] * m[(3, 2)];
        let coef02: T = m[(2, 1)] * m[(3, 3)] - m[(2, 3)] * m[(3, 1)];
        let coef03: T = m[(2, 1)] * m[(3, 2)] - m[(2, 2)] * m[(3, 1)];

        let coef04: T = m[(1, 2)] * m[(3, 3)] - m[(1, 3)] * m[(3, 2)];
        let coef06: T = m[(1, 1)] * m[(3, 3)] - m[(1, 3)] * m[(3, 1)];
        let coef07: T = m[(1, 1)] * m[(3, 2)] - m[(1, 2)] * m[(3, 1)];

        let coef08: T = m[(1, 2)] * m[(2, 3)] - m[(1, 3)] * m[(2, 2)];
        let coef10: T = m[(1, 1)] * m[(2, 3)] - m[(1, 3)] * m[(2, 1)];
        let coef11: T = m[(1, 1)] * m[(2, 2)] - m[(1, 2)] * m[(2, 1)];

        let coef12: T = m[(0, 2)] * m[(3, 3)] - m[(0, 3)] * m[(3, 2)];
        let coef14: T = m[(0, 1)] * m[(3, 3)] - m[(0, 3)] * m[(3, 1)];
        let coef15: T = m[(0, 1)] * m[(3, 2)] - m[(0, 2)] * m[(3, 1)];

        let coef16: T = m[(0, 2)] * m[(2, 3)] - m[(0, 3)] * m[(2, 2)];
        let coef18: T = m[(0, 1)] * m[(2, 3)] - m[(0, 3)] * m[(2, 1)];
        let coef19: T = m[(0, 1)] * m[(2, 2)] - m[(0, 2)] * m[(2, 1)];

        let coef20: T = m[(0, 2)] * m[(1, 3)] - m[(0, 3)] * m[(1, 2)];
        let coef22: T = m[(0, 1)] * m[(1, 3)] - m[(0, 3)] * m[(1, 1)];
        let coef23: T = m[(0, 1)] * m[(1, 2)] - m[(0, 2)] * m[(1, 1)];

        let fac0 = TVec4::<T>::new(coef00, coef00, coef02, coef03);
        let fac1 = TVec4::<T>::new(coef04, coef04, coef06, coef07);
        let fac2 = TVec4::<T>::new(coef08, coef08, coef10, coef11);
        let fac3 = TVec4::<T>::new(coef12, coef12, coef14, coef15);
        let fac4 = TVec4::<T>::new(coef16, coef16, coef18, coef19);
        let fac5 = TVec4::<T>::new(coef20, coef20, coef22, coef23);

        let vec0 = TVec4::<T>::new(m[(0, 1)], m[(0, 0)], m[(0, 0)], m[(0, 0)]);
        let vec1 = TVec4::<T>::new(m[(1, 1)], m[(1, 0)], m[(1, 0)], m[(1, 0)]);
        let vec2 = TVec4::<T>::new(m[(2, 1)], m[(2, 0)], m[(2, 0)], m[(2, 0)]);
        let vec3 = TVec4::<T>::new(m[(3, 1)], m[(3, 0)], m[(3, 0)], m[(3, 0)]);

        let inv0 = vec1 * fac0 - vec2 * fac1 + vec3 * fac2;
        let inv1 = vec0 * fac0 - vec2 * fac3 + vec3 * fac4;
        let inv2 = vec0 * fac1 - vec1 * fac3 + vec3 * fac5;
        let inv3 = vec0 * fac2 - vec1 * fac4 + vec2 * fac5;

        let sign_a = TVec4::<T>::new(T::one(), -T::one(), T::one(), -T::one());
        let sign_b = TVec4::<T>::new(-T::one(), T::one(), -T::one(), T::one());
        *m.get_column_ref_mut(0) = inv0 * sign_a;
        *m.get_column_ref_mut(1) = inv1 * sign_b;
        *m.get_column_ref_mut(2) = inv2 * sign_a;
        *m.get_column_ref_mut(3) = inv3 * sign_b;
        let inverse = m;

        let row0 = TVec4::<T>::new(
            inverse[(0, 0)],
            inverse[(0, 1)],
            inverse[(0, 2)],
            inverse[(0, 3)],
        );

        let dot0 = m_col0 * row0;
        let dot1: T = (dot0[0] + dot0[1]) + (dot0[2] + dot0[3]);

        let one_over_determinant = T::one() / dot1;

        inverse * one_over_determinant
    }
}

impl<T: Real> InverseAssign for TMat4x4<T> {
    fn inverse_assign(&mut self) {
        let m = self;

        let m_col0 = m.get_column(0);

        let coef00: T = m[(2, 2)] * m[(3, 3)] - m[(2, 3)] * m[(3, 2)];
        let coef02: T = m[(2, 1)] * m[(3, 3)] - m[(2, 3)] * m[(3, 1)];
        let coef03: T = m[(2, 1)] * m[(3, 2)] - m[(2, 2)] * m[(3, 1)];

        let coef04: T = m[(1, 2)] * m[(3, 3)] - m[(1, 3)] * m[(3, 2)];
        let coef06: T = m[(1, 1)] * m[(3, 3)] - m[(1, 3)] * m[(3, 1)];
        let coef07: T = m[(1, 1)] * m[(3, 2)] - m[(1, 2)] * m[(3, 1)];

        let coef08: T = m[(1, 2)] * m[(2, 3)] - m[(1, 3)] * m[(2, 2)];
        let coef10: T = m[(1, 1)] * m[(2, 3)] - m[(1, 3)] * m[(2, 1)];
        let coef11: T = m[(1, 1)] * m[(2, 2)] - m[(1, 2)] * m[(2, 1)];

        let coef12: T = m[(0, 2)] * m[(3, 3)] - m[(0, 3)] * m[(3, 2)];
        let coef14: T = m[(0, 1)] * m[(3, 3)] - m[(0, 3)] * m[(3, 1)];
        let coef15: T = m[(0, 1)] * m[(3, 2)] - m[(0, 2)] * m[(3, 1)];

        let coef16: T = m[(0, 2)] * m[(2, 3)] - m[(0, 3)] * m[(2, 2)];
        let coef18: T = m[(0, 1)] * m[(2, 3)] - m[(0, 3)] * m[(2, 1)];
        let coef19: T = m[(0, 1)] * m[(2, 2)] - m[(0, 2)] * m[(2, 1)];

        let coef20: T = m[(0, 2)] * m[(1, 3)] - m[(0, 3)] * m[(1, 2)];
        let coef22: T = m[(0, 1)] * m[(1, 3)] - m[(0, 3)] * m[(1, 1)];
        let coef23: T = m[(0, 1)] * m[(1, 2)] - m[(0, 2)] * m[(1, 1)];

        let fac0 = TVec4::<T>::new(coef00, coef00, coef02, coef03);
        let fac1 = TVec4::<T>::new(coef04, coef04, coef06, coef07);
        let fac2 = TVec4::<T>::new(coef08, coef08, coef10, coef11);
        let fac3 = TVec4::<T>::new(coef12, coef12, coef14, coef15);
        let fac4 = TVec4::<T>::new(coef16, coef16, coef18, coef19);
        let fac5 = TVec4::<T>::new(coef20, coef20, coef22, coef23);

        let vec0 = TVec4::<T>::new(m[(0, 1)], m[(0, 0)], m[(0, 0)], m[(0, 0)]);
        let vec1 = TVec4::<T>::new(m[(1, 1)], m[(1, 0)], m[(1, 0)], m[(1, 0)]);
        let vec2 = TVec4::<T>::new(m[(2, 1)], m[(2, 0)], m[(2, 0)], m[(2, 0)]);
        let vec3 = TVec4::<T>::new(m[(3, 1)], m[(3, 0)], m[(3, 0)], m[(3, 0)]);

        let inv0 = vec1 * fac0 - vec2 * fac1 + vec3 * fac2;
        let inv1 = vec0 * fac0 - vec2 * fac3 + vec3 * fac4;
        let inv2 = vec0 * fac1 - vec1 * fac3 + vec3 * fac5;
        let inv3 = vec0 * fac2 - vec1 * fac4 + vec2 * fac5;

        let sign_a = TVec4::<T>::new(T::one(), -T::one(), T::one(), -T::one());
        let sign_b = TVec4::<T>::new(-T::one(), T::one(), -T::one(), T::one());
        *m.get_column_ref_mut(0) = inv0 * sign_a;
        *m.get_column_ref_mut(1) = inv1 * sign_b;
        *m.get_column_ref_mut(2) = inv2 * sign_a;
        *m.get_column_ref_mut(3) = inv3 * sign_b;
        let inverse = m;

        let row0 = TVec4::<T>::new(
            inverse[(0, 0)],
            inverse[(0, 1)],
            inverse[(0, 2)],
            inverse[(0, 3)],
        );

        let dot0 = m_col0 * row0;
        let dot1: T = (dot0[0] + dot0[1]) + (dot0[2] + dot0[3]);

        let one_over_determinant = T::one() / dot1;

        inverse.mul_assign(one_over_determinant);
    }
}

#[cfg(test)]
pub mod tests;
