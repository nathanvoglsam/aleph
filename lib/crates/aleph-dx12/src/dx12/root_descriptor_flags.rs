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

use windows::Win32::Graphics::Direct3D12::D3D12_ROOT_DESCRIPTOR_FLAGS;

#[repr(transparent)]
#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Debug, Hash)]
pub struct RootDescriptorFlags(pub u32);

impl RootDescriptorFlags {
    pub const NONE: Self = Self(0);
    pub const DATA_VOLATILE: Self = Self(2);
    pub const DATA_STATIC_WHILE_SET_AT_EXECUTE: Self = Self(4);
    pub const DATA_STATIC: Self = Self(8);
}

impl Default for RootDescriptorFlags {
    #[inline]
    fn default() -> Self {
        Self::NONE
    }
}

windows::flags_bitwise_impl!(RootDescriptorFlags);

impl From<RootDescriptorFlags> for D3D12_ROOT_DESCRIPTOR_FLAGS {
    #[inline]
    fn from(v: RootDescriptorFlags) -> Self {
        v.0
    }
}
