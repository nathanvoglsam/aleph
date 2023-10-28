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

use crate::internal::*;
use crate::render_pass::CallbackRenderPass;
use crate::resource::ResourceId;
use crate::{FrameGraph, IRenderPass, ResourceMut, ResourceRef};
use aleph_arena_drop_list::DropLink;
use aleph_rhi_api::*;
use bumpalo::Bump;
use std::ptr::NonNull;

pub struct FrameGraphBuilder {
    /// An arena that will be moved into the FrameGraph once the graph is finalized. This can be
    /// used to store anything that persists to the fully constructed graph.
    pub(crate) graph_arena: Bump,

    // /// An arena used temporarily while constructing the frame graph. Will be freed with the
    // /// [FrameGraphBuilder] instance. This can be used to allocate anything that only needs to exist
    // /// as long as the graph is being built.
    // pub(crate) build_arena: Bump,

    /// The list of all the render passes in the graph. The index of the pass in this list is the
    /// identity of the pass and is used to key to a number of different names
    pub(crate) render_passes: Vec<RenderPass>,

    pub(crate) root_resources: Vec<ResourceRoot>,
    pub(crate) resource_versions: Vec<ResourceVersion>,

    /// Stores debug information for each resource handle generated at a resource rename event. This
    /// is used to help validate resources are accessed in a valid way.
    pub(crate) resource_handles: Vec<ResourceHandleInfo>,

    /// This will hold the collected pass information, such as reads/writes/creates for whatever
    /// pass is being setup current. The data stored in here is ephemeral and will be cleared
    /// between each 'add pass' call. It simply serves to accumulate the information from a pass
    /// setup callback.
    pub(crate) pass_access_info: PassAccessInfo,

    /// The head of the dropper linked-list that contains all the drop functions for the render
    /// passes.
    pub(crate) pass_dropper_head: Option<NonNull<DropLink>>,

    /// The head of the dropper linked-list that contains droppers for the callback pass payloads.
    pub(crate) payload_dropper_head: Option<NonNull<DropLink>>,
}

impl FrameGraphBuilder {
    pub fn new() -> Self {
        Self {
            graph_arena: Default::default(),
            // build_arena: Default::default(),
            render_passes: Default::default(),
            root_resources: Default::default(),
            resource_versions: Default::default(),
            resource_handles: Default::default(),
            pass_access_info: Default::default(),
            pass_dropper_head: Default::default(),
            payload_dropper_head: Default::default(),
        }
    }

    pub fn import_texture(&mut self, desc: &TextureImportDesc) -> ResourceMut {
        let imported = ImportedTexture {
            resource: desc.resource.upgrade(),
            before_sync: desc.before_sync,
            before_usage: desc.before_usage,
            before_layout: desc.before_layout,
            after_sync: desc.after_sync,
            after_usage: desc.after_usage,
            after_layout: desc.after_layout,
        };
        let r_type = ResourceType::Texture {
            create_desc: TextureCreate::default(),
            import_info: Some(imported),
        };

        // render pass index doesn't matter here as imported resources aren't created by a render
        // pass
        let r = self.create_new_handle(usize::MAX);
        self.set_resource_type_for(r, r_type);

        r
    }

    pub fn import_buffer(&mut self, desc: &BufferImportDesc) -> ResourceMut {
        let imported = ImportedBuffer {
            resource: desc.resource.upgrade(),
            before_sync: desc.before_sync,
            before_usage: desc.before_usage,
            after_sync: desc.after_sync,
            after_usage: desc.after_usage,
        };
        let r_type = ResourceType::Buffer {
            create_desc: BufferCreate::default(),
            import_info: Some(imported),
        };

        // render pass index doesn't matter here as imported resources aren't created by a render
        // pass
        let r = self.create_new_handle(usize::MAX);
        self.set_resource_type_for(r, r_type);

        r
    }

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
            DropLink::append_drop_list(&self.graph_arena, &mut self.payload_dropper_head, payload);

            // We need to use the pointer here as the mutable ref created by arena.alloc will get
            // moved into the NonNull instance created as &mut doesn't impl Copy. This is still safe
            // though as we don't _use_ the payload anywhere until we give it to the setup fn, or
            // if the builder gets dropped.
            {
                self.pass_access_info.current_pass_index = self.render_passes.len();
                let mut resources = ResourceRegistry(self);
                setup_fn(payload.as_mut(), &mut resources);
            }

            self.pass_access_info.clear();

