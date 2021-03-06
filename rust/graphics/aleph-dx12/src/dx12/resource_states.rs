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

use windows_raw::win32::direct3d12::D3D12_RESOURCE_STATES;

#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Debug, Hash)]
#[repr(transparent)]
pub struct ResourceStates(pub i32);

impl ResourceStates {
    pub const COMMON: Self = Self(0i32);
    pub const VERTEX_AND_CONSTANT_BUFFER: Self = Self(1i32);
    pub const INDEX_BUFFER: Self = Self(2i32);
    pub const RENDER_TARGET: Self = Self(4i32);
    pub const UNORDERED_ACCESS: Self = Self(8i32);
    pub const DEPTH_WRITE: Self = Self(16i32);
    pub const DEPTH_READ: Self = Self(32i32);
    pub const NON_PIXEL_SHADER_RESOURCE: Self = Self(64i32);
    pub const PIXEL_SHADER_RESOURCE: Self = Self(128i32);
    pub const STREAM_OUT: Self = Self(256i32);
    pub const INDIRECT_ARGUMENT: Self = Self(512i32);
    pub const COPY_DEST: Self = Self(1024i32);
    pub const COPY_SOURCE: Self = Self(2048i32);
    pub const RESOLVE_DEST: Self = Self(4096i32);
    pub const RESOLVE_SOURCE: Self = Self(8192i32);
    pub const RAYTRACING_ACCELERATION_STRUCTURE: Self = Self(4194304i32);
    pub const SHADING_RATE_SOURCE: Self = Self(16777216i32);
    pub const GENERIC_READ: Self = Self(2755i32);
    pub const PRESENT: Self = Self(0i32);
    pub const PREDICATION: Self = Self(512i32);
    pub const VIDEO_DECODE_READ: Self = Self(65536i32);
    pub const VIDEO_DECODE_WRITE: Self = Self(131072i32);
    pub const VIDEO_PROCESS_READ: Self = Self(262144i32);
    pub const VIDEO_PROCESS_WRITE: Self = Self(524288i32);
    pub const VIDEO_ENCODE_READ: Self = Self(2097152i32);
    pub const VIDEO_ENCODE_WRITE: Self = Self(8388608i32);
}

impl Default for ResourceStates {
    fn default() -> Self {
        Self::COMMON
    }
}

crate::flags_bitwise_impl!(ResourceStates);

impl Into<D3D12_RESOURCE_STATES> for ResourceStates {
    fn into(self) -> D3D12_RESOURCE_STATES {
        D3D12_RESOURCE_STATES(self.0)
    }
}
