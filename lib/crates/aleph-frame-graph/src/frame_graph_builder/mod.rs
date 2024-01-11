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
//! Idea for Aliasing Resource in the Frame Graph
//!
//! - Graph is constructed without doing any aliasing analysis
//! - Graph gets linearized into a series of passes on a timeline with all the necessary barriers
//!   still without any aliasing analysis
//! - Track each resource's first and last pass that they're used by in our serialized pass order
//! - Once we know when a resource is created and destroyed we can walk over our ordered list of
//!   passes and, using an arbitrary logical address space, 'allocate' and 'free' each resource as
//!   their lifetime progresses.
//! - This effectively replays the frame (without actually recording it) and allows us to allocate
//!   resources with disparate memory on top of the same address space, giving us aliasing.
//! - The final step of this is to keep track of which resources intersected a particular region
//!   of memory so we know what resources are aliased so we can insert aliasing barriers
//! - This means we only have to care about resource aliasing at the end of the pipeline, the graph
//!   construction doesn't need to care about it
//! - This ignores optimization choices when linearizing the graph into a sequence of passes. We
//!   could choice a specific order that maximizes aliasing, but this is typically at odds with
//!   having our pipelines fed as it encourages narrow pipes and short resource lifetimes. Open
//!   problem to be solved when it matters.
//!

use crate::access::ResourceUsageFlagsExt;
use crate::internal::*;
use crate::render_pass::CallbackRenderPass;
use crate::resource::ResourceId;
use crate::FrameGraphResources;
use crate::{FrameGraph, IRenderPass, ResourceMut, ResourceRef};
use aleph_arena_drop_list::DropLink;
use aleph_rhi_api::*;
use bumpalo::collections::Vec as BVec;
use bumpalo::Bump;
use std::ptr::NonNull;

#[cfg(test)]
mod tests;

/// Provides a description for importing a resource into the frame graph.
///
/// This encodes the full set of sync flags that covers what usages that the resource must be
/// synchronized with from outside of the graph. The 'before_*' flags cover the 'before' scope of
/// a barrier that will be used for the graph to take ownership of the resource. The 'after_*'
/// flags cover the 'after' scope of a barrier that will be executed once the graph completes to
/// release ownership of the resource to other users outside the graph.
pub struct TextureImportDesc<'a> {
    /// The texture resource to import into the frame graph
    pub desc: &'a TextureDesc<'a>,

    /// The pipeline stage to synchronize with on first use within the frame graph
    pub before_sync: BarrierSync,

    /// The access flags to synchronize with before the first use of the resource within the frame
    /// graph
    pub before_access: BarrierAccess,

    /// The image layout the resource is expected to be in prior to the frame graph executing
    pub before_layout: ImageLayout,

    /// The pipeline stage to synchronize with as the immediate use after the frame graph is
    /// completed
    pub after_sync: BarrierSync,

    /// The access flags to synchronize with as the immediate use after the frame graph is completed
    pub after_access: BarrierAccess,

    /// The image layout the resource is expected to be transitioned to when completing the frame
    /// graph
    pub after_layout: ImageLayout,
}

/// Provides a description for importing a resource into the frame graph.
///
/// This encodes the full set of sync flags that covers what usages that the resource must be
/// synchronized with from outside of the graph. The 'before_*' flags cover the 'before' scope of
/// a barrier that will be used for the graph to take ownership of the resource. The 'after_*'
/// flags cover the 'after' scope of a barrier that will be executed once the graph completes to
/// release ownership of the resource to other users outside the graph.
pub struct BufferImportDesc<'a> {
    /// The desc of the buffer resource to import into the frame graph
    pub desc: &'a BufferDesc<'a>,

    /// The pipeline stage to synchronize with on first use within the frame graph
    pub before_sync: BarrierSync,

    /// The access flags to synchronize with before the first use of the resource within the frame
    /// graph
    pub before_access: BarrierAccess,

    /// The pipeline stage to synchronize with as the immediate use after the frame graph is
    /// completed
    pub after_sync: BarrierSync,

    /// The access flags to synchronize with as the immediate use after the frame graph is completed
    pub after_access: BarrierAccess,
}

#[derive(Default)]
pub struct GraphVizOutputOptions {
    /// Enables the option for outputting nodes 'previous' edges instead of only the 'next' edges.
    ///
    /// This option is intended for debugging to ensure that the graph is doubly linked correctly.
    /// The output quality goes down so it is not recommended to use this in the general case.
    pub output_previous_links: bool,
}

#[derive(Default)]
pub struct FrameGraphBuilder {
    /// An arena that will be moved into the FrameGraph once the graph is finalized. This can be
    /// used to store anything that persists to the fully constructed graph.
    pub(crate) arena: Bump,

    /// The list of all the render passes in the graph. The index of the pass in this list is the
    /// identity of the pass and is used to key to a number of different names
    pub(crate) render_passes: Vec<RenderPass>,

    /// The backing storage used for all of the root resourcs objects. A root resource represents
    /// a concrete [ITexture] or [IBuffer] as created by the graph. This includes both the created
    /// transient resources as well as imported resources. Imported resources are identified by
    /// having their index in the 'imported_resources' set.
    pub(crate) root_resources: Vec<ResourceRoot>,

    /// The backing storage used for all the resource version objects. A resource version is an
    /// indexed set that is used to identify a particular version of a root resource.
    ///
    /// A 'ResourceVersion' contains the accumulated usages of the resource as well as a link to
    /// the previous version to form a linked-list of resource versions back to the first version
    /// when the resource was created or imported.
    ///
    /// These entries are critical as resource versions are what form the core of the graph. They
    /// are what allows the graph to construct a stable program order via an SSA form graph
    /// construction.
    pub(crate) resource_versions: Vec<ResourceVersion>,

    /// The set of resources within the graph that were imported, stored as indices into the
    /// root_resources array.
    pub(crate) imported_resources: Vec<u16>,

    /// The head of the dropper linked-list that contains all the drop functions for objects
    /// allocated from the graph arena
    pub(crate) drop_head: Option<NonNull<DropLink>>,
}

impl FrameGraphBuilder {
    /// Creates a new, empty [FrameGraphBuilder]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a new pass to the frame graph with the given name.
    ///
    /// - `setup_fn` is a closure that will be called only once immediately inside
    ///   [FrameGraphBuilder::add_pass] that is used to declare the reads, writes, creates and
    ///   imports of the pass.
    /// - `exec_fn` is the pass closure. This will be called once during an execution run of the
    ///   graph when it is time for the pass to record commands into a command buffer. This closure
    ///   is persistent and will remain alive for the full lifetime of the [FrameGraph] object.
    pub fn add_pass<
        T: Send + Default + 'static,
        SetupFn: FnOnce(&mut T, &mut ResourceRegistry),
        ExecFn: FnMut(&T, &FrameGraphResources) + Send + 'static,
    >(
        &mut self,
        name: &str,
        setup_fn: SetupFn,
        exec_fn: ExecFn,
    ) {
        // Safety: CallbackRenderPass requires us to allocate a buffer for 'name' that outlives the
        //         pass and is immutable borrowed by the pass. We do this by allocating in our
        //         bump arena and guaranteeing that the render pass will be dropped before clearing
        //         the arena.
        unsafe {
            // Default initialize the payload and allocate the payload into the arena
            let payload = self.arena.alloc(T::default());
            let mut payload = NonNull::from(payload);
            DropLink::append_drop_list(&self.arena, &mut self.drop_head, payload);

            // We need to use the pointer here as the mutable ref created by arena.alloc will get
            // moved into the NonNull instance created as &mut doesn't impl Copy. This is still safe
            // though as we don't _use_ the payload anywhere until we give it to the setup fn, or
            // if the builder gets dropped.
            {
                let current_pass_index = self.render_passes.len();
                let mut resources = ResourceRegistry {
                    builder: self,
                    render_pass: current_pass_index,
                };
                setup_fn(payload.as_mut(), &mut resources);
            }

            // Construct the CallbackRenderPass instance and handoff to add_pass
            let callback_pass = CallbackRenderPass::new(payload, exec_fn);
            self.add_pass_internal(name, callback_pass);
        }
    }

    /// Finalize the graph and fully resolve all the declared passes into a [FrameGraph]. Once the
    /// graph has been built passes can no longer be added or removed, nor can resources be added
    /// or removed.
    ///
    /// This will run a suite of passes that will extract a total program order from the graph of
    /// passes constructed earlier. This function is expected to be expensive, so don't build new
    /// graphs often. It is intended for a graph to be built once and run many times and invalidated
    /// rarely for extenuating circum stances like the size of the backbuffer changing.
    pub fn build(self) -> FrameGraph {
        // We have to constrain the type of the writer even though we don't use it here, so we just
        // use Sink.
        //
        // This can't error unless we pass a writer, so we _could_ use unwrap_unchecked. The cost
        // is miniscule so just check anyway so we don't have unsafe code here.
        self.build_internal::<std::io::Sink>("", None).unwrap()
    }

    /// This is an alternate form of [FrameGraphBuilder::build] that accepts a writer for the graph
    /// builder to output a DOT format graph into while constructing the graph. This graph will
    /// represent the computed execution dependencies of the graph (i.e. what pass depends on what
    /// other passes).
    pub fn build_with_graph_viz(
        self,
        graph_name: &str,
        writer: &mut impl std::io::Write,
        options: &GraphVizOutputOptions,
    ) -> std::io::Result<FrameGraph> {
        self.build_internal(graph_name, Some((writer, options)))
    }

    fn build_internal<T: std::io::Write>(
        mut self,
        graph_name: &str,
        writer: Option<(&mut T, &GraphVizOutputOptions)>,
    ) -> std::io::Result<FrameGraph> {
        // An arena allocator used for allocating resources that only live as long as the graph is
        // being built
        let build_arena = Bump::new();

        self.validate_imported_resource_usages();

        let num_passes = self.render_passes.len();

        let mut ir_builder = IRBuilder::new(&build_arena, writer, num_passes);
        ir_builder.build(&self, graph_name)?;

        let mut pass_order_builder = PassOrderBuilder::new(&build_arena, &ir_builder);
        pass_order_builder.build(&self, &ir_builder);

        let execution_bundles = pass_order_builder.bundles;
        let ir_nodes = ir_builder.nodes;

        let arena = std::mem::take(&mut self.arena);
        let render_passes = std::mem::take(&mut self.render_passes);
        let root_resources = std::mem::take(&mut self.root_resources);
        let resource_versions = std::mem::take(&mut self.resource_versions);
        let imported_resources = std::mem::take(&mut self.imported_resources);
        let drop_head = std::mem::take(&mut self.drop_head);

        Ok(FrameGraph {
            _arena: arena,
            execution_bundles,
            ir_nodes,
            render_passes,
            root_resources,
            resource_versions,
            imported_resources,
            drop_head,
        })
    }
}

/// An interface constrained way to access the frame graph builder for collecting information from
/// render pass setup callbacks.
pub struct ResourceRegistry<'a> {
    builder: &'a mut FrameGraphBuilder,
    render_pass: usize,
}

