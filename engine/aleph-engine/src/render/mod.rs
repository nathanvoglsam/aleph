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

mod async_loader;
mod config;
mod core;
mod egui;
mod shaders;

use std::sync::Arc;

use ::egui::AEguiRenderData;
use aleph_magnesium::renderer::builder::RendererBuilder;
use aleph_magnesium::renderer::render_plane::DefaultRenderPlane;
use aleph_magnesium::renderer::shader_accessor::ShaderAccessor;
use aleph_shader_db::ArchivedShaderDatabase;
use api::ecs::world::World;
use api::make_plugin_description_for_crate;
use api::platform::*;
use api::plugin::*;
use api::rhi::ARhiProvider;
use mg::renderer::builder::ApplicationSurface;

use crate::render::config::Config;
use crate::render::core::resources::render_scene::RenderSceneResource;
use crate::render::core::systems::capture_previous_transforms::CapturePreviousTransformsSystem;
use crate::render::core::systems::publish_egui_scene::PublishEguiSceneSystem;
use crate::render::core::systems::publish_render_scene::PublishRenderSceneSystem;
use crate::render::core::systems::render::RenderSystem;
use crate::render::core::systems::surface_sender::SurfaceSenderSystem;
use crate::render::egui::render_plane::EguiRenderPlane;

pub struct PluginRender {
    device: Option<Arc<dyn rhi::IDevice>>,
}

impl PluginRender {
    pub fn new() -> Self {
        Self { device: None }
    }
}

impl IPlugin for PluginRender {
    fn get_description(&self) -> PluginDescription {
        make_plugin_description_for_crate!()
    }

    fn register(&mut self, registrar: &mut dyn IPluginRegistrar) {
        registrar.requires::<AWindow>(InitOrder::After);
        registrar.requires::<ARhiProvider>(InitOrder::After);

        registrar.uses::<AEguiRenderData>(InitOrder::After);
    }

    fn on_init(&mut self, registry: &mut dyn IRegistryAccessor) {
        let config = registry.config("render").unwrap();
        let config: Config = serde_json::from_value(config.clone()).unwrap();
        config.log();

        // Get the handle for the window
        let window = registry.get_interface::<AWindow>().unwrap().get();

        // Get the render data slot for egui and the egui provider
        let render_data = registry.get_interface::<AEguiRenderData>().map(|v| v.get());

        let rhi_provider = registry.get_interface::<ARhiProvider>().unwrap();
        let surface = rhi_provider.surface().unwrap();
        let adapter = rhi_provider.adapter();
        let device = rhi_provider.device();

        self.device = Some(device.clone());

        Self::log_gpu_info(device.as_ref(), adapter.as_ref());

        let drawable_size = window.drawable_size();
        let drawable_size = rhi::Extent2D::new(drawable_size.0, drawable_size.1);

        // Try load the shader db, first from the immediate working directory and then from the
        // potential aleph project's .aleph/shaders directory.
        // TODO: we need a better way of managing and configuring where we get our shader db from
        let shader_db_bin = std::fs::read("shaders.shaderdb")
            .or_else(|_| std::fs::read(".aleph/shaders/shaders.shaderdb"))
            .unwrap()
            .leak(); // Leak so we get a static lifetime
        let shader_db = unsafe { rkyv::access_unchecked::<ArchivedShaderDatabase>(shader_db_bin) };
        shader_db.validate_header();
        let shader_db = ShaderAccessor::new(shader_db);

        let (surface_send, surface_recv) = crossbeam::channel::unbounded();

        // Construct the renderer object and add the resource into the registry
        let mut renderer = RendererBuilder::new();
        renderer.device(device.clone());
        renderer.surface(ApplicationSurface {
            surface,
            extent: drawable_size,
            notify: Box::new(surface_recv),
        });
        renderer.shader_db(Box::new(shader_db));
        renderer.render_plane(DefaultRenderPlane::default());
        if render_data.is_some() {
            renderer.render_plane(EguiRenderPlane::new(window.clone()));
        }
        renderer.render_ahead_frames(config.render_ahead_frames as usize);
        registry.core().resources.insert(renderer.build().unwrap());

        // Construct and register the __render__ scene resource. This is distinct from the
        // simulation scene.
        registry.core().resources.insert(RenderSceneResource {
            scene: World::new(),
        });

        // System to take the send events about the rendering surface into the renderer over the
        // channel that we gave it.
        {
            let system = SurfaceSenderSystem::new(window.clone(), surface_send);
            system.register(&mut registry.core().schedule);
        }

        // System to update the transform history of entities so we get correct motion vectors. We
        // do this immediately after the main render job gets kicked off as we want to capture the
        // transforms immediately after the last render frame.
        {
            let system = CapturePreviousTransformsSystem;
            system.register(&mut registry.core().schedule);
        }

        // System to copy the simulation scene into the render scene
        {
            let system = PublishRenderSceneSystem;
            system.register(&mut registry.core().schedule);
        }

        // System to publish the egui scene into the render scene
        if let Some(render_data) = render_data {
            let system = PublishEguiSceneSystem::new(render_data);
            system.register(&mut registry.core().schedule);
        }

        // System that kicks off the main render job
        {
            let system = RenderSystem::new(device, &config);
            system.register(&mut registry.core().schedule);
        }
    }

    fn on_exit(&mut self) {
        if let Some(device) = self.device.as_deref() {
            // When existing we need to flush all still active GPU work and force a GC cycle to
            // release all references being held live by the resource tracking system. The resource
            // tracking system creates cycles in the object graph so if we don't clear them then
            // we'll leak GPU objects.
            device.wait_idle().unwrap();
            device.garbage_collect().unwrap();
        }
    }

    fn on_shutdown(&mut self) {
        if let Some(device) = self.device.take() {
            log::debug!(
                "IDevice::strong_count = '{}' at 'on_shutdown'",
                device.strong_count()
            );
            log::debug!(
                "IDevice::weak_count = '{}' at 'on_shutdown'",
                device.weak_count()
            );
        }
    }
}

impl Default for PluginRender {
    fn default() -> Self {
        Self::new()
    }
}

impl PluginRender {
    ///
    /// Internal function for logging info about the CPU that is being used
    ///
    #[allow(clippy::erasing_op)]
    fn log_gpu_info(device: &dyn rhi::IDevice, adapter: &dyn rhi::IAdapter) {
        let info = adapter.description();

        let gpu_vendor = info.vendor;
        let gpu_name = info.name;
        let dvmem = 0 /* info.DedicatedVideoMemory */ / 1_000_000;
        let dsmem = 0 /* info.DedicatedSystemMemory */ / 1_000_000;
        let ssmem = 0 /* info.SharedSystemMemory */ / 1_000_000;

        log::info!("=== GPU INFO ===");
        log::info!("GPU Vendor    : {}", gpu_vendor);
        log::info!("GPU Name      : {}", gpu_name);
        log::info!("Memory        : {}MB | {}MB | {}MB", dvmem, dsmem, ssmem);
        log::info!("Backend:      : {}", device.get_backend_api());
    }
}
