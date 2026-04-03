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

pub mod builder;
pub mod config;
pub mod draw_options;
pub mod frame_graph;
pub mod immediate_resource_builder;
pub mod pass;
pub mod render_plane;
pub mod shader_accessor;
pub mod state_cache;
pub mod surface_notify;

use std::sync::Arc;

use aleph_alloc::instrumentation::system;
use aleph_alloc::{BBox, BVec};
use aleph_ecs::entity::EntityHandle;
use aleph_ecs::world::World;
use aleph_frame_graph::ImportBundle;
use aleph_object_system::unsafe_impl_iobject;
use parking_lot::Mutex;
use smallbox::SmallBox;
use smallbox::space::S8;

use crate::async_resource_loader::loader_notify::LoaderNotify;
use crate::async_resource_loader::{AsyncResourceLoader, AsyncResourceLoaderConfig};
use crate::internal::MgSystem;
use crate::internal::async_resource_loader::renderer_channel::{
    GenericLoaderMessageDispatcher, LoaderDispatcherError, LoaderMessageDispatcher,
};
use crate::internal::buffer::BufferObjectStore;
use crate::internal::material_instance::{MaterialInstanceObject, MaterialInstanceStore};
use crate::internal::renderer::deferred_deletion_manager::{
    DeferredDeletionManager, DeletionBundle,
};
use crate::internal::renderer::frame_manager::FrameManager;
use crate::internal::renderer::graph_manager::GraphManager;
use crate::internal::renderer::immediate_upload_queue::ImmediateUploadQueue;
use crate::internal::renderer::last_use_tracker::LastUseTracker;
use crate::internal::renderer::object_deletion_queue::ObjectDeleteQueue;
use crate::internal::renderer::swap_manager::SwapManager;
use crate::internal::texture::TextureObjectStore;
use crate::material::Material;
use crate::material_instance::{MaterialInstanceDesc, MaterialInstanceHandle};
use crate::renderer::builder::RendererBuilder;
use crate::renderer::config::RendererConfig;
use crate::renderer::draw_options::DrawOptions;
use crate::renderer::frame_graph::{GraphArgsLayout, GraphSwapImageInfo};
use crate::renderer::immediate_resource_builder::ImmediateResourceBuilder;
use crate::renderer::state_cache::StateCache;
use crate::renderer::surface_notify::ISurfaceNotify;
use crate::resource::buffer::BufferHandle;
use crate::resource::texture::TextureHandle;
use crate::resource::texture::simple::SimpleTextureDesc;
use crate::resource_loader::mip_upload::MipUploadDesc;
use crate::resource_loader::upload_buffer::IUploadBuffer;

pub struct Renderer {
    pub(crate) config: RendererConfig,
    pub(crate) device: Arc<dyn rhi::IDevice>,
    pub(crate) queue: Arc<dyn rhi::IQueue>,
    pub(crate) frame_fence: rhi::FenceHandle,
    pub(crate) async_loader_dispatchers:
        BVec<BBox<dyn LoaderMessageDispatcher, MgSystem>, MgSystem>,
    pub(crate) graph_manager: GraphManager,
    pub(crate) frame_manager: FrameManager,
    pub(crate) swap_manager: SwapManager,
    pub(crate) deferred_deletion_manager: DeferredDeletionManager,
    pub(crate) texture_object_store: TextureObjectStore,
    pub(crate) buffer_object_store: BufferObjectStore,
    pub(crate) material_instance_store: MaterialInstanceStore,
    pub(crate) object_delete_queue: ObjectDeleteQueue,
    pub(crate) immediate_upload_queue: ImmediateUploadQueue,
    pub(crate) state_cache: Mutex<StateCache>,
}
unsafe_impl_iobject!(Renderer, "019b34ce-ad2b-7d50-9be0-875eac512690");

impl Renderer {
    pub fn builder() -> RendererBuilder {
        RendererBuilder::new()
    }

    /// Gets the GPU device handle the renderer is using
    #[inline]
    pub fn device(&self) -> &dyn rhi::IDevice {
        self.device.as_ref()
    }

