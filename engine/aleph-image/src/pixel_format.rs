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

#[derive(Copy, Clone, Zeroable, Pod)]
#[repr(transparent)]
pub struct PixR<T: PixelChannelType>([T; 1]);

#[derive(Copy, Clone, Zeroable, Pod)]
#[repr(transparent)]
pub struct PixRG<T: PixelChannelType>([T; 2]);

#[derive(Copy, Clone, Zeroable, Pod)]
#[repr(transparent)]
pub struct PixRGB<T: PixelChannelType>([T; 3]);

#[derive(Copy, Clone, Zeroable, Pod)]
#[repr(transparent)]
pub struct PixRGBA<T: PixelChannelType>([T; 4]);

pub trait PixelFormat {
    type Storage: PixelChannelType;

    const COMPONENTS: usize;

    fn from_storage(v: &[Self::Storage]) -> Self;

    fn write_at(&self, v: &mut [Self::Storage]);

    fn as_vec4(&self) -> Vec4 {
        self.as_vec4_with_default(0.0)
    }

    fn as_vec4_with_default(&self, default: f32) -> Vec4;

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

pub trait PixelChannelType:
    Copy + Clone + PartialEq + PartialOrd + Default + Sized + Pod + AnyBitPattern + NoUninit
{
    fn into_float(self) -> f32;

    fn from_float(v: f32) -> Self;

    fn to_le(&mut self);
}

impl PixelChannelType for u8 {
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
