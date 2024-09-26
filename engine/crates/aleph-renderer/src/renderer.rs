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

use std::any::Any;
use std::num::NonZeroU8;
use std::sync::Arc;

use aleph_any::AnyArc;
use aleph_frame_graph::{FrameGraph, FrameGraphBuilder, ImportBundle, ResourceMut, ResourceRef};
use aleph_nstr::nstr;
use aleph_pin_board::{BoardScope, PinBoard};
use aleph_rhi_api::*;

use crate::pass::{self, GraphArgs, GraphArgsLayout, GraphSwapImageInfo};
use crate::{
    BufferHandle, BufferLoader, BufferPool, BufferUploadSource, DeletionPool,
    ShaderDatabaseAccessor, TextureHandle, TextureLoader, TexturePool, TextureUploadSource,
};

pub trait IRenderSurface: Any {
    fn get_render_extent(&self) -> Extent2D;
    fn get_swap_chain(&self) -> &dyn ISwapChain;
    fn needs_rebuild(&self) -> bool;
}

pub trait IRenderPlane: Any {
    fn register_passes(
        &self,
        frame_graph: &mut FrameGraphBuilder<GraphArgs>,
        device: &dyn IDevice,
        pin_board: &PinBoard,
        shader_db: &ShaderDatabaseAccessor,
    ) -> RenderPlaneOutput;
}

pub struct RenderPlaneOutput {
    /// Readable handle to the final (texture) resource that can be considered the output of this
    /// render plane. This will be composited together with the other planes before being presented
    /// to the screen.
    pub id: ResourceRef,

    /// A description of the texture resource. We need this so we know the format/size/etc of the
    /// image so we can correctly handle and validate when performing the merge pass.
    pub desc: TextureDesc<'static>,
}

#[derive(Default)]
pub struct DefaultRenderPlane();

impl IRenderPlane for DefaultRenderPlane {
    fn register_passes(
        &self,
        frame_graph: &mut FrameGraphBuilder<GraphArgs>,
        device: &dyn IDevice,
        pin_board: &PinBoard,
        shader_db: &ShaderDatabaseAccessor,
    ) -> RenderPlaneOutput {
        use crate::pass;

        pass::main_gbuffer::pass(frame_graph, device, pin_board, shader_db);
        pass::lighting_resolve::pass(frame_graph, device, pin_board, shader_db);
        let result = pass::tone_map::pass(frame_graph, device, pin_board, shader_db);

        result
    }
}

pub struct RendererBuilder {
    device: Option<AnyArc<dyn IDevice>>,
    surface: Option<Box<dyn IRenderSurface>>,
    frames_in_flight: usize,
    shader_db: Option<ShaderDatabaseAccessor<'static>>,
    render_planes: Vec<Box<dyn IRenderPlane>>,
}

impl Default for RendererBuilder {
    fn default() -> Self {
        Self {
            device: None,
            surface: None,
            frames_in_flight: 2,
            shader_db: None,
            render_planes: Vec::new(),
        }
    }
}

impl RendererBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn device(&mut self, device: AnyArc<dyn IDevice>) -> &mut Self {
        self.device = Some(device);
        self
    }

    pub fn surface(&mut self, surface: impl IRenderSurface) -> &mut Self {
        self.surface = Some(Box::new(surface));
        self
    }

    pub fn frames_in_flight(&mut self, frames_in_flight: usize) -> &mut Self {
        self.frames_in_flight = frames_in_flight;
        self
    }

    pub fn shader_db(&mut self, shader_db: ShaderDatabaseAccessor<'static>) -> &mut Self {
        self.shader_db = Some(shader_db);
        self
    }

    pub fn render_plane(&mut self, plane: impl IRenderPlane) -> &mut Self {
        self.render_planes.push(Box::new(plane));
        self
    }

    pub fn build<'a>(self) -> Option<Renderer> {
        let device = self.device.expect("Device missing!");
        let queue = device.get_queue(QueueType::General).unwrap();
        let shader_db = self.shader_db.expect("Shader DB missing!");

        let swap_manager = SwapManager {
            surface: self.surface.expect("RenderSurface missing!"),
            images: Vec::new(),
            desc: TextureDesc::default(),
            needs_rebuild: true,
        };

        let mut frames = Vec::new();
        frames.resize_with(self.frames_in_flight, || {
            FrameResources::new(device.as_ref())
        });
        let frame_manager = FrameManager { frames, current: 0 };

        let render_planes = self.render_planes;
        let graph_manager = GraphManager {
            frame_graph: None,
            pin_board: PinBoard::new(),
            render_planes,
            swap_image_id: None,
        };

        let out = Renderer {
            config: RendererConfig {
                frames_in_flight: self.frames_in_flight,
            },
            device,
            queue,
            shader_db,
            swap_manager,
            texture_loader: Arc::new(TextureLoader::new()),
            buffer_loader: Arc::new(BufferLoader::new()),
            texture_pool: TexturePool::new(NonZeroU8::new(1).unwrap()),
            buffer_pool: BufferPool::new(NonZeroU8::new(2).unwrap()),
            frame_manager,
            graph_manager,
        };
        Some(out)
    }
}