    /// Creates a new simple texture immediately, taking the given data.
    ///
    /// This will immediately return a texture handle. The new texture will be added to the
    /// immediate upload queue and the data will be available for use in the current frame.
    ///
    /// By default, 'data' must provide data for the entire texture, including the entire mip chain,
    /// in the given buffer. If the caller has enabled 'generate_mips' in [`SimpleTextureOptions`]
    /// then only mip level 0 needs to be provided. The remaining levels will be generated on the
    /// GPU by downsampling from level 0.
    #[inline]
    pub fn create_simple_texture_immediate<T: SimpleTextureDesc>(
        &mut self,
        desc: &T,
        data: MipUploadDesc,
        options: &SimpleTextureOptions,
    ) -> Result<TextureHandle, ()> {
        self.immediate_resource_builder()
            .create_simple_texture_immediate(desc, data, options)
    }

    /// Creates a new buffer immediately, taking the given data.
    ///
    /// This will immediately return a handle. The new object will be added to the immediate upload
    /// queue and the data will be available for use in the current frame.
    #[inline]
    pub fn create_buffer_immediate(
        &mut self,
        size: u64,
        data: Option<SmallBox<dyn IUploadBuffer, S8>>,
        options: &BufferOptions,
    ) -> Result<BufferHandle, ()> {
        self.immediate_resource_builder()
            .create_buffer_immediate(size, data, options)
    }

