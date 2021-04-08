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

use windows_raw::Win32::Direct3D12::D3D12_HEAP_FLAGS;

#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Debug, Hash)]
pub struct HeapFlags(pub i32);

impl HeapFlags {
    pub const NONE: Self = Self(0i32);
    pub const SHARED: Self = Self(1i32);
    pub const DENY_BUFFERS: Self = Self(4i32);
    pub const ALLOW_DISPLAY: Self = Self(8i32);
    pub const SHARED_CROSS_ADAPTER: Self = Self(32i32);
    pub const DENY_RT_DS_TEXTURES: Self = Self(64i32);
    pub const DENY_NON_RT_DS_TEXTURES: Self = Self(128i32);
    pub const HARDWARE_PROTECTED: Self = Self(256i32);
    pub const ALLOW_WRITE_WATCH: Self = Self(512i32);
    pub const ALLOW_SHADER_ATOMICS: Self = Self(1024i32);
    pub const CREATE_NOT_RESIDENT: Self = Self(2048i32);
    pub const CREATE_NOT_ZEROED: Self = Self(4096i32);
    pub const ALLOW_ALL_BUFFERS_AND_TEXTURES: Self = Self(0i32);
    pub const ALLOW_ONLY_BUFFERS: Self = Self(192i32);
    pub const ALLOW_ONLY_NON_RT_DS_TEXTURES: Self = Self(68i32);
    pub const ALLOW_ONLY_RT_DS_TEXTURES: Self = Self(132i32);
}

impl Default for HeapFlags {
    #[inline]
    fn default() -> Self {
        Self::NONE
    }
}

windows_raw::flags_bitwise_impl!(HeapFlags);

impl Into<D3D12_HEAP_FLAGS> for HeapFlags {
    #[inline]
    fn into(self) -> D3D12_HEAP_FLAGS {
        D3D12_HEAP_FLAGS(self.0)
    }
}
