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

use std::sync::Arc;

use aleph_alloc::BVec;
use aleph_alloc::instrumentation::system;
use aleph_pin_board::PinBoard;
use parking_lot::Mutex;

use crate::internal::buffer::BufferObjectStore;
use crate::internal::material_instance::MaterialInstanceStore;
use crate::internal::renderer::deferred_deletion_manager::DeferredDeletionManager;
use crate::internal::renderer::frame_manager::FrameManager;
use crate::internal::renderer::graph_manager::GraphManager;
use crate::internal::renderer::immediate_upload_queue::ImmediateUploadQueue;
use crate::internal::renderer::object_deletion_queue::ObjectDeleteQueue;
use crate::internal::renderer::surface::SharedSurface;
use crate::internal::renderer::swap_manager::SwapManager;
use crate::internal::texture::TextureObjectStore;
use crate::renderer::Renderer;
use crate::renderer::config::RendererConfig;
use crate::renderer::immediate_resource_builder::ImmediateResourceBuilder;
use crate::renderer::render_plane::IRenderPlane;
use crate::renderer::shader_accessor::IShaderAccessor;
use crate::renderer::state_cache::StateCache;
use crate::renderer::surface_notify::ISurfaceNotify;

pub struct RendererBuilder {
    device: Option<Arc<dyn rhi::IDevice>>,
    surface: Option<ApplicationSurface>,
    shader_db: Option<Box<dyn IShaderAccessor + Send + Sync + 'static>>,
    render_planes: Vec<Box<dyn IRenderPlane>>,
    render_ahead_frames: usize,
}

impl Default for RendererBuilder {
    fn default() -> Self {
        Self {
            device: None,
            surface: None,
            render_ahead_frames: 1,
            shader_db: None,
            render_planes: Vec::new(),
        }
    }
}

impl RendererBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn device(&mut self, device: Arc<dyn rhi::IDevice>) -> &mut Self {
        self.device = Some(device);
        self
    }

    pub fn surface(&mut self, surface: ApplicationSurface) -> &mut Self {
        self.surface = Some(surface);
        self
    }

    pub fn shader_db(
        &mut self,
        shader_db: Box<dyn IShaderAccessor + Send + Sync + 'static>,
    ) -> &mut Self {
        self.shader_db = Some(shader_db);
        self
    }

    pub fn render_plane(&mut self, plane: impl IRenderPlane) -> &mut Self {
        self.render_planes.push(Box::new(plane));
        self
    }

    pub fn render_ahead_frames(&mut self, render_ahead_frames: usize) -> &mut Self {
        self.render_ahead_frames = render_ahead_frames;
        self
    }

    pub fn build<'a>(self) -> Option<Renderer> {
        let device = self.device.expect("Device missing!");
        let queue = device.get_queue(rhi::QueueType::General).unwrap();

        let frame_fence = device.create_fence(0).ok()?;

        let graph_manager = GraphManager {
            frame_graph: None,
            pin_board: PinBoard::new(),
            render_planes: self.render_planes,
            swap_image_id: None,
        };

        let frame_manager = FrameManager::new(device.clone(), self.render_ahead_frames);

        let async_loader_dispatchers = BVec::new_in(system());

        let surface = self.surface.unwrap();
        let swap_config = rhi::SwapChainConfiguration {
            format: rhi::Format::Bgra8UnormSrgb,
            width: surface.extent.width,
            height: surface.extent.height,
            present_mode: rhi::PresentationMode::Fifo,
            buffer_count: 3,
            present_queue: rhi::QueueType::General,
        };
        let swap_chain = surface
            .surface
            .create_swap_chain(device.as_ref(), &swap_config)
            .unwrap();
        assert!(swap_chain.present_supported_on_queue(rhi::QueueType::General));
        let swap_manager = SwapManager {
            surface: SharedSurface {
                _surface: surface.surface,
                notify: surface.notify,
                swap_chain,
            },
            desc: rhi::TextureDesc::default(),
            needs_rebuild: true,
            extent: Default::default(),
        };

        let deferred_deletion_manager = DeferredDeletionManager::new(device.clone());

        let texture_object_store = TextureObjectStore::new();
        let buffer_object_store = BufferObjectStore::new();
        let material_instance_store = MaterialInstanceStore::new();

        let object_delete_queue = ObjectDeleteQueue::new();

        let immediate_upload_queue = ImmediateUploadQueue::new(device.clone());

        let shader_db = self.shader_db.expect("Shader DB missing!");
        let state_cache = Mutex::new(StateCache::new(shader_db));

        let mut out = Renderer {
            config: RendererConfig {
                render_ahead_frames: self.render_ahead_frames,
            },
            device,
            queue,
            frame_fence,
            async_loader_dispatchers,
            graph_manager,
            frame_manager,
            swap_manager,
            deferred_deletion_manager,
            texture_object_store,
            buffer_object_store,
            material_instance_store,
            object_delete_queue,
            immediate_upload_queue,
            state_cache,
        };

        out.graph_manager
            .init_graph_resources(&mut ImmediateResourceBuilder {
                device: out.device.as_ref(),
                texture_object_store: &mut out.texture_object_store,
                buffer_object_store: &mut out.buffer_object_store,
                immediate_upload_queue: &mut out.immediate_upload_queue,
            });

        Some(out)
    }
}

pub struct ApplicationSurface {
    pub surface: Arc<dyn rhi::ISurface>,
    pub extent: rhi::Extent2D,
    pub notify: Box<dyn ISurfaceNotify + Send + Sync>,
}