impl<'a> ResourceRegistry<'a> {
    /// Declares that this pass would like to import the given resource into the frame graph with
    /// the given parameters.
    ///
    /// This is a wrapper over [FrameGraphBuilder::import_texture].
    pub fn import_texture(
        &mut self,
        desc: &TextureImportDesc,
        sync: BarrierSync,
        access: ResourceUsageFlags,
    ) -> ResourceMut {
        self.builder
            .import_texture_internal(self.render_pass, desc, sync, access)
    }

    /// Declares that this pass would like to import the given resource into the frame graph with
    /// the given parameters.
    ///
    /// This is a wrapper over [FrameGraphBuilder::import_buffer].
    pub fn import_buffer(
        &mut self,
        desc: &BufferImportDesc,
        sync: BarrierSync,
        access: ResourceUsageFlags,
    ) -> ResourceMut {
        self.builder
            .import_buffer_internal(self.render_pass, desc, sync, access)
    }

    /// Declares a read access to the given texture, with the given sync parameters.
    ///
    /// The returned resource handle is equal to the handle given in 'r'. It is returned simply as
    /// a utility to mirror the write declaration functions.
    ///
    /// When 'sync' is equal to `BarrierSync::default()` (empty) default sync flags are chosen that
    /// covers all possible [BarrierSync] values that are applicable to the [ResourceUsageFlags]
    /// declared as 'access'.
    pub fn read_texture<R: Into<ResourceRef>>(
        &mut self,
        resource: R,
        sync: BarrierSync,
        access: ResourceUsageFlags,
    ) -> ResourceRef {
        self.builder
            .read_texture_internal(self.render_pass, resource, sync, access)
    }

    /// Declares a read access to the given buffer, with the given sync parameters.
    ///
    /// The returned resource handle is equal to the handle given in 'r'. It is returned simply as
    /// a utility to mirror the write declaration functions.
    ///
    /// When 'sync' is equal to `BarrierSync::default()` (empty) default sync flags are chosen that
    /// covers all possible [BarrierSync] values that are applicable to the [ResourceUsageFlags]
    /// declared as 'access'.
    pub fn read_buffer<R: Into<ResourceRef>>(
        &mut self,
        resource: R,
        sync: BarrierSync,
        access: ResourceUsageFlags,
    ) -> ResourceRef {
        self.builder
            .read_buffer_internal(self.render_pass, resource, sync, access)
    }

    /// Declares a write access to the given texture, with the given sync parameters.
    ///
    /// Any write access to a resource will produce a new unique resource handle, which this
    /// function will return. This resource handle is a reference to the resource in the state that
    /// this render pass will leave it in after whatever write operations are performed.
    ///
    /// It is invalid to write to a resource through the same handle more than once. Any future
    /// writes must use the handle returned by this function. This constraint is to allow a total
    /// program order to be derived unambiguously from the set of passes submitted to the graph.
    ///
    /// When 'sync' is equal to `BarrierSync::default()` (empty) default sync flags are chosen that
    /// covers all possible [BarrierSync] values that are applicable to the [ResourceUsageFlags]
    /// declared as 'access'.
    pub fn write_texture<R: Into<ResourceMut>>(
        &mut self,
        resource: R,
        sync: BarrierSync,
        access: ResourceUsageFlags,
    ) -> ResourceMut {
        self.builder
            .write_texture_internal(self.render_pass, resource, sync, access)
    }

    /// Declares a write access to the given buffer, with the given sync parameters.
    ///
    /// Any write access to a resource will produce a new unique resource handle, which this
    /// function will return. This resource handle is a reference to the resource in the state that
    /// this render pass will leave it in after whatever write operations are performed.
    ///
    /// It is invalid to write to a resource through the same handle more than once. Any future
    /// writes must use the handle returned by this function. This constraint is to allow a total
    /// program order to be derived unambiguously from the set of passes submitted to the graph.
    ///
    /// When 'sync' is equal to `BarrierSync::default()` (empty) default sync flags are chosen that
    /// covers all possible [BarrierSync] values that are applicable to the [ResourceUsageFlags]
    /// declared as 'access'.
    pub fn write_buffer<R: Into<ResourceMut>>(
        &mut self,
        resource: R,
        sync: BarrierSync,
        access: ResourceUsageFlags,
    ) -> ResourceMut {
        self.builder
            .write_buffer_internal(self.render_pass, resource, sync, access)
    }

    /// Declares that a new, transient texture will be created and used by the pass. Use 'access' to
    /// specify how the creating pass will use the resource.
    ///
    /// The resource will be created with the given parameters with only a single exception. The
    /// resource usage flags in the [TextureDesc] will be ignored. The frame graph implementation
    /// will not use the given flags, and instead will collect all the unique ways the resource was
    /// used within the frame graph and initialize the resource with the usage flag it calculates
    /// itself. This is a noteworthy difference compared to the documentation on the [TextureDesc],
    /// which _will_ say otherwise, as it is intended for creating new resources at the RHI level.
    ///
    /// It would be intractable to require specifying all the usage flags up front with this
    /// function as it is impossible for a frame graph pass to know all the ways the resource will
    /// be used in the graph. Requiring a graph pass to know this would either have passes
    /// specifying overly broad usage flags or would cause the passes to be very poorly composable.
    ///
    /// When 'sync' is equal to `BarrierSync::default()` (empty) default sync flags are chosen that
    /// covers all possible [BarrierSync] values that are applicable to the [ResourceUsageFlags]
    /// declared as 'access'.
    pub fn create_texture(
        &mut self,
        desc: &TextureDesc,
        sync: BarrierSync,
        access: ResourceUsageFlags,
    ) -> ResourceMut {
        self.builder
            .create_texture_internal(self.render_pass, desc, sync, access)
    }

    /// Declares that a new, transient buffer will be created and used by the pass. Use 'access' to
    /// specify how the creating pass will use the resource.
    ///
    /// The resource will be created with the given parameters with only a single exception. The
    /// resource usage flags in the [BufferDesc] will be ignored. The frame graph implementation
    /// will not use the given flags, and instead will collect all the unique ways the resource was
    /// used within the frame graph and initialize the resource with the usage flag it calculates
    /// itself. This is a noteworthy difference compared to the documentation on the [BufferDesc],
    /// which _will_ say otherwise, as it is intended for creating new resources at the RHI level.
    ///
    /// It would be intractable to require specifying all the usage flags up front with this
    /// function as it is impossible for a frame graph pass to know all the ways the resource will
    /// be used in the graph. Requiring a graph pass to know this would either have passes
    /// specifying overly broad usage flags or would cause the passes to be very poorly composable.
    ///
    /// When 'sync' is equal to `BarrierSync::default()` (empty) default sync flags are chosen that
    /// covers all possible [BarrierSync] values that are applicable to the [ResourceUsageFlags]
    /// declared as 'access'.
    pub fn create_buffer(
        &mut self,
        desc: &BufferDesc,
        sync: BarrierSync,
        access: ResourceUsageFlags,
    ) -> ResourceMut {
        self.builder
            .create_buffer_internal(self.render_pass, desc, sync, access)
    }
}

// =================================================================================================
// INTERNAL IMPLEMENTATION
// =================================================================================================

// Internal functions exposed through ResourceRegistry
impl FrameGraphBuilder {
    pub(crate) fn add_pass_internal<T: IRenderPass>(&mut self, name: &str, pass: T) {
        let name = self.arena.alloc_str(name);
        let name = NonNull::from(name);
        let pass = self.arena.alloc(pass);
        let mut pass = NonNull::from(pass);
        DropLink::append_drop_list(&self.arena, &mut self.drop_head, pass);

        unsafe {
            let pass = NonNull::from(pass.as_mut() as &mut dyn IRenderPass);
            let pass = RenderPass { pass, name };
            self.render_passes.push(pass);
        }
    }

    pub(crate) fn import_texture_internal(
        &mut self,
        render_pass: usize,
        desc: &TextureImportDesc,
        sync: BarrierSync,
        access: ResourceUsageFlags,
    ) -> ResourceMut {
        debug_assert!(
            access.is_valid_texture_usage(),
            "{:?} is not valid texture usage",
            access
        );

        let format = desc.desc.format;
        let sync = get_given_or_default_sync_flags_for(access, sync, false, format);

        let imported = ImportedResource {
            allowed_usage: desc.desc.usage,
            before_sync: desc.before_sync,
            before_access: desc.before_access,
            before_layout: desc.before_layout,
            after_sync: desc.after_sync,
            after_access: desc.after_access,
            after_layout: desc.after_layout,
        };
        let name = desc.desc.name.map(|v| self.arena.alloc_str(v));
        let name = name.map(|v| NonNull::from(v));
        let r_type = ResourceTypeTexture {
            import: Some(imported),
            desc: FrameGraphTextureDesc {
                width: desc.desc.width,
                height: desc.desc.height,
                depth: desc.desc.depth,
                format: desc.desc.format,
                dimension: desc.desc.dimension,
                clear_value: desc.desc.clear_value.clone(),
                array_size: desc.desc.array_size,
                mip_levels: desc.desc.mip_levels,
                sample_count: desc.desc.sample_count,
                sample_quality: desc.desc.sample_quality,
                name,
            },
        };

        // render pass index doesn't matter here as imported resources aren't created by a render
        // pass
        let r = self.create_new_handle(render_pass, sync, access, r_type);
        self.add_imported_resource_to_list(r);

        r
    }

    pub(crate) fn import_buffer_internal(
        &mut self,
        render_pass: usize,
        desc: &BufferImportDesc,
        sync: BarrierSync,
        access: ResourceUsageFlags,
    ) -> ResourceMut {
        debug_assert!(
            ResourceUsageFlags::BUFFER_USAGE_MASK.contains(access),
            "Attempting to declare non-buffer compatible access flags {:?}",
            access
        );

        let sync = get_given_or_default_sync_flags_for(access, sync, false, Default::default());
        let imported = ImportedResource {
            allowed_usage: desc.desc.usage,
            before_sync: desc.before_sync,
            before_access: desc.before_access,
            before_layout: Default::default(),
            after_sync: desc.after_sync,
            after_access: desc.after_access,
            after_layout: Default::default(),
        };
        let name = desc.desc.name.map(|v| self.arena.alloc_str(v));
        let name = name.map(|v| NonNull::from(v));
        let r_type = ResourceTypeBuffer {
            import: Some(imported),
            desc: FrameGraphBufferDesc {
                size: desc.desc.size,
                name,
            },
        };

        // render pass index doesn't matter here as imported resources aren't created by a render
        // pass
        let r = self.create_new_handle(render_pass, sync, access, r_type);
        self.add_imported_resource_to_list(r);

        r
    }

    pub(crate) fn read_texture_internal<R: Into<ResourceRef>>(
        &mut self,
        render_pass: usize,
        resource: R,
        sync: BarrierSync,
        access: ResourceUsageFlags,
    ) -> ResourceRef {
        debug_assert!(
            access.is_valid_texture_usage(),
            "{:?} is not valid texture usage",
            access
        );

        let r = resource.into();
        let root_resource = self.assert_resource_handle_is_texture(r);
        let format = root_resource.desc.format;
        self.add_flags_to_version_for(r, access);
        self.add_flags_to_root_for(r, access);
        let sync = get_given_or_default_sync_flags_for(access, sync, true, format);
        self.append_read_to_version_for(r, render_pass, sync, access);

        r
    }

