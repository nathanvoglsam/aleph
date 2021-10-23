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

use windows_raw::Win32::Direct3D12::D3D12_RESOURCE_STATES;

#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Debug, Hash)]
#[repr(transparent)]
pub struct ResourceStates(pub i32);

impl ResourceStates {
    // ----
    // PURE FLAGS
    // ----

    pub const COMMON: Self = Self(0);
    pub const VERTEX_AND_CONSTANT_BUFFER: Self = Self(0x1);
    pub const INDEX_BUFFER: Self = Self(0x2);
    pub const RENDER_TARGET: Self = Self(0x4);
    pub const UNORDERED_ACCESS: Self = Self(0x8);
    pub const DEPTH_WRITE: Self = Self(0x10);
    pub const DEPTH_READ: Self = Self(0x20);
    pub const NON_PIXEL_SHADER_RESOURCE: Self = Self(0x40);
    pub const PIXEL_SHADER_RESOURCE: Self = Self(0x80);
    pub const STREAM_OUT: Self = Self(0x100);
    pub const INDIRECT_ARGUMENT: Self = Self(0x200);
    pub const COPY_DEST: Self = Self(0x400);
    pub const COPY_SOURCE: Self = Self(0x800);
    pub const RESOLVE_DEST: Self = Self(0x1000);
    pub const RESOLVE_SOURCE: Self = Self(0x2000);
    pub const RAYTRACING_ACCELERATION_STRUCTURE: Self = Self(0x400000);
    pub const SHADING_RATE_SOURCE: Self = Self(0x1000000);
    pub const VIDEO_DECODE_READ: Self = Self(0x10000);
    pub const VIDEO_DECODE_WRITE: Self = Self(0x20000);
    pub const VIDEO_PROCESS_READ: Self = Self(0x40000);
    pub const VIDEO_PROCESS_WRITE: Self = Self(0x80000);
    pub const VIDEO_ENCODE_READ: Self = Self(0x200000);
    pub const VIDEO_ENCODE_WRITE: Self = Self(0x800000);

    // ----
    // ALIAS FLAGS
    // ----

    pub const PRESENT: Self = Self::COMMON;
    pub const PREDICATION: Self = Self::INDIRECT_ARGUMENT;

    // ----
    // COMPOSITE FLAGS
    // ----

    pub const GENERIC_READ: Self = Self(
        Self::VERTEX_AND_CONSTANT_BUFFER.0
            | Self::INDEX_BUFFER.0
            | Self::NON_PIXEL_SHADER_RESOURCE.0
            | Self::PIXEL_SHADER_RESOURCE.0
            | Self::INDIRECT_ARGUMENT.0
            | Self::COPY_SOURCE.0,
    );
    pub const ALL_SHADER_RESOURCE: Self =
        Self(Self::NON_PIXEL_SHADER_RESOURCE.0 | Self::PIXEL_SHADER_RESOURCE.0);

    pub const ALL_READ_ONLY: Self = Self(
        Self::VERTEX_AND_CONSTANT_BUFFER.0
            | Self::INDEX_BUFFER.0
            | Self::DEPTH_READ.0
            | Self::NON_PIXEL_SHADER_RESOURCE.0
            | Self::PIXEL_SHADER_RESOURCE.0
            | Self::INDIRECT_ARGUMENT.0
            | Self::COPY_SOURCE.0
            | Self::RESOLVE_SOURCE.0
            | Self::SHADING_RATE_SOURCE.0
            | Self::VIDEO_DECODE_READ.0
            | Self::VIDEO_ENCODE_READ.0
            | Self::VIDEO_PROCESS_READ.0,
    );

    pub const ALL_WRITES_ONLY: Self = Self(
        Self::RENDER_TARGET.0
            | Self::STREAM_OUT.0
            | Self::COPY_DEST.0
            | Self::RESOLVE_DEST.0
            | Self::DEPTH_WRITE.0
            | Self::VIDEO_DECODE_WRITE.0
            | Self::VIDEO_ENCODE_WRITE.0
            | Self::VIDEO_PROCESS_WRITE.0,
    );

