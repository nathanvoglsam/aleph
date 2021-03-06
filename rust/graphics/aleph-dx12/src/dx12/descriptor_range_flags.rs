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

use windows_raw::win32::direct3d12::D3D12_DESCRIPTOR_RANGE_FLAGS;

#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Debug, Hash)]
pub struct DescriptorRangeFlags(pub i32);

impl DescriptorRangeFlags {
    pub const NONE: Self = Self(0i32);
    pub const DESCRIPTORS_VOLATILE: Self = Self(1i32);
    pub const DATA_VOLATILE: Self = Self(2i32);
    pub const DATA_STATIC_WHILE_SET_AT_EXECUTE: Self = Self(4i32);
    pub const DATA_STATIC: Self = Self(8i32);
    pub const DESCRIPTORS_STATIC_KEEPING_BUFFER_BOUNDS_CHECKS: Self = Self(65536i32);
}

impl Default for DescriptorRangeFlags {
    fn default() -> Self {
        Self::NONE
    }
}

windows_raw::flags_bitwise_impl!(DescriptorRangeFlags);

impl Into<D3D12_DESCRIPTOR_RANGE_FLAGS> for DescriptorRangeFlags {
    fn into(self) -> D3D12_DESCRIPTOR_RANGE_FLAGS {
        D3D12_DESCRIPTOR_RANGE_FLAGS(self.0)
    }
}