    pub(crate) fn read_buffer_internal<R: Into<ResourceRef>>(
        &mut self,
        render_pass: usize,
        resource: R,
        sync: BarrierSync,
        access: ResourceUsageFlags,
    ) -> ResourceRef {
        debug_assert!(
            ResourceUsageFlags::BUFFER_USAGE_MASK.contains(access),
            "Attempting to declare non-buffer compatible access flags {:?}",
            access
        );

        let r = resource.into();
        self.assert_resource_handle_is_buffer(r);
        self.add_flags_to_version_for(r, access);
        self.add_flags_to_root_for(r, access);
        let sync = get_given_or_default_sync_flags_for(access, sync, true, Default::default());
        self.append_read_to_version_for(r, render_pass, sync, access);

        r
    }

    pub(crate) fn write_texture_internal<R: Into<ResourceMut>>(
        &mut self,
        render_pass: usize,
        resource: R,
        sync: BarrierSync,
        access: ResourceUsageFlags,
    ) -> ResourceMut {
        debug_assert!(
            access.is_valid_texture_usage(),
            "{:?} is not valid texture usage",
            access
        );

        let r = resource.into();
        let root_resource = self.assert_resource_handle_is_texture(r);
        let format = root_resource.desc.format;
        self.validate_and_update_for_handle_write(r);
        self.add_flags_to_root_for(r, access);
        let sync = get_given_or_default_sync_flags_for(access, sync, false, format);
        let renamed_r = self.increment_handle_for_write(r, render_pass, sync, access);

        renamed_r
    }

    pub(crate) fn write_buffer_internal<R: Into<ResourceMut>>(
        &mut self,
        render_pass: usize,
        resource: R,
        sync: BarrierSync,
        access: ResourceUsageFlags,
    ) -> ResourceMut {
        debug_assert!(
            ResourceUsageFlags::BUFFER_USAGE_MASK.contains(access),
            "Attempting to declare non-buffer compatible access flags {:?}",
            access
        );

        let r = resource.into();
        self.assert_resource_handle_is_buffer(r);
        self.validate_and_update_for_handle_write(r);
        self.add_flags_to_root_for(r, access);
        let sync = get_given_or_default_sync_flags_for(access, sync, false, Default::default());
        let renamed_r = self.increment_handle_for_write(r, render_pass, sync, access);

        renamed_r
    }

    pub(crate) fn create_texture_internal(
        &mut self,
        render_pass: usize,
        desc: &TextureDesc,
        sync: BarrierSync,
        access: ResourceUsageFlags,
    ) -> ResourceMut {
        debug_assert!(
            desc.usage.is_empty(),
            "The value of desc.usage is ignored, do not use it!"
        );
        debug_assert!(
            access.is_valid_texture_usage(),
            "{:?} is not valid texture usage",
            access
        );

        let format = desc.format;
        let sync = get_given_or_default_sync_flags_for(access, sync, true, format);
        let name = desc.name.map(|v| self.arena.alloc_str(v));
        let name = name.map(|v| NonNull::from(v));
        let create_desc = FrameGraphTextureDesc {
            width: desc.width,
            height: desc.height,
            depth: desc.depth,
            format: desc.format,
            dimension: desc.dimension,
            clear_value: desc.clear_value.clone(),
            array_size: desc.array_size,
            mip_levels: desc.mip_levels,
            sample_count: desc.sample_count,
            sample_quality: desc.sample_quality,
            name,
        };
        let r = self.create_new_handle(
            render_pass,
            sync,
            access,
            ResourceTypeTexture {
                import: None,
                desc: create_desc,
            },
        );

        r
    }

    pub(crate) fn create_buffer_internal(
        &mut self,
        render_pass: usize,
        desc: &BufferDesc,
        sync: BarrierSync,
        access: ResourceUsageFlags,
    ) -> ResourceMut {
        debug_assert!(
            desc.usage.is_empty(),
            "The value of desc.usage is ignored, do not use it!"
        );
        debug_assert!(
            ResourceUsageFlags::BUFFER_USAGE_MASK.contains(access),
            "Attempting to declare non-buffer compatible access flags {:?}",
            access
        );

        let sync = get_given_or_default_sync_flags_for(access, sync, false, Default::default());
        let name = desc.name.map(|v| self.arena.alloc_str(v));
        let name = name.map(|v| NonNull::from(v));
        let create_desc = FrameGraphBufferDesc {
            size: desc.size,
            name,
        };
        let r = self.create_new_handle(
            render_pass,
            sync,
            access,
            ResourceTypeBuffer {
                import: None,
                desc: create_desc,
            },
        );

        r
    }

    /// Validate the write status for the resource handle and update it if it's valid to write
    /// to this handle.
    ///
    /// A ResourceMut can only be used for a single write_<resource> call as otherwise it would
    /// not be possible to extract a single program order. If two passes tried to write to the
    /// same resource with the same handle, which write should execute first? The only real
    /// solution would be to choose whichever pass was registered first, but then the graph order
    /// is dependent on pass submission order which is something we really _don't_ want.
    pub(crate) fn validate_and_update_for_handle_write(&mut self, r: ResourceMut) {
        let version_id = r.0.version_id() as usize;
        let version_info = &mut self.resource_versions[version_id];
        if version_info.is_written() {
            panic!("Attempted to write a resource through the same handle more than once!");
        }
        version_info.mark_written();
    }

    pub(crate) fn add_imported_resource_to_list(&mut self, r: impl Into<ResourceRef>) {
        let r = r.into();
        self.imported_resources.push(r.0.root_id());
    }

    /// Add the requested usage flags to the resource version's usage set
    pub(crate) fn add_flags_to_version_for(
        &mut self,
        r: impl Into<ResourceRef>,
        access: ResourceUsageFlags,
    ) {
        let r = r.into();
        let version_id = r.0.version_id();
        self.resource_versions[version_id as usize].version_total_access |= access;
    }

    /// Pull the version index from the resource ref and append a new read to the read list.
    pub(crate) fn append_read_to_version_for(
        &mut self,
        r: impl Into<ResourceRef>,
        render_pass: usize,
        sync: BarrierSync,
        access: ResourceUsageFlags,
    ) {
        let r = r.into();
        let version_id = r.0.version_id();
        let previous_read = self.resource_versions[version_id as usize].reads;
        let read = self.arena.alloc(VersionReaderLink {
            next: previous_read,
            render_pass,
            sync,
            access,
        });
        let read = NonNull::from(read);
        DropLink::append_drop_list(&self.arena, &mut self.drop_head, read);
        self.resource_versions[version_id as usize].reads = Some(read);
        self.resource_versions[version_id as usize].read_count += 1;
    }

    /// Add the requested usage flags to the resource root's  total usage set
    pub(crate) fn add_flags_to_root_for(
        &mut self,
        r: impl Into<ResourceRef>,
        access: ResourceUsageFlags,
    ) {
        let r = r.into();
        let root_id = r.0.root_id();
        self.root_resources[root_id as usize].total_access_flags |= access;
    }

    /// Constructs a new handle to describe the new version of a resource created by declaring a
    /// write within a render pass.
    pub(crate) fn increment_handle_for_write(
        &mut self,
        r: ResourceMut,
        render_pass: usize,
        sync: BarrierSync,
        access: ResourceUsageFlags,
    ) -> ResourceMut {
        let base = r.0.root_id();
        let version = u32::try_from(self.resource_versions.len()).unwrap();
        let id = ResourceId::new(base, version);
        self.resource_versions.push(ResourceVersion {
            // We need the root resource here to allow iterations over the version array to easily
            // link back to their roots
            root_resource: base,

            // A write will set the previous version to whatever was stored in the resource handle
            // we were given.
            previous_version: VersionIndex::new(r.0.version_id()).unwrap(),

            version_total_access: access,
            creator_sync: sync,
            creator_access: access,
            creator_pass: render_pass,
            read_count: 0,
            reads: None,
            debug_written: false,
        });
        self.root_resources[base as usize].final_version = VersionIndex(version);

        // Assert we never create u32::MAX versions. This is _critical_ as u32::MAX is a niche
        // value to encode a 'null' version. By guaranteeing u32::MAX is never a valid index into this
        // array we can rely on rust's bounds checking to assert that we never access the u32::MAXth
        // version by construction (we never make it).
        assert!(self.resource_versions.len() < ((u32::MAX) as usize));

        ResourceMut(id)
    }

    pub(crate) fn create_new_handle(
        &mut self,
        render_pass: usize,
        sync: BarrierSync,
        access: ResourceUsageFlags,
        r_type: impl Into<ResourceType>,
    ) -> ResourceMut {
        let base = u16::try_from(self.root_resources.len()).unwrap();
        let version = u32::try_from(self.resource_versions.len()).unwrap();
        let id = ResourceId::new(base, version);
        let version = VersionIndex::new(version).unwrap();
        self.root_resources.push(ResourceRoot {
            resource_type: r_type.into(),
            total_access_flags: access,
            final_version: version,
        });
        self.resource_versions.push(ResourceVersion {
            // We need the root resource here to allow iterations over the version array to easily
            // link back to their roots
            root_resource: base,

            // A create is by definition the first version of a resource so the 'previous' link is
            // initialized as an invalid id. This encodes the 'end' of the list
            previous_version: VersionIndex::INVALID,

            version_total_access: access,
            creator_sync: sync,
            creator_access: access,
            creator_pass: render_pass,
            read_count: 0,
            reads: None,
            debug_written: false,
        });

        // Assert we never create u16::MAX versions. This is _critical_ as u32::MAX is a niche
        // value to encode a 'null' version. By guaranteeing u32::MAX is never a valid index into this
        // array we can rely on rust's bounds checking to assert that we never access the u32::MAXth
        // version by construction (we never make it).
        assert!(self.resource_versions.len() < ((u16::MAX) as usize));

        ResourceMut(id)
    }

    pub(crate) fn assert_resource_handle_is_texture(
        &self,
        r: impl Into<ResourceRef>,
    ) -> &ResourceTypeTexture {
        let r = r.into();
        let root_type = &self.root_resources[r.0.root_id() as usize].resource_type;
        assert!(matches!(root_type, ResourceType::Texture(_)));
        root_type.unwrap_texture()
    }

    pub(crate) fn assert_resource_handle_is_buffer(
        &self,
        r: impl Into<ResourceRef>,
    ) -> &ResourceTypeBuffer {
        let r = r.into();
        let root_type = &self.root_resources[r.0.root_id() as usize].resource_type;
        assert!(matches!(root_type, ResourceType::Buffer(_)));
        root_type.unwrap_buffer()
    }

