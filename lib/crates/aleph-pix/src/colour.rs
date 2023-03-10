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

/// A wrapper over the PIX colour format.
///
/// PIX uses a `u64` in the ABI for colour values, but it only uses the low 32 bits to encode an
/// ARGB format colour with 8-bits per channel stored as unsigned-normalized bytes.
///
/// While an alpha channel is provided (the format *is* ARGB), it must always be set to 255.
///
/// The high bytes should always be initialized to 0.
#[repr(transparent)]
#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Debug, Hash)]
pub struct Colour(u64);

impl Colour {
    /// Produces a new [Colour] instance based on the given RGB values. The colour values are UNORM
    /// values, so 0-255 maps to 0-1.
    #[inline]
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        let r = (r as u64) << 16;
        let g = (g as u64) << 8;
        let b = b as u64;
        let a = 0xFF000000u64;
        Self(r | g | b | a)
    }
}

impl Colour {
    pub const RED: Self = Self(0xFFFF0000);
    pub const GREEN: Self = Self(0xFF00FF00);
    pub const BLUE: Self = Self(0xFF0000FF);
    pub const YELLOW: Self = Self(0xFFFFFF00);
    pub const MAGENTA: Self = Self(0xFFFF00FF);
    pub const CYAN: Self = Self(0xFF00FFFF);
    pub const WHITE: Self = Self(0xFFFFFFFF);
}

impl From<u64> for Colour {
    #[inline(always)]
    fn from(v: u64) -> Self {
        Self(v)
    }
}

impl From<Colour> for u64 {
    #[inline(always)]
    fn from(v: Colour) -> Self {
        v.0
    }
}
