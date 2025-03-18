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

use core::f32;

use aleph_math::Vec4;
use bytemuck::{AnyBitPattern, NoUninit, Pod, Zeroable};
use half::f16;
use image::{Luma, LumaA, Pixel, Rgb, Rgba};

use crate::utils::{f32_to_unorm_u16, f32_to_unorm_u8, unorm_u16_to_f32, unorm_u8_to_f32};

/// A single channel pixel with a generic channel type.
#[derive(Copy, Clone, Zeroable, Pod)]
#[repr(transparent)]
pub struct PixR<T: PixelChannelType>(pub [T; 1]);

/// A single channel pixel with a generic channel type.
#[derive(Copy, Clone, Zeroable, Pod)]
#[repr(transparent)]
pub struct PixRG<T: PixelChannelType>(pub [T; 2]);

/// A single channel pixel with a generic channel type.
#[derive(Copy, Clone, Zeroable, Pod)]
#[repr(transparent)]
pub struct PixRGB<T: PixelChannelType>(pub [T; 3]);

/// A single channel pixel with a generic channel type.
#[derive(Copy, Clone, Zeroable, Pod)]
#[repr(transparent)]
pub struct PixRGBA<T: PixelChannelType>(pub [T; 4]);

/// The interface exposed by our pixel types. This provides basic, generic read/write access to
/// individual pixels as well as generic conversion operations to and from a floating point vec4
/// representation.
pub trait PixelFormat {
    /// The channel type the implementing pixel type stores.
    type Storage: PixelChannelType;

    /// The number of components this pixel format contains
    const COMPONENTS: usize;

    /// Loads a pixel from a flat array of [`PixelFormat::Storage`] elements. This will load exactly
    /// [`PixelFormat::COMPONENTS`] elements from the given array.
    fn from_storage(v: &[Self::Storage]) -> Self;

    /// Stores a pixel into the target array. This will store exactly [`PixelFormat::COMPONENTS`]
    /// into the array, overwriting the first 'n' elements in the array.
    fn write_at(&self, v: &mut [Self::Storage]);

    /// Converts the pixel into a universal fp32, vec4 representation. Pixel formats with less than
    /// 4 channels will have any missing channels default to 0.
    ///
    /// This should internally use [`PixelChannelType::into_float`] to convert from the storage
    /// type into the floating point pixel. This includes any Unorm mapping required.
    fn as_vec4(&self) -> Vec4 {
        self.as_vec4_with_default(0.0)
    }

    /// A twin to [`PixelFormat::as_vec4`] that offers a configurable default value.
    fn as_vec4_with_default(&self, default: f32) -> Vec4;

    /// The inverse of [`PixelFormat::as_vec4`]. Converts the given floating point sample _back_
    /// into the matching encoded pixel value.
    fn from_vec4(v: Vec4) -> Self;
}

impl<T: PixelChannelType> PixelFormat for PixR<T> {
    type Storage = T;

    const COMPONENTS: usize = 1;

    #[inline]
    fn from_storage(v: &[Self::Storage]) -> Self {
        Self([v[0]])
    }

    #[inline]
    fn write_at(&self, v: &mut [Self::Storage]) {
        v[0] = self.0[0];
    }

    #[inline]
    fn as_vec4_with_default(&self, default: f32) -> Vec4 {
        Vec4::new(self.0[0].into_float(), default, default, default)
    }

    #[inline]
    fn from_vec4(v: Vec4) -> Self {
        Self([T::from_float(v.x)])
    }
}

impl<T: PixelChannelType> PixelFormat for PixRG<T> {
    type Storage = T;

    const COMPONENTS: usize = 2;

    #[inline]
    fn from_storage(v: &[Self::Storage]) -> Self {
        Self([v[0], v[1]])
    }

    #[inline]
    fn write_at(&self, v: &mut [Self::Storage]) {
        v[0] = self.0[0];
        v[1] = self.0[1];
    }

    #[inline]
    fn as_vec4_with_default(&self, default: f32) -> Vec4 {
        Vec4::new(
            self.0[0].into_float(),
            self.0[1].into_float(),
            default,
            default,
        )
    }

    #[inline]
    fn from_vec4(v: Vec4) -> Self {
        Self([T::from_float(v.x), T::from_float(v.y)])
    }
}

impl<T: PixelChannelType> PixelFormat for PixRGB<T> {
    type Storage = T;

    const COMPONENTS: usize = 3;

    #[inline]
    fn from_storage(v: &[Self::Storage]) -> Self {
        Self([v[0], v[1], v[2]])
    }

    #[inline]
    fn write_at(&self, v: &mut [Self::Storage]) {
        v[0] = self.0[0];
        v[1] = self.0[1];
        v[2] = self.0[2];
    }

    #[inline]
    fn as_vec4_with_default(&self, default: f32) -> Vec4 {
        Vec4::new(
            self.0[0].into_float(),
            self.0[1].into_float(),
            self.0[2].into_float(),
            default,
        )
    }

    #[inline]
    fn from_vec4(v: Vec4) -> Self {
        Self([T::from_float(v.x), T::from_float(v.y), T::from_float(v.z)])
    }
}

impl<T: PixelChannelType> PixelFormat for PixRGBA<T> {
    type Storage = T;

    const COMPONENTS: usize = 4;

    #[inline]
    fn from_storage(v: &[Self::Storage]) -> Self {
        Self([v[0], v[1], v[2], v[3]])
    }

    #[inline]
    fn write_at(&self, v: &mut [Self::Storage]) {
        v[0] = self.0[0];
        v[1] = self.0[1];
        v[2] = self.0[2];
        v[3] = self.0[3];
    }