    /// Checks against imported resource's usage flags to ensure that no pass within the graph is
    /// using an imported resource in a usage it wasn't created to support.
    ///
    /// This will query the creation desc from the resource and assert that the sum of all usages
    /// within the graph is not a superset of the usages the resource was created to support.
    pub(crate) fn validate_imported_resource_usages(&self) {
        for root in self.root_resources.iter() {
            match &root.resource_type {
                ResourceType::Buffer(ResourceTypeBuffer {
                    import: Some(import),
                    desc,
                }) => {
                    let r_name =
                        unsafe { desc.name.map(|v| v.as_ref()).unwrap_or("Unnamed resource") };
                    let root_usage = root.total_access_flags;
                    assert!(
                        import.allowed_usage.contains(root_usage),
                        "Resource '{}' used in unsupported usage. Allowed: {:?}. Attempted: {:?}",
                        r_name,
                        import.allowed_usage,
                        root_usage
                    );
                }
                ResourceType::Texture(ResourceTypeTexture {
                    import: Some(import),
                    desc,
                }) => {
                    let r_name =
                        unsafe { desc.name.map(|v| v.as_ref()).unwrap_or("Unnamed resource") };
                    let root_usage = root.total_access_flags;
                    assert!(
                        import.allowed_usage.contains(root_usage),
                        "Resource '{}' used in unsupported usage. Allowed: {:?}. Attempted: {:?}",
                        r_name,
                        import.allowed_usage,
                        root_usage
                    );
                }
                _ => {}
            }
        }
    }
}

impl Drop for FrameGraphBuilder {
    fn drop(&mut self) {
        // Safety: implementation and API guarantees that dropper only gets called once per
        //         object, and always on the correct type.
        unsafe {
            DropLink::drop_and_null(&mut self.drop_head);
        }
    }
}

struct IRBuilder<'arena, 'b, 'c, T: std::io::Write> {
    /// A reference to a temporary bump allocated arena that should be used to store all temporary
    /// allocations that will not outlive the full frame graph build operation.
    ///
    /// That is: this arena will remain live for the scope of [FrameGraphBuilder::build_internal].
    arena: &'arena Bump,

    /// Our target writter for our graphviz output, if we have one.
    writer: Option<(&'b mut T, &'c GraphVizOutputOptions)>,

    /// Backing storage for the IR nodes that forms part of the output of this builder utility.
    ///
    /// After [IRBuilder::build] this will contain all the IR nodes that form the graph.
    ///
    /// There are some implicit rules for this list that can be exploited. It is guaranteed that,
    /// given 'n' render passes in a [FrameGraphBuilder], the first 'n' nodes in 'ir_nodes' will
    /// be [RenderPassIRNode] objects and the indices for those first 'n' nodes will be associative
    /// between [IRBuilder::ir_nodes] and [FrameGraphBuilder::render_passes].
    ///
    /// That is, given an index 'i': `ir_builder.ir_nodes[i]` will hold a [RenderPassIRNode] that
    /// points to `frame_graph_build.render_passes[i]`.
    nodes: Vec<IRNode>,

    /// A temporary list used as part of the build algorithm. This list is SoA mapped to each render
    /// pass node in the IR graph. Each entry in this list is another Vec (arena allocated) that
    /// will be used to accumulate the 'prev' edges of the render pass IR nodes before they get
    /// patched onto their IR nodes at the end of the build phase.
    pass_prevs: &'arena mut [BVec<'arena, usize>],

    /// See [IRBuilder::pass_prevs]. This is logically the same list, but instead used for storing
    /// the 'next' edges.
    pass_nexts: &'arena mut [BVec<'arena, usize>],
}

impl<'arena, 'b, 'c, T: std::io::Write> IRBuilder<'arena, 'b, 'c, T> {
    pub fn new(
        arena: &'arena Bump,
        writer: Option<(&'b mut T, &'c GraphVizOutputOptions)>,
        num_passes: usize,
    ) -> Self {
        let mut ir_nodes: Vec<IRNode> = Vec::with_capacity(num_passes);
        for i in 0..num_passes {
            let ir_node = RenderPassIRNode {
                prev: NonNull::from(&[]),
                next: NonNull::from(&[]),
                render_pass: i,
            };
            ir_nodes.push(ir_node.into());
        }

        let pass_prevs = arena.alloc_slice_fill_with(num_passes, |_| BVec::new_in(&arena));
        let pass_nexts = arena.alloc_slice_fill_with(num_passes, |_| BVec::new_in(&arena));

        Self {
            arena,
            writer,
            nodes: ir_nodes,
            pass_prevs,
            pass_nexts,
        }
    }

    pub fn build(&mut self, builder: &FrameGraphBuilder, graph_name: &str) -> std::io::Result<()> {
        self.emit_graph_viz_start(graph_name)?;

        // The first step is to emit all the barriers and graph edges by iterating through our
        // resource versions. We collect enough information that we can emit all our needed barriers
        // based on what's stored in the resource_versions array.
        //
        // The basic idea of the algorithm is that we can only emit barriers _between_ passes. This
        // means that you can't have a chain of edges like the following: barrier->pass->pass or
        // pass->barrier->barrier. What this means is that any piece of code emitting a barrier into
        // our IR graph only needs to know about the previous and next render pass, and doesn't need
        // to know _anything_ about the other barriers emitted elsewhere. This is the key constraint
        // that allows us to emit all barriers with a single iteration over resource_versions.
        //
        // The processor functions handle six fundamental conditions:
        // - Read after Write
        //   - Emitted between a writer pass and any reads to the newly created resource version
        // - Read after Read
        //   - A special case for images, creates a chain of reads with layout transitions
        // - Write after Write
        //   - Emitted between writer passes when a resource version has no read-only accessors
        // - Write after Read
        //   - Emitted between all a version's readers and the writer of the _next_ version of the
        //     resource.
        // - Import
        //   - A special barrier emitted between 'outside the graph' and the pass that imports a
        //     resource. Is always a 'write-after-x' barrier, with x depending on the external
        //     access declared when importing the resource.
        // - Export after Read
        //   - A special barrier emitted between the final reading passes of an imported resource
        //     and 'outside the graph'. This is an 'x-after-read' barrier, with x depending on the
        //     external access declared when importing the resource.
        // - Export after Write
        //   - Very similar to the 'Export after Read' case, but only used when the final version of
        //     a resource has no readers meaning the last pass to synchronize with is the final
        //     writer pass.
        // - Initialization
        //   - These barriers are emitted for the first usage of a transient resource. They are used
        //     to initialize resource metadata, namely for images, and are logically similar to
        //     import barriers but with a very vague 'before' scope for execution synchronization.
        //
        // All these conditions can be detected and correctly handled by considering only the
        // following:
        // - The information on the version we're considering.
        // - The information on the previous version of the resource we're considering, with the
        //   previous version identified by the initial version we're processing.
        // - A table to identify which resources are 'imported'.
        //
        // These pieces contain:
        // - All the passes that read a resource, including _how_ they are read
        // - The pass that writes (and thus creates) a resource version, including the sync flags
        // - The previous version (if any) of the resource
        //
        // That is all the information we need to emit our IR graph of barriers and passes. Pass
        // order is defined by the version order, we follow an SSA-like form for passes where
        // writing a resource creates a new handle that must be used to declare any access to the
        // created resource.
        for (version_index, version) in builder.resource_versions.iter().enumerate() {
            let version_index = VersionIndex(version_index as u32);
            let root = &builder.root_resources[version.root_resource as usize];
            match &root.resource_type {
                // Buffers are much simpler to handle as we don't need to care about image layout
                // transitions that promote read-after-read accesses to writes because of required
                // layout changes.
                ResourceType::Buffer(root_variant) => {
                    self.build_emit_barriers_for_buffer_version(
                        builder,
                        version,
                        version_index,
                        root,
                        root_variant,
                    )?;
                }
                ResourceType::Texture(root_variant) => {
                    self.build_emit_barriers_for_texture_version(
                        builder,
                        version,
                        version_index,
                        root,
                        root_variant,
                    )?;
                }
            }
        }

        // Render pass nodes need to have their edges patched in after we've emitted all our
        // barriers. We don't know the full number of edges each pass will need until we've fully
        // generated all our barriers so we need to use a dynamic array for these edge arrays. Only
        // once we've fully built the IR graph do we patch these arrays (once their size is static)
        // into the IR graph to get our fully formed graph.
        //
        // It is required for correctness that the vectors backing these lists to live as long as
        // the IR nodes. This is currently being guaranteed by how we store the vectors. Currently
        // we don't free the vectors at all, instead just relying on the bump allocator being freed
        // to do it for us. If this changes this code must be re-evaluated for correctness.
        for (i, _pass) in builder.render_passes.iter().enumerate() {
            let pass_node = &mut self.nodes[i];
            pass_node.set_prev(NonNull::from(self.pass_prevs[i].as_slice()));
            pass_node.set_next(NonNull::from(self.pass_nexts[i].as_slice()));
        }

        // The IR graph is now fully formed. If we're outputting a graphviz graph then we need to
        // emit the edges.
        if let Some((writer, options)) = self.writer.take() {
            for (i, ir_node) in self.nodes.iter().enumerate() {
                match ir_node {
                    IRNode::RenderPass(node) => {
                        let pass = &builder.render_passes[node.render_pass];
                        let pass_name = unsafe { pass.name.as_ref() };
                        writeln!(
                            writer,
                            "    node{i} [shape=box,label=\"Render Pass: \\\"{pass_name}\\\"\"];"
                        )?;
                    }
                    IRNode::Barrier(v) => {
                        self.emit_graph_viz_barrier_node(writer, builder, i, v)?
                    }
                    IRNode::LayoutChange(v) => {
                        self.emit_graph_viz_layout_change_node(writer, builder, i, v)?
                    }
                }
            }

            for (i, ir_node) in self.nodes.iter().enumerate() {
                let prevs = unsafe { ir_node.prev().as_ref() };
                let nexts = unsafe { ir_node.next().as_ref() };

                if options.output_previous_links {
                    for prev in prevs {
                        writeln!(writer, "    node{i} -> node{prev}")?;
                    }
                }
                for next in nexts {
                    writeln!(writer, "    node{i} -> node{next}")?;
                }
            }

            self.writer = Some((writer, options));
        }

        self.debug_graph_validation();

        self.emit_graph_viz_end()?;

        Ok(())
    }