            // Construct the CallbackRenderPass instance and handoff to add_pass
            let payload = NonNull::from(payload);
            let callback_pass = CallbackRenderPass::new(payload, exec_fn);
            self.add_pass_internal(name, callback_pass);
        }
    }

    pub fn build(mut self) -> FrameGraph {
        // With the graph finalized we can now iterate all our resource versions and collect the
        // full set of usage flags the resources have been declared to be used with.
        self.collect_resource_usages();
        self.validate_imported_resource_usages();

        let arena = std::mem::take(&mut self.graph_arena);
        let render_passes = std::mem::take(&mut self.render_passes);
        let root_resources = std::mem::take(&mut self.root_resources);
        let resource_versions = std::mem::take(&mut self.resource_versions);
        let resource_handles = std::mem::take(&mut self.resource_handles);
        let pass_dropper_head = std::mem::take(&mut self.pass_dropper_head);
        let payload_dropper_head = std::mem::take(&mut self.payload_dropper_head);

        FrameGraph {
            arena,
            render_passes,
            root_resources,
            resource_versions,
            resource_handles,
            pass_dropper_head,
            payload_dropper_head,
        }
    }
}

// Internal functions exposed through ResourceRegistry
impl FrameGraphBuilder {
    pub(crate) fn add_pass_internal<T: IRenderPass>(&mut self, name: &str, pass: T) {
        let name = self.graph_arena.alloc_str(name);
        let name = NonNull::from(name);
        let pass = self.graph_arena.alloc(pass);
        let mut pass = NonNull::from(pass);
        DropLink::append_drop_list(&self.graph_arena, &mut self.pass_dropper_head, pass);

        unsafe {
            let pass = NonNull::from(pass.as_mut() as &mut dyn IRenderPass);
            let pass = RenderPass {
                pass,
                name,
            };
            self.render_passes.push(pass);
        }
    }

    pub(crate) fn read_texture<R: Into<ResourceRef>>(
        &mut self,
        resource: R,
        sync: BarrierSync,
        usage: TextureUsageFlags,
        layout: ImageLayout,
    ) -> ResourceRef {
        let r = resource.into();

        self.assert_resource_handle_is_texture(r);
        self.add_texture_flags_to_version_for(r, usage);

        let desc = TextureAccess {
            texture: r.0,
            sync,
            usage,
            layout,
        };
        self.pass_access_info
            .reads
            .push(ResourceAccess::Texture(desc));
        r
    }

    pub(crate) fn read_buffer<R: Into<ResourceRef>>(
        &mut self,
        resource: R,
        sync: BarrierSync,
        usage: BufferUsageFlags,
    ) -> ResourceRef {
        let r = resource.into();

        self.assert_resource_handle_is_buffer(r);
        self.add_buffer_flags_to_version_for(r, usage);

        let desc = BufferAccess {
            buffer: r.0,
            sync,
            usage,
        };
        self.pass_access_info
            .reads
            .push(ResourceAccess::Buffer(desc));
        r
    }

    pub(crate) fn write_texture<R: Into<ResourceMut>>(
        &mut self,
        resource: R,
        sync: BarrierSync,
        usage: TextureUsageFlags,
        layout: ImageLayout,
    ) -> ResourceMut {
        let r = resource.into();

        self.assert_resource_handle_is_texture(r);
        self.validate_and_update_for_handle_write(r);
        let renamed_r = self.increment_handle_for_write(r, self.render_passes.len());

        // Add the requested usage flags to the resource version's usage set
        //
        // This _MUST_ happen after increment_handle_for_write, as otherwise there will be no
        // matching entry in resource_versions to write our usage flags into.
        self.add_texture_flags_to_version_for(renamed_r, usage);

        let desc = TextureAccess {
            texture: r.0,
            sync,
            usage,
            layout,
        };
        self.pass_access_info
            .writes
            .push(ResourceAccess::Texture(desc));

        renamed_r
    }

    pub(crate) fn write_buffer<R: Into<ResourceMut>>(
        &mut self,
        resource: R,
        sync: BarrierSync,
        usage: BufferUsageFlags,
    ) -> ResourceMut {
        let r = resource.into();

        self.assert_resource_handle_is_buffer(r);
        self.validate_and_update_for_handle_write(r);
        let renamed_r = self.increment_handle_for_write(r, self.render_passes.len());

        // Add the requested usage flags to the resource version's usage set
        //
        // This _MUST_ happen after increment_handle_for_write, as otherwise there will be no
        // matching entry in resource_versions to write our usage flags into.
        self.add_buffer_flags_to_version_for(renamed_r, usage);

        let desc = BufferAccess {
            buffer: r.0,
            sync,
            usage,
        };
        self.pass_access_info
            .writes
            .push(ResourceAccess::Buffer(desc));

        renamed_r
    }

