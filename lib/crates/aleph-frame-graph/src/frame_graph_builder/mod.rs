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
use crate::{FrameGraph, IRenderPass, ResourceMut, ResourceRef};
use aleph_arena_drop_list::DropLink;
use aleph_rhi_api::*;
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
    pub resource: &'a dyn ITexture,

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
    /// The buffer resource to import into the frame graph
    pub resource: &'a dyn IBuffer,

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
pub struct FrameGraphBuilder {
    /// An arena that will be moved into the FrameGraph once the graph is finalized. This can be
    /// used to store anything that persists to the fully constructed graph.
    pub(crate) graph_arena: Bump,

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

    /// Stores debug information for each resource handle generated at a resource rename event. This
    /// is used to help validate resources are accessed in a valid way.
    pub(crate) resource_handles: Vec<ResourceHandleInfo>,

    /// The set of resources within the graph that were imported, stored as indices into the
    /// root_resources array.
    pub(crate) imported_resources: Vec<u16>,

    /// This will hold the collected pass information, such as reads/writes/creates for whatever
    /// pass is being setup current. The data stored in here is ephemeral and will be cleared
    /// between each 'add pass' call. It simply serves to accumulate the information from a pass
    /// setup callback.
    pub(crate) pass_access_info: PassAccessInfo,

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
        ExecFn: FnMut(&T) + Send + 'static,
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
            let payload = self.graph_arena.alloc(T::default());
            let mut payload = NonNull::from(payload);
            DropLink::append_drop_list(&self.graph_arena, &mut self.drop_head, payload);

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
    pub fn build(mut self) -> FrameGraph {
        self.validate_imported_resource_usages();

        // We need some extra state per resource version that allows us to track what resources are
        // ready to be written, ready to be read and which are completely retired.
        let mut resource_version_states = Vec::new();
        resource_version_states.resize(
            self.resource_versions.len(),
            ResourceVersionState::default(),
        );

        // We need some more per version state that holds the number of reads to a resource that
        // have _not_ been scheduled. This is seeded from the version itself as the number of
        // readers as stored in the 'resource_versions' array.
        let mut resource_version_pending_reads: Vec<_> = self
            .resource_versions
            .iter()
            .map(|v| {
                let mut read_count = 0usize;
                let mut current = v.reads;
                while let Some(v) = current {
                    unsafe {
                        read_count += 1;
                        current = v.as_ref().next;
                    }
                }
                read_count
            })
            .collect();

        let mut render_pass_order = Vec::with_capacity(self.render_passes.len());
        loop {
            let previous_scheduled_pass_count = render_pass_order.len();

            for (i, pass) in self.render_passes.iter().enumerate() {
                let (reads, writes) = unsafe { (pass.reads.as_ref(), pass.writes.as_ref()) };

                let all_reads_ready = reads.iter().all(|v| {
                    let version_index = v.resource.version_id() as usize;
                    let version_state = resource_version_states[version_index];

                    // If this resource is written then it is ready to be read, and only if it is
                    // written. Once retired it is no longer safer to be read
                    version_state == ResourceVersionState::Written
                });

                let all_writes_ready = writes.iter().all(|v| {
                    let version_index = v.resource.version_id() as usize;
                    let version = &self.resource_versions[version_index];
                    let version_state = resource_version_states[version_index];

                    // If the previous version is retired and the current version is still waiting
                    // then this version is ready to be written to
                    let is_previous_retired = {
                        // We should only lookup previous state information if there is a previous
                        // resource.
                        if version.previous_version.is_valid() {
                            let previous_version = version.previous_version.0 as usize;
                            let previous_state = resource_version_states[previous_version];
                            previous_state == ResourceVersionState::Retired
                        } else {
                            // An invalid ID means our current resource is the first version of the
                            // resource. In this case the 'previous resource' is always retired by
                            // definition as there is not previous resource to wait on.
                            true
                        }
                    };
                    is_previous_retired && version_state == ResourceVersionState::Waiting
                });

                // If all the dependent resources are ready then the pass is ready to be scheduled.
                // This means we can add the pass to our pass order and then update the read/written
                // resource
                if all_reads_ready && all_writes_ready {
                    render_pass_order.push(i);

                    // Walk through all the writes declared on this pass and mark the resource
                    // versions that are written with the 'Written' state.
                    for v in writes {
                        let version_index = v.resource.version_id() as usize;
                        // Sometimes we may have resources that are only every used with a write
                        // declaration so we need to handle directly retiring these resources. If
                        // there are no pending reads on the resource we can skip directly to
                        // retiring the resource.
                        if resource_version_pending_reads[version_index] == 0 {
                            resource_version_states[version_index] = ResourceVersionState::Retired;
                        } else {
                            resource_version_states[version_index] = ResourceVersionState::Written;
                        }
                    }

                    // Walk through all the reads declared on this pass and decrement the pending
                    // read count for the version that was read. If the
                    for v in reads {
                        let version_index = v.resource.version_id() as usize;
                        let pending_reads = &mut resource_version_pending_reads[version_index];
                        *pending_reads -= 1;
                        if *pending_reads == 0 {
                            resource_version_states[version_index] = ResourceVersionState::Retired;
                        }
                    }
                }
            }

            // If we've failed to schedule any passes in this cycle then we have created a deadlock
            // where it's impossible for the scheduler to schedule any passes. We can detect this
            // case and panic as otherwise we would be stuck in an endless loop.
            if render_pass_order.len() == previous_scheduled_pass_count {
                panic!("FrameGraph deadlock detected!");
            }

            // All passes are scheduled then we can break from the loop, our work here is done.
            if render_pass_order.len() == self.render_passes.len() {
                break;
            }
        }

        debug_assert!(resource_version_states
            .drain(..)
            .all(|v| v == ResourceVersionState::Retired));

        let arena = std::mem::take(&mut self.graph_arena);
        let render_passes = std::mem::take(&mut self.render_passes);
        let root_resources = std::mem::take(&mut self.root_resources);
        let resource_versions = std::mem::take(&mut self.resource_versions);
        let drop_head = std::mem::take(&mut self.drop_head);

        FrameGraph {
            _arena: arena,
            render_pass_order,
            render_passes,
            root_resources,
            resource_versions,
            drop_head,
        }
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
        let name = self.graph_arena.alloc_str(name);
        let name = NonNull::from(name);
        let pass = self.graph_arena.alloc(pass);
        let mut pass = NonNull::from(pass);
        DropLink::append_drop_list(&self.graph_arena, &mut self.drop_head, pass);

        unsafe {
            let reads = self
                .graph_arena
                .alloc_slice_clone(&self.pass_access_info.reads);
            let reads = NonNull::from(reads);
            let writes = self
                .graph_arena
                .alloc_slice_clone(&self.pass_access_info.writes);
            let writes = NonNull::from(writes);

            let pass = NonNull::from(pass.as_mut() as &mut dyn IRenderPass);
            let pass = RenderPass {
                pass,
                name,
                reads,
                writes,
            };
            self.render_passes.push(pass);
        }

        // Reset the pass info accumulator. This still holds onto allocated memory so we should stop
        // allocating once we've grown to the size of our biggest pass
        self.pass_access_info.clear();
    }

