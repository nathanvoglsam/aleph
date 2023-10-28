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

//!
//! This module contains a bunch of internal only structs that should never be exported from the
//! API. These are collected inside this 'internal' module
//!

use crate::resource::ResourceId;
use crate::IRenderPass;
use aleph_any::AnyArc;
use aleph_rhi_api::*;
use std::ptr::NonNull;

pub struct RenderPass {
    pub pass: NonNull<dyn IRenderPass>,
    pub name: NonNull<str>,
}

#[derive(Default)]
pub struct ResourceRoot {
    /// The type of this resource
    pub resource_type: ResourceType,

    /// The accumulated usage flags for a texture resource. This is the union of all the ways a
    /// resource is used as within the frame graph.
    pub usage_tex: TextureUsageFlags,

    /// The accumulated usage flags for a buffer resource. This is the union of all the ways a
    /// resource is used as within the frame graph.
    pub usage_buf: BufferUsageFlags,
}

pub enum ResourceType {
    Uninitialized,
    Buffer {
        import_info: Option<ImportedBuffer>,
        create_desc: BufferCreate,
    },
    Texture {
        import_info: Option<ImportedTexture>,
        create_desc: TextureCreate,
    },
}

impl Default for ResourceType {
    fn default() -> Self {
        ResourceType::Uninitialized
    }
}

pub struct ImportedBuffer {
    pub resource: AnyArc<dyn IBuffer>,
    pub before_sync: BarrierSync,
    pub before_usage: BufferUsageFlags,
    pub after_sync: BarrierSync,
    pub after_usage: BufferUsageFlags,
}

pub struct ImportedTexture {
    pub resource: AnyArc<dyn ITexture>,
    pub before_sync: BarrierSync,
    pub before_usage: TextureUsageFlags,
    pub before_layout: ImageLayout,
    pub after_sync: BarrierSync,
    pub after_usage: TextureUsageFlags,
    pub after_layout: ImageLayout,
}

#[derive(Default)]
pub struct BufferCreate {
    /// The size of the buffer to be created
    pub size: u64,

    // Implicitly GPU only
    // pub cpu_access: CpuAccessMode,
    /// The immediate usage flags that the buffer is requested to be used as. This will not be
    /// an exhaustive list of all usage flags. When collected from a pass this only contains the
    /// immediate use needed for the creating pass. The full set of usage flags will be defined by
    /// all usages within the graph
    pub usage: BufferUsageFlags,

    /// The name of the resource. This is a pointer to a region within the main frame graph arena
    /// that the passes are stored in. It is only sound to access this string immutably, and the
    /// caller must ensure the relevant arena is still live.
    pub name: Option<NonNull<str>>,
}

#[derive(Default)]
pub struct TextureCreate {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub format: Format,
    pub dimension: TextureDimension,
    pub clear_value: Option<OptimalClearValue>,
    pub array_size: u32,
    pub mip_levels: u32,
    pub sample_count: u32,
    pub sample_quality: u32,

    /// The immediate usage flags that the texture is requested to be used as. This will not be
    /// an exhaustive list of all usage flags. When collected from a pass this only contains the
    /// immediate use needed for the creating pass. The full set of usage flags will be defined by
    /// all usages within the graph
    pub usage: TextureUsageFlags,

    /// The name of the resource. This is a pointer to a region within the main frame graph arena
    /// that the passes are stored in. It is only sound to access this string immutably, and the
    /// caller must ensure the relevant arena is still live.
    pub name: Option<NonNull<str>>,
}

#[derive(Default)]
pub struct PassAccessInfo {
    pub current_pass_index: usize,
    pub reads: Vec<ResourceAccess>,
    pub writes: Vec<ResourceAccess>,
}

impl PassAccessInfo {
    pub fn clear(&mut self) {
        self.reads.clear();
        self.writes.clear();
    }
}

pub enum ResourceAccess {
    Buffer(BufferAccess),
    Texture(TextureAccess),
}

/// Stores the requested access for a single buffer access edge. Could be read or write, depending
/// on the flags stored inside.
pub struct BufferAccess {
    /// The destructured resource ID. ResourceRef/ResourceMut is for the external API
    pub buffer: ResourceId,

    /// Pipeline stage/stages the buffer will be used in
    pub sync: BarrierSync,

    /// The ways the buffer will be accessed
    pub usage: BufferUsageFlags,
}

/// Stores the requested access for a single texture access edge. Could be a read or a write,
/// depending on the flags stored inside.
pub struct TextureAccess {
    /// The destructured resource ID. ResourceRef/ResourceMut is for the external API
    pub texture: ResourceId,

    /// Pipeline stage/stages the texture will be used in
    pub sync: BarrierSync,

    /// The ways the texture will be accessed
    pub usage: TextureUsageFlags,

    /// The image layout the texture needs to be in for the registering pass
    pub layout: ImageLayout,
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Default, Debug)]
pub struct VersionIndex(pub u16);

impl VersionIndex {
    pub const INVALID: Self = Self(u16::MAX);

    pub const fn new(v: u16) -> Option<VersionIndex> {
        let v = Self(v);
        if v.is_valid() {
            Some(v)
        } else {
            None
        }
    }

    pub const fn is_valid(&self) -> bool {
        self.0 != Self::INVALID.0
    }
}

pub struct ResourceVersion {
    /// The index of the root resource this [ResourceVersion] encodes a version of. This allows
    /// easily mapping any version back to the underlying resource it represents a view of.
    ///
    /// This is critical for allowing iterating the version array to accumulate information about
    /// the graph.
    pub root_resource: u16,

    /// The index of the previous version of the resource in the version array. Can be
    /// VersionIndex::INVALID when there is no previous version.
    ///
    /// Starting from the final state of a resource this encodes a linked list of all the versions
    /// of a resource within the graph.
    pub previous: VersionIndex,

    /// The union of all the ways this particular version of the resource is used. This is the OR of
    /// all the flags declared by the write that creates this version and all the reads of this
    /// version of th root resource.
    pub usage_tex: TextureUsageFlags,

    /// See usage_tex. This is the same but for buffer usages.
    pub usage_buf: BufferUsageFlags,

    /// The index of the render pass that caused the new resource version to be created. This could
    /// be through creating a new transient resource or through writing an existing resource.
    pub render_pass: usize,
}

/// An internal struct used for debug information about individual generated resource handles.
///
/// This information is vestigial once the graph is constructed so it can be discarded.
#[derive(Default)]
pub struct ResourceHandleInfo {
    /// Flags whether the resource has been written
    pub written: bool,
}

impl ResourceHandleInfo {
    pub fn mark_written(&mut self) {
        self.written = true;
    }

    pub fn is_written(&self) -> bool {
        self.written
    }
}