    pub(crate) fn validate_and_update_for_handle_write(&mut self, r: ResourceMut) {
        // Validate the write status for the resource handle and update it if it's valid to write
        // to this handle.
        //
        // A ResourceMut can only be used for a single write_<resource> call as otherwise it would
        // not be possible to extract a single program order. If two passes tried to write to the
        // same resource with the same handle, which write should execute first? The only real
        // solution would be to choose whichever pass was registered first, but then the graph order
        // is dependent on pass submission order which is something we really _don't_ want.
        let handle_id = r.0.handle_id() as usize;
        let handle_info = &mut self.resource_handles[handle_id];
        if handle_info.is_written() {
            panic!("Attempted to write a resource through the same handle more than once!");
        }
        handle_info.mark_written();
    }

    pub(crate) fn create_texture(&mut self, desc: &TextureDesc) -> ResourceMut {
        let name = desc.name.map(|v| self.graph_arena.alloc_str(v));
        let name = name.map(|v| NonNull::from(v));
        let create_desc = TextureCreate {
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
            usage: desc.usage,
            name,
        };

        let r = self.create_new_handle(self.render_passes.len());
        self.set_resource_type_for(
            r,
            ResourceType::Texture {
                create_desc,
                import_info: None,
            },
        );
        self.add_texture_flags_to_version_for(r, desc.usage);

        r
    }

    pub(crate) fn create_buffer(&mut self, desc: &BufferDesc) -> ResourceMut {
        let name = desc.name.map(|v| self.graph_arena.alloc_str(v));
        let name = name.map(|v| NonNull::from(v));
        let create_desc = BufferCreate {
            size: desc.size,
            usage: desc.usage,
            name,
        };

        let r = self.create_new_handle(self.render_passes.len());
        self.set_resource_type_for(
            r,
            ResourceType::Buffer {
                create_desc,
                import_info: None,
            },
        );
        self.add_buffer_flags_to_version_for(r, desc.usage);

        r
    }

    pub(crate) fn set_resource_type_for(
        &mut self,
        r: impl Into<ResourceRef>,
        r_type: ResourceType,
    ) {
        let r = r.into();

        let root = &mut self.root_resources[r.0.root_id() as usize];
        root.resource_type = r_type;
    }

    pub(crate) fn add_buffer_flags_to_version_for(
        &mut self,
        r: impl Into<ResourceRef>,
        usage: BufferUsageFlags,
    ) {
        let r = r.into();

        // Add the requested usage flags to the resource version's usage set
        let version_id = r.0.version_id();
        self.resource_versions[version_id as usize].usage_buf |= usage;
    }

    pub(crate) fn add_texture_flags_to_version_for(
        &mut self,
        r: impl Into<ResourceRef>,
        usage: TextureUsageFlags,
    ) {
        let r = r.into();

        // Add the requested usage flags to the resource version's usage set
        let version_id = r.0.version_id();
        self.resource_versions[version_id as usize].usage_tex |= usage;
    }

