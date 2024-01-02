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

    pub fn build2(self) -> FrameGraph {
        // We have to constrain the type of the writer even though we don't use it here, so we just
        // use Sink.
        //
        // This can't error unless we pass a writer, so we _could_ use unwrap_unchecked. The cost
        // is miniscule so just check anyway so we don't have unsafe code here.
        self.build_internal2::<std::io::Sink>("", None).unwrap()
    }

    pub fn build2_with_graph_viz(
        self,
        graph_name: &str,
        writer: &mut impl std::io::Write,
        options: &GraphVizOutputOptions,
    ) -> std::io::Result<FrameGraph> {
        self.build_internal2(graph_name, Some((writer, options)))
    }

    fn build_internal<T: std::io::Write>(
        mut self,
        graph_name: &str,
        mut writer: Option<(&mut T, &GraphVizOutputOptions)>,
    ) -> std::io::Result<FrameGraph> {
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
            .map(|v| v.read_count)
            .collect();

        self.emit_graph_viz_start(graph_name, &mut writer)?;

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
                    for written_resource in writes {
                        let version_index = written_resource.resource.version_id() as usize;
                        // Sometimes we may have resources that are only every used with a write
                        // declaration so we need to handle directly retiring these resources. If
                        // there are no pending reads on the resource we can skip directly to
                        // retiring the resource.
                        if resource_version_pending_reads[version_index] == 0 {
                            resource_version_states[version_index] = ResourceVersionState::Retired;
                        } else {
                            resource_version_states[version_index] = ResourceVersionState::Written;
                        }

                        // If the writer is present we output a graph edge when we schedule a pass
                        if let Some((v, _options)) = writer.as_mut() {
                            let previous_version =
                                self.resource_versions[version_index].previous_version;

                            let creator = if previous_version.is_valid() {
                                let version = &self.resource_versions[previous_version.0 as usize];
                                Some(version.creator_render_pass)
                            } else {
                                let root_id = written_resource.resource.root_id();
                                if self.imported_resources.contains(&root_id) {
                                    Some(usize::MAX)
                                } else {
                                    None
                                }
                            };

                            if let Some(creator) = creator {
                                writeln!(v, "    node{creator} -> node{i};")?;
                            }
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

                        if let Some((v, _options)) = writer.as_mut() {
                            let version = &self.resource_versions[version_index as usize];
                            let creator = version.creator_render_pass;
                            writeln!(v, "    node{creator} -> node{i};")?;
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

        self.emit_graph_viz_end(&mut writer)?;

        let arena = std::mem::take(&mut self.graph_arena);
        let render_passes = std::mem::take(&mut self.render_passes);
        let root_resources = std::mem::take(&mut self.root_resources);
        let resource_versions = std::mem::take(&mut self.resource_versions);
        let imported_resources = std::mem::take(&mut self.imported_resources);
        let drop_head = std::mem::take(&mut self.drop_head);

        Ok(FrameGraph {
            _arena: arena,
            render_pass_order,
            render_passes,
            root_resources,
            resource_versions,
            imported_resources,
            drop_head,
        })
    }

    fn build_internal2<T: std::io::Write>(
        mut self,
        graph_name: &str,
        mut writer: Option<(&mut T, &GraphVizOutputOptions)>,
    ) -> std::io::Result<FrameGraph> {
        // An arena allocator used for allocating resources that only live as long as the graph is
        // being built
        let build_arena = Bump::new();

        self.validate_imported_resource_usages();

        self.emit_graph_viz_start(graph_name, &mut writer)?;

        let num_passes = self.render_passes.len();

        let mut ir_nodes: Vec<IRNode> = Vec::with_capacity(num_passes);
        for i in 0..num_passes {
            let ir_node = RenderPassIRNode {
                prev: NonNull::from(&[]),
                next: NonNull::from(&[]),
                render_pass: i,
            };
            ir_nodes.push(ir_node.into());
        }

        let pass_prevs =
            build_arena.alloc_slice_fill_with(num_passes, |_| BVec::new_in(&build_arena));
        let pass_nexts =
            build_arena.alloc_slice_fill_with(num_passes, |_| BVec::new_in(&build_arena));

        for (version_i, version) in self.resource_versions.iter().enumerate() {
            let version_i = VersionIndex(version_i as u32);
            let root = &self.root_resources[version.root_resource as usize];
            match &root.resource_type {
                // Buffers are much simpler to handle as we don't need to care about image layout
                // transitions that promote read-after-read accesses to writes because of required
                // layout changes.
                ResourceType::Buffer(root_variant) => {
                    // The first thing to check for are read-after-write (and for images
                    // read-after-read) barriers. If a version has at least a single read then we
                    // must emit a read-after-write barrier and schedule the reader pass.
                    if version.read_count > 0 {
                        // We form a 'next' edge will all the reads to this buffer, collecting
                        // the full set of usage/sync flags as the 'after' scope of the barrier.
                        //
                        // The before scope is defined by the access declared on the creator pass,
                        // with that creator pass being the sole 'previous' edge for this barrier.
                        let mut all_read_sync = BarrierSync::default();
                        let mut all_read_usage = ResourceUsageFlags::default();
                        let barrier_next = build_arena.alloc_slice_fill_copy(version.read_count, 0);
                        for (i, read) in version.reads_iter().enumerate() {
                            all_read_sync |= read.sync;
                            all_read_usage |= read.access;
                            barrier_next[i] = read.render_pass;
                        }

                        let barrier_prev =
                            build_arena.alloc_slice_fill_copy(1, version.creator_render_pass);
                        self.emit_barrier_ir_node(
                            &mut writer,
                            &mut ir_nodes,
                            pass_prevs,
                            pass_nexts,
                            "Read",
                            barrier_prev,
                            barrier_next,
                            version_i,
                            version.creator_sync,
                            version.creator_access.barrier_access_for_write(),
                            all_read_sync,
                            all_read_usage.barrier_access_for_read(),
                        )?;

                        if let Some(import_desc) = root_variant.import.as_ref() {
                            if root.final_version == version_i {
                                // The 'next' for the previous barrier becomes the 'prev' for the export
                                // barrier.
                                let barrier_prev = barrier_next;
                                self.emit_barrier_ir_node(
                                    &mut writer,
                                    &mut ir_nodes,
                                    pass_prevs,
                                    pass_nexts,
                                    "Export after Read",
                                    barrier_prev,
                                    &[],
                                    version_i,
                                    all_read_sync,
                                    all_read_usage.barrier_access_for_read(),
                                    import_desc.after_sync,
                                    import_desc.after_access,
                                )?;
                            }
                        }
                    } else if let Some(import_desc) = root_variant.import.as_ref() {
                        if root.final_version == version_i {
                            let barrier_prev =
                                build_arena.alloc_slice_fill_copy(1, version.creator_render_pass);
                            self.emit_barrier_ir_node(
                                &mut writer,
                                &mut ir_nodes,
                                pass_prevs,
                                pass_nexts,
                                "Export after Write",
                                barrier_prev,
                                &[],
                                version_i,
                                version.creator_sync,
                                version.creator_access.barrier_access_for_write(),
                                import_desc.after_sync,
                                import_desc.after_access,
                            )?;
                        }
                    }

                    // The next class of barrier is 'write-after-x' barriers. These barriers are
                    // emitted between a previous usage and a write access to a resource. This could
                    // either be a direct write-after-write barrier with no intervening reads or it
                    // could be a write-after-read barrier between the reads of the previous
                    // resource version and the pass that writes out the new version of the
                    // resource.
                    //
                    // These are needed when the resource we're handling has a previous version.
                    //
                    // The two classes of barrier are detected based on the number of reads declared
                    // on the previous version
                    if version.previous_version.is_valid() {
                        let previous_version_index = version.previous_version.0 as usize;
                        let previous_version = &self.resource_versions[previous_version_index];

                        // If there are any reads on the previous resource version then we must emit
                        // a write-after-read barrier between those reads and the subsequent write
                        // that creates the new resource version.
                        if previous_version.read_count > 0 {
                            // We form a 'previous' edge for this barrier with all the reads to the
                            // previous version of the resource. This also has us collect all the
                            // sync/usage flags so we can define our 'before' sync scope of our
                            // barrier.
                            let mut all_read_sync = BarrierSync::default();
                            let mut all_read_usage = ResourceUsageFlags::default();
                            let barrier_prev =
                                build_arena.alloc_slice_fill_copy(previous_version.read_count, 0);
                            for (i, read) in previous_version.reads_iter().enumerate() {
                                all_read_sync |= read.sync;
                                all_read_usage |= read.access;
                                barrier_prev[i] = read.render_pass;
                            }

                            // The sole 'next' edge of the barrier is the creator of _this_ version
                            // of the resource.
                            //
                            // We're creating a barrier between the previous reads and the pass that
                            // writes out this new version of the resource. The 'after' sync scope
                            // is easily derived from the pass's declared access flags.
                            let barrier_next =
                                build_arena.alloc_slice_fill_copy(1, version.creator_render_pass);
                            self.emit_barrier_ir_node(
                                &mut writer,
                                &mut ir_nodes,
                                pass_prevs,
                                pass_nexts,
                                "Write after Read",
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
                            let barrier_prev = build_arena
                                .alloc_slice_fill_copy(1, previous_version.creator_render_pass);
                            let barrier_next =
                                build_arena.alloc_slice_fill_copy(1, version.creator_render_pass);
                            self.emit_barrier_ir_node(
                                &mut writer,
                                &mut ir_nodes,
                                pass_prevs,
                                pass_nexts,
                                "Write after Write",
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
                        let barrier_next =
                            build_arena.alloc_slice_fill_copy(1, version.creator_render_pass);
                        self.emit_barrier_ir_node(
                            &mut writer,
                            &mut ir_nodes,
                            pass_prevs,
                            pass_nexts,
                            "Import",
                            &[],
                            barrier_next,
                            version_i,
                            import_desc.before_sync,
                            import_desc.before_access,
                            version.creator_sync,
                            version.creator_access.barrier_access_for_write(),
                        )?;
                    }
                }
                ResourceType::Texture(root_variant) => {
                    // The first thing to check for are read-after-write (and for images
                    // read-after-read) barriers. If a version has at least a single read then we
                    // must emit a read-after-write barrier and schedule the reader pass.
                    //
                    // Images are special too, as we may need to emit read-after-read barriers as
                    // well if the set of reads require different image layouts.
                    //
                    // This is the single most complicated case to handle in our graph construction.
                    if version.read_count > 0 {
                        // First we need all the reads of this resource version in an array, sorted
                        // by the required image layout.
                        //
                        // This forms the core of how we detect read-after-read image layout
                        // transitions.
                        let reads = version.reads_sorted_by_image_layout_in(
                            root_variant.desc.format,
                            &build_arena,
                        );

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
                        let mut previous_reads: BVec<usize> =
                            BVec::with_capacity_in(reads.len(), &build_arena);

                        // This list is used to accumulate the pending reads when we're still
                        // searching for layout edges. Once a layout edge is found this will contain
                        // the set of reads that form a read batch to emit a barrier for.
                        //
                        // Once a barrier is emitted we flush this into 'previous_reads'
                        let mut pending_reads: BVec<usize> =
                            BVec::with_capacity_in(reads.len(), &build_arena);

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
                                let barrier_next =
                                    build_arena.alloc_slice_fill_copy(pending_reads.len(), 0);
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
                                    build_arena
                                        .alloc_slice_fill_copy(1, version.creator_render_pass)
                                } else {
                                    // Link to every read access scheduled in the previous read
                                    // batch
                                    build_arena.alloc_slice_fill_iter(
                                        previous_reads.drain(..).map(|v| reads[v].0.render_pass),
                                    )
                                };

                                // We now emit the barrier
                                self.emit_layout_change_ir_node(
                                    &mut writer,
                                    &mut ir_nodes,
                                    pass_prevs,
                                    pass_nexts,
                                    "Read",
                                    barrier_prev,
                                    barrier_next,
                                    version_i,
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
                            if root.final_version == version_i {
                                // The 'next' for the previous barrier becomes the 'prev' for the export
                                // barrier.
                                let barrier_prev =
                                    build_arena.alloc_slice_fill_iter(previous_reads.drain(..));
                                self.emit_layout_change_ir_node(
                                    &mut writer,
                                    &mut ir_nodes,
                                    pass_prevs,
                                    pass_nexts,
                                    "Export after Read",
                                    barrier_prev,
                                    &[],
                                    version_i,
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
                        if root.final_version == version_i {
                            let barrier_prev =
                                build_arena.alloc_slice_fill_copy(1, version.creator_render_pass);
                            self.emit_layout_change_ir_node(
                                &mut writer,
                                &mut ir_nodes,
                                pass_prevs,
                                pass_nexts,
                                "Export after Write",
                                barrier_prev,
                                &[],
                                version_i,
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

                    // The next class of barrier is 'write-after-x' barriers. These barriers are
                    // emitted between a previous usage and a write access to a resource. This could
                    // either be a direct write-after-write barrier with no intervening reads or it
                    // could be a write-after-read barrier between the reads of the previous
                    // resource version and the pass that writes out the new version of the
                    // resource.
                    //
                    // These are needed when the resource we're handling has a previous version.
                    //
                    // The two classes of barrier are detected based on the number of reads declared
                    // on the previous version
                    if version.previous_version.is_valid() {
                        let previous_version_index = version.previous_version.0 as usize;
                        let previous_version = &self.resource_versions[previous_version_index];

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
                            let reads = previous_version.reads_sorted_by_image_layout_in(
                                root_variant.desc.format,
                                &build_arena,
                            );

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
                            let barrier_prev =
                                build_arena.alloc_slice_fill_copy(num_reads_for_prev, 0);
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
                            let barrier_next =
                                build_arena.alloc_slice_fill_copy(1, version.creator_render_pass);
                            self.emit_layout_change_ir_node(
                                &mut writer,
                                &mut ir_nodes,
                                pass_prevs,
                                pass_nexts,
                                "Write after Read",
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
                            let barrier_prev = build_arena
                                .alloc_slice_fill_copy(1, previous_version.creator_render_pass);
                            let barrier_next =
                                build_arena.alloc_slice_fill_copy(1, version.creator_render_pass);
                            self.emit_layout_change_ir_node(
                                &mut writer,
                                &mut ir_nodes,
                                pass_prevs,
                                pass_nexts,
                                "Write after Write",
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
                        let barrier_next =
                            build_arena.alloc_slice_fill_copy(1, version.creator_render_pass);
                        self.emit_layout_change_ir_node(
                            &mut writer,
                            &mut ir_nodes,
                            pass_prevs,
                            pass_nexts,
                            "Import",
                            &[],
                            barrier_next,
                            version_i,
                            import_desc.before_sync,
                            import_desc.before_access,
                            import_desc.before_layout,
                            version.creator_sync,
                            version.creator_access.barrier_access_for_write(),
                            version
                                .creator_access
                                .image_layout(false, root_variant.desc.format),
                        )?;
                    }
                }
            }
        }

        for (i, _pass) in self.render_passes.iter().enumerate() {
            let pass_node = &mut ir_nodes[i];
            pass_node.set_prev(NonNull::from(pass_prevs[i].as_slice()));
            pass_node.set_next(NonNull::from(pass_nexts[i].as_slice()));
        }

        if let Some((v, options)) = writer.as_mut() {
            for (i, ir_node) in ir_nodes.iter().enumerate() {
                let prevs = unsafe { ir_node.prev().as_ref() };
                let nexts = unsafe { ir_node.next().as_ref() };

                if options.output_previous_links {
                    for prev in prevs {
                        writeln!(v, "    node{i} -> node{prev}")?;
                    }
                }
                for next in nexts {
                    writeln!(v, "    node{i} -> node{next}")?;
                }
            }
        }

        // Find the root and leaf nodes of the graph by iterating all the nodes and filtering them
        // into the apropriate category based on whether they have an previous or next nodes.
        //
        // Nodes with no 'previous' are considered roots and nodes with no 'next' are considered
        // leaves.
        let mut roots = BVec::with_capacity_in(ir_nodes.len() / 2, &build_arena);
        let mut leafs = BVec::with_capacity_in(ir_nodes.len() / 2, &build_arena);
        for (i, v) in ir_nodes.iter().enumerate() {
            let prev = unsafe { v.prev().as_ref() };
            let next = unsafe { v.next().as_ref() };
            if prev.is_empty() {
                roots.push(i);
            }
            if next.is_empty() {
                leafs.push(i);
            }
        }

        self.emit_graph_viz_end(&mut writer)?;

        let arena = std::mem::take(&mut self.graph_arena);
        let render_passes = std::mem::take(&mut self.render_passes);
        let root_resources = std::mem::take(&mut self.root_resources);
        let resource_versions = std::mem::take(&mut self.resource_versions);
        let imported_resources = std::mem::take(&mut self.imported_resources);
        let drop_head = std::mem::take(&mut self.drop_head);

        Ok(FrameGraph {
            _arena: arena,
            render_pass_order: Default::default(),
            render_passes,
            root_resources,
            resource_versions,
            imported_resources,
            drop_head,
        })
    }

    /// Output the start of the DOT graph if we have a writer
    fn emit_graph_viz_start<T: std::io::Write>(
        &self,
        graph_name: &str,
        writer: &mut Option<(&mut T, &GraphVizOutputOptions)>,
    ) -> std::io::Result<()> {
        if let Some((v, _options)) = writer {
            writeln!(v, "digraph {graph_name} {{")?;
            for (i, pass) in self.render_passes.iter().enumerate() {
                let pass_name = unsafe { pass.name.as_ref() };
                writeln!(
                    v,
                    "    node{i} [shape=box,label=\"Render Pass: \\\"{pass_name}\\\"\"];"
                )?;
            }
        }

        Ok(())
    }

    /// Output the end of the DOT graph if we have a writer
    fn emit_graph_viz_end<T: std::io::Write>(
        &self,
        writer: &mut Option<(&mut T, &GraphVizOutputOptions)>,
    ) -> std::io::Result<()> {
        if let Some((v, _options)) = writer {
            writeln!(v, "}}")?;
        }

        Ok(())
    }

    fn get_resource_name_for_version_index(&self, version: VersionIndex) -> &str {
        let version = &self.resource_versions[version.0 as usize];
        let root_index = version.root_resource as usize;
        let root_resource = &self.root_resources[root_index];
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

    fn emit_graph_viz_barrier_node<T: std::io::Write>(
        &self,
        writer: &mut Option<(&mut T, &GraphVizOutputOptions)>,
        barrier_type: &str,
        barrier_ir_node_index: usize,
        ir_node: &BarrierIRNode,
    ) -> std::io::Result<()> {
        if let Some((v, _options)) = writer.as_mut() {
            let resource_name = self.get_resource_name_for_version_index(ir_node.version);
            writeln!(
                v,
                "    node{} [label=\"{} Barrier: \\Resource: {} (v_id#{})\\nBeforeSync: {:?}\\nBeforeAccess: {:?}\\nAfterSync: {:?}\\nAfterAccess: {:?}\"]",
                barrier_ir_node_index,
                barrier_type,
                resource_name,
                ir_node.version.0,
                ir_node.before_sync,
                ir_node.before_access,
                ir_node.after_sync,
                ir_node.after_access
            )?;
        }

        Ok(())
    }

    fn emit_graph_viz_layout_change_node<T: std::io::Write>(
        &self,
        writer: &mut Option<(&mut T, &GraphVizOutputOptions)>,
        barrier_type: &str,
        barrier_ir_node_index: usize,
        ir_node: &LayoutChangeIRNode,
    ) -> std::io::Result<()> {
        if let Some((v, _options)) = writer.as_mut() {
            let resource_name = self.get_resource_name_for_version_index(ir_node.version);
            writeln!(
                v,
                "    node{} [label=\"{} Layout Change Barrier: \\Resource: {} (v_id#{})\\nBeforeSync: {:?}\\nBeforeAccess: {:?}\\nBeforeLayout: {:?}\\nAfterSync: {:?}\\nAfterAccess: {:?}\\nAfterLayout: {:?}\"]",
                barrier_ir_node_index,
                barrier_type,
                resource_name,
                ir_node.version.0,
                ir_node.before_sync,
                ir_node.before_access,
                ir_node.before_layout,
                ir_node.after_sync,
                ir_node.after_access,
                ir_node.after_layout,
            )?;
        }

        Ok(())
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
    fn emit_barrier_ir_node<T: std::io::Write>(
        &self,
        writer: &mut Option<(&mut T, &GraphVizOutputOptions)>,
        ir_nodes: &mut Vec<IRNode>,
        pass_prevs: &mut [BVec<usize>],
        pass_nexts: &mut [BVec<usize>],
        barrier_type: &str,
        barrier_prev: &[usize],
        barrier_next: &[usize],
        version: VersionIndex,
        before_sync: BarrierSync,
        before_access: BarrierAccess,
        after_sync: BarrierSync,
        after_access: BarrierAccess,
    ) -> std::io::Result<usize> {
        // Current length of the ir_node buffer will become the index of the node we insert
        let ir_node_index = ir_nodes.len();

        // Add the second half of our double linked graph. We only defined the links out of the new
        // IR node, we need to patch the new links into the new node's linked nodes.
        //
        // We assume that a barrier node will only link to render pass nodes. This means we can
        // just insert the ir_node_index into the vec stored in the pass_nexts/pass_prevs arrays by
        // indexing with the new ir node's outward link indices.
        for prev in barrier_prev.iter().copied() {
            pass_nexts[prev].push(ir_node_index);
        }
        for next in barrier_next.iter().copied() {
            pass_prevs[next].push(ir_node_index);
        }

        let ir_node = BarrierIRNode {
            prev: NonNull::from(barrier_prev),
            next: NonNull::from(barrier_next),
            version,
            before_sync,
            before_access,
            after_sync,
            after_access,
        };

        self.emit_graph_viz_barrier_node(writer, barrier_type, ir_node_index, &ir_node)?;

        ir_nodes.push(ir_node.into());

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
    fn emit_layout_change_ir_node<T: std::io::Write>(
        &self,
        writer: &mut Option<(&mut T, &GraphVizOutputOptions)>,
        ir_nodes: &mut Vec<IRNode>,
        pass_prevs: &mut [BVec<usize>],
        pass_nexts: &mut [BVec<usize>],
        barrier_type: &str,
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
        let ir_node_index = ir_nodes.len();

        // Add the second half of our double linked graph. We only defined the links out of the new
        // IR node, we need to patch the new links into the new node's linked nodes.
        //
        // We assume that a barrier node will only link to render pass nodes. This means we can
        // just insert the ir_node_index into the vec stored in the pass_nexts/pass_prevs arrays by
        // indexing with the new ir node's outward link indices.
        for prev in barrier_prev.iter().copied() {
            pass_nexts[prev].push(ir_node_index);
        }
        for next in barrier_next.iter().copied() {
            pass_prevs[next].push(ir_node_index);
        }

        let ir_node = LayoutChangeIRNode {
            prev: NonNull::from(barrier_prev),
            next: NonNull::from(barrier_next),
            version,
            before_sync,
            before_access,
            before_layout,
            after_sync,
            after_access,
            after_layout,
        };

        self.emit_graph_viz_layout_change_node(writer, barrier_type, ir_node_index, &ir_node)?;

        ir_nodes.push(ir_node.into());

        Ok(ir_node_index)
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
        let name = desc.desc.name.map(|v| self.graph_arena.alloc_str(v));
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
        let imported = ImportedResource {
            allowed_usage: desc.desc.usage,
            before_sync: desc.before_sync,
            before_access: desc.before_access,
            before_layout: Default::default(),
            after_sync: desc.after_sync,
            after_access: desc.after_access,
            after_layout: Default::default(),
        };
        let name = desc.desc.name.map(|v| self.graph_arena.alloc_str(v));
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
        self.add_flags_to_root_for(r, access);
        let sync = get_given_or_default_sync_flags_for(access, sync, true, Default::default());
        self.append_read_to_version_for(r, render_pass, sync, access);

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
        let sync = get_given_or_default_sync_flags_for(access, sync, false, Default::default());
        let renamed_r = self.increment_handle_for_write(r, render_pass, sync, access);

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
            access.is_valid_texture_usage(),
            "{:?} is not valid texture usage",
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
        let read = self.graph_arena.alloc(VersionReaderLink {
            next: previous_read,
            render_pass,
            sync,
            access,
        });
        let read = NonNull::from(read);
        DropLink::append_drop_list(&self.graph_arena, &mut self.drop_head, read);
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
            creator_render_pass: render_pass,
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
            initial_version: version,
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
            creator_render_pass: render_pass,
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
