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

use crate::access::ResourceUsageFlagsExt;
use crate::resource::ResourceId;
use crate::IRenderPass;
use aleph_rhi_api::*;
use std::ptr::NonNull;

pub(crate) struct RenderPass {
    pub pass: NonNull<dyn IRenderPass>,
    pub name: NonNull<str>,
    pub reads: NonNull<[ResourceAccess]>,
    pub writes: NonNull<[ResourceAccess]>,
}

pub(crate) struct ResourceRoot {
    /// The type of this resource
    pub resource_type: ResourceType,

    /// The accumulated access flags for a resource. This is the union of all the ways a
    /// resource is used as within the frame graph.
    pub total_access_flags: ResourceUsageFlags,

    /// An index to the first version of this resource
    pub initial_version: VersionIndex,
}

pub(crate) struct ResourceVersion {
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

    /// The sync flags that the resource version will be used with in the creating pass.
    pub creator_sync: BarrierSync,

    /// How the resource will be accessed within the render pass that creates the resource version.
    pub creator_access: ResourceUsageFlags,

    /// The index of the render pass that caused the new resource version to be created. This could
    /// be through creating a new transient resource or through writing an existing resource.
    pub creator_render_pass: usize,

    /// The number of read entries in the 'reads' linked list
    pub read_count: usize,

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

    pub fn reads_iter(&self) -> VersionReaderLinkIter {
        VersionReaderLinkIter {
            current: self.reads.map(|v| unsafe { v.as_ref() }),
        }
    }

    /// collect all the reads into an array and sort them by the image layout the resource should be
    /// in for that read.
    ///
    /// This is done by first creating a vector of correct size, filling it from
    /// an iterator and then sorting by the computed image layout.
    pub fn reads_sorted_by_image_layout_in<'a, 'b>(
        &'a self,
        format: Format,
        bump: &'b bumpalo::Bump,
    ) -> bumpalo::collections::Vec<'b, (&'a VersionReaderLink, ImageLayout)> {
        use bumpalo::collections::Vec as BVec;

        let mut reads = BVec::with_capacity_in(self.read_count, bump);
        reads.extend(
            self.reads_iter()
                .map(|v| (v, v.access.image_layout(true, format))),
        );
        reads.sort_by_key(|v| v.1);

        reads
    }
}

pub(crate) struct ResourceTypeBuffer {
    pub import: Option<ImportedResource>,
    pub desc: FrameGraphBufferDesc,
}

impl Into<ResourceType> for ResourceTypeBuffer {
    fn into(self) -> ResourceType {
        ResourceType::Buffer(self)
    }
}

pub(crate) struct ResourceTypeTexture {
    pub import: Option<ImportedResource>,
    pub desc: FrameGraphTextureDesc,
}

impl Into<ResourceType> for ResourceTypeTexture {
    fn into(self) -> ResourceType {
        ResourceType::Texture(self)
    }
}

pub(crate) enum ResourceType {
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

pub(crate) struct ImportedResource {
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
pub(crate) struct FrameGraphBufferDesc {
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
pub(crate) struct FrameGraphTextureDesc {
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
pub(crate) struct PassAccessInfo {
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
pub(crate) struct ResourceAccess {
    /// The destructured resource ID. ResourceRef/ResourceMut is for the external API
    pub resource: ResourceId,

    /// Pipeline stage/stages the buffer will be used in
    pub sync: BarrierSync,

    /// How the resource will be accessed within the render pass
    pub access: ResourceUsageFlags,
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Default, Debug)]
pub(crate) struct VersionIndex(pub u32);

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

#[derive(Clone)]
pub(crate) struct VersionReaderLink {
    pub next: Option<NonNull<VersionReaderLink>>,
    pub render_pass: usize,
    pub sync: BarrierSync,
    pub access: ResourceUsageFlags,
}

#[repr(transparent)]
pub(crate) struct VersionReaderLinkIter<'a> {
    pub current: Option<&'a VersionReaderLink>,
}

impl<'a> Iterator for VersionReaderLinkIter<'a> {
    type Item = &'a VersionReaderLink;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.current;
        self.current = self
            .current
            .map(|v| v.next.map(|v| unsafe { v.as_ref() }))
            .flatten();
        out
    }
}

/// An internal enum used for tracking the state machine of a resource version through the pass
/// scheduler.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub(crate) enum ResourceVersionState {
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

#[derive(Clone, Debug)]
pub(crate) enum IRNode {
    /// This [IRNode] is used to represent a render-pass in the intermediate node graph.
    RenderPass(RenderPassIRNode),