    pub(crate) fn increment_handle_for_write(
        &mut self,
        r: ResourceMut,
        render_pass: usize,
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
            previous: VersionIndex::new(r.0.version_id()).unwrap(),

            // Defaults to empty, this will be initialized elsewhere
            usage_tex: Default::default(),
            usage_buf: Default::default(),
            render_pass,
        });
        self.resource_handles.push(ResourceHandleInfo::default());

        // Assert we never create u16::MAX versions. This is _critical_ as 65535 is a niche
        // value to encode a 'null' version. By guaranteeing 65535 is never a valid index into this
        // array we can rely on rust's bounds checking to assert that we never access the 65535th
        // version by construction (we never make it).
        assert!(self.resource_versions.len() < ((u16::MAX) as usize));

        ResourceMut(id)
    }

    pub(crate) fn create_new_handle(&mut self, render_pass: usize) -> ResourceMut {
        let base = u16::try_from(self.root_resources.len()).unwrap();
        let version = u16::try_from(self.resource_versions.len()).unwrap();
        let handle = u16::try_from(self.resource_handles.len()).unwrap();
        let id = ResourceId::new(base, version, handle);
        self.root_resources.push(ResourceRoot::default());
        self.resource_versions.push(ResourceVersion {
            // We need the root resource here to allow iterations over the version array to easily
            // link back to their roots
            root_resource: base,

            // A create is by definition the first version of a resource so the 'previous' link is
            // initialized as an invalid id. This encodes the 'end' of the list
            previous: VersionIndex::INVALID,

            // Defaults to empty, this will be initialized elsewhere
            usage_tex: Default::default(),
            usage_buf: Default::default(),

            render_pass,
        });
        self.resource_handles.push(ResourceHandleInfo::default());

        // Assert we never create u16::MAX versions. This is _critical_ as 65535 is a niche
        // value to encode a 'null' version. By guaranteeing 65535 is never a valid index into this
        // array we can rely on rust's bounds checking to assert that we never access the 65535th
        // version by construction (we never make it).
        assert!(self.resource_versions.len() < ((u16::MAX) as usize));

        ResourceMut(id)
    }

    pub(crate) fn assert_resource_handle_is_texture(&self, r: impl Into<ResourceRef>) {
        let r = r.into();
        let root_type = &self.root_resources[r.0.root_id() as usize].resource_type;
        assert!(matches!(root_type, ResourceType::Texture { .. }));
    }

    pub(crate) fn assert_resource_handle_is_buffer(&self, r: impl Into<ResourceRef>) {
        let r = r.into();
        let root_type = &self.root_resources[r.0.root_id() as usize].resource_type;
        assert!(matches!(root_type, ResourceType::Buffer { .. }));
    }

    /// Iterates all resource versions and accumulates their usage flags into the root resource.
    ///
    /// This is the final step in determining the full set of usage flags for how a resource is used
    /// by all the passes within the graph.
    ///
    /// This requires that each version has had its usages fully collected before hand.
    pub(crate) fn collect_resource_usages(&mut self) {
        for version in self.resource_versions.iter() {
            let root = &mut self.root_resources[version.root_resource as usize];
            root.usage_tex |= version.usage_tex;
            root.usage_buf |= version.usage_buf;
        }
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
                ResourceType::Buffer {
                    import_info: Some(import),
                    ..
                } => {
                    let desc = import.resource.desc();
                    let r_name = desc.name.unwrap_or("Unnamed resource");
                    assert!(
                        desc.usage.contains(root.usage_buf),
                        "Resource '{}' used in unsupported usage. Allowed: {:?}. Attempted: {:?}",
                        r_name,
                        desc.usage,
                        root.usage_buf
                    );
                }
                ResourceType::Texture {
                    import_info: Some(import),
                    ..
                } => {
                    let desc = import.resource.desc();
                    let r_name = desc.name.unwrap_or("Unnamed resource");
                    assert!(
                        desc.usage.contains(root.usage_tex),
                        "Resource '{}' used in unsupported usage. Allowed: {:?}. Attempted: {:?}",
                        r_name,
                        desc.usage,
                        root.usage_buf
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
            DropLink::drop_and_null(&mut self.pass_dropper_head);
            DropLink::drop_and_null(&mut self.payload_dropper_head);
        }
    }
}

/// An interface constrained way to access the frame graph builder for collecting information from
/// render pass setup callbacks.
pub struct ResourceRegistry<'a>(&'a mut FrameGraphBuilder);

impl<'a> ResourceRegistry<'a> {
    /// Declares that this pass would like to import the given resource into the frame graph with
    /// the given parameters.
    ///
    /// This is a wrapper over [FrameGraphBuilder::import_texture].
    pub fn import_texture(&mut self, desc: &TextureImportDesc) -> ResourceMut {
        self.0.import_texture(desc)
    }

    /// Declares that this pass would like to import the given resource into the frame graph with
    /// the given parameters.
    ///
    /// This is a wrapper over [FrameGraphBuilder::import_buffer].
    pub fn import_buffer(&mut self, desc: &BufferImportDesc) -> ResourceMut {
        self.0.import_buffer(desc)
    }

    /// Declares a read access to the given texture, with the given sync parameters.
    ///
    /// The returned resource handle is equal to the handle given in 'r'. It is returned simply as
    /// a utility to mirror the write declaration functions.
    pub fn read_texture<R: Into<ResourceRef>>(
        &mut self,
        resource: R,
        sync: BarrierSync,
        usage: TextureUsageFlags,
        layout: ImageLayout,
    ) -> ResourceRef {
        self.0.read_texture(resource, sync, usage, layout)
    }