    fn build_emit_barriers_for_buffer_version(
        &mut self,
        builder: &FrameGraphBuilder,
        version: &ResourceVersion,
        version_index: VersionIndex,
        root: &ResourceRoot,
        root_variant: &ResourceTypeBuffer,
    ) -> Result<(), std::io::Error> {
        if version.read_count > 0 {
            // We form a 'next' edge will all the reads to this buffer, collecting
            // the full set of usage/sync flags as the 'after' scope of the barrier.
            //
            // The before scope is defined by the access declared on the creator pass,
            // with that creator pass being the sole 'previous' edge for this barrier.
            let (barrier_next, all_read_sync, all_read_usage) =
                self.collect_read_flags_for_version(version);

            let barrier_prev = self.alloc_single_edge_list(version.creator_pass);
            self.emit_barrier_ir_node(
                IRBarrierType::ReadAfterWrite,
                barrier_prev,
                barrier_next,
                version_index,
                version.creator_sync,
                version.creator_access.barrier_access_for_write(),
                all_read_sync,
                all_read_usage.barrier_access_for_read(),
            )?;

            if let Some(import_desc) = root_variant.import.as_ref() {
                if root.final_version == version_index {
                    // The 'next' for the previous barrier becomes the 'prev' for the export
                    // barrier.
                    // We take a copy of the slice to avoid any potential surprises with two nodes
                    // sharing the same edge array.
                    let barrier_prev = self.arena.alloc_slice_copy(barrier_next);
                    self.emit_barrier_ir_node(
                        IRBarrierType::ExportAfterRead,
                        barrier_prev,
                        &[],
                        version_index,
                        all_read_sync,
                        all_read_usage.barrier_access_for_read(),
                        import_desc.after_sync,
                        import_desc.after_access,
                    )?;
                }
            }
        } else if let Some(import_desc) = root_variant.import.as_ref() {
            if root.final_version == version_index {
                let barrier_prev = self.alloc_single_edge_list(version.creator_pass);
                self.emit_barrier_ir_node(
                    IRBarrierType::ExportAfterWrite,
                    barrier_prev,
                    &[],
                    version_index,
                    version.creator_sync,
                    version.creator_access.barrier_access_for_write(),
                    import_desc.after_sync,
                    import_desc.after_access,
                )?;
            }
        }
        if version.previous_version.is_valid() {
            let previous_version_index = version.previous_version.0 as usize;
            let previous_version = &builder.resource_versions[previous_version_index];

            // If there are any reads on the previous resource version then we must emit
            // a write-after-read barrier between those reads and the subsequent write
            // that creates the new resource version.
            if previous_version.read_count > 0 {
                // We form a 'previous' edge for this barrier with all the reads to the
                // previous version of the resource. This also has us collect all the
                // sync/usage flags so we can define our 'before' sync scope of our
                // barrier.
                let (barrier_prev, all_read_sync, all_read_usage) =
                    self.collect_read_flags_for_version(previous_version);

                // The sole 'next' edge of the barrier is the creator of _this_ version
                // of the resource.
                //
                // We're creating a barrier between the previous reads and the pass that
                // writes out this new version of the resource. The 'after' sync scope
                // is easily derived from the pass's declared access flags.
                let barrier_next = self.alloc_single_edge_list(version.creator_pass);
                self.emit_barrier_ir_node(
                    IRBarrierType::WriteAfterRead,
                    barrier_prev,
                    barrier_next,
                    version.previous_version,
                    all_read_sync,
                    all_read_usage.barrier_access_for_read(),
                    version.creator_sync,
                    version.creator_access.barrier_access_for_write(),
                )?;
            } else {
                // This is one of the simplest barriers to emit, write-after-write.
                //
                // We have a simple 1:1 mapping from previous pass to next pass, with
                // the sync scopes trivially pulled from each pass's respective access
                // declarations.
                let barrier_prev = self.alloc_single_edge_list(previous_version.creator_pass);
                let barrier_next = self.alloc_single_edge_list(version.creator_pass);
                self.emit_barrier_ir_node(
                    IRBarrierType::WriteAfterWrite,
                    barrier_prev,
                    barrier_next,
                    version.previous_version,
                    previous_version.creator_sync,
                    previous_version.creator_access.barrier_access_for_write(),
                    version.creator_sync,
                    version.creator_access.barrier_access_for_write(),
                )?;
            }
        } else if let Some(import_desc) = &root_variant.import.as_ref() {
            // The final class of barrier, which is quite special, is an import barrier.
            // These are quite simple to implement and are emitted as a link between
            // usage outside of the graph and the first usage of the resource in the
            // graph.
            //
            // These are only needed for imported resources, and are simple to implement
            // as they always map 1:1 between 'external' and 'first-usage-pass'.

            // Only link is the 'next' link to the pass that imported the resource.
            //
            // An edge to 'external' is implicitly formed by having no previous links,
            // which also makes this a root node.
            //
            // Otherwise the 'before' scope is scooped directly from import desc and the
            // after scope is directly pulled from the importing pass's access
            // declaration.
            let barrier_next = self.alloc_single_edge_list(version.creator_pass);
            self.emit_barrier_ir_node(
                IRBarrierType::Import,
                &[],
                barrier_next,
                version_index,
                import_desc.before_sync,
                import_desc.before_access,
                version.creator_sync,
                version.creator_access.barrier_access_for_write(),
            )?;
        } else {
            let barrier_next = self.alloc_single_edge_list(version.creator_pass);
            self.emit_barrier_ir_node(
                IRBarrierType::Initialization,
                &[],
                barrier_next,
                version_index,
                BarrierSync::NONE,
                BarrierAccess::NONE,
                version.creator_sync,
                version.creator_access.barrier_access_for_write(),
            )?;
        }
        Ok(())
    }