    pub(crate) fn import_texture_internal(
        &mut self,
        render_pass: usize,
        desc: &TextureImportDesc,
        sync: BarrierSync,
        access: ResourceUsageFlags,
    ) -> ResourceMut {
        debug_assert!(
            ResourceUsageFlags::TEXTURE_USAGE_MASK.contains(access),
            "Attempting to declare non-texture compatible access flags {:?}",
            access
        );

        let format = desc.resource.desc().format;
        let sync = get_given_or_default_sync_flags_for(access, sync, false, format);

        let imported = ImportedTexture {
            resource: desc.resource.upgrade(),
            before_sync: desc.before_sync,
            before_access: desc.before_access,
            before_layout: desc.before_layout,
            after_sync: desc.after_sync,
            after_access: desc.after_access,
            after_layout: desc.after_layout,
        };
        let resource_desc = desc.resource.desc();
        let name = resource_desc.name.map(|v| self.graph_arena.alloc_str(v));
        let name = name.map(|v| NonNull::from(v));
        let r_type = ResourceTypeTexture {
            import: Some(imported),
            desc: FrameGraphTextureDesc {
                width: resource_desc.width,
                height: resource_desc.height,
                depth: resource_desc.depth,
                format: resource_desc.format,
                dimension: resource_desc.dimension,
                clear_value: resource_desc.clear_value,
                array_size: resource_desc.array_size,
                mip_levels: resource_desc.mip_levels,
                sample_count: resource_desc.sample_count,
                sample_quality: resource_desc.sample_quality,
                name,
            },
        };

        // render pass index doesn't matter here as imported resources aren't created by a render
        // pass
        let r = self.create_new_handle(render_pass, sync, access, r_type);
        self.add_imported_resource_to_list(r);
        let desc = ResourceAccess {
            resource: r.0,
            sync,
            access,
        };
        self.pass_access_info.writes.push(desc);

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
        let imported = ImportedBuffer {
            resource: desc.resource.upgrade(),
            before_sync: desc.before_sync,
            before_access: desc.before_access,
            after_sync: desc.after_sync,
            after_access: desc.after_access,
        };
        let resource_desc = desc.resource.desc();
        let name = resource_desc.name.map(|v| self.graph_arena.alloc_str(v));
        let name = name.map(|v| NonNull::from(v));
        let r_type = ResourceTypeBuffer {
            import: Some(imported),
            desc: FrameGraphBufferDesc {
                size: resource_desc.size,
                name,
            },
        };

        // render pass index doesn't matter here as imported resources aren't created by a render
        // pass
        let r = self.create_new_handle(render_pass, sync, access, r_type);
        self.add_imported_resource_to_list(r);
        let desc = ResourceAccess {
            resource: r.0,
            sync,
            access,
        };
        self.pass_access_info.writes.push(desc);

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
            ResourceUsageFlags::TEXTURE_USAGE_MASK.contains(access),
            "Attempting to declare non-texture compatible access flags {:?}",
            access
        );

