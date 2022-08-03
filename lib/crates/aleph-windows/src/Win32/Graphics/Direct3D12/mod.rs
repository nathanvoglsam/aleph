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

use bitflags::bitflags;
use std::ops::Deref;
use windows::Win32::Foundation::BOOL;
pub use windows::Win32::Graphics::Direct3D12::*;

pub const D3D12_FEATURE_D3D12_OPTIONS12: D3D12_FEATURE = D3D12_FEATURE(41i32);

#[repr(transparent)]
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct D3D12_TRI_STATE(pub i32);

impl D3D12_TRI_STATE {
    pub const UNKNOWN: Self = Self(-1);
    pub const FALSE: Self = Self(0);
    pub const TRUE: Self = Self(1);
}

#[repr(C)]
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct D3D12_FEATURE_DATA_D3D12_OPTIONS12 {
    pub MSPrimitivesPipelineStatisticIncludesCulledPrimitives: D3D12_TRI_STATE,
    pub EnhancedBarriersSupported: BOOL,
    pub RelaxedFormatCastingSupported: BOOL,
}

#[repr(transparent)]
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct D3D12_BARRIER_LAYOUT(pub u32);

impl D3D12_BARRIER_LAYOUT {
    pub const UNDEFINED: Self = Self(0xffffffff);
    pub const COMMON: Self = Self(0);
    pub const PRESENT: Self = Self(0);
    pub const GENERIC_READ: Self = Self(1);
    pub const RENDER_TARGET: Self = Self(2);
    pub const UNORDERED_ACCESS: Self = Self(3);
    pub const DEPTH_STENCIL_WRITE: Self = Self(4);
    pub const DEPTH_STENCIL_READ: Self = Self(5);
    pub const SHADER_RESOURCE: Self = Self(6);
    pub const COPY_SOURCE: Self = Self(7);
    pub const COPY_DEST: Self = Self(8);
    pub const RESOLVE_SOURCE: Self = Self(9);
    pub const RESOLVE_DEST: Self = Self(10);
    pub const SHADING_RATE_SOURCE: Self = Self(11);
    pub const VIDEO_DECODE_READ: Self = Self(12);
    pub const VIDEO_DECODE_WRITE: Self = Self(13);
    pub const VIDEO_PROCESS_READ: Self = Self(14);
    pub const VIDEO_PROCESS_WRITE: Self = Self(15);
    pub const VIDEO_ENCODE_READ: Self = Self(16);
    pub const VIDEO_ENCODE_WRITE: Self = Self(17);
    pub const DIRECT_QUEUE_COMMON: Self = Self(18);
    pub const DIRECT_QUEUE_GENERIC_READ: Self = Self(19);
    pub const DIRECT_QUEUE_UNORDERED_ACCESS: Self = Self(20);
    pub const DIRECT_QUEUE_SHADER_RESOURCE: Self = Self(21);
    pub const DIRECT_QUEUE_COPY_SOURCE: Self = Self(22);
    pub const DIRECT_QUEUE_COPY_DEST: Self = Self(23);
    pub const COMPUTE_QUEUE_COMMON: Self = Self(24);
    pub const COMPUTE_QUEUE_GENERIC_READ: Self = Self(25);
    pub const COMPUTE_QUEUE_UNORDERED_ACCESS: Self = Self(26);
    pub const COMPUTE_QUEUE_SHADER_RESOURCE: Self = Self(27);
    pub const COMPUTE_QUEUE_COPY_SOURCE: Self = Self(28);
    pub const COMPUTE_QUEUE_COPY_DEST: Self = Self(29);
    pub const VIDEO_QUEUE_COMMON: Self = Self(30);
}

bitflags! {
    #[repr(transparent)]
    pub struct D3D12_BARRIER_SYNC: u32 {
        const NONE                                                     = 0x0;
        const ALL                                                      = 0x1;
        const DRAW                                                     = 0x2;
        const INPUT_ASSEMBLER                                          = 0x4;
        const VERTEX_SHADING                                           = 0x8;
        const PIXEL_SHADING                                            = 0x10;
        const DEPTH_STENCIL                                            = 0x20;
        const RENDER_TARGET                                            = 0x40;
        const COMPUTE_SHADING                                          = 0x80;
        const RAYTRACING                                               = 0x100;
        const COPY                                                     = 0x200;
        const RESOLVE                                                  = 0x400;
        const EXECUTE_INDIRECT                                         = 0x800;
        const PREDICATION                                              = 0x800;
        const ALL_SHADING                                              = 0x1000;
        const NON_PIXEL_SHADING                                        = 0x2000;
        const EMIT_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO    = 0x4000;
        const CLEAR_UNORDERED_ACCESS_VIEW                              = 0x8000;
        const VIDEO_DECODE                                             = 0x100000;
        const VIDEO_PROCESS                                            = 0x200000;
        const VIDEO_ENCODE                                             = 0x400000;
        const BUILD_RAYTRACING_ACCELERATION_STRUCTURE                  = 0x800000;
        const COPY_RAYTRACING_ACCELERATION_STRUCTURE                   = 0x1000000;
        const SPLIT                                                    = 0x80000000;
    }
}

