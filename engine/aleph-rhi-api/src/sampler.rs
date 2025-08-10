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

use aleph_object_system::ArcObject;

use crate::*;

#[derive(Clone)]
pub struct SamplerHandle {
    inner: ArcObject,
}

impl SamplerHandle {
    /// # Safety
    ///
    /// It is the caller's responsibility to ensure that the given object refers to an object that
    /// the inner RHI implementation considers a semaphore objec.
    pub const unsafe fn new(inner: ArcObject) -> Self {
        Self { inner }
    }

    ///
    /// Gets the number of strong ([`SamplerHandle`]) pointers to this allocation.
    ///
    /// # Safety
    ///
    /// This method by itself is safe, but using it correctly requires extra care.
    /// Another thread can change the strong count at any time,
    /// including potentially between calling this method and acting on the result.
    ///
    /// # Info
    ///
    /// This is just a wrapper around [`std::sync::Arc::strong_count`]
    ///
    #[inline]
    #[must_use]
    pub fn strong_count(&self) -> usize {
        self.inner.strong_count()
    }

    /// Unwrap the [`SamplerHandle`] and get the inner [`ArcObject`]
    #[inline]
    pub fn into_inner(self) -> ArcObject {
        self.inner
    }

    /// Get the inner [`ArcObject`]
    pub const fn get(&self) -> &ArcObject {
        &self.inner
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum SamplerAddressMode {
    Wrap,
    Mirror,
    Clamp,
    Border,
    MirrorOnce,
}

impl std::fmt::Display for SamplerAddressMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SamplerAddressMode::Wrap => f.write_str("Wrap"),
            SamplerAddressMode::Mirror => f.write_str("Mirror"),
            SamplerAddressMode::Clamp => f.write_str("Clamp"),
            SamplerAddressMode::Border => f.write_str("Border"),
            SamplerAddressMode::MirrorOnce => f.write_str("MirrorOnce"),
        }
    }
}

impl Default for SamplerAddressMode {
    #[inline(always)]
    fn default() -> Self {
        Self::Wrap
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum SamplerFilter {
    Nearest,
    Linear,
}

impl std::fmt::Display for SamplerFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SamplerFilter::Nearest => f.write_str("Nearest"),
            SamplerFilter::Linear => f.write_str("Linear"),
        }
    }
}

impl Default for SamplerFilter {
    #[inline(always)]
    fn default() -> Self {
        Self::Nearest
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum SamplerMipFilter {
    Nearest,
    Linear,
}

impl std::fmt::Display for SamplerMipFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SamplerMipFilter::Nearest => f.write_str("Nearest"),
            SamplerMipFilter::Linear => f.write_str("Linear"),
        }
    }
}

impl Default for SamplerMipFilter {
    #[inline(always)]
    fn default() -> Self {
        Self::Nearest
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum SamplerBorderColor {
    BlackTransparent,
    BlackOpaque,
    WhiteOpaque,
}

impl std::fmt::Display for SamplerBorderColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SamplerBorderColor::BlackTransparent => f.write_str("BlackTransparent"),
            SamplerBorderColor::BlackOpaque => f.write_str("BlackOpaque"),
            SamplerBorderColor::WhiteOpaque => f.write_str("WhiteOpaque"),
        }
    }
}

impl Default for SamplerBorderColor {
    #[inline(always)]
    fn default() -> Self {
        Self::BlackTransparent
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SamplerDesc<'a> {
    pub min_filter: SamplerFilter,
    pub mag_filter: SamplerFilter,
    pub mip_filter: SamplerMipFilter,
    pub address_mode_u: SamplerAddressMode,
    pub address_mode_v: SamplerAddressMode,
    pub address_mode_w: SamplerAddressMode,
    pub lod_bias: f32,
    pub min_lod: f32,
    pub max_lod: f32,
    pub enable_anisotropy: bool,
    pub max_anisotropy: u32,
    pub compare_op: Option<CompareOp>,
    pub border_color: SamplerBorderColor,

    /// The name of the object
    pub name: Option<&'a str>,
}

impl<'a> Default for SamplerDesc<'a> {
    #[inline]
    fn default() -> Self {
        Self {
            min_filter: SamplerFilter::Linear,
            mag_filter: SamplerFilter::Linear,
            mip_filter: SamplerMipFilter::Linear,
            address_mode_u: SamplerAddressMode::Clamp,
            address_mode_v: SamplerAddressMode::Clamp,
            address_mode_w: SamplerAddressMode::Clamp,
            lod_bias: 0.0,
            min_lod: 0.0,
            max_lod: 1000.0,
            enable_anisotropy: false,
            max_anisotropy: 0,
            compare_op: Default::default(),
            border_color: Default::default(),
            name: None,
        }
    }
}

impl<'a> SamplerDesc<'a> {
    /// A utility function that strips the debug name from the description so we can get a static
    /// lifetime on the desc
    pub const fn strip_name(self) -> SamplerDesc<'static> {
        SamplerDesc::<'static> {
            min_filter: self.min_filter,
            mag_filter: self.mag_filter,
            mip_filter: self.mip_filter,
            address_mode_u: self.address_mode_u,
            address_mode_v: self.address_mode_v,
            address_mode_w: self.address_mode_w,
            lod_bias: self.lod_bias,
            min_lod: self.min_lod,
            max_lod: self.max_lod,
            enable_anisotropy: self.enable_anisotropy,
            max_anisotropy: self.max_anisotropy,
            compare_op: self.compare_op,
            border_color: self.border_color,
            name: None,
        }
    }
}