    /// Get access to a [`ImmediateResourceBuilder`] over this renderer.
    #[inline]
    pub fn immediate_resource_builder(&mut self) -> ImmediateResourceBuilder<'_> {
        ImmediateResourceBuilder {
            device: self.device.as_ref(),
            texture_object_store: &mut self.texture_object_store,
            buffer_object_store: &mut self.buffer_object_store,
            immediate_upload_queue: &mut self.immediate_upload_queue,
        }
    }

    pub fn create_async_resource_loader<C: Send + 'static>(
        &mut self,
        config: AsyncResourceLoaderConfig,
    ) -> Option<(AsyncResourceLoader<C>, LoaderNotify<C>)> {
        let device = self.device.clone();
        let queue = device.get_queue(rhi::QueueType::Transfer)?;
        let fence = device.create_fence(0).ok()?;

        // Construct the channel that connects the loader to the renderer.
        //
        // This is the channel over which the loader will publish messages that will notify the
        // renderer of completed or canceled uploads.
        //
        // The renderer's job is to consume these messages, allocate resource handles for the new
        // buffers/textures, and then to publish new messages onto the 'notify' channel with the
        // new handles. It must also pass cancelation messages along.
        //
        // The reason we must dispatch the messages inside the renderer, rather than somewhere else
        // earlier in the frame to make the resource available sooner, is because we need exclusive
        // access to the resource pools to allocate the handles. Ideally we'd like to dispatch
        // before we call 'draw_frame', but resolving ownership makes this very difficult. We accept
        // the extra latency for now.
        //
        // TODO: should the loader push cancel messages directly to the notify channel instead?
        let (loader_sender, renderer_receiver) = crossbeam::channel::unbounded();

        // Construct the channel that connects any outside listeners to the renderer.
        //
        // This is the channel over which the renderer will publish messages that will export the
        // complete buffer/texture handles to the outside world.
        //
        // The renderer's job is to consume messages from the loader, process and make resources
        // available via resource handles, and then export those handles to the connected listeners.
        let (renderer_sender, notify_receiver) = crossbeam::channel::unbounded();

        // Add a type erased dispatcher that handles receiving and processing the messages from the
        // loader we just made.
        //
        // We must type erase the dispatcher because we can't associate the 'C' with the type of
        // the renderer, lest we force all loaders to use the same 'C'. By erasing the type a 'C'
        // can be picked for each loader. The renderer doesn't care about the contents of the
        // cookie.
        //
        // It's up to the dispatcher to forward the messages and create the resource handles for the
        // uploaded resources.
        let generic_dispatcher = GenericLoaderMessageDispatcher {
            device: device.clone(),
            renderer_receiver,
            renderer_sender,
        };
        let dispatcher = BBox::new_in(generic_dispatcher, system());
        self.async_loader_dispatchers
            .push(aleph_alloc::unsize_box!(dispatcher));

        let notify = LoaderNotify {
            receiver: notify_receiver,
        };

        let loader = AsyncResourceLoader::<C>::new(device, queue, fence, loader_sender, config)?;

        Some((loader, notify))
    }

    pub fn create_material_instance(
        &mut self,
        material: &Arc<Material>,
        desc: &MaterialInstanceDesc,
    ) -> Option<MaterialInstanceHandle> {
        let mut object = MaterialInstanceObject::new(material.clone());

        if desc.bindings.len() < object.bindings.len() {
            // Not enough bindings to init material instance resources
            return None;
        }

        object.double_sided = desc.double_sided;

        debug_assert!(object.pre_validate_binding_write(desc.bindings).is_some());
        object.write_bindings_from_array(desc.bindings);

        let handle = self.material_instance_store.pool.alloc(object);
        Some(handle)
    }

    /// Marks the given texture for destruction at the end of the next [`Renderer::draw_frame`]
    /// call. The texture handle will remain alive until the end of the next call to 'draw_frame'.
    ///
    /// The memory owned by the texture object will be released some time later, once the texture
    /// can no longer be in use on the GPU.
    pub fn destroy_texture(&mut self, texture: TextureHandle) {
        self.object_delete_queue.texture.push(texture);
    }

    /// Marks the given buffer for destruction at the end of the next [`Renderer::draw_frame`]
    /// call. The buffer handle will remain alive until the end of the next call to 'draw_frame'.
    ///
    /// The memory owned by the buffer object will be released some time later, once the buffer
    /// can no longer be in use on the GPU.
    pub fn destroy_buffer(&mut self, buffer: BufferHandle) {
        self.object_delete_queue.buffer.push(buffer);
    }

    pub fn draw_frame(&mut self, options: &DrawOptions, scene: &mut World, camera: EntityHandle) {
        unsafe {
            // Each frame we maintain a rolling history of each buffer/texture's last use within
            // the frame. This lets us automatically issue barriers.
            //
            // This is primarily used by commands issued to the renderer. The frame graph, which
            // drives the core of our renderer, does _not_ use this directly. The frame graph
            // handles barriers on its own. Work outside the frame graph will use this.
            let mut last_uses = LastUseTracker::new();

            // Acquire the swap image first. This may stall (varies wildly between platforms).
            //
            // Ideally stalling here prevents us from stalling below.
            let acquired_image = self.swap_manager.acquire_next_image();
            let acquired_tex = acquired_image.swap_image.texture();

            // If the swap chain was rebuilt (or this is the first frame, and we haven't built the
            // frame graph yet) we need to (re)build the frame graph!
            let no_graph = self.graph_manager.frame_graph.is_none();
            let swap_chain_rebuilt = acquired_image.rebuilt;
            let force_rebuild = options.force_rebuild_frame_graph;
            if no_graph || swap_chain_rebuilt || force_rebuild {
                // Must be called before 'frame_manager.get_next_frame' otherwise we'll stall
                // waiting for a frame that will never come
                self.frame_manager.wait_all_retired();
                self.graph_manager.build_graph(
                    &self.config,
                    self.state_cache.get_mut(),
                    self.device.as_ref(),
                    &self.swap_manager,
                );
            }

            // Mark the beginning of our new frame. Ask for a new index from the frame manager and
            // provide a handle to the fence that we will associate with the frame's queue
            // submission.
            //
            // This may stall if we're running too far ahead of the GPU. When we stall is configured
            // by 'render_ahead_frames'. We grab the swap image first before we wait here as it's
            // likely the stall on the swap chain may take long enough we don't have to wait here.
            let frame = self.frame_manager.get_next_frame(self.frame_fence.clone());

            // Create a new deletion bundle. This is where all deferred deletion resources should
            // be registered with to extend their lifetime until the current frame has been retired.
            //
            // We extend the lifetime of any deleted resource until the end of the frame currently
            // being recorded. We provide the bundle a handle to the frame's fence. Once it's been
            // enqueued the resources will be kept alive until we find the fence has been signaled.
            //
            // Once the work is retired we drop all the handles, allowing the resources to be
            // deallocated.
            let mut delete_bundle =
                DeletionBundle::new(self.frame_fence.clone(), frame.signal_value());

            // Iterate through the live list of loader dispatchers and consume all the messages
            // that have been published since last time we checked.
            let mut i = 0;
            loop {
                if i >= self.async_loader_dispatchers.len() {
                    break;
                }

                let result = self.async_loader_dispatchers[i].dispatch_messages(
                    &mut last_uses,
                    &mut self.buffer_object_store,
                    &mut self.texture_object_store,
                );
                match result {
                    Ok(_) => {
                        // Successfully dispatched all our messages?
                        i += 1;
                    }
                    Err(LoaderDispatcherError::LoaderDisconnected)
                    | Err(LoaderDispatcherError::ListenerDisconnected) => {
                        // If the loader or listener is disconnected we drop the dispatcher which
                        // will close all the channels. This notifies the other party that there's
                        // nobody on the other end of the channel. The end result should be that
                        // if one side disconnects, all parties disconnect and clean up all their
                        // resources.
                        //
                        // We don't care who disconnected from who.
                        let _ = self.async_loader_dispatchers.swap_remove(i);
                    }
                }
            }

            let mut list = self
                .device
                .create_command_list(&rhi::CommandListDesc {
                    queue_type: rhi::QueueType::General,
                    name: None,
                })
                .unwrap();

            {
                let mut cmd = list.begin_general().unwrap();

                // The very first thing to do is to record our copy commands for the immediate
                // upload resources that were created since the last frame was rendered. These
                // resources must be seen as if the data was immediately uploaded...
                //
                // Doing this first meets that requirement, the first use of the resource will see
                // the uploaded data.
                self.immediate_upload_queue.record_upload_commands(
                    &mut last_uses,
                    &mut delete_bundle,
                    &mut cmd,
                    &self.buffer_object_store,
                    &self.texture_object_store,
                );

                // Before we render the scene we need to flush any unsynchronized accesses to the
                // non-frame-graph resources. This will issue a barrier on any dirty resources,
                // making any preceding writes visible to the frame graph render passes.
                last_uses.flush_for_read_in_frame_graph(
                    &mut cmd,
                    &self.buffer_object_store,
                    &self.texture_object_store,
                );

                // Execute the frame graph to draw the scene
                let mut import_bundle = ImportBundle::default();
                import_bundle.add_resource(
                    self.graph_manager.swap_image_id.clone().unwrap(),
                    acquired_tex,
                );
                let size = acquired_image.extent;
                let _ = scene.insert_singleton(GraphSwapImageInfo {
                    desc: self.swap_manager.desc.clone(),
                    aspect: size.width as f32 / size.height as f32,
                });
                let args = GraphArgsLayout {
                    scene,
                    camera,
                    texture_pool: self.texture_object_store.accessor(),
                    buffer_pool: self.buffer_object_store.accessor(),
                    material_instance_pool: self.material_instance_store.accessor(),
                    state_cache: &self.state_cache,
                };
                self.graph_manager.execute_graph(
                    &self.config,
                    frame.frame_index,
                    &import_bundle,
                    &mut cmd,
                    &args,
                );

                // Issue a 'release' barrier onto the general queue that drains all pipelines and
                // ensures all work in frame N-1 is complete before frame N can execute.
                //
                // Without this there is nothing that explicitly prevents the FG execution of the
                // previous frame overlapping with the current frame. The FG does not issue before
                // scopes for transient resources which means that back to back executions of an
                // FG could overlap and lead to all sorts of sadness. We do know what the before
                // scope _would_ be, it's the after scope of the last use of that resource. But
                // using that knowledge is likely not worth it.
                //
                // We _could_ change the before scope for our discard barriers to sync with the
                // final usage from a previous run of the same FG. However, this is likely to create
                // pipeline bubbles within the frame for no reason. Combining all the 'after' scopes
                // of our final transient resources is very likely to equal a full pipeline flush.
                // Issuing these barriers _before_ FG execution will place that bubble in the middle
                // of our frame. This will prevent work overlap between frame graphs instances or
                // non-fg work.
                //
                // Instead, we issue a wild card all->all barrier after our whole frame has
                // executed. This flushes all the transient resources and work from pipelines/caches
                // at the end of the frame at a time when we're going to do this anyway as part of
                // the sync with the presentation engine. This means our discard operations in the
                // FG can use a null before scope and get maximal work overlap.
                cmd.resource_barrier(
                    &[rhi::GlobalBarrier {
                        before_sync: rhi::BarrierSync::ALL,
                        after_sync: rhi::BarrierSync::ALL,
                        before_access: rhi::BarrierAccess::COMMON,
                        after_access: rhi::BarrierAccess::COMMON,
                    }],
                    &[],
                    &[],
                );

                cmd.close().unwrap();
            }

            self.queue
                .submit(&rhi::QueueSubmitDesc {
                    command_lists: &[Some(list).into()],
                    wait_fences: &[],
                    wait_values: &[],
                    signal_fences: &[&self.frame_fence],
                    signal_values: &[frame.signal_value()],
                    swap_image: Some(acquired_image.swap_image.as_ref()),
                })
                .unwrap();

            self.swap_manager
                .present(self.queue.as_ref(), acquired_image);

            for buffer in self.object_delete_queue.buffer.drain(..) {
                if let Some(object) = self.buffer_object_store.pool.free(buffer) {
                    if let Some(handle) = object.object {
                        delete_bundle.buffers.push(handle)
                    }
                }
            }

            for texture in self.object_delete_queue.texture.drain(..) {
                if let Some(object) = self.texture_object_store.pool.free(texture) {
                    if let Some(handle) = object.object {
                        delete_bundle.textures.push(handle)
                    }
                }
            }

            self.deferred_deletion_manager.push_bundle(delete_bundle);
            self.deferred_deletion_manager.delete_retired_bundles();
        }
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        // Wait on all in-flight submissions... Must do this first to make sure the GPU is no longer
        // in flight.
        self.frame_manager.wait_all_retired();

        // General wait idle just to be double sure the GPU is no longer working
        match self.device.wait_idle() {
            Ok(_) | Err(rhi::QueueWaitError::DeviceLost) => {
                // We can continue normally. Device lost in 'garbage_collect' is not fatal.
            }
            Err(err) => {
                // Anything else is fatal though
                panic!("Platform Error: {:#?}", err);
            }
        }

        // Call clean up on the internal managers.
        self.graph_manager.clean_up();
        // self.frame_manager: handled by wait_all_retired
        // self.swap_manager: nothing to clean up
        self.deferred_deletion_manager.assert_delete_all();
        self.texture_object_store.clean_up();
        self.buffer_object_store.clean_up();
        self.material_instance_store.clean_up();
        self.object_delete_queue.clean_up();
        self.immediate_upload_queue.clean_up();
        self.state_cache.get_mut().clear();

        // Garbage collect after freeing all the internally held handles
        match self.device.garbage_collect() {
            Ok(_) | Err(rhi::QueueGarbageCollectError::DeviceLost) => {
                // We can continue normally. Device lost in 'garbage_collect' is not fatal.
            }
            Err(err) => {
                // Anything else is fatal though
                panic!("Platform Error: {:#?}", err);
            }
        }
    }
}