pub struct Renderer {
    config: RendererConfig,
    device: AnyArc<dyn IDevice>,
    queue: AnyArc<dyn IQueue>,
    shader_db: ShaderDatabaseAccessor<'static>,
    swap_manager: SwapManager,
    texture_loader: Arc<TextureLoader>,
    buffer_loader: Arc<BufferLoader>,
    texture_pool: TexturePool,
    buffer_pool: BufferPool,
    frame_manager: FrameManager,
    graph_manager: GraphManager,
}

impl Renderer {
    pub fn device(&self) -> &dyn IDevice {
        self.device.as_ref()
    }

    pub fn get_texture_loader_handle(&self) -> Arc<TextureLoader> {
        self.texture_loader.clone()
    }

    pub fn get_texture_loader(&self) -> &TextureLoader {
        self.texture_loader.as_ref()
    }

    pub fn get_buffer_loader_handle(&self) -> Arc<BufferLoader> {
        self.buffer_loader.clone()
    }

    pub fn get_buffer_loader(&self) -> &BufferLoader {
        self.buffer_loader.as_ref()
    }

    pub fn create_texture(&mut self, data: TextureUploadSource) -> Option<TextureHandle> {
        let handle = self.texture_pool.create_texture(None);

        self.texture_loader
            .immediate_upload(None, handle, data)
            .ok()?;

        Some(handle)
    }

    pub fn create_buffer(&mut self, data: BufferUploadSource) -> Option<BufferHandle> {
        let handle = self.buffer_pool.create_buffer(None);

        self.buffer_loader
            .immediate_upload(None, handle, data)
            .ok()?;

        Some(handle)
    }

    pub unsafe fn draw_next_frame(&mut self, options: &DrawOptions, board: &mut BoardScope) {
        let CurrentFrameResources {
            frame_index,
            acquire_semaphore,
            present_semaphore,
            done_fence,
            deletion_pool,
        } = self.frame_manager.get_next_frame();

        // If we're producing frames faster than the GPU is producing them and we run out of frames
        // in flight then we need to wait for the oldest frame to complete before we can start
        // doing anything.
        //
        // This will stall until the oldest frame is complete, applying back pressure up the
        // pipeline.
        assert_eq!(
            self.device.wait_fences(&[done_fence], true, u32::MAX),
            FenceWaitResult::Complete
        );
        self.device.reset_fences(&[done_fence]);

        // We are now definitely recording the frame, we've proven it has been retired on the GPU
        // timeline and that means we can purge anything that was being held alive for that GPU
        // frame in the deletion pool.
        deletion_pool.purge();

        // Acquire the next image from the swap chain. This will also trigger rebuilding the swap
        // chain if it is out of date.
        //
        // If the swap chain is rebuilt we will acquire an image from the swap chain after we have
        // rebuilt it and hand that out, this papers over rebuilds.
        //
        // If we rebuild we do need to rebuild the swap chain though!
        let acquired_image = self.swap_manager.acquire_next_image(acquire_semaphore);

        // If the swap chain was rebuilt (or this is the first frame and we haven't built the frame
        // graph yet) we need to (re)build the frame graph!
        if self.graph_manager.frame_graph.is_none()
            || acquired_image.rebuilt
            || options.force_rebuild_frame_graph
        {
            if options.force_rebuild_frame_graph {
                self.device.wait_idle();
            }
            self.graph_manager.build_graph(
                &self.config,
                &self.shader_db,
                self.device.as_ref(),
                &self.swap_manager,
            );
        }

        let mut list = self
            .device
            .create_command_list(&CommandListDesc {
                queue_type: QueueType::General,
                name: None,
            })
            .unwrap();

        {
            let mut encoder = list.begin_general().unwrap();

            encoder.begin_event(Color::BLUE, nstr!("Upload Streaming Requests"));
            // TODO: we want to batch all of these, so we need a better interface so we can bundle
            //       the barriers and copy commands for all our loaders into a single batch.
            //
            //       either that or we unify to a single loader type.
            self.buffer_loader.upload_requests(
                &mut self.buffer_pool,
                deletion_pool,
                self.device.as_ref(),
                encoder.as_mut(),
                usize::MAX,
            );
            self.texture_loader.upload_requests(
                &mut self.texture_pool,
                deletion_pool,
                self.device.as_ref(),
                encoder.as_mut(),
                usize::MAX,
            );
            encoder.end_event();

            let mut import_bundle = ImportBundle::default();

            import_bundle.add_resource(
                self.graph_manager.swap_image_id.clone().unwrap(),
                acquired_image.image.as_ref(),
            );

            board.publish::<GraphSwapImageInfo>(GraphSwapImageInfo {
                desc: self.swap_manager.desc.clone(),
            });
            let args = GraphArgsLayout {
                board,
                texture_pool: &self.texture_pool,
                buffer_pool: &self.buffer_pool,
            };

            self.graph_manager
                .execute_graph(frame_index, &import_bundle, encoder.as_mut(), &args);
        }

        self.queue
            .submit(&QueueSubmitDesc {
                command_lists: &[Some(list).into()],
                wait_semaphores: &[acquire_semaphore],
                signal_semaphores: &[present_semaphore],
                fence: Some(done_fence),
            })
            .unwrap();

        self.swap_manager
            .present(self.queue.as_ref(), &[present_semaphore], acquired_image);
    }
}