bitflags! {
    #[repr(transparent)]
    pub struct D3D12_BARRIER_ACCESS: u32 {
        const COMMON                                     = 0x0;
        const VERTEX_BUFFER                              = 0x1;
        const CONSTANT_BUFFER                            = 0x2;
        const INDEX_BUFFER                               = 0x4;
        const RENDER_TARGET                              = 0x8;
        const UNORDERED_ACCESS                           = 0x10;
        const DEPTH_STENCIL_WRITE                        = 0x20;
        const DEPTH_STENCIL_READ                         = 0x40;
        const SHADER_RESOURCE                            = 0x80;
        const STREAM_OUTPUT                              = 0x100;
        const INDIRECT_ARGUMENT                          = 0x200;
        const PREDICATION                                = 0x200;
        const COPY_DEST                                  = 0x400;
        const COPY_SOURCE                                = 0x800;
        const RESOLVE_DEST                               = 0x1000;
        const RESOLVE_SOURCE                             = 0x2000;
        const RAYTRACING_ACCELERATION_STRUCTURE_READ     = 0x4000;
        const RAYTRACING_ACCELERATION_STRUCTURE_WRITE    = 0x8000;
        const SHADING_RATE_SOURCE                        = 0x10000;
        const VIDEO_DECODE_READ                          = 0x20000;
        const VIDEO_DECODE_WRITE                         = 0x40000;
        const VIDEO_PROCESS_READ                         = 0x80000;
        const VIDEO_PROCESS_WRITE                        = 0x100000;
        const VIDEO_ENCODE_READ                          = 0x200000;
        const VIDEO_ENCODE_WRITE                         = 0x400000;
        const NO_ACCESS                                  = 0x80000000;
    }
}

#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct D3D12_BARRIER_TYPE(pub u32);

impl D3D12_BARRIER_TYPE {
    pub const GLOBAL: Self = Self(0);
    pub const TEXTURE: Self = Self(1);
    pub const BUFFER: Self = Self(2);
}

bitflags! {
    #[repr(transparent)]
    pub struct D3D12_TEXTURE_BARRIER_FLAGS: u32 {
        const NONE = 0x0;
        const DISCARD = 0x1;
    }
}

#[repr(C)]
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct D3D12_BARRIER_SUBRESOURCE_RANGE {
    pub IndexOrFirstMipLevel: u32,
    pub NumMipLevels: u32,
    pub FirstArraySlice: u32,
    pub NumArraySlices: u32,
    pub FirstPlane: u32,
    pub NumPlanes: u32,
}

#[repr(C)]
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct D3D12_GLOBAL_BARRIER {
    pub SyncBefore: D3D12_BARRIER_SYNC,
    pub SyncAfter: D3D12_BARRIER_SYNC,
    pub AccessBefore: D3D12_BARRIER_ACCESS,
    pub AccessAfter: D3D12_BARRIER_ACCESS,
}

#[repr(C)]
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct D3D12_TEXTURE_BARRIER {
    pub SyncBefore: D3D12_BARRIER_SYNC,
    pub SyncAfter: D3D12_BARRIER_SYNC,
    pub AccessBefore: D3D12_BARRIER_ACCESS,
    pub AccessAfter: D3D12_BARRIER_ACCESS,
    pub LayoutBefore: D3D12_BARRIER_LAYOUT,
    pub LayoutAfter: D3D12_BARRIER_LAYOUT,
    pub pResource: Option<ID3D12Resource>,
    pub Subresources: D3D12_BARRIER_SUBRESOURCE_RANGE,
    pub Flags: D3D12_TEXTURE_BARRIER_FLAGS,
}

#[repr(C)]
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct D3D12_BUFFER_BARRIER {
    pub SyncBefore: D3D12_BARRIER_SYNC,
    pub SyncAfter: D3D12_BARRIER_SYNC,
    pub AccessBefore: D3D12_BARRIER_ACCESS,
    pub AccessAfter: D3D12_BARRIER_ACCESS,
    pub pResource: Option<ID3D12Resource>,
    pub Offset: u64,
    pub Size: u64,
}

#[repr(C)]
pub struct D3D12_BARRIER_GROUP {
    pub Type: D3D12_BARRIER_TYPE,
    pub NumBarriers: u32,
    pub Anonymous: D3D12_BARRIER_GROUP_0,
}

