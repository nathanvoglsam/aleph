//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use core::f64::consts::PI;
use core::fmt::{Debug, Display};
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use num_traits::Float;

pub enum FloatType {
    F32,
    F64,
}

///
/// This type represents a real number. A macro trait that makes using generic floats much easier
///
pub trait Real:
    Float
    + Add
    + AddAssign
    + Sub
    + SubAssign
    + Mul
    + MulAssign
    + Div
    + DivAssign
    + Copy
    + Clone
    + Pi
    + IntoDegrees
    + IntoRadians
    + PartialEq
    + PartialOrd
    + Debug
    + Display
{
    ///
    /// Is this type a single-precision floating point. Useful for identifying the underlying
    /// representation for SIMD purposes
    ///
    fn is_f32() -> bool;

    ///
    /// Is this type a double-precision floating point. Useful for identifying the underlying
    /// representation for SIMD purposes
    ///
    fn is_f64() -> bool;

    ///
    /// Return the floating point type as an enum. Mostly just a convinience instead of using
    /// `is_f32()` or `is_f64()`
    ///
    fn float_type() -> FloatType;

    ///
    /// Force get this value as single-precision
    ///
    fn as_f32(self) -> f32;

    ///
    /// Force get this value as double-precision
    ///
    fn as_f64(self) -> f64;
}

impl Real for f32 {
    #[inline]
    fn is_f32() -> bool {
        true
    }

    #[inline]
    fn is_f64() -> bool {
        false
    }

    #[inline]
    fn float_type() -> FloatType {
        FloatType::F32
    }

    #[inline]
    fn as_f32(self) -> f32 {
        self
    }

    #[inline]
    fn as_f64(self) -> f64 {
        f64::from(self)
    }
}

impl Real for f64 {
    #[inline]
    fn is_f32() -> bool {
        false
    }

    #[inline]
    fn is_f64() -> bool {
        true
    }

    #[inline]
    fn float_type() -> FloatType {
        FloatType::F64
    }

    #[inline]
    fn as_f32(self) -> f32 {
        self as f32
    }

    #[inline]
    fn as_f64(self) -> f64 {
        self
    }
}

///
/// This type can perform linear interpolate from a to b with factor
///
pub trait Lerp<F: Real> {
    ///
    /// Return the result of interpolating between self and b with factor `factor`
    ///
    fn lerp(&self, b: &Self, factor: F) -> Self;
}

impl Lerp<f32> for f32 {
    fn lerp(&self, b: &Self, factor: f32) -> Self {
        *self + ((*self - *b) * factor)
    }
}

impl Lerp<f64> for f64 {
    fn lerp(&self, b: &Self, factor: f64) -> Self {
        *self + ((*self - *b) * factor)
    }
}

///
/// This type has a constant PI
///
pub trait Pi {
    ///
    /// Get a value that represents Pi
    ///
    fn pi() -> Self;
}

impl<T: Real> Pi for T {
    fn pi() -> Self {
        T::from(PI).unwrap()
    }
}

///
/// This type can convert from degrees to radians
///
pub trait IntoRadians {
    ///
    /// Consume self and return it after converting the internal elements from degrees to radians
    ///
    fn into_radians(self) -> Self;
}

impl<T: Real + Pi> IntoRadians for T {
    fn into_radians(self) -> Self {
        crate::units::radians(self)
    }
}

///
/// This type can convert from radians to degrees
///
pub trait IntoDegrees {
    ///
    /// Consume self and return it after converting the internal elements from radians to degrees
    ///
    fn into_degrees(self) -> Self;
}

impl<T: Real + Pi> IntoDegrees for T {
    fn into_degrees(self) -> Self {
        crate::units::degrees(self)
    }
}

///
/// This type can be used to produce a dot product
///
pub trait DotProduct<T: Real> {
    ///
    /// Produce the dot product of self and rhs
    ///
    fn dot(&self, rhs: &Self) -> T;
}

///
/// This type can be used to produce a cross product
///
pub trait CrossProduct {
    ///
    /// Produce the cross product of self and rhs
    ///
    /// # Note
    ///
    /// This can only really be implemented for a 3 component vector but is a trait to allow for
    /// separating storage from implementation
    ///
    fn cross(&self, rhs: &Self) -> Self;
}

///
/// This types supports performing a matrix transpose
///
/// # Info
///
/// Similar to the `Add` or `Mul` traits in that it takes ownership and passes the underlying object
/// through the function.
///
pub trait Transpose {
    fn transpose(self) -> Self;
}

///
/// This type supports performing a matrix transpose
///
/// # Info
///
/// Similar to the `AddAssign` or `MulAssign` traits in that it takes a mutable reference to the
/// underlying object and performs the transpose in place.
///
pub trait TransposeAssign {
    fn transpose_assign(&mut self);
}

// TODO: Document me

pub trait Inverse {
    fn inverse(self) -> Self;
}

pub trait InverseAssign {
    fn inverse_assign(&mut self);
}

///
/// Packing the underlying data of a vector or matrix
///
pub trait Pack {
    type GLSLOutput;
    type HLSLOutput;
    type GLSLOutputArray;
    type HLSLOutputArray;
    type CPUOutput;

