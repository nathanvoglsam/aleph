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
use aleph_rhi_api::*;
use std::ptr::NonNull;

pub struct RenderPass {
    pub pass: NonNull<dyn IRenderPass>,
    pub name: NonNull<str>,
    pub reads: NonNull<[ResourceAccess]>,
    pub writes: NonNull<[ResourceAccess]>,
}

pub struct ResourceRoot {
    /// The type of this resource
    pub resource_type: ResourceType,

    /// The sync flags that the resource will be used with in the creating pass.
    pub creator_sync: BarrierSync,

    /// How the resource will be accessed within the render pass that creates the resource.
    pub creator_access: ResourceUsageFlags,

    /// The accumulated access flags for a resource. This is the union of all the ways a
    /// resource is used as within the frame graph.
    pub total_access_flags: ResourceUsageFlags,

    /// An index to the first version of this resource
    pub initial_version: VersionIndex,
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
    pub previous_version: VersionIndex,

    /// The union of all the ways this particular version of the resource is used. This is the OR of
    /// all the flags declared by the write that creates this version and all the reads of this
    /// version of the resource.
    pub version_total_access: ResourceUsageFlags,

    /// The index of the render pass that caused the new resource version to be created. This could
    /// be through creating a new transient resource or through writing an existing resource.
    pub creator_render_pass: usize,

    /// The head of a linked list that contains entries for every read of this resource version by
    /// a render pass
    pub reads: Option<NonNull<VersionReaderLink>>,

    /// Boolean flag that stores whether this resource version has been written to once. Used to
    /// detect when multiple passes try and write to the same resource version.
    pub debug_written: bool,
}

impl ResourceVersion {
    pub fn mark_written(&mut self) {
        self.debug_written = true;
    }

    pub fn is_written(&self) -> bool {
        self.debug_written
    }
}

pub struct ResourceTypeBuffer {
    pub import: Option<ImportedResource>,
    pub desc: FrameGraphBufferDesc,
}

impl Into<ResourceType> for ResourceTypeBuffer {
    fn into(self) -> ResourceType {
        ResourceType::Buffer(self)
    }
}

pub struct ResourceTypeTexture {
    pub import: Option<ImportedResource>,
    pub desc: FrameGraphTextureDesc,
}

impl Into<ResourceType> for ResourceTypeTexture {
    fn into(self) -> ResourceType {
        ResourceType::Texture(self)
    }
}

pub enum ResourceType {
    Buffer(ResourceTypeBuffer),
    Texture(ResourceTypeTexture),
}

impl ResourceType {
    pub(crate) fn unwrap_buffer(&self) -> &ResourceTypeBuffer {
        match self {
            Self::Buffer(v) => v,
            _ => panic!("self is not a ResourceType::Buffer"),
        }
    }

    pub(crate) fn unwrap_texture(&self) -> &ResourceTypeTexture {
        match self {
            Self::Texture(v) => v,
            _ => panic!("self is not a ResourceType::Texture"),
        }
    }

    pub(crate) fn is_import(&self) -> bool {
        match self {
            ResourceType::Buffer(v) => v.import.is_some(),
            ResourceType::Texture(v) => v.import.is_some(),
        }
    }

    /// # Safety
    ///
    /// It is the caller's responsibility to ensure that the name pointer inside the resource desc
    /// is valid.
    ///
    /// In our case these are allocated into the frame graph's bump arena so they remain valid as
    /// long as the arena is valid. Assuming you only ever access the string immutably.
    pub(crate) unsafe fn name(&self) -> Option<&str> {
        match self {
            ResourceType::Buffer(v) => v.desc.name.map(|v| v.as_ref()),
            ResourceType::Texture(v) => v.desc.name.map(|v| v.as_ref()),
        }
    }
}

pub struct ImportedResource {
    pub allowed_usage: ResourceUsageFlags,
    pub before_sync: BarrierSync,
    pub before_access: BarrierAccess,
    pub before_layout: ImageLayout,
    pub after_sync: BarrierSync,
    pub after_access: BarrierAccess,
    pub after_layout: ImageLayout,
}