    fn build_emit_barriers_for_texture_version(
        &mut self,
        builder: &FrameGraphBuilder,
        version: &ResourceVersion,
        version_index: VersionIndex,
        root: &ResourceRoot,
        root_variant: &ResourceTypeTexture,
    ) -> Result<(), std::io::Error> {
        if version.read_count > 0 {
            // First we need all the reads of this resource version in an array, sorted
            // by the required image layout.
            //
            // This forms the core of how we detect read-after-read image layout
            // transitions.
            let reads =
                version.reads_sorted_by_image_layout_in(root_variant.desc.format, self.arena);

            //
            // The next stage of the algorithm will iterate over the list of reads and
            // emit barriers for every layout change. This requires a bunch of state
            // to implement.
            //

            // This block is used to store the previous sync scope parameters that will
            // encompass the synchronization needed for the before scope of the next
            // barrier to be emitted.
            //
            // This is seeded from the render pass that created the resource version and
            // servers as the first edge in our chain of barriers.
            //
            // These values will be updated in our walk over the sorted reads list.
            let mut barrier_type = IRBarrierType::ReadAfterWrite;
            let mut before_sync = version.creator_sync;
            let mut before_access = version.creator_access.barrier_access_for_write();
            let mut before_layout = version
                .creator_access
                .image_layout(false, root_variant.desc.format);

            // This stores the current layout we're expecting to see. This is used to
            // detect a layout change in our reads list.
            let mut current_layout = reads[0].1;

            // This list is used to store which reads are in the previous read batch. It
            // is used when handling all read batches after the first and is used for
            // determining the 'previous' edges for a barrier. This list stores indices
            // into the 'reads' array.
            //
            // Every time a barrier is emitted we drain 'pending_reads' into
            // 'previous_reads' as, obviously, what was the 'pending_reads' are now the
            // 'previous_reads'.
            let mut previous_reads: BVec<usize> = BVec::with_capacity_in(reads.len(), self.arena);

            // This list is used to accumulate the pending reads when we're still
            // searching for layout edges. Once a layout edge is found this will contain
            // the set of reads that form a read batch to emit a barrier for.
            //
            // Once a barrier is emitted we flush this into 'previous_reads'
            let mut pending_reads: BVec<usize> = BVec::with_capacity_in(reads.len(), self.arena);

            let mut iter = reads.iter().enumerate().peekable();
            while let Some((read_i, (_, layout))) = iter.next() {
                // We detect a layout change by comparing the expected layout
                // 'current_layout' with the image layout on the current read. If they
                // differ then we have found a layout edge and we need to emit a barrier
                let layout_changed = *layout != current_layout;

                // We also have a special edge to handle, when we've reached the end of
                // the read set. We won't detect the final set of reads naively, as
                // we'll just walk off the end of the read set instead of detecting a
                // layout change. To handle this edge case we also detect when we've hit
                // the end of the read set and emit a barrier too.
                let last_batch = iter.peek().is_none();
                if layout_changed || last_batch {
                    // The 'pending_reads' list will not contain the current read as we
                    // rely on the previous loop iteration to insert the indices in all
                    // other cases. To handle this edge case we insert it early so the
                    // rest of the code doesn't need to know if we're in the final read
                    // batch.
                    if last_batch {
                        pending_reads.push(read_i);
                    }

                    // Walk our list of pending reads that we're about to issue a
                    // barrier for to allow them to execute. Here we accumulate the sync
                    // flags and access flags so we know our 'after' sync scope. We also
                    // add 'next' edges for the reading passes to our barrier.
                    let mut pending_read_sync = BarrierSync::NONE;
                    let mut pending_read_access = ResourceUsageFlags::NONE;
                    let barrier_next = self.arena.alloc_slice_fill_copy(pending_reads.len(), 0);
                    for (pending_read_i, next) in
                        pending_reads.iter().copied().zip(barrier_next.iter_mut())
                    {
                        let (read, _) = reads[pending_read_i];
                        pending_read_sync |= read.sync;
                        pending_read_access |= read.access;
                        *next = read.render_pass;
                    }

                    // Next we collect our 'previous' links for the barrier. If we're
                    // emitting a barrier for the first read batch then our 'before'
                    // sync scope is actually a write access from the pass that created
                    // the resource, otherwise the 'before' scope is equal to the
                    // 'after' scope of the previous read batch.
                    //
                    // We can detect this 'first batch' by checking if 'previous_reads'
                    // is empty, as previous_reads can only be empty when processing the
                    // first batch.
                    let barrier_prev = if previous_reads.is_empty() {
                        // Single link to creator render pass
                        self.arena.alloc_slice_copy(&[version.creator_pass])
                    } else {
                        // Link to every read access scheduled in the previous read
                        // batch
                        self.arena.alloc_slice_fill_iter(
                            previous_reads.drain(..).map(|v| reads[v].0.render_pass),
                        )
                    };

                    // We now emit the barrier
                    self.emit_layout_change_ir_node(
                        barrier_type,
                        barrier_prev,
                        barrier_next,
                        version_index,
                        before_sync,
                        before_access,
                        before_layout,
                        pending_read_sync,
                        pending_read_access.barrier_access_for_read(),
                        current_layout,
                    )?;

                    // What _was_ our pending reads in this batch becomes the previous
                    // reads for the next batch
                    previous_reads.clear();
                    previous_reads.extend(pending_reads.drain(..));

                    // And following on, what _was_ our 'after' sync scope for this
                    // barrier becomes our 'before' sync scope for the next barrier.
                    barrier_type = IRBarrierType::ReadAfterRead;
                    before_sync = pending_read_sync;
                    before_access = pending_read_access.barrier_access_for_read();
                    before_layout = current_layout;

                    // Lastly we change what the expected layout is so we can keep
                    // walking until we find the next layout edge.
                    current_layout = *layout;
                }
                // And finally, we add the current read to the pending reads set. If we
                // handled a layout transition above then nothing that affects the read
                // identified by 'read_i' will have been done. That read will still be
                // pending processing which will be handled when we hit the next layout
                // transition.
                //
                // The one exception is when we hit the end of the iterator. That is
                // handled specially in the code above us and this push here is not
                // observable as far as this loop is concerned so it doesn't matter if
                // we push read_i again even if we just handled it above.
                pending_reads.push(read_i);
            }

            if let Some(import_desc) = root_variant.import.as_ref() {
                if root.final_version == version_index {
                    // The 'next' for the previous barrier becomes the 'prev' for the export
                    // barrier.
                    let barrier_prev = self.arena.alloc_slice_fill_iter(previous_reads.drain(..));
                    self.emit_layout_change_ir_node(
                        IRBarrierType::ExportAfterRead,
                        barrier_prev,
                        &[],
                        version_index,
                        before_sync,
                        before_access,
                        before_layout,
                        import_desc.after_sync,
                        import_desc.after_access,
                        import_desc.after_layout,
                    )?;
                }
            }
        } else if let Some(import_desc) = root_variant.import.as_ref() {
            if root.final_version == version_index {
                let barrier_prev = self.alloc_single_edge_list(version.creator_pass);
                self.emit_layout_change_ir_node(
                    IRBarrierType::ExportAfterWrite,
                    barrier_prev,
                    &[],
                    version_index,
                    version.creator_sync,
                    version.creator_access.barrier_access_for_write(),
                    version
                        .creator_access
                        .image_layout(false, root_variant.desc.format),
                    import_desc.after_sync,
                    import_desc.after_access,
                    import_desc.after_layout,
                )?;
            }
        }
        if version.previous_version.is_valid() {
            let previous_version_index = version.previous_version.0 as usize;
            let previous_version = &builder.resource_versions[previous_version_index];

            // If there are any reads on the previous resource version then we must emit
            // a write-after-read barrier between those reads and the subsequent write
            // that creates the new resource version.
            //
            // Images are special in this case where we only need a write-after-read
            // edge to the last 'read batch'. See the read-after-read barrier for more
            // indepth discussion on 'read batches', but this forms the last part of
            // handling read barriers. In short we handle layout transitions on image
            // resources by making chains of read-after-read barriers to perform layout
            // changes. We only link the write-after-read barrier to the final read
            // batch.
            if previous_version.read_count > 0 {
                // First we need all the reads of this resource version in an array,
                // sorted by the required image layout.
                //
                // This forms the core of how we determine which read batch is the last
                // one to be scheduled.
                //
                // It is _absolutely_ critical that this produces the exact same
                // ordering as what is produced when handling read-after-read barriers
                // so we can correctly determine the previous passes to form 'previous'
                // edges to.
                let reads = previous_version
                    .reads_sorted_by_image_layout_in(root_variant.desc.format, self.arena);

                // First we need to get the image layout of the last read batch and
                // find out how many reads are in that read batch. This is trivially
                // done by first grabbing the layout of the last element and walking
                // backwards over the array until we find a layout change. The number
                // of steps we take is the number of reads in the final read batch.
                let mut num_reads_for_prev = 0;
                let last_read_layout = reads.last().unwrap().1;
                for (_, l) in reads.iter().rev() {
                    if *l != last_read_layout {
                        break;
                    }
                    num_reads_for_prev += 1;
                }

                // With the number of reads known we can allocate the barrier_prev array
                // and fill out the 'prev' links and accumulate the sync flags.
                let barrier_prev = self.arena.alloc_slice_fill_copy(num_reads_for_prev, 0);
                let mut all_read_sync = BarrierSync::default();
                let mut all_read_usage = ResourceUsageFlags::default();
                for ((v, _), prev) in reads
                    .iter()
                    .rev()
                    .take(num_reads_for_prev)
                    .zip(barrier_prev.iter_mut())
                {
                    all_read_sync |= v.sync;
                    all_read_usage |= v.access;
                    *prev = v.render_pass;
                }

                // The 'next' link is always to this resource version's creator. The
                // before sync scope is defined by the read accesses from the last read
                // batch, and the after scope is pulled from the destination render
                // pass's declared access.
                let barrier_next = self.alloc_single_edge_list(version.creator_pass);
                self.emit_layout_change_ir_node(
                    IRBarrierType::WriteAfterRead,
                    barrier_prev,
                    barrier_next,
                    version.previous_version,
                    all_read_sync,
                    all_read_usage.barrier_access_for_read(),
                    last_read_layout,
                    version.creator_sync,
                    version.creator_access.barrier_access_for_write(),
                    version
                        .creator_access
                        .image_layout(false, root_variant.desc.format),
                )?;
            } else {
                // This is one of the simplest barriers to emit, write-after-write.
                //
                // We have a simple 1:1 mapping from previous pass to next pass, with
                // the sync scopes trivially pulled from each pass's respective access
                // declarations.
                let barrier_prev = self.alloc_single_edge_list(previous_version.creator_pass);
                let barrier_next = self.alloc_single_edge_list(version.creator_pass);
                self.emit_layout_change_ir_node(
                    IRBarrierType::WriteAfterWrite,
                    barrier_prev,
                    barrier_next,
                    version.previous_version,
                    previous_version.creator_sync,
                    previous_version.creator_access.barrier_access_for_write(),
                    previous_version
                        .creator_access
                        .image_layout(false, root_variant.desc.format),
                    version.creator_sync,
                    version.creator_access.barrier_access_for_write(),
                    version
                        .creator_access
                        .image_layout(false, root_variant.desc.format),
                )?;
            }
        } else if let Some(import_desc) = &root_variant.import.as_ref() {
            // The final class of barrier, which is quite special, is an import barrier.
            // These are quite simple to implement and are emitted as a link between
            // usage outside of the graph and the first usage of the resource in the
            // graph.
            //
            // These are only needed for imported resources, and are simple to implement
            // as they always map 1:1 between 'external' and 'first-usage-pass'.

            // Only link is the 'next' link to the pass that imported the resource.
            //
            // An edge to 'external' is implicitly formed by having no previous links,
            // which also makes this a root node.
            //
            // Otherwise the 'before' scope is scooped directly from import desc and the
            // after scope is directly pulled from the importing pass's access
            // declaration.
            let barrier_next = self.arena.alloc_slice_copy(&[version.creator_pass]);
            self.emit_layout_change_ir_node(
                IRBarrierType::Import,
                &[],
                barrier_next,
                version_index,
                import_desc.before_sync,
                import_desc.before_access,
                import_desc.before_layout,
                version.creator_sync,
                version.creator_access.barrier_access_for_write(),
                version
                    .creator_access
                    .image_layout(false, root_variant.desc.format),
            )?;
        } else {
            let barrier_next = self.alloc_single_edge_list(version.creator_pass);
            self.emit_layout_change_ir_node(
                IRBarrierType::Initialization,
                &[],
                barrier_next,
                version_index,
                BarrierSync::NONE,
                BarrierAccess::NONE,
                ImageLayout::Undefined,
                version.creator_sync,
                version.creator_access.barrier_access_for_write(),
                version
                    .creator_access
                    .image_layout(false, root_variant.desc.format),
            )?;
        }
        Ok(())
    }

    fn alloc_single_edge_list(&self, v: usize) -> &'arena mut [usize] {
        self.arena.alloc_slice_copy(&[v])
    }

    fn collect_read_flags_for_version(
        &self,
        v: &ResourceVersion,
    ) -> (&'arena mut [usize], BarrierSync, ResourceUsageFlags) {
        let mut sync = BarrierSync::default();
        let mut usage = ResourceUsageFlags::default();
        let edges = self.arena.alloc_slice_fill_copy(v.read_count, 0);
        for (i, read) in v.reads_iter().enumerate() {
            sync |= read.sync;
            usage |= read.access;
            edges[i] = read.render_pass;
        }

        (edges, sync, usage)
    }

    /// Output the start of the DOT graph if we have a writer
    fn emit_graph_viz_start(&mut self, graph_name: &str) -> std::io::Result<()> {
        if let Some((v, _options)) = self.writer.as_mut() {
            writeln!(v, "digraph {graph_name} {{")?;
        }
        Ok(())
    }

    /// Output the end of the DOT graph if we have a writer
    fn emit_graph_viz_end(&mut self) -> std::io::Result<()> {
        if let Some((v, _options)) = self.writer.as_mut() {
            writeln!(v, "}}")?;
        }
        Ok(())
    }

    fn get_resource_name_for_version_index(
        &self,
        builder: &FrameGraphBuilder,
        version: VersionIndex,
    ) -> &str {
        let version = &builder.resource_versions[version.0 as usize];
        let root_index = version.root_resource as usize;
        let root_resource = &builder.root_resources[root_index];
        match &root_resource.resource_type {
            ResourceType::Buffer(v) => v
                .desc
                .name
                .map(|v| unsafe { v.as_ref() })
                .unwrap_or("Unnamed Resource"),
            ResourceType::Texture(v) => v
                .desc
                .name
                .map(|v| unsafe { v.as_ref() })
                .unwrap_or("Unnamed Resource"),
        }
    }

    fn emit_graph_viz_barrier_node(
        &self,
        writer: &'b mut T,
        builder: &FrameGraphBuilder,
        barrier_ir_node_index: usize,
        ir_node: &BarrierIRNode,
    ) -> std::io::Result<()> {
        let resource_name = self.get_resource_name_for_version_index(builder, ir_node.version);
        write!(writer, "    ")?;
        ir_node.write_graph_viz(writer, resource_name, barrier_ir_node_index)
    }

    fn emit_graph_viz_layout_change_node(
        &self,
        writer: &'b mut T,
        builder: &FrameGraphBuilder,
        barrier_ir_node_index: usize,
        ir_node: &LayoutChangeIRNode,
    ) -> std::io::Result<()> {
        let resource_name = self.get_resource_name_for_version_index(builder, ir_node.version);
        write!(writer, "    ")?;
        ir_node.write_graph_viz(writer, resource_name, barrier_ir_node_index)
    }

    /// Internal function used for inserting new barrier IR nodes into the frame graph.
    ///
    /// # Safety
    ///
    /// This function itself is not unsafe to call, but the caller _must_ ensure that the
    /// barrier_prev and barrier_next arrays are backed by allocations that outlive the graph. They
    /// are cast to raw pointers inside [BarrierIRNode], so to safely dereference those pointers the
    /// allocations have to life long enough.
    ///
    /// Use an arena, or just leak memory. Absoultely do _not_ store these on the stack.
    ///
    /// There is a _single_ exception, the empty array. The empty array will not dereference the
    /// pointer as there's no elements to load. No allocation is needed at all for these arrays.
    fn emit_barrier_ir_node(
        &mut self,
        barrier_type: IRBarrierType,
        barrier_prev: &'arena [usize],
        barrier_next: &'arena [usize],
        version: VersionIndex,
        before_sync: BarrierSync,
        before_access: BarrierAccess,
        after_sync: BarrierSync,
        after_access: BarrierAccess,
    ) -> std::io::Result<usize> {
        // Current length of the ir_node buffer will become the index of the node we insert
        let ir_node_index = self.nodes.len();

        // Add the second half of our double linked graph. We only defined the links out of the new
        // IR node, we need to patch the new links into the new node's linked nodes.
        //
        // We assume that a barrier node will only link to render pass nodes. This means we can
        // just insert the ir_node_index into the vec stored in the pass_nexts/pass_prevs arrays by
        // indexing with the new ir node's outward link indices.
        for prev in barrier_prev.iter().copied() {
            self.pass_nexts[prev].push(ir_node_index);
        }
        for next in barrier_next.iter().copied() {
            self.pass_prevs[next].push(ir_node_index);
        }

        let ir_node = BarrierIRNode {
            prev: NonNull::from(barrier_prev),
            next: NonNull::from(barrier_next),
            version,
            barrier_type,
            before_sync,
            before_access,
            after_sync,
            after_access,
        };

        self.nodes.push(ir_node.into());

        Ok(ir_node_index)
    }

    /// Internal function used for inserting new barrier IR nodes into the frame graph.
    ///
    /// # Safety
    ///
    /// This function itself is not unsafe to call, but the caller _must_ ensure that the
    /// barrier_prev and barrier_next arrays are backed by allocations that outlive the graph. They
    /// are cast to raw pointers inside [BarrierIRNode], so to safely dereference those pointers the
    /// allocations have to life long enough.
    ///
    /// Use an arena, or just leak memory. Absoultely do _not_ store these on the stack.
    ///
    /// There is a _single_ exception, the empty array. The empty array will not dereference the
    /// pointer as there's no elements to load. No allocation is needed at all for these arrays.
    fn emit_layout_change_ir_node(
        &mut self,
        barrier_type: IRBarrierType,
        barrier_prev: &[usize],
        barrier_next: &[usize],
        version: VersionIndex,
        before_sync: BarrierSync,
        before_access: BarrierAccess,
        before_layout: ImageLayout,
        after_sync: BarrierSync,
        after_access: BarrierAccess,
        after_layout: ImageLayout,
    ) -> std::io::Result<usize> {
        // Current length of the ir_node buffer will become the index of the node we insert
        let ir_node_index = self.nodes.len();

        // Add the second half of our double linked graph. We only defined the links out of the new
        // IR node, we need to patch the new links into the new node's linked nodes.
        //
        // We assume that a barrier node will only link to render pass nodes. This means we can
        // just insert the ir_node_index into the vec stored in the pass_nexts/pass_prevs arrays by
        // indexing with the new ir node's outward link indices.
        for prev in barrier_prev.iter().copied() {
            self.pass_nexts[prev].push(ir_node_index);
        }
        for next in barrier_next.iter().copied() {
            self.pass_prevs[next].push(ir_node_index);
        }

        let ir_node = LayoutChangeIRNode {
            prev: NonNull::from(barrier_prev),
            next: NonNull::from(barrier_next),
            version,
            barrier_type,
            before_sync,
            before_access,
            before_layout,
            after_sync,
            after_access,
            after_layout,
        };

        self.nodes.push(ir_node.into());

        Ok(ir_node_index)
    }

    fn debug_graph_validation(&mut self) {
        if !cfg!(debug_assertions) {
            return;
        }

        for node in self.nodes.iter() {
            match node {
                IRNode::RenderPass(v) => {
                    self.assert_edges_link_to_barriers(v);
                }
                IRNode::Barrier(v) => {
                    self.assert_edges_link_to_render_passes(v);
                }
                IRNode::LayoutChange(v) => {
                    self.assert_edges_link_to_render_passes(v);
                }
            }
        }
    }

    fn assert_edges_link_to_barriers(&self, v: &impl IIRNode) {
        debug_assert!(v.is_render_pass());

        let next = unsafe { v.next().as_ref() };
        for next in next.iter().copied() {
            assert!(self.nodes[next].is_barrier());
        }

        let prev = unsafe { v.prev().as_ref() };
        for prev in prev.iter().copied() {
            assert!(self.nodes[prev].is_barrier());
        }
    }

    fn assert_edges_link_to_render_passes(&self, v: &impl IIRNode) {
        debug_assert!(v.is_barrier());

        let next = unsafe { v.next().as_ref() };
        for next in next.iter().copied() {
            assert!(self.nodes[next].is_render_pass());
        }

        let prev = unsafe { v.prev().as_ref() };
        for prev in prev.iter().copied() {
            assert!(self.nodes[prev].is_render_pass());
        }
    }
}

