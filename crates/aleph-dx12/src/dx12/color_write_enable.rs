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

use windows_raw::Win32::Direct3D12::D3D12_COLOR_WRITE_ENABLE;

#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Debug, Hash)]
pub struct ColorWriteEnable(pub u8);

impl ColorWriteEnable {
    pub const NONE: Self = Self(0);
    pub const RED: Self = Self(1);
    pub const GREEN: Self = Self(2);
    pub const BLUE: Self = Self(4);
    pub const ALPHA: Self = Self(8);

    #[inline]
    pub fn all() -> Self {
        Self::RED | Self::BLUE | Self::GREEN | Self::ALPHA
    }
}

impl Default for ColorWriteEnable {
    #[inline]
    fn default() -> Self {
        Self::NONE
    }
}

windows_raw::flags_bitwise_impl!(ColorWriteEnable);

impl Into<D3D12_COLOR_WRITE_ENABLE> for ColorWriteEnable {
    #[inline]
    fn into(self) -> D3D12_COLOR_WRITE_ENABLE {
        D3D12_COLOR_WRITE_ENABLE(self.0 as i32)
    }
}
