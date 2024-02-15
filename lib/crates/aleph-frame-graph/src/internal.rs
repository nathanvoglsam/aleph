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

use std::ptr::NonNull;

use aleph_rhi_api::*;

use crate::access::ResourceUsageFlagsExt;
use crate::resource::ResourceId;
use crate::IRenderPass;
use crate::ResourceVariant;

pub(crate) struct RenderPass {
    pub pass: NonNull<dyn IRenderPass>,
    pub name: NonNull<str>,
}

pub(crate) struct ResourceRoot {
    /// The type of this resource
    pub resource_type: ResourceType,

    /// The accumulated access flags for a resource. This is the union of all the ways a
    /// resource is used as within the frame graph.
    pub total_access_flags: ResourceUsageFlags,

    /// An index to the final version of this resource
    pub final_version: VersionIndex,
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
    pub creator_pass: usize,

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

    /// What kind of CPU access is allowed.
    /// - None -> device local
    /// - Read -> read back
    /// - Write -> upload
    pub cpu_access: CpuAccessMode,

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

pub trait IIRNode: Clone + core::fmt::Debug {
    fn prev(&self) -> NonNull<[usize]>;
    fn next(&self) -> NonNull<[usize]>;
    fn set_prev(&mut self, v: NonNull<[usize]>);
    fn set_next(&mut self, v: NonNull<[usize]>);
    fn is_render_pass(&self) -> bool;
    fn is_barrier(&self) -> bool;
    fn is_layout_transition(&self) -> bool;

    fn resource_id(&self) -> ResourceId {
        ResourceId::new(0, 0)
    }

    fn render_pass(&self) -> usize {
        Default::default()
    }

    fn barrier_type(&self) -> IRBarrierType {
        Default::default()
    }

    fn before_scope(&self) -> (BarrierSync, BarrierAccess, ImageLayout) {
        (Default::default(), Default::default(), Default::default())
    }

    fn after_scope(&self) -> (BarrierSync, BarrierAccess, ImageLayout) {
        (Default::default(), Default::default(), Default::default())
    }

    fn write_graph_viz<T: std::io::Write>(
        &self,
        writer: &mut T,
        name: &str,
        node_index: usize,
    ) -> std::io::Result<()>;
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

impl IIRNode for IRNode {
    fn prev(&self) -> NonNull<[usize]> {
        match self {
            IRNode::RenderPass(v) => v.prev(),
            IRNode::Barrier(v) => v.prev(),
            IRNode::LayoutChange(v) => v.prev(),
        }
    }

    fn next(&self) -> NonNull<[usize]> {
        match self {
            IRNode::RenderPass(v) => v.next(),
            IRNode::Barrier(v) => v.next(),
            IRNode::LayoutChange(v) => v.next(),
        }
    }

    fn set_prev(&mut self, v: NonNull<[usize]>) {
        match self {
            IRNode::RenderPass(x) => x.set_prev(v),
            IRNode::Barrier(x) => x.set_prev(v),
            IRNode::LayoutChange(x) => x.set_prev(v),
        }
    }

    fn set_next(&mut self, v: NonNull<[usize]>) {
        match self {
            IRNode::RenderPass(x) => x.set_next(v),
            IRNode::Barrier(x) => x.set_next(v),
            IRNode::LayoutChange(x) => x.set_next(v),
        }
    }

    fn is_render_pass(&self) -> bool {
        match self {
            IRNode::RenderPass(v) => v.is_render_pass(),
            IRNode::Barrier(v) => v.is_render_pass(),
            IRNode::LayoutChange(v) => v.is_render_pass(),
        }
    }

    fn is_barrier(&self) -> bool {
        match self {
            IRNode::RenderPass(v) => v.is_barrier(),
            IRNode::Barrier(v) => v.is_barrier(),
            IRNode::LayoutChange(v) => v.is_barrier(),
        }
    }

    fn is_layout_transition(&self) -> bool {
        match self {
            IRNode::RenderPass(v) => v.is_layout_transition(),
            IRNode::Barrier(v) => v.is_layout_transition(),
            IRNode::LayoutChange(v) => v.is_layout_transition(),
        }
    }

    fn resource_id(&self) -> ResourceId {
        match self {
            IRNode::RenderPass(v) => v.resource_id(),
            IRNode::Barrier(v) => v.resource_id(),
            IRNode::LayoutChange(v) => v.resource_id(),
        }
    }

    fn render_pass(&self) -> usize {
        match self {
            IRNode::RenderPass(v) => v.render_pass(),
            IRNode::Barrier(v) => v.render_pass(),
            IRNode::LayoutChange(v) => v.render_pass(),
        }
    }

    fn barrier_type(&self) -> IRBarrierType {
        match self {
            IRNode::RenderPass(v) => v.barrier_type(),
            IRNode::Barrier(v) => v.barrier_type(),
            IRNode::LayoutChange(v) => v.barrier_type(),
        }
    }