struct PassOrderBuilder<'arena> {
    /// A reference to a temporary bump allocated arena that should be used to store all temporary
    /// allocations that will not outlive the full frame graph build operation.
    ///
    /// That is: this arena will remain live for the scope of [FrameGraphBuilder::build_internal].
    arena: &'arena Bump,

    /// A temporary buffer that associates with ir.nodes that flags whether the matching node has
    /// been scheduled or not.
    node_scheduled: &'arena mut [bool],

    /// A temporary buffer that associates with ir.nodes that counts the number of 'prev' links
    /// for each node have yet to be scheduled. This helps know when we can schedule an ir pass
    /// when building our pass order.
    waiting_on_nums: &'arena mut [usize],

    /// A temporary buffer used in the scheduling loop that is associatively mapped to ir.nodes. It
    /// is expected to contain the number of 'prev' nodes for the associated ir node that are
    /// immediately ready to execute in a given scheduler loop iteration.
    runnable_prev_nums: &'arena mut [usize],

    /// This list is our primary output, and contains the ordered list of execution bundles.
    bundles: Vec<PassOrderBundle>,
}

impl<'arena> PassOrderBuilder<'arena> {
    pub fn new<T: std::io::Write>(arena: &'arena Bump, ir: &IRBuilder<'_, '_, '_, T>) -> Self {
        let node_scheduled: &mut [bool] = arena.alloc_slice_fill_default(ir.nodes.len());

        let waiting_on_nums = arena
            .alloc_slice_fill_iter(ir.nodes.iter().map(|v| unsafe { v.prev().as_ref().len() }));

        let runnable_prev_nums: &mut [usize] = arena.alloc_slice_fill_default(ir.nodes.len());

        Self {
            arena,
            node_scheduled,
            waiting_on_nums,
            runnable_prev_nums,
            bundles: Vec::new(),
        }
    }

    pub fn build<T: std::io::Write>(
        &mut self,
        graph_builder: &FrameGraphBuilder,
        ir: &IRBuilder<'arena, '_, '_, T>,
    ) {
        let (roots, leafs) = self.find_root_and_leaf_nodes(graph_builder, ir);
        let barrier_type_counts = Self::sum_barrier_type_counts(ir);

        let (import_barriers, export_barriers) = self.schedule_import_and_export_barriers(
            graph_builder,
            ir,
            &barrier_type_counts,
            &roots,
            &leafs,
        );

        self.bundles.push(PassOrderBundle {
            barriers: NonNull::from(import_barriers.as_slice()),
            passes: NonNull::from(&[]),
        });

        self.schedule_passes(graph_builder, ir);

        self.bundles.push(PassOrderBundle {
            barriers: NonNull::from(export_barriers.as_slice()),
            passes: NonNull::from(&[]),
        });

        self.validate_bundles(graph_builder, ir);
    }

    fn schedule_passes<T: std::io::Write>(
        &mut self,
        graph_builder: &FrameGraphBuilder,
        ir: &IRBuilder<'_, '_, '_, T>,
    ) {
        // List that stores the candidates we should consider in a loop iteration. This is
        // initialized to contain all the remaining unscheduled nodes in the graph. The algorithm
        // works by looping over the candidates and progressively scheduling them as they become
        // executable. We keep looping until the candidates list is empty.
        let mut candidates = BVec::with_capacity_in(ir.nodes.len(), self.arena);

        // This list is used to while looping. Instead of erasing from 'candidates' in the loop
        // (slow) we drain from 'candidates' and move into 'next_candidates' selectively. Scheduled
        // nodes aren't added to 'next_candidates' which functionally removes them from the
        // candidate set when we drain back from 'next_candidates' into 'candidates'
        let mut next_candidates = BVec::with_capacity_in(ir.nodes.len(), self.arena);

        // Temporary buffer we accumulate barriers scheduled in a single loop iteration into. This
        // is drained at the end of the iteration into a execution bundle, forming the barrier half
        // of the execution bundle.
        let mut barriers = BVec::with_capacity_in(16, self.arena);

        // Logically the same as 'barriers', but for
        let mut passes = BVec::with_capacity_in(8, self.arena);

        // Add any unscheduled nodes to the candidate set
        for (i, _) in ir.nodes.iter().enumerate() {
            if !self.node_scheduled[i] {
                candidates.push(i);
            }
        }
        debug_assert!(is_sorted(candidates.as_slice()));
        loop {
            // We want to ensure that no scheduled nodes are in the candidate set
            for &candidate in candidates.iter() {
                debug_assert!(!self.node_scheduled[candidate]);
            }

            // We also require that the candidate stays sorted so that we always process the render
            // pass IR nodes first whenever we iterate. More on that later...
            debug_assert!(is_sorted(candidates.as_slice()));

            // Clear the 'runnable_prev_nums' entries so we can re-accumulate the number of
            // runnable dependencies for each node.
            //
            // Next we iterate over all the barriers that are still run candidates and update the
            // 'runnable_prev_nums' slot for their next links if they can be executed. This is
            // important because we only scheduled barriers if they allow a render pass to be
            // executed in the same bundle as the barrier.
            //
            // Doing this keeps resource lifetimes constrained as otherwise initialization barriers
            // would execute long before the first usage of a resource, extending the lifetime of
            // the resources signifcantly which would massively reduce opporunities for resource
            // aliasing.
            self.runnable_prev_nums.fill(0);
            for &candidate in candidates.iter() {
                match &ir.nodes[candidate] {
                    IRNode::RenderPass(_) => {}
                    IRNode::Barrier(node_variant) => {
                        self.check_node_can_run_and_update_runnable_counts(node_variant, candidate);
                    }
                    IRNode::LayoutChange(node_variant) => {
                        self.check_node_can_run_and_update_runnable_counts(node_variant, candidate);
                    }
                }
            }

            // The second phase of the algorithm has us now checking our renderpass nodes. In the
            // previous phase we summed up which barriers are immediately ready to execute. In this
            // next phase we use this information to determine which render passes are ready to be
            // run given we schedule the barriers they're still waiting on in this execution bundle.
            //
            // This is determined by comparing the 'runnable_prev_num' with the 'waiting_on_num'.
            // When these two values for a pass are equal it means that all oustanding prev nodes
            // for a pass are ready, so we schedule the pass into the 'passes' set and the barriers
            // into the 'barriers' set. The barriers in the same bundle always run first so
            // execution order is maintained.
            for candidate in candidates.drain(..) {
                let should_keep = match &ir.nodes[candidate] {
                    IRNode::RenderPass(node_variant) => {
                        // We can tell if the render pass is ready to be run by subtracting the
                        // number of runnable dependencies (barriers) from the number of
                        // dependencies the pass is still waiting on.
                        //
                        // Once that reaches zero then the pass is ready to be run. We then proceed
                        // to schedule the pass and schedule the remaining runnable dependencies.
                        //
                        // This works because we guarantee that all prev links for a pass node point
                        // to barriers. Any prevs are scheduled into the barriers pool of the
                        // current execuction bundle.
                        let runnable_prev_num = self.runnable_prev_nums[candidate];
                        let waiting_on_num = self.waiting_on_nums[candidate];
                        if runnable_prev_num >= waiting_on_num {
                            self.schedule_pass_barriers(&mut barriers, ir, node_variant);

                            passes.push(candidate);
                            self.mark_node_as_scheduled(node_variant, candidate);

                            false
                        } else {
                            true
                        }
                    }
                    IRNode::Barrier(_) | IRNode::LayoutChange(_) => {
                        // It is important to talk about how this works. The above code for
                        // RenderPass may schedule some barriers to execute in this bundle. We must
                        // filter these barriers out of the candidate set, but the only mechanism
                        // we have for doing this is the 'should_keep' flag. Naively this would mean
                        // we would need an extra iteration over candidates to fully drain the set
                        // of all scheduled nodes. This is not the case for us though.
                        //
                        // We carefully exploit some implicit properties of the layout of ir.nodes
                        // and the candidate list. Firstly, the candidate list must always remain
                        // sorted. This guarantees when iterating it we always handle lower node
                        // indices first in this loop. Secondly, we rely on the fact that the first
                        // 'n' nodes in ir.nodes will always be the 'n' render pass nodes. What
                        // this means is that given a sorted list of indices we are guaranteed to
                        // always process the render pass nodes first. Exactly what happens in this
                        // loop.
                        //
                        // The result is that we always process the passes first, meaning by the
                        // time we're hitting this branch of the match expression we're guaranteed
                        // to have processed all the remaining candidate render pass nodes. This
                        // means that all the scheduled _barriers_ will be flagged now, so this
                        // match branch will correctly filter all the scheduled barriers from the
                        // candidate set.
                        //
                        // It is critical we retain these two properties, otherwise we would need
                        // another iteration to do the final filter on the 'node_scheduled' flag.
                        !self.node_scheduled[candidate]
                    }
                };

                if should_keep {
                    next_candidates.push(candidate);
                }
            }

            // next_candidates becomes the candidates array for the next loop iteration.
            std::mem::swap(&mut candidates, &mut next_candidates);

            // We now take the list of barriers and passes for this iteration and copy them into
            // arrays that are allocated from the frame graph's arena so they're safe to outlive
            // the graph builder.
            let bundle_barriers = graph_builder.arena.alloc_slice_copy(&barriers);
            let bundle_passes = graph_builder.arena.alloc_slice_copy(&passes);
            self.bundles.push(PassOrderBundle {
                barriers: NonNull::from(bundle_barriers),
                passes: NonNull::from(bundle_passes),
            });
            barriers.clear();
            passes.clear();

            // We keep looping until all nodes have been scheduled, which is known when candidates
            // is empty after a loop iteration.
            if candidates.is_empty() {
                break;
            }
        }
    }