    pub const ALL_READ_WRITES: Self =
        Self(Self::UNORDERED_ACCESS.0 | Self::RAYTRACING_ACCELERATION_STRUCTURE.0);

    pub const TEXTURE_USAGES: Self = Self(
        Self::RENDER_TARGET.0
            | Self::UNORDERED_ACCESS.0
            | Self::DEPTH_WRITE.0
            | Self::DEPTH_READ.0
            | Self::NON_PIXEL_SHADER_RESOURCE.0
            | Self::PIXEL_SHADER_RESOURCE.0
            | Self::COPY_DEST.0
            | Self::COPY_SOURCE.0
            | Self::RESOLVE_SOURCE.0
            | Self::RESOLVE_DEST.0
            | Self::SHADING_RATE_SOURCE.0,
    );

    pub const BUFFER_USAGES: Self = Self(
        Self::VERTEX_AND_CONSTANT_BUFFER.0
            | Self::INDEX_BUFFER.0
            | Self::UNORDERED_ACCESS.0
            | Self::NON_PIXEL_SHADER_RESOURCE.0
            | Self::PIXEL_SHADER_RESOURCE.0
            | Self::STREAM_OUT.0
            | Self::INDIRECT_ARGUMENT.0
            | Self::COPY_DEST.0
            | Self::COPY_SOURCE.0
            | Self::RAYTRACING_ACCELERATION_STRUCTURE.0
            | Self::PREDICATION.0,
    );
}

impl ResourceStates {
    /// Returns whether all of the states that are enabled in self are compatible with a texture
    /// resource
    #[inline]
    pub fn is_texture_compatible(&self) -> bool {
        self.is_subset_of(&Self::TEXTURE_USAGES)
    }

    /// Returns whether all of the states that are enabled in self are compatible with a buffer
    /// resource
    #[inline]
    pub fn is_buffer_compatible(&self) -> bool {
        self.is_subset_of(&Self::BUFFER_USAGES)
    }

    /// Returns whether the state of `self` represent a purely read-only resource state.
    #[inline]
    pub fn is_read_only(&self) -> bool {
        self.is_subset_of(&Self::ALL_READ_ONLY)
    }

    /// Returns whether the state of `self` would allow read access to the resource. This does not
    /// mean the access is read-only, use [`ResourceStates::is_read_only`] to detect read-only
    /// states.
    #[inline]
    pub fn is_read(&self) -> bool {
        self.intersects(&Self::ALL_READ_ONLY) || self.intersects(&Self::ALL_READ_WRITES)
    }

    /// Returns whether the state of `self` allows both read and write access to the resource. This
    /// will check if the state is composed of a combination of read-only states and write-only
    /// states or if the state is one of the read-write states (i.e UNORDERED_ACCESS).
    #[inline]
    pub fn is_read_write(&self) -> bool {
        self.intersects(&Self::ALL_READ_WRITES)
            || (self.intersects(&Self::ALL_WRITES_ONLY) && self.intersects(&Self::ALL_READ_ONLY))
    }

    /// Returns whether the state of `self` represent a purely write-only resource state.
    #[inline]
    pub fn is_write_only(&self) -> bool {
        self.is_subset_of(&Self::ALL_WRITES_ONLY)
    }

    /// Returns whether the state of `self` would allow write access to the resource. This does not
    /// mean the access is write-only, use [`ResourceStates::is_write_only`] to detect write-only
    /// states.
    #[inline]
    pub fn is_write(&self) -> bool {
        self.intersects(&Self::ALL_WRITES_ONLY) || self.intersects(&Self::ALL_READ_WRITES)
    }
}

impl Default for ResourceStates {
    #[inline]
    fn default() -> Self {
        Self::COMMON
    }
}

windows_raw::flags_bitwise_impl!(ResourceStates);

impl Into<D3D12_RESOURCE_STATES> for ResourceStates {
    #[inline]
    fn into(self) -> D3D12_RESOURCE_STATES {
        D3D12_RESOURCE_STATES(self.0)
    }
}
