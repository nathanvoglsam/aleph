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

use crate::render_pass::{CallbackRenderPass, IRenderPass};
use crate::resource::ResourceId;
use crate::{ResourceMut, ResourceRef, ResourceRegistry};
use aleph_any::AnyArc;
use aleph_arena_drop_list::DropLink;
use aleph_rhi_api::*;
use bumpalo::collections::Vec as BVec;
use bumpalo::Bump;
use std::mem::{forget, ManuallyDrop};
use std::ptr::NonNull;

pub struct FrameGraphBuilder {
    /// An arena that will be moved into the FrameGraph once the graph is finalized. This can be
    /// used to store anything that persists to the fully constructed graph.
    graph_arena: Bump,

    /// An arena used temporarily while constructing the frame graph. Will be freed with the
    /// [FrameGraphBuilder] instance. This can be used to allocate anything that only needs to exist
    /// as long as the graph is being built.
    build_arena: Bump,

    /// The list of all the render passes in the graph. The index of the pass in this list is the
    /// identity of the pass and is used to key to a number of different names
    render_passes: Vec<NonNull<dyn IRenderPass>>,

    /// Stores the names of each render pass keyed by the matching index in the render_passes list.
    render_pass_names: Vec<(*const u8, usize)>,

    /// The head of the dropper linked-list that contains all the drop functions for the render
    /// passes.
    pass_dropper_head: Option<NonNull<DropLink>>,

    /// The head of the dropper linked-list that contains droppers for the callback pass payloads.
    payload_dropper_head: Option<NonNull<DropLink>>,
}

impl FrameGraphBuilder {
    pub fn new() -> Self {
        Self {
            graph_arena: Default::default(),
            build_arena: Default::default(),
            render_passes: Vec::new(),
            render_pass_names: Vec::new(),
            pass_dropper_head: None,
            payload_dropper_head: None,
        }
    }

    pub fn import_texture(&mut self, desc: &TextureImportDesc) -> ResourceMut {
        let _imported = ImportedResource {
            resource: ResourceVariant::Texture(desc.resource.upgrade()),
            before_sync: desc.before_sync,
            before_usage_buf: Default::default(),
            before_usage_tex: desc.before_usage,
            before_layout: desc.before_layout,
            after_sync: desc.after_sync,
            after_usage_buf: Default::default(),
            after_usage_tex: desc.after_usage,
            after_layout: desc.after_layout,
        };
        todo!()
    }

    pub fn import_buffer(&mut self, desc: &BufferImportDesc) -> ResourceMut {
        let _imported = ImportedResource {
            resource: ResourceVariant::Buffer(desc.resource.upgrade()),
            before_sync: desc.before_sync,
            before_usage_buf: desc.before_usage,
            before_usage_tex: Default::default(),
            before_layout: Default::default(),
            after_sync: desc.after_sync,
            after_usage_buf: desc.after_usage,
            after_usage_tex: Default::default(),
            after_layout: Default::default(),
        };
        todo!()
    }

    pub fn add_pass<T: IRenderPass>(&mut self, name: &str, pass: T) {
        let name = self.graph_arena.alloc_str(name);
        let name = (name.as_ptr(), name.len());
        let pass = self.graph_arena.alloc(pass);
        let mut pass = NonNull::from(pass);
        DropLink::append_drop_list(&self.graph_arena, &mut self.pass_dropper_head, pass);

        unsafe {
            let pass = NonNull::from(pass.as_mut() as &mut dyn IRenderPass);
            self.render_passes.push(pass);
            self.render_pass_names.push(name);
        }
    }

