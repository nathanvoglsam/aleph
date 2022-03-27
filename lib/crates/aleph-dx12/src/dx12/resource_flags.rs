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

use windows::Win32::Graphics::Direct3D12::D3D12_RESOURCE_FLAGS;

#[repr(transparent)]
#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Debug, Hash)]
pub struct ResourceFlags(pub u32);

impl ResourceFlags {
    pub const NONE: Self = Self(0);
    pub const ALLOW_RENDER_TARGET: Self = Self(1);
    pub const ALLOW_DEPTH_STENCIL: Self = Self(2);
    pub const ALLOW_UNORDERED_ACCESS: Self = Self(4);
    pub const DENY_SHADER_RESOURCE: Self = Self(8);
    pub const ALLOW_CROSS_ADAPTER: Self = Self(16);
    pub const ALLOW_SIMULTANEOUS_ACCESS: Self = Self(32);
    pub const VIDEO_DECODE_REFERENCE_ONLY: Self = Self(64);
}

impl Default for ResourceFlags {
    #[inline]
    fn default() -> Self {
        Self::NONE
    }
}

windows::flags_bitwise_impl!(ResourceFlags);

impl From<ResourceFlags> for D3D12_RESOURCE_FLAGS {
    #[inline]
    fn from(v: ResourceFlags) -> Self {
        D3D12_RESOURCE_FLAGS(v.0)
    }
}