/// A collection of tweakable options that can be provided ad-hoc on any call to
/// [`Renderer::draw_next_frame`].
#[derive(Clone, Debug)]
pub struct DrawOptions {
    /// Force the frame-graph to be rebuilt every frame, even if it wouldn't need to be otherwise.
    ///
    /// This is primarily intended to be used for profiling the cost of a frame-graph rebuild.
    pub force_rebuild_frame_graph: bool,
}

impl Default for DrawOptions {
    fn default() -> Self {
        Self {
            force_rebuild_frame_graph: false,
        }
    }
}

struct RendererConfig {
    /// Maximum number of frames in flight
    frames_in_flight: usize,
}

struct SwapManager {
    surface: Box<dyn IRenderSurface>,
    images: Vec<AnyArc<dyn ITexture>>,
    desc: TextureDesc<'static>,
    needs_rebuild: bool,
}

impl SwapManager {
    unsafe fn acquire_next_image(&mut self, signal_semaphore: &dyn ISemaphore) -> AcquiredImage {
        // Query if the surface wants to rebuild this frame and coerce the 'needs_rebuild' flag to
        // true if it wasn't already flagged.
        self.needs_rebuild = self.needs_rebuild || self.surface.needs_rebuild();

        let mut rebuilt = false;
        let mut attempts_remaining = 2;
        while attempts_remaining != 0 {
            attempts_remaining -= 1;

            if self.needs_rebuild {
                self.rebuild();
                rebuilt = true; // Flag if we had to rebuild before giving out the image.
            }

            let acquired_index = match self
                .surface
                .get_swap_chain()
                .acquire_next_image(&AcquireDesc { signal_semaphore })
            {
                Ok(i) => i,
                Err(ImageAcquireError::SubOptimal(i)) => {
                    // We should queue a rebuild for the next frame, but we can still render with
                    // this attachment so rather than stalling immediately we render now with the
                    // suboptimal attachment
                    self.needs_rebuild = true;
                    i
                }
                Err(ImageAcquireError::OutOfDate) => {
                    // If the swapchain is out of date then we have to rebuild immediately, so
                    // that's exactly what we do by forcing the rebuild flag on and looping around
                    // again.
                    //
                    // We can only try to acquire twice. If we fail a second time then we panic as
                    // we could end up stuck in a loop.
                    self.needs_rebuild = true;
                    continue;
                }
                v => v.unwrap(),
            };
            let acquired_image = self.images[acquired_index as usize].clone();

            return AcquiredImage {
                image: acquired_image,
                index: acquired_index as usize,
                rebuilt,
            };
        }
        panic!("Unable to acquire swap chain image!");
    }

    unsafe fn present(
        &mut self,
        queue: &dyn IQueue,
        wait_semaphores: &[&dyn ISemaphore],
        image: AcquiredImage,
    ) {
        let submit_result = queue.present(&QueuePresentDesc {
            swap_chain: self.surface.get_swap_chain(),
            image_index: image.index as u32,
            wait_semaphores,
        });
        match submit_result {
            Ok(_) => {}
            Err(QueuePresentError::OutOfDate) | Err(QueuePresentError::SubOptimal) => {
                self.needs_rebuild = true;
            }
            v @ Err(_) => v.unwrap(),
        }
    }

    fn rebuild(&mut self) {
        let swap_chain = self.surface.get_swap_chain();

        self.images.clear();

        let drawable_size = self.surface.get_render_extent();
        let new_config = swap_chain.rebuild(Some(drawable_size)).unwrap();

        let mut images: Vec<_> = (0..new_config.buffer_count).map(|_| None).collect();
        swap_chain.get_images(&mut images);

        self.images = images.into_iter().map(|v| v.unwrap()).collect();
        self.desc = self.images[0].desc().strip_name();
        self.needs_rebuild = false;
    }
}