    /// Declares a read access to the given buffer, with the given sync parameters.
    ///
    /// The returned resource handle is equal to the handle given in 'r'. It is returned simply as
    /// a utility to mirror the write declaration functions.
    pub fn read_buffer<R: Into<ResourceRef>>(
        &mut self,
        resource: R,
        sync: BarrierSync,
        usage: BufferUsageFlags,
    ) -> ResourceRef {
        self.0.read_buffer(resource, sync, usage)
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
    pub fn write_texture<R: Into<ResourceMut>>(
        &mut self,
        resource: R,
        sync: BarrierSync,
        usage: TextureUsageFlags,
        layout: ImageLayout,
    ) -> ResourceMut {
        self.0.write_texture(resource, sync, usage, layout)
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
    pub fn write_buffer<R: Into<ResourceMut>>(
        &mut self,
        resource: R,
        sync: BarrierSync,
        usage: BufferUsageFlags,
    ) -> ResourceMut {
        self.0.write_buffer(resource, sync, usage)
    }

    /// Declares that a new, transient texture will be created and used by the pass.
    ///
    /// The resource will be created with the given parameters with only a single exception. The
    /// resource 'usage' flagged in the resource description 'desc' encodes only what usage flags
    /// the creating pass will use the resource as and _NOT_ the set of all usage flags the resource
    /// can ever be used as. This is a noteworthy difference compared to the documentation on the
    /// desc struct, which _will_ say otherwise, as it is intended for creating new resources at
    /// the RHI level.
    ///
    /// The transient resource will be created by the frame graph internally. The full set of usage
    /// flags will be deduced as the union of all the usages declared for the resource across all
    /// passes within the graph. Otherwise a pass would have to know all the ways a resource would
    /// be used, either leading to overly broad usage flags or poorly composable passes.
    pub fn create_texture(&mut self, desc: &TextureDesc) -> ResourceMut {
        self.0.create_texture(desc)
    }

    /// Declares that a new, transient buffer will be created and used by the pass.
    ///
    /// The resource will be created with the given parameters with only a single exception. The
    /// resource 'usage' flagged in the resource description 'desc' encodes only what usage flags
    /// the creating pass will use the resource as and _NOT_ the set of all usage flags the resource
    /// can ever be used as. This is a noteworthy difference compared to the documentation on the
    /// desc struct, which _will_ say otherwise, as it is intended for creating new resources at
    /// the RHI level.
    ///
    /// The transient resource will be created by the frame graph internally. The full set of usage
    /// flags will be deduced as the union of all the usages declared for the resource across all
    /// passes within the graph. Otherwise a pass would have to know all the ways a resource would
    /// be used, either leading to overly broad usage flags or poorly composable passes.
    pub fn create_buffer(&mut self, desc: &BufferDesc) -> ResourceMut {
        self.0.create_buffer(desc)
    }
}

pub struct TextureImportDesc<'a> {
    /// The texture resource to import into the frame graph
    pub resource: &'a dyn ITexture,

    /// The pipeline stage to synchronize with on first use within the frame graph
    pub before_sync: BarrierSync,

    /// The usage flags to synchronize with before the first use of the resource within the frame
    /// graph
    pub before_usage: TextureUsageFlags,

    /// The image layout the resource is expected to be in prior to the frame graph executing
    pub before_layout: ImageLayout,

    /// The pipeline stage to synchronize with as the immediate use after the frame graph is
    /// completed
    pub after_sync: BarrierSync,

    /// The usage flags to synchronize with as the immediate use after the frame graph is completed
    pub after_usage: TextureUsageFlags,

    /// The image layout the resource is expected to be transitioned to when completing the frame
    /// graph
    pub after_layout: ImageLayout,
}

pub struct BufferImportDesc<'a> {
    /// The buffer resource to import into the frame graph
    pub resource: &'a dyn IBuffer,

    /// The pipeline stage to synchronize with on first use within the frame graph
    pub before_sync: BarrierSync,

    /// The usage flags to synchronize with before the first use of the resource within the frame
    /// graph
    pub before_usage: BufferUsageFlags,

    /// The pipeline stage to synchronize with as the immediate use after the frame graph is
    /// completed
    pub after_sync: BarrierSync,

    /// The usage flags to synchronize with as the immediate use after the frame graph is completed
    pub after_usage: BufferUsageFlags,
}

#[cfg(test)]
mod tests;