/// An internal mirror of [BufferDesc] that removes the 'usage' field (it's automatically deduced)
/// and replaces the name reference with a pointer so that it can store a pointer into an internal
/// arena
#[derive(Default)]
pub struct FrameGraphBufferDesc {
    /// The size of the buffer to be created
    pub size: u64,

    /// The name of the resource. This is a pointer to a region within the main frame graph arena
    /// that the passes are stored in. It is only sound to access this string immutably, and the
    /// caller must ensure the relevant arena is still live.
    pub name: Option<NonNull<str>>,
}

/// An internal mirror of [TextureDesc] that removes the 'usage' field (it's automatically deduced)
/// and replaces the name reference with a pointer so that it can store a pointer into an internal
/// arena
#[derive(Default)]
pub struct FrameGraphTextureDesc {
    /// The width of the texture
    pub width: u32,

    /// The height of the texture
    pub height: u32,

    /// The depth of the texture
    pub depth: u32,

    /// The pixel format of the texture
    pub format: Format,

    /// The dimensionality of the texture.
    ///
    /// Declares whether the texture should be a 1D, 2D, 3D or cube texture.
    pub dimension: TextureDimension,

    /// An optional clear value that will be 'optimal' for the underlying implementation.
    pub clear_value: Option<OptimalClearValue>,

    /// Number of image array elements.
    ///
    /// A value of '1' means to create a regular, non-array texture. Setting this to a value >1
    /// declares the texture as a texture array.
    pub array_size: u32,

    /// Number of mip levels.
    pub mip_levels: u32,

    /// Sample count, for MSAA texture.
    ///
    /// A value of '1' means a regular, non MSAA texture. This value must always be a power of two.
    /// Setting this to a value >1 declares the texture as an MSAA texture.
    pub sample_count: u32,

    /// Sample quality, for MSAA texture
    pub sample_quality: u32,

    /// The name of the resource. This is a pointer to a region within the main frame graph arena
    /// that the passes are stored in. It is only sound to access this string immutably, and the
    /// caller must ensure the relevant arena is still live.
    pub name: Option<NonNull<str>>,
}

#[derive(Default)]
pub struct PassAccessInfo {
    pub reads: Vec<ResourceAccess>,
    pub writes: Vec<ResourceAccess>,
}

impl PassAccessInfo {
    pub fn clear(&mut self) {
        self.reads.clear();
        self.writes.clear();
    }
}

/// Stores the requested access for a single resource access edge. Could be read or write.
#[derive(Clone)]
pub struct ResourceAccess {
    /// The destructured resource ID. ResourceRef/ResourceMut is for the external API
    pub resource: ResourceId,

    /// Pipeline stage/stages the buffer will be used in
    pub sync: BarrierSync,

    /// How the resource will be accessed within the render pass
    pub access: ResourceUsageFlags,
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Default, Debug)]
pub struct VersionIndex(pub u32);

impl VersionIndex {
    pub const INVALID: Self = Self(u32::MAX);

    pub const fn new(v: u32) -> Option<VersionIndex> {
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

pub struct VersionReaderLink {
    pub next: Option<NonNull<VersionReaderLink>>,
    pub render_pass: usize,
    pub access: ResourceUsageFlags,
}

/// An internal enum used for tracking the state machine of a resource version through the pass
/// scheduler.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum ResourceVersionState {
    /// A resource version in this state has yet to be written to, and can not be read from.
    Waiting,

    /// A resource version in this state has been written to, and can now be read from.
    Written,

    /// A resource version in this state has now had all its readers scheduled and can be considered
    /// 'retired'. It is safe for the following version of a resource to be written in this state.
    ///
    /// It is not valid for a resource in this state to be accessed in any way.
    Retired,
}

impl Default for ResourceVersionState {
    fn default() -> Self {
        Self::Waiting
    }
}