struct AcquiredImage {
    /// Handle to the image we ended up acquiring
    image: AnyArc<dyn ITexture>,

    /// The image index of the image we acquired
    index: usize,

    /// Flags whether the swap chain was rebuilt in order to acquire this image
    rebuilt: bool,
}

struct FrameManager {
    frames: Vec<FrameResources>,
    current: usize,
}

impl FrameManager {
    fn get_next_frame(&mut self) -> CurrentFrameResources {
        self.current = (self.current + 1) % self.frames.len();
        let frame = &mut self.frames[self.current];
        CurrentFrameResources {
            frame_index: self.current,
            acquire_semaphore: frame.acquire_semaphore.as_ref(),
            present_semaphore: frame.present_semaphore.as_ref(),
            done_fence: frame.done_fence.as_ref(),
            deletion_pool: &mut frame.deletion_pool,
        }
    }
}

struct FrameResources {
    /// Used for syncing on the swap chain acquisition.
    acquire_semaphore: AnyArc<dyn ISemaphore>,

    /// Used for syncing the present operation on the completion of the frame's final submission.
    present_semaphore: AnyArc<dyn ISemaphore>,

    /// Used for notifying the CPU when the GPU frame is complete.
    done_fence: AnyArc<dyn IFence>,

    /// Pool for placing any resource that was deleted within the frame but must remain alive until
    /// that frame is finally retired on the GPU.
    deletion_pool: DeletionPool,
}

impl FrameResources {
    fn new(device: &dyn IDevice) -> Self {
        Self {
            acquire_semaphore: device.create_semaphore().unwrap(),
            present_semaphore: device.create_semaphore().unwrap(),
            done_fence: device.create_fence(true).unwrap(),
            deletion_pool: DeletionPool::new(),
        }
    }
}

struct CurrentFrameResources<'a> {
    frame_index: usize,
    acquire_semaphore: &'a dyn ISemaphore,
    present_semaphore: &'a dyn ISemaphore,
    done_fence: &'a dyn IFence,
    deletion_pool: &'a mut DeletionPool,
}

struct GraphManager {
    /// The frame graph object itself. Can be `None` if it has not yet been built for the first
    /// time.
    frame_graph: Option<FrameGraph<GraphArgs>>,

    /// Pin-board used (and re-used) for both building the frame graph, and when executing the frame
    /// graph.
    pin_board: PinBoard,

    /// Ordered list of render plane objects that make up the frame graph we'll be building and
    /// executing
    render_planes: Vec<Box<dyn IRenderPlane>>,

    /// The ID of the swap chain image resource inside the frame graph. This is needed so that you
    /// can import the swap chain image into the frame graph.
    swap_image_id: Option<ResourceMut>,
}

impl GraphManager {
    #[aleph_profile::function]
    unsafe fn build_graph(
        &mut self,
        config: &RendererConfig,
        shader_db: &ShaderDatabaseAccessor,
        device: &dyn IDevice,
        swap_manager: &SwapManager,
    ) {
        self.pin_board.clear();

        self.pin_board.publish(GraphSwapImageInfo {
            desc: swap_manager.desc.clone(),
        });

        let (builder, swap_id) = self.register_graph_passes(shader_db, device);

        let mut frame_graph = builder.build(device);
        frame_graph.allocate_transients(config.frames_in_flight);

        self.frame_graph = Some(frame_graph);
        self.swap_image_id = Some(swap_id);
    }

    #[aleph_profile::function]
    unsafe fn register_graph_passes(
        &mut self,
        shader_db: &ShaderDatabaseAccessor,
        device: &dyn IDevice,
    ) -> (FrameGraphBuilder<GraphArgs>, ResourceMut) {
        let mut frame_graph = FrameGraph::<GraphArgs>::builder();

        let mut outputs = Vec::with_capacity(self.render_planes.capacity());
        for plane in self.render_planes.iter_mut() {
            let output =
                plane.register_passes(&mut frame_graph, device, &self.pin_board, shader_db);
            outputs.push(output);
        }

        let swap_id = pass::composite_planes::pass(
            &mut frame_graph,
            device,
            &self.pin_board,
            shader_db,
            &outputs,
        );
        (frame_graph, swap_id)
    }

    unsafe fn execute_graph(
        &mut self,
        frame_index: usize,
        import_bundle: &ImportBundle,
        encoder: &mut dyn IGeneralEncoder,
        args: &GraphArgsLayout,
    ) {
        let fg = self.frame_graph.as_mut().unwrap();
        fg.execute(frame_index, import_bundle, encoder, args)
    }
}