    #[inline]
    fn as_vec4_with_default(&self, _default: f32) -> Vec4 {
        Vec4::new(
            self.0[0].into_float(),
            self.0[1].into_float(),
            self.0[2].into_float(),
            self.0[3].into_float(),
        )
    }

    #[inline]
    fn from_vec4(v: Vec4) -> Self {
        Self([
            T::from_float(v.x),
            T::from_float(v.y),
            T::from_float(v.z),
            T::from_float(v.w),
        ])
    }
}

/// This is a (largely internal) trait used to enable our generic conversions from the 'image'
/// crate's [`image::ImageBuffer`] type into our own [`crate::ImageBuffer`] type. This allows us
/// to get the appropriate [`image::Pixel`] type for one of our own [`crate::PixelFormat`] types
/// in generic code.
pub trait FromImagePixel: PixelFormat {
    type Source: Pixel<Subpixel = Self::Storage>;
}

impl<P> FromImagePixel for PixR<P>
where
    Luma<P>: Pixel<Subpixel = P>,
    P: PixelChannelType,
{
    type Source = Luma<P>;
}

impl<P> FromImagePixel for PixRG<P>
where
    LumaA<P>: Pixel<Subpixel = P>,
    P: PixelChannelType,
{
    type Source = LumaA<P>;
}

impl<P> FromImagePixel for PixRGB<P>
where
    Rgb<P>: Pixel<Subpixel = P>,
    P: PixelChannelType,
{
    type Source = Rgb<P>;
}

impl<P> FromImagePixel for PixRGBA<P>
where
    Rgba<P>: Pixel<Subpixel = P>,
    P: PixelChannelType,
{
    type Source = Rgba<P>;
}

/// Interface expected of a numerical type that forms one channel of a whole pixel.
///
/// This is the '16Float' or '8Unorm' of the whole pixel format description ('RG8Unorm').
///
/// This trait is effectively constrained such that only basic numeric types can be used
/// (efficiently).
pub trait PixelChannelType:
    Copy + Clone + PartialEq + PartialOrd + Default + Sized + Pod + AnyBitPattern + NoUninit
{
    /// The zero value for this pixel channel.
    ///
    /// Useful for getting a zero value in a const context for zero-initializing arrays in generic
    /// code.
    const ZERO: Self;

    /// Converts the value 'self' into the appropriate floating point representation.
    ///
    /// This is where our sample mapping occurs. Unorm conversion happens here.
    ///
    /// - Unorm formats map to a float in the 0..1 range.
    /// - Float formats map directly, without clamping. This may produce NaNs if narrowing from
    ///   fp64. However we don't currently implement this trait for fp64
    fn into_float(self) -> f32;

    /// This is the inverse of [`PixelChannelType::into_float`] that performs the inverse operation
    /// to map back from a float into the pixel channel format.
    fn from_float(v: f32) -> Self;

    /// Performs an in-place conversion from native-endian to little-endian. This is a no-op on
    /// little endian platforms, and an endian swap operation on big endian platforms.
    fn to_le(&mut self);
}

impl PixelChannelType for u8 {
    const ZERO: Self = 0;

    #[inline(always)]
    fn into_float(self) -> f32 {
        unorm_u8_to_f32(self)
    }

    #[inline(always)]
    fn from_float(v: f32) -> Self {
        f32_to_unorm_u8(v)
    }

    #[inline(always)]
    fn to_le(&mut self) {}
}

impl PixelChannelType for u16 {
    const ZERO: Self = 0;

    #[inline(always)]
    fn into_float(self) -> f32 {
        unorm_u16_to_f32(self)
    }

    #[inline(always)]
    fn from_float(v: f32) -> Self {
        f32_to_unorm_u16(v)
    }

    #[inline(always)]
    fn to_le(&mut self) {
        if !cfg!(target_endian = "big") {
            return;
        }
        *self = bytemuck::cast::<_, Self>(self.to_le_bytes());
    }
}

impl PixelChannelType for u32 {
    const ZERO: Self = 0;

    #[inline(always)]
    fn into_float(self) -> f32 {
        // // TODO: this likely sucks
        // let v = self as f32;
        // let v = v / (u32::MAX as f32);
        // v
        let v = self / 256; // Compress into 24 bits
        let v = v as f32 / 16777216.0; // Convert to float and normalize
        v
    }

    #[inline(always)]
    fn from_float(v: f32) -> Self {
        // // TODO: this likely sucks
        // let v = v * (u32::MAX as f32);
        // let v = v as u32;
        // v
        let v = v * 16777216.0;
        let v = v as u32 * 256;
        v
    }

    #[inline(always)]
    fn to_le(&mut self) {
        if !cfg!(target_endian = "big") {
            return;
        }
        *self = bytemuck::cast::<_, Self>(self.to_le_bytes());
    }
}

impl PixelChannelType for f16 {
    const ZERO: Self = f16::from_f32_const(0.0);

    #[inline(always)]
    fn into_float(self) -> f32 {
        f16::to_f32(self)
    }

    #[inline(always)]
    fn from_float(v: f32) -> Self {
        f16::from_f32(v)
    }

    #[inline(always)]
    fn to_le(&mut self) {
        *self = bytemuck::cast::<_, Self>(self.to_le_bytes());
    }
}

impl PixelChannelType for f32 {
    const ZERO: Self = 0.0;

    #[inline(always)]
    fn into_float(self) -> f32 {
        self
    }

    #[inline(always)]
    fn from_float(v: f32) -> Self {
        v
    }

    #[inline(always)]
    fn to_le(&mut self) {
        if !cfg!(target_endian = "big") {
            return;
        }
        *self = bytemuck::cast::<_, Self>(self.to_le_bytes());
    }
}