/// Set of options that can be used to configure a simple texture upload.
#[derive(Clone, Default)]
pub struct SimpleTextureOptions {
    /// Should the renderer generate a complete mip chain for this texture?
    ///
    /// If true, then the only mip level that needs to be uploaded is level 0. The remaining levels
    /// are generated dynamically on the GPU before first use.
    pub generate_mips: bool,
}

/// Set of options that can be used to configure a buffer upload.
#[derive(Clone, Default)]
pub struct BufferOptions {
    /// Zero initialize the buffer if no data is provided.
    pub zero: bool,
}

/// Description of a surface provided by a host application that the renderer will manage a swap
/// chain over. The host application is responsible for the window or whatever OS mechanism is
/// providing the surface, the renderer is responsible for swap chain management.
///
/// A channel must be provided that the renderer will listen on. The host must send important
/// notifications about the surface so that the renderer may respond to them.
pub struct SharedSurfaceDesc {
    /// Handle to an [`rhi::ISurface`] that has been shared with the renderer by the host. The
    /// renderer will manage any swap chains constructed on this surface.
    pub surface: Arc<dyn rhi::ISurface>,

    /// Receiver half of a channel that the renderer may receive notifications about changes to the
    /// surface from the host application.
    pub channel: Box<dyn ISurfaceNotify>,
}