    /// This [IRNode] is used to represent a plain barrier within the graph for some resource. No
    /// layout change is performed.
    ///
    /// This simply represents a cache flush/invalidation event and is distinct from
    /// [IRNode::LayoutChange] because this operation does not 'write' the resource with a layout
    /// change. All buffer read/write edges can be handled by this node, images may require layout
    /// changes (even for read-after-read).
    Barrier(BarrierIRNode),

    /// This [IRNode] is used to represent a layout-change operation within the intermediate node
    /// graph.
    ///
    /// This is logically an exclusive write operation, and will be emitted to represent barriers
    /// that must be executed within the graph.
    LayoutChange(LayoutChangeIRNode),
}

impl IRNode {
    pub fn unwrap_render_pass(&self) -> &RenderPassIRNode {
        if let IRNode::RenderPass(v) = self {
            v
        } else {
            panic!("{:?} is not a RenderPass IRNode!", self);
        }
    }

    pub fn unwrap_barrier(&self) -> &BarrierIRNode {
        if let IRNode::Barrier(v) = self {
            v
        } else {
            panic!("{:?} is not a Barrier IRNode!", self);
        }
    }

    pub fn unwrap_layout_change(&self) -> &LayoutChangeIRNode {
        if let IRNode::LayoutChange(v) = self {
            v
        } else {
            panic!("{:?} is not a LayoutChange IRNode!", self);
        }
    }

    pub fn prev(&self) -> NonNull<[usize]> {
        match self {
            IRNode::RenderPass(v) => v.prev,
            IRNode::Barrier(v) => v.prev,
            IRNode::LayoutChange(v) => v.prev,
        }
    }

    pub fn next(&self) -> NonNull<[usize]> {
        match self {
            IRNode::RenderPass(v) => v.next,
            IRNode::Barrier(v) => v.next,
            IRNode::LayoutChange(v) => v.next,
        }
    }

    pub fn set_prev(&mut self, v: NonNull<[usize]>) {
        match self {
            IRNode::RenderPass(n) => n.prev = v,
            IRNode::Barrier(n) => n.prev = v,
            IRNode::LayoutChange(n) => n.prev = v,
        }
    }

    pub fn set_next(&mut self, v: NonNull<[usize]>) {
        match self {
            IRNode::RenderPass(n) => n.next = v,
            IRNode::Barrier(n) => n.next = v,
            IRNode::LayoutChange(n) => n.next = v,
        }
    }
}

impl From<RenderPassIRNode> for IRNode {
    fn from(value: RenderPassIRNode) -> Self {
        IRNode::RenderPass(value)
    }
}

impl From<BarrierIRNode> for IRNode {
    fn from(value: BarrierIRNode) -> Self {
        IRNode::Barrier(value)
    }
}

impl From<LayoutChangeIRNode> for IRNode {
    fn from(value: LayoutChangeIRNode) -> Self {
        IRNode::LayoutChange(value)
    }
}

#[derive(Clone, Debug)]
pub(crate) struct RenderPassIRNode {
    pub prev: NonNull<[usize]>,
    pub next: NonNull<[usize]>,

    /// The index of the render pass, uniquely identifying the pass in the graph
    pub render_pass: usize,
}

#[derive(Clone, Debug)]
pub(crate) struct BarrierIRNode {
    pub prev: NonNull<[usize]>,
    pub next: NonNull<[usize]>,

    /// The version of the resource we are encoding a barrier for
    pub version: VersionIndex,

    pub before_sync: BarrierSync,
    pub before_access: BarrierAccess,
    pub after_sync: BarrierSync,
    pub after_access: BarrierAccess,
}

#[derive(Clone, Debug)]
pub(crate) struct LayoutChangeIRNode {
    pub prev: NonNull<[usize]>,
    pub next: NonNull<[usize]>,

    /// The version of the resource we are forcing a layout change for
    pub version: VersionIndex,
    pub before_sync: BarrierSync,
    pub before_access: BarrierAccess,
    pub before_layout: ImageLayout,
    pub after_sync: BarrierSync,
    pub after_access: BarrierAccess,
    pub after_layout: ImageLayout,
}