    fn before_scope(&self) -> (BarrierSync, BarrierAccess, ImageLayout) {
        match self {
            IRNode::RenderPass(v) => v.before_scope(),
            IRNode::Barrier(v) => v.before_scope(),
            IRNode::LayoutChange(v) => v.before_scope(),
        }
    }

    fn after_scope(&self) -> (BarrierSync, BarrierAccess, ImageLayout) {
        match self {
            IRNode::RenderPass(v) => v.after_scope(),
            IRNode::Barrier(v) => v.after_scope(),
            IRNode::LayoutChange(v) => v.after_scope(),
        }
    }

    fn write_graph_viz<T: std::io::Write>(
        &self,
        writer: &mut T,
        name: &str,
        node_index: usize,
    ) -> std::io::Result<()> {
        match self {
            IRNode::RenderPass(v) => v.write_graph_viz(writer, name, node_index),
            IRNode::Barrier(v) => v.write_graph_viz(writer, name, node_index),
            IRNode::LayoutChange(v) => v.write_graph_viz(writer, name, node_index),
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
    pub resource_id: ResourceId,

    /// The type of barrier is node represents
    pub barrier_type: IRBarrierType,

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
    pub resource_id: ResourceId,

    /// The type of barrier is node represents
    pub barrier_type: IRBarrierType,

    /// The subresource range the barrier applies to. This will always be the niggers
    pub subresource_range: TextureSubResourceSet,

    pub before_sync: BarrierSync,
    pub before_access: BarrierAccess,
    pub before_layout: ImageLayout,
    pub after_sync: BarrierSync,
    pub after_access: BarrierAccess,
    pub after_layout: ImageLayout,
}

impl IIRNode for RenderPassIRNode {
    #[inline(always)]
    fn prev(&self) -> NonNull<[usize]> {
        self.prev
    }

    #[inline(always)]
    fn next(&self) -> NonNull<[usize]> {
        self.next
    }

    #[inline(always)]
    fn set_prev(&mut self, v: NonNull<[usize]>) {
        self.prev = v;
    }

    #[inline(always)]
    fn set_next(&mut self, v: NonNull<[usize]>) {
        self.next = v;
    }

    #[inline(always)]
    fn is_render_pass(&self) -> bool {
        true
    }

    #[inline(always)]
    fn is_barrier(&self) -> bool {
        false
    }

    #[inline(always)]
    fn is_layout_transition(&self) -> bool {
        false
    }

    fn render_pass(&self) -> usize {
        self.render_pass
    }

    fn write_graph_viz<T: std::io::Write>(
        &self,
        writer: &mut T,
        name: &str,
        node_index: usize,
    ) -> std::io::Result<()> {
        writeln!(
            writer,
            "node{node_index} [shape=box,label=\"Render Pass: \\\"{name}\\\"\"];"
        )
    }
}

impl IIRNode for BarrierIRNode {
    #[inline(always)]
    fn prev(&self) -> NonNull<[usize]> {
        self.prev
    }

    #[inline(always)]
    fn next(&self) -> NonNull<[usize]> {
        self.next
    }

    #[inline(always)]
    fn set_prev(&mut self, v: NonNull<[usize]>) {
        self.prev = v;
    }

    #[inline(always)]
    fn set_next(&mut self, v: NonNull<[usize]>) {
        self.next = v;
    }

    #[inline(always)]
    fn is_render_pass(&self) -> bool {
        false
    }

    #[inline(always)]
    fn is_barrier(&self) -> bool {
        true
    }

    #[inline(always)]
    fn is_layout_transition(&self) -> bool {
        false
    }

    #[inline(always)]
    fn resource_id(&self) -> ResourceId {
        self.resource_id
    }

    #[inline(always)]
    fn barrier_type(&self) -> IRBarrierType {
        self.barrier_type
    }

    #[inline(always)]
    fn before_scope(&self) -> (BarrierSync, BarrierAccess, ImageLayout) {
        (self.before_sync, self.before_access, Default::default())
    }

    #[inline(always)]
    fn after_scope(&self) -> (BarrierSync, BarrierAccess, ImageLayout) {
        (self.after_sync, self.after_access, Default::default())
    }

    fn write_graph_viz<T: std::io::Write>(
        &self,
        writer: &mut T,
        name: &str,
        node_index: usize,
    ) -> std::io::Result<()> {
        writeln!(
            writer,
            "node{} [label=\"{} Barrier: Resource: {} (v_id#{})\\nBeforeSync: {:?}\\nBeforeAccess: {:?}\\nAfterSync: {:?}\\nAfterAccess: {:?}\"];",
            node_index,
            self.barrier_type.graphviz_text(),
            name,
            self.resource_id.version,
            self.before_sync,
            self.before_access,
            self.after_sync,
            self.after_access
        )
    }
}

impl IIRNode for LayoutChangeIRNode {
    #[inline(always)]
    fn prev(&self) -> NonNull<[usize]> {
        self.prev
    }

    #[inline(always)]
    fn next(&self) -> NonNull<[usize]> {
        self.next
    }

    #[inline(always)]
    fn set_prev(&mut self, v: NonNull<[usize]>) {
        self.prev = v;
    }

    #[inline(always)]
    fn set_next(&mut self, v: NonNull<[usize]>) {
        self.next = v;
    }

    #[inline(always)]
    fn is_render_pass(&self) -> bool {
        false
    }

    #[inline(always)]
    fn is_barrier(&self) -> bool {
        true
    }

    #[inline(always)]
    fn is_layout_transition(&self) -> bool {
        true
    }

    #[inline(always)]
    fn resource_id(&self) -> ResourceId {
        self.resource_id
    }

    #[inline(always)]
    fn barrier_type(&self) -> IRBarrierType {
        self.barrier_type
    }

    #[inline(always)]
    fn before_scope(&self) -> (BarrierSync, BarrierAccess, ImageLayout) {
        (self.before_sync, self.before_access, self.before_layout)
    }

    #[inline(always)]
    fn after_scope(&self) -> (BarrierSync, BarrierAccess, ImageLayout) {
        (self.after_sync, self.after_access, self.after_layout)
    }

    fn write_graph_viz<T: std::io::Write>(
        &self,
        writer: &mut T,
        name: &str,
        node_index: usize,
    ) -> std::io::Result<()> {
        writeln!(
            writer,
            "node{} [label=\"{} Layout Change Barrier: Resource: {} (v_id#{})\\nBeforeSync: {:?}\\nBeforeAccess: {:?}\\nBeforeLayout: {:?}\\nAfterSync: {:?}\\nAfterAccess: {:?}\\nAfterLayout: {:?}\"];",
            node_index,
            self.barrier_type.graphviz_text(),
            name,
            self.resource_id.version,
            self.before_sync,
            self.before_access,
            self.before_layout,
            self.after_sync,
            self.after_access,
            self.after_layout,
        )
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub(crate) enum IRBarrierType {
    /// A barrier used to initialize what would be an uninitialized transient resource.
    Initialization,

    /// A barrier used to import an external resource.
    Import,

    /// A barrier used to export an external resource from the graph after its final usage within
    /// the frame graph. Specifically this barrier is exporting after a preceeding read access.
    ExportAfterRead,

    /// A barrier used to export an external resource from the graph after its final usage within
    /// the frame graph. Specifically this barrier is exporting after a preceeding write access.
    ExportAfterWrite,

    /// A regular internal barrier, specifically a barrier for read access after a preceeding read
    /// access.
    ReadAfterRead,

    /// A regular internal barrier, specifically a barrier for read access after a preceeding write
    /// access.
    ReadAfterWrite,

    /// A regular internal barrier, specifically a barrier for write access after a preceeding read
    /// access.
    WriteAfterRead,

    /// A regular internal barrier, specifically a barrier for write access after a preceeding write
    /// access.
    WriteAfterWrite,
}

impl Default for IRBarrierType {
    fn default() -> Self {
        Self::Initialization
    }
}

impl IRBarrierType {
    /// The number of variants of the [IRBarrierType] enum.
    pub const NUM_VARIANTS: usize = 8;

    pub fn graphviz_text(&self) -> &'static str {
        match self {
            IRBarrierType::Initialization => "Initialization",
            IRBarrierType::Import => "Import",
            IRBarrierType::ExportAfterRead => "Export after Read",
            IRBarrierType::ExportAfterWrite => "Export after Write",
            IRBarrierType::ReadAfterRead => "Read after Read",
            IRBarrierType::ReadAfterWrite => "Read after Write",
            IRBarrierType::WriteAfterRead => "Write after Read",
            IRBarrierType::WriteAfterWrite => "Write after Write",
        }
    }
}

pub(crate) struct PassOrderBundle {
    /// The ordered list of barriers that must be executed before executing the passes in
    /// [PassOrderBundle::passes]. These store indices into the ir.nodes array.
    pub barriers: NonNull<[usize]>,

    /// The ordered list of render passes that must be executed after the barriers in
    /// [PassOrderBundle::barriers]. These store indices into the ir.nodes array.
    pub passes: NonNull<[usize]>,
}

impl core::fmt::Debug for PassOrderBundle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let barriers = unsafe { self.barriers.as_ref() };
        let passes = unsafe { self.passes.as_ref() };
        f.debug_struct("PassOrderBundle")
            .field("barriers", &barriers)
            .field("passes", &passes)
            .finish()
    }
}

#[derive(Default)]
pub struct TransientResourceBundle {
    pub(crate) transients: std::collections::HashMap<u16, ResourceVariant>,
}

impl TransientResourceBundle {
    pub(crate) fn add_resource(&mut self, i: u16, r: impl Into<ResourceVariant>) -> &mut Self {
        let r = r.into();

        let existed = self.transients.insert(i, r).is_some();
        assert!(
            !existed,
            "It is invalid to insert a handle for the same resource ID twice"
        );

        self
    }

    pub(crate) fn get_resource(&self, i: u16) -> Option<&ResourceVariant> {
        self.transients.get(&i)
    }
}