    fn schedule_pass_barriers<T: std::io::Write>(
        &mut self,
        barriers: &mut BVec<'_, usize>,
        ir: &IRBuilder<'_, '_, '_, T>,
        node_variant: &RenderPassIRNode,
    ) {
        let prevs = unsafe { node_variant.prev.as_ref() };
        for &prev in prevs {
            let prev_can_run = self.waiting_on_nums[prev] == 0;
            let prev_already_run = self.node_scheduled[prev];
            if prev_can_run && !prev_already_run {
                barriers.push(prev);
                self.mark_node_as_scheduled(&ir.nodes[prev], prev);
            }
        }
    }

    fn schedule_import_and_export_barriers<'graph, T: std::io::Write>(
        &mut self,
        graph_builder: &'graph FrameGraphBuilder,
        ir: &IRBuilder<'arena, '_, '_, T>,
        barrier_type_counts: &[usize; 8],
        roots: &[usize],
        leafs: &[usize],
    ) -> (BVec<'graph, usize>, BVec<'graph, usize>) {
        let num_export_barriers = barrier_type_counts[IRBarrierType::ExportAfterRead as usize]
            + barrier_type_counts[IRBarrierType::ExportAfterWrite as usize];

        let mut import_barriers: BVec<usize> = BVec::with_capacity_in(
            barrier_type_counts[IRBarrierType::Import as usize],
            &graph_builder.arena,
        );
        let mut export_barriers: BVec<usize> =
            BVec::with_capacity_in(num_export_barriers, &graph_builder.arena);
        for node_index in roots.iter().copied() {
            let root_node = &ir.nodes[node_index];
            match root_node {
                IRNode::RenderPass(_) => {}
                IRNode::Barrier(node_variant) => match node_variant.barrier_type {
                    IRBarrierType::Import => {
                        import_barriers.push(node_index);
                        self.mark_node_as_scheduled(node_variant, node_index);
                    }
                    _ => {}
                },
                IRNode::LayoutChange(node_variant) => match node_variant.barrier_type {
                    IRBarrierType::Import => {
                        import_barriers.push(node_index);
                        self.mark_node_as_scheduled(node_variant, node_index);
                    }
                    _ => {}
                },
            }
        }
        for node_index in leafs.iter().copied() {
            let root_node = &ir.nodes[node_index];
            match root_node {
                IRNode::RenderPass(_) => {}
                IRNode::Barrier(node_variant) => match node_variant.barrier_type {
                    IRBarrierType::ExportAfterRead | IRBarrierType::ExportAfterWrite => {
                        export_barriers.push(node_index);
                        self.mark_node_as_scheduled(node_variant, node_index);
                    }
                    _ => {}
                },
                IRNode::LayoutChange(node_variant) => match node_variant.barrier_type {
                    IRBarrierType::ExportAfterRead | IRBarrierType::ExportAfterWrite => {
                        export_barriers.push(node_index);
                        self.mark_node_as_scheduled(node_variant, node_index);
                    }
                    _ => {}
                },
            }
        }
        (import_barriers, export_barriers)
    }

    fn mark_node_as_scheduled(&mut self, node: &impl IIRNode, node_index: usize) {
        // Sets the scheduled flag to true for this node, followed by decrementing the wait num for
        // all the 'next' edges of this node.
        self.node_scheduled[node_index] = true;
        unsafe {
            let node_nexts = node.next().as_ref();
            for &next in node_nexts {
                self.waiting_on_nums[next] -= 1;

                // We also decrement the 'runnable_prev_nums' count so that all the 'next' edges we
                // process this for don't think that there are more runnable nodes than there really
                // are.
                //
                // Saturating arithmetic is critical here as it's possible for us to call this on a
                // 0 value. Rust's plain arithmetic would underflow (or assert on debug) which would
                // be a big problem as it would affectively mean any pass that underflowed would
                // look ready to run. To save future pain we use saturating arithmetic so it just
                // clamps to zero.
                self.runnable_prev_nums[next] = self.runnable_prev_nums[next].saturating_sub(1);
            }
        }
    }

    fn check_node_can_run_and_update_runnable_counts(
        &mut self,
        node: &impl IIRNode,
        node_index: usize,
    ) {
        let can_execute = self.waiting_on_nums[node_index] == 0;
        if can_execute {
            let node_nexts = unsafe { node.next().as_ref() };
            for &next in node_nexts {
                self.runnable_prev_nums[next] += 1;
            }
        }
    }

    fn sum_barrier_type_counts<T: std::io::Write>(ir: &IRBuilder<'_, '_, '_, T>) -> [usize; 8] {
        // This loop will sum the total number of each barrier type into the 'barrier_type_counts'
        // array, where that array is indexed by the IRBarrierType enum.
        let mut barrier_type_counts = [0usize; IRBarrierType::NUM_VARIANTS];
        for node in ir.nodes.iter() {
            match node {
                IRNode::RenderPass(_) => {}
                IRNode::Barrier(v) => {
                    barrier_type_counts[v.barrier_type as usize] += 1;
                }
                IRNode::LayoutChange(v) => {
                    barrier_type_counts[v.barrier_type as usize] += 1;
                }
            }
        }
        barrier_type_counts
    }

    fn find_root_and_leaf_nodes<T: std::io::Write>(
        &mut self,
        graph_builder: &FrameGraphBuilder,
        ir: &IRBuilder<'arena, '_, '_, T>,
    ) -> (BVec<'arena, usize>, BVec<'arena, usize>) {
        // Find the root and leaf nodes of the graph by iterating all the nodes and filtering them
        // into the apropriate category based on whether they have an previous or next nodes.
        //
        // Nodes with no 'previous' are considered roots and nodes with no 'next' are considered
        // leaves.
        //
        // The worst case for the number of root nodes is the number of imported resources, as that
        // would be every imported resource being imported by a unique pass.
        //
        // The worst case for leaf nodes is unbounded, so we pick a guess of half the total number
        // of nodes.
        let num_imports = graph_builder.imported_resources.len();
        let mut roots = BVec::with_capacity_in(num_imports, self.arena);
        let mut leafs = BVec::with_capacity_in(ir.nodes.len() / 2, self.arena);
        for (i, v) in ir.nodes.iter().enumerate() {
            let prev = unsafe { v.prev().as_ref() };
            let next = unsafe { v.next().as_ref() };
            if prev.is_empty() {
                roots.push(i);
            }
            if next.is_empty() {
                leafs.push(i);
            }
        }

        if cfg!(debug_assertions) {
            for node_index in roots.iter().copied() {
                let root_node = &ir.nodes[node_index];
                debug_assert!(
                    matches!(root_node, IRNode::Barrier(_) | IRNode::LayoutChange(_)),
                    "All root nodes must be barriers!"
                );
            }
        }

        (roots, leafs)
    }

    fn validate_bundles<T: std::io::Write>(
        &mut self,
        _graph_builder: &FrameGraphBuilder,
        ir: &IRBuilder<'arena, '_, '_, T>,
    ) {
        if !cfg!(debug_assertions) {
            return;
        }

        // Validate that the computed bundle order will never execute an IR node before all of its
        // predecessors have been executed.
        //
        // This is critical for correct synchronization.
        let node_executed: &mut [bool] = self.arena.alloc_slice_fill_default(ir.nodes.len());
        for bundle in self.bundles.iter() {
            let barriers = unsafe { bundle.barriers.as_ref() };
            let passes = unsafe { bundle.passes.as_ref() };

            for &barrier in barriers {
                let barrier_node = &ir.nodes[barrier];
                let prevs = unsafe { barrier_node.prev().as_ref() };
                for &prev in prevs {
                    debug_assert!(node_executed[prev]);
                }
                node_executed[barrier] = true;
            }

            for &pass in passes {
                let pass_node = &ir.nodes[pass];
                let prevs = unsafe { pass_node.prev().as_ref() };
                for &prev in prevs {
                    debug_assert!(node_executed[prev]);
                }
                node_executed[pass] = true;
            }
        }
    }
}

/// A utility function that wraps around 'default_barrier_sync' that will either return the given
/// [BarrierSync] flags, or the default sync flags for the provided [ResourceUsageFlags] if the
/// given sync flags are not set (all 0).
fn get_given_or_default_sync_flags_for(
    access: ResourceUsageFlags,
    sync: BarrierSync,
    read_only: bool,
    format: Format,
) -> BarrierSync {
    if sync.is_empty() {
        access.default_barrier_sync(read_only, format)
    } else {
        sync
    }
}

fn is_sorted<T>(data: &[T]) -> bool
where
    T: Ord,
{
    data.windows(2).all(|w| w[0] <= w[1])
}