    ///
    /// Convert the struct into packed data ready to be uploaded and consumed with hlsl standard
    /// conventions.
    ///
    /// This will often round vectors up to alignment multiple sizes with padding bytes and is
    /// important for matrices as hlsl shaders are expecting the matrices to be row major.
    ///
    /// # Warning
    ///
    /// If the matrix this is implemented on is row major it will have to perform an implict
    /// transpose and so CAN CHANGE the underlying data beyond adding GPU required padding.
    ///
    fn into_packed_glsl(self) -> Self::GLSLOutput;

    ///
    /// Convert the struct into packed data ready to be uploaded and consumed with hlsl standard
    /// conventions.
    ///
    /// This will often round vectors up to alignment multiple sizes with padding bytes and is
    /// important for matrices as hlsl shaders are expecting the matrices to be row major.
    ///
    /// # Warning
    ///
    /// If the matrix this is implemented on is column major it will have to perform an implict
    /// transpose and so CAN CHANGE the underlying data beyond adding GPU required padding
    ///
    fn into_packed_hlsl(self) -> Self::HLSLOutput;

    ///
    /// Convert the struct into packed data ready to be uploaded and consumed with hlsl standard
    /// conventions.
    ///
    /// This function produces a semantically similar result to `into_packed_hlsl` but differs in
    /// that for some GPU packing conventions (read: std430) the padding for an item, like a vec3,
    /// differs whether it is on it's own or if it is an array element.
    ///
    /// This will often round vectors up to alignment multiple sizes with padding bytes and is
    /// important for matrices as hlsl shaders are expecting the matrices to be row major.
    ///
    /// # Warning
    ///
    /// If the matrix this is implemented on is row major it will have to perform an implict
    /// transpose and so CAN CHANGE the underlying data beyond adding GPU required padding.
    ///
    fn into_packed_glsl_array(self) -> Self::GLSLOutputArray;

    ///
    /// Convert the struct into packed data ready to be uploaded and consumed with hlsl standard
    /// conventions.
    ///
    /// This function produces a semantically similar result to `into_packed_hlsl` but differs in
    /// that for some GPU packing conventions (read: std430) the padding for an item, like a vec3,
    /// differs whether it is on it's own or if it is an array element.
    ///
    /// This will often round vectors up to alignment multiple sizes with padding bytes and is
    /// important for matrices as hlsl shaders are expecting the matrices to be row major.
    ///
    /// # Warning
    ///
    /// If the matrix this is implemented on is column major it will have to perform an implict
    /// transpose and so CAN CHANGE the underlying data beyond adding GPU required padding.
    ///
    fn into_packed_hlsl_array(self) -> Self::HLSLOutputArray;

    ///
    /// Convert the struct into packed data for general purpose use on the CPU. This would be
    /// ideal for things like serialization where you don't need to conform to special GPU alignment
    /// and padding rules.
    ///
    /// This should, by general convention, just produce a flat array of the individual components
    /// and should match the underlying number of components (3 for a Vec3, etc).
    ///
    /// # Warning
    ///
    /// There should be no padding in the results of this function.
    ///
    fn into_packed_cpu(self) -> Self::CPUOutput;
}

///
/// This type abstracts a matrix where you can get a copy of a column
///
pub trait Column {
    type Output;

    ///
    /// Get a copy of a given column of a matrix
    ///
    fn get_column(&self, col: usize) -> Self::Output;
}

///
/// This type abstracts a matrix where you can get a reference of a column
///
pub trait ColumnRef: Column {
    ///
    /// Get a reference to a given column of a matrix
    ///
    fn get_column_ref(&self, col: usize) -> &Self::Output;
}

///
/// This type abstracts a matrix where you can get a mutable reference of a column
///
pub trait ColumnRefMut: Column {
    ///
    /// Get a mutable reference to a given column of a matrix
    ///
    fn get_column_ref_mut(&mut self, col: usize) -> &mut Self::Output;
}

///
/// This type abstracts a matrix where you can get a copy of a row
///
pub trait Row {
    type Output;

    ///
    /// Get a copy of a given row of a matrix
    ///
    fn get_row(&self, row: usize) -> Self::Output;
}

///
/// This type abstracts a matrix where you can get a reference of a row
///
pub trait RowRef: Row {
    ///
    /// Get a reference to a given row of a matrix
    ///
    fn get_row_ref(&self, row: usize) -> &Self::Output;
}

///
/// This type abstracts a matrix where you can get a mutable reference of a row
///
pub trait RowRefMut: Row {
    ///
    /// Get a mutable reference to a given row of a matrix
    ///
    fn get_row_ref_mut(&mut self, row: usize) -> &mut Self::Output;
}

///
/// This type abstracts a vector or other object that can represents a length
///
pub trait Length {
    type Output;

    fn length(&self) -> Self::Output;
}

///
/// This type abstracts a vector or other object that can represents a length. Get's the square of
/// the length as this can often skip an expensive square root calculation.
///
pub trait LengthSquared {
    type Output;

    fn length_squared(&self) -> Self::Output;
}

///
/// This type abstracts a vector or other object that can be normalized to represent the same
/// direction while having a length of 1
///
pub trait Normalize {
    fn normalize(self) -> Self;
}

///
/// This type abstracts a vector or other object that can be normalized to represent the same
/// direction while having a length of 1
///
pub trait NormalizeAssign {
    fn normalize_assign(&mut self);
}