        let r = resource.into();

        self.add_flags_to_version_for(r, access);
        self.append_read_to_version_for(r, access, render_pass);
        self.add_flags_to_root_for(r, access);

        let root_resource = self.assert_resource_handle_is_texture(r);
        let format = root_resource.desc.format;
        let sync = get_given_or_default_sync_flags_for(access, sync, true, format);
        let desc = ResourceAccess {
            resource: r.0,
            sync,
            access,
        };
        self.pass_access_info.reads.push(desc);
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
        self.append_read_to_version_for(r, access, render_pass);
        self.add_flags_to_root_for(r, access);

        let sync = get_given_or_default_sync_flags_for(access, sync, true, Default::default());
        let desc = ResourceAccess {
            resource: r.0,
            sync,
            access,
        };
        self.pass_access_info.reads.push(desc);
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
            ResourceUsageFlags::TEXTURE_USAGE_MASK.contains(access),
            "Attempting to declare non-texture compatible access flags {:?}",
            access
        );

        let r = resource.into();

        self.validate_and_update_for_handle_write(r);
        self.add_flags_to_root_for(r, access);
        let renamed_r = self.increment_handle_for_write(r, render_pass, access);

        let root_resource = self.assert_resource_handle_is_texture(r);
        let format = root_resource.desc.format;
        let sync = get_given_or_default_sync_flags_for(access, sync, false, format);
        let desc = ResourceAccess {
            resource: renamed_r.0,
            sync,
            access,
        };
        self.pass_access_info.writes.push(desc);

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
        let renamed_r = self.increment_handle_for_write(r, render_pass, access);

        let sync = get_given_or_default_sync_flags_for(access, sync, false, Default::default());
        let desc = ResourceAccess {
            resource: renamed_r.0,
            sync,
            access,
        };
        self.pass_access_info.writes.push(desc);

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
            ResourceUsageFlags::TEXTURE_USAGE_MASK.contains(access),
            "Attempting to declare non-texture compatible access flags {:?}",
            access
        );

        let format = desc.format;
        let sync = get_given_or_default_sync_flags_for(access, sync, true, format);
        let name = desc.name.map(|v| self.graph_arena.alloc_str(v));
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
        let desc = ResourceAccess {
            resource: r.0,
            sync,
            access,
        };
        self.pass_access_info.writes.push(desc);

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
        let name = desc.name.map(|v| self.graph_arena.alloc_str(v));
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
        let desc = ResourceAccess {
            resource: r.0,
            sync,
            access,
        };
        self.pass_access_info.writes.push(desc);

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
        let handle_id = r.0.handle_id() as usize;
        let handle_info = &mut self.resource_handles[handle_id];
        if handle_info.is_written() {
            panic!("Attempted to write a resource through the same handle more than once!");
        }
        handle_info.mark_written();
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
        access: ResourceUsageFlags,
        render_pass: usize,
    ) {
        let r = r.into();
        let version_id = r.0.version_id();
        let previous_read = self.resource_versions[version_id as usize].reads;
        let read = self.graph_arena.alloc(VersionReaderLink {
            next: previous_read,
            render_pass,
            access,
        });
        let read = NonNull::from(read);
        DropLink::append_drop_list(&self.graph_arena, &mut self.drop_head, read);
        self.resource_versions[version_id as usize].reads = Some(read);
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
        access: ResourceUsageFlags,
    ) -> ResourceMut {
        let base = r.0.root_id();
        let version = u16::try_from(self.resource_versions.len()).unwrap();
        let handle = u16::try_from(self.resource_handles.len()).unwrap();
        let id = ResourceId::new(base, version, handle);
        self.resource_versions.push(ResourceVersion {
            // We need the root resource here to allow iterations over the version array to easily
            // link back to their roots
            root_resource: base,

            // A write will set the previous version to whatever was stored in the resource handle
            // we were given.
            previous_version: VersionIndex::new(r.0.version_id()).unwrap(),

            version_total_access: access,
            creator_render_pass: render_pass,
            reads: None,
        });
        self.resource_handles.push(ResourceHandleInfo::default());

        // Assert we never create u16::MAX versions. This is _critical_ as 65535 is a niche
        // value to encode a 'null' version. By guaranteeing 65535 is never a valid index into this
        // array we can rely on rust's bounds checking to assert that we never access the 65535th
        // version by construction (we never make it).
        assert!(self.resource_versions.len() < ((u16::MAX) as usize));

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
        let version = u16::try_from(self.resource_versions.len()).unwrap();
        let handle = u16::try_from(self.resource_handles.len()).unwrap();
        let id = ResourceId::new(base, version, handle);
        self.root_resources.push(ResourceRoot {
            resource_type: r_type.into(),
            total_access_flags: access,
            initial_version: VersionIndex::new(version).unwrap(),
            creator_sync: sync,
            creator_access: access,
        });
        self.resource_versions.push(ResourceVersion {
            // We need the root resource here to allow iterations over the version array to easily
            // link back to their roots
            root_resource: base,

            // A create is by definition the first version of a resource so the 'previous' link is
            // initialized as an invalid id. This encodes the 'end' of the list
            previous_version: VersionIndex::INVALID,

            version_total_access: access,
            creator_render_pass: render_pass,
            reads: None,
        });
        self.resource_handles.push(ResourceHandleInfo::default());

        // Assert we never create u16::MAX versions. This is _critical_ as 65535 is a niche
        // value to encode a 'null' version. By guaranteeing 65535 is never a valid index into this
        // array we can rely on rust's bounds checking to assert that we never access the 65535th
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
                ResourceType::Uninitialized => unreachable!(),
                ResourceType::Buffer(ResourceTypeBuffer {
                    import: Some(import),
                    ..
                }) => {
                    let desc = import.resource.desc();
                    let r_name = desc.name.unwrap_or("Unnamed resource");
                    let root_usage = root.total_access_flags;
                    assert!(
                        desc.usage.contains(root_usage),
                        "Resource '{}' used in unsupported usage. Allowed: {:?}. Attempted: {:?}",
                        r_name,
                        desc.usage,
                        root_usage
                    );
                }
                ResourceType::Texture(ResourceTypeTexture {
                    import: Some(import),
                    ..
                }) => {
                    let desc = import.resource.desc();
                    let r_name = desc.name.unwrap_or("Unnamed resource");
                    let root_usage = root.total_access_flags;
                    assert!(
                        desc.usage.contains(root_usage),
                        "Resource '{}' used in unsupported usage. Allowed: {:?}. Attempted: {:?}",
                        r_name,
                        desc.usage,
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