    pub fn add_callback_pass<
        T: Send + Default + 'static,
        SetupFn: FnOnce(&mut T, &mut ResourceRegistry),
        ExecFn: FnMut(&T) + Send + 'static,
    >(
        &mut self,
        name: &str,
        setup_fn: SetupFn,
        exec_fn: ExecFn,
    ) {
        // TODO: How to collect info for the pass
        let mut resources = ResourceRegistry {
            reads: BVec::new_in(&self.build_arena),
            writes: BVec::new_in(&self.build_arena),
            creates: BVec::new_in(&self.build_arena),
        };

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
            setup_fn(payload.as_mut(), &mut resources);
            drop(resources); // TODO: What to do with this after we've run setup

            // Construct the CallbackRenderPass instance and handoff to add_pass
            let payload = NonNull::from(payload);
            let callback_pass = CallbackRenderPass::new(payload, exec_fn);
            self.add_pass(name, callback_pass);
        }
    }

    pub fn build(self) -> FrameGraph {
        // Safety: We're doing some shenanigans to destructure the FrameGraphBuilder, which we can't
        //         do directly because the type implements Drop. To work-around this we move the
        //         values out manually and then mem::forget self. This is just to work around the
        //         compiler preventing us from doing this the easy way (for good reasons), but we
        //         know this is okay as we wrote the drop impl we're skipping.
        unsafe {
            let v = ManuallyDrop::new(self);
            let arena = std::ptr::read(&v.graph_arena);
            let build_arena = std::ptr::read(&v.build_arena);
            let render_passes = std::ptr::read(&v.render_passes);
            let render_pass_names = std::ptr::read(&v.render_pass_names);
            let pass_dropper_head = v.pass_dropper_head;
            let payload_dropper_head = v.payload_dropper_head;

            // Drop the build_arena explicitly and then forget 'v' so we don't try and drop it
            // again (it's in a ManuallyDrop, but I want to make this explicit).
            drop(build_arena);
            forget(v);

            FrameGraph {
                arena,
                render_passes,
                render_pass_names,
                pass_dropper_head,
                payload_dropper_head,
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

pub struct FrameGraph {
    /// The bump allocation arena that provides the backing memory for the render passes and any
    /// other memory that's needed for them.
    ///
    /// This typically includes the IRenderPass objects themselves, their name strings and the
    /// payload objects for callback passes.
    arena: Bump,

    /// The list of all the render passes in the graph. The index of the pass in this list is the
    /// identity of the pass and is used to key to a number of different names
    render_passes: Vec<NonNull<dyn IRenderPass>>,

    /// Stores the names of each render pass keyed by the matching index in the render_passes list.
    render_pass_names: Vec<(*const u8, usize)>,

    /// The head of the dropper linked-list that contains all the drop functions for the render
    /// passes.
    pass_dropper_head: Option<NonNull<DropLink>>,

    /// The head of the dropper linked-list that contains droppers for the callback pass payloads.
    payload_dropper_head: Option<NonNull<DropLink>>,
}

impl FrameGraph {
    pub fn builder() -> FrameGraphBuilder {
        FrameGraphBuilder::new()
    }

    pub unsafe fn execute(&mut self) {}
}

impl Drop for FrameGraph {
    fn drop(&mut self) {
        // Safety: implementation and API guarantees that dropper only gets called once per
        //         object, and always on the correct type.
        unsafe {
            DropLink::drop_and_null(&mut self.pass_dropper_head);
            DropLink::drop_and_null(&mut self.payload_dropper_head);
        }
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

/// Enum that encodes a resource as a combination of either a buffer or a texture
pub enum ResourceVariant {
    Buffer(AnyArc<dyn IBuffer>),
    Texture(AnyArc<dyn ITexture>),
}

pub struct ImportedResource {
    pub resource: ResourceVariant,
    pub before_sync: BarrierSync,
    pub before_usage_buf: BufferUsageFlags,
    pub before_usage_tex: TextureUsageFlags,
    pub before_layout: ImageLayout,
    pub after_sync: BarrierSync,
    pub after_usage_buf: BufferUsageFlags,
    pub after_usage_tex: TextureUsageFlags,
    pub after_layout: ImageLayout,
}

#[cfg(test)]
mod tests {
    use crate::resource::ResourceId;
    use crate::{FrameGraph, ResourceMut, ResourceRef, ResourceRegistry};
    use std::num::NonZeroU64;

    // fn dummy_resource_ref() -> ResourceRef {
    //     ResourceRef(ResourceId(NonZeroU64::new(420).unwrap()))
    // }

    fn dummy_resource_mut() -> ResourceMut {
        ResourceMut(ResourceId(NonZeroU64::new(360).unwrap()))
    }

    #[test]
    pub fn test_builder() {
        #[derive(Default)]
        struct TestPassData {
            value: u32,
            resource: Option<ResourceMut>,
        }
        #[derive(Default)]
        struct TestPassData2 {
            value: i16,
            resource: Option<ResourceRef>,
        }

        let a_resource = dummy_resource_mut();
        let mut out_write = None;
        let mut out_read = None;

        let mut builder = FrameGraph::builder();

        builder.add_callback_pass(
            "test-pass",
            |data: &mut TestPassData, resources: &mut ResourceRegistry| {
                data.value = 1234;

                data.resource = Some(resources.write_resource(a_resource));
                out_write = data.resource;
            },
            |data: &TestPassData| {
                assert_eq!(data.value, 1234);
            },
        );

        builder.add_callback_pass(
            "test-pass-2",
            |data: &mut TestPassData2, resources: &mut ResourceRegistry| {
                data.value = -432;

                data.resource = Some(resources.read_resource(out_write.unwrap()));
                out_read = data.resource;
            },
                assert_eq!(data.value, -432);
            },
        );

        let mut graph = builder.build();

        unsafe {
            graph.execute();
        }
    }
}