#[repr(C)]
pub union D3D12_BARRIER_GROUP_0 {
    pub pGlobalBarriers: *const D3D12_GLOBAL_BARRIER,
    pub pTextureBarriers: *const D3D12_TEXTURE_BARRIER,
    pub pBufferBarriers: *const D3D12_BUFFER_BARRIER,
}

type ID3D12GraphicsCommandList6Vtbl = ID3D12GraphicsCommandList6_Vtbl;

#[windows_interface::interface("DD171223-8B61-4769-90E3-160CCDE4E2C1")]
pub unsafe trait ID3D12GraphicsCommandList7: ID3D12GraphicsCommandList6 {
    fn Barrier(&self, numbarriergroups: u32, pbarriergroups: *const D3D12_BARRIER_GROUP);
}

impl Deref for ID3D12GraphicsCommandList7 {
    type Target = ID3D12GraphicsCommandList6;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<ID3D12GraphicsCommandList7> for ID3D12Object {
    fn from(value: ID3D12GraphicsCommandList7) -> Self {
        unsafe { core::mem::transmute(value) }
    }
}
impl From<&ID3D12GraphicsCommandList7> for ID3D12Object {
    fn from(value: &ID3D12GraphicsCommandList7) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl From<ID3D12GraphicsCommandList7> for ID3D12DeviceChild {
    fn from(value: ID3D12GraphicsCommandList7) -> Self {
        unsafe { core::mem::transmute(value) }
    }
}
impl From<&ID3D12GraphicsCommandList7> for ID3D12DeviceChild {
    fn from(value: &ID3D12GraphicsCommandList7) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl From<ID3D12GraphicsCommandList7> for ID3D12CommandList {
    fn from(value: ID3D12GraphicsCommandList7) -> Self {
        unsafe { core::mem::transmute(value) }
    }
}
impl From<&ID3D12GraphicsCommandList7> for ID3D12CommandList {
    fn from(value: &ID3D12GraphicsCommandList7) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl From<ID3D12GraphicsCommandList7> for ID3D12GraphicsCommandList {
    fn from(value: ID3D12GraphicsCommandList7) -> Self {
        unsafe { core::mem::transmute(value) }
    }
}
impl From<&ID3D12GraphicsCommandList7> for ID3D12GraphicsCommandList {
    fn from(value: &ID3D12GraphicsCommandList7) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl From<ID3D12GraphicsCommandList7> for ID3D12GraphicsCommandList1 {
    fn from(value: ID3D12GraphicsCommandList7) -> Self {
        unsafe { core::mem::transmute(value) }
    }
}
impl From<&ID3D12GraphicsCommandList7> for ID3D12GraphicsCommandList1 {
    fn from(value: &ID3D12GraphicsCommandList7) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl From<ID3D12GraphicsCommandList7> for ID3D12GraphicsCommandList2 {
    fn from(value: ID3D12GraphicsCommandList7) -> Self {
        unsafe { core::mem::transmute(value) }
    }
}
impl From<&ID3D12GraphicsCommandList7> for ID3D12GraphicsCommandList2 {
    fn from(value: &ID3D12GraphicsCommandList7) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl From<ID3D12GraphicsCommandList7> for ID3D12GraphicsCommandList3 {
    fn from(value: ID3D12GraphicsCommandList7) -> Self {
        unsafe { core::mem::transmute(value) }
    }
}
impl From<&ID3D12GraphicsCommandList7> for ID3D12GraphicsCommandList3 {
    fn from(value: &ID3D12GraphicsCommandList7) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl From<ID3D12GraphicsCommandList7> for ID3D12GraphicsCommandList4 {
    fn from(value: ID3D12GraphicsCommandList7) -> Self {
        unsafe { core::mem::transmute(value) }
    }
}
impl From<&ID3D12GraphicsCommandList7> for ID3D12GraphicsCommandList4 {
    fn from(value: &ID3D12GraphicsCommandList7) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl From<ID3D12GraphicsCommandList7> for ID3D12GraphicsCommandList5 {
    fn from(value: ID3D12GraphicsCommandList7) -> Self {
        unsafe { core::mem::transmute(value) }
    }
}
impl From<&ID3D12GraphicsCommandList7> for ID3D12GraphicsCommandList5 {
    fn from(value: &ID3D12GraphicsCommandList7) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl From<ID3D12GraphicsCommandList7> for ID3D12GraphicsCommandList6 {
    fn from(value: ID3D12GraphicsCommandList7) -> Self {
        unsafe { core::mem::transmute(value) }
    }
}
impl From<&ID3D12GraphicsCommandList7> for ID3D12GraphicsCommandList6 {
    fn from(value: &ID3D12GraphicsCommandList7) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
