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

use std::ops::Deref;

use aleph_alloc::crossbeam;
use aleph_alloc::crossbeam::channel::Sender;
use aleph_frame_graph::FrameGraphBuilder;
use aleph_magnesium::renderer::Renderer;
use aleph_magnesium::renderer::builder::RendererBuilder;
use aleph_magnesium::renderer::render_plane::DefaultRenderPlane;
use aleph_magnesium::renderer::shader_accessor::ShaderAccessor;
use aleph_shader_db::ArchivedShaderDatabase;
use api::any::{AnyArc, QueryInterface, declare_interfaces};
use api::components::{self, Camera, Transform, TransformHistory};
use api::ecs::world::World;
use api::ecs::world::query::{Has, Read, Write};
use api::label::make_label;
use api::make_plugin_description_for_crate;
use api::object_system::unsafe_impl_iobject;
use api::platform::*;
use api::plugin::*;
use api::rhi::IRhiProvider;
use api::schedule::{CoreStage, WorldResource};
use api::scheduler::{Res, ResMut};
use mg::renderer::builder::ApplicationSurface;
use mg::renderer::draw_options::DrawOptions;
use mg::renderer::frame_graph::GraphArgs;
use mg::renderer::immediate_resource_builder::ImmediateResourceBuilder;
use mg::renderer::render_plane::{IRenderPlane, RenderPlaneOutput};
use mg::renderer::state_cache::StateCache;
use mg::renderer::surface_notify::SurfaceNotification;
use mg::scene::components::{DynamicObject, PerspectiveCamera, RenderTransform, StaticMesh};
use serde::Deserialize;

use crate::render::egui_draw;
use crate::render::egui_draw::EguiPassContext;
use crate::render::egui_font_texture::EguiFontTexture;

pub struct PluginRender {
    device: Option<AnyArc<dyn rhi::IDevice>>,
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
        registrar.requires::<dyn IWindow>(InitOrder::After);
        registrar.requires::<dyn IRhiProvider>(InitOrder::After);

        registrar.uses::<dyn egui::IEguiRenderData>(InitOrder::After);
    }

    fn on_init(&mut self, registry: &mut dyn IRegistryAccessor) {
        let config = registry.config("render").unwrap();
        let config: Config = serde_json::from_value(config.clone()).unwrap();
        config.log();

        // Get the handle for the window
        let window = registry.get_interface::<dyn IWindow>().unwrap();

        // Get the render data slot for egui and the egui provider
        let render_data = registry.get_interface::<dyn egui::IEguiRenderData>();

        let rhi_provider = registry.get_interface::<dyn IRhiProvider>().unwrap();
        let surface = rhi_provider.surface().unwrap();
        let adapter = rhi_provider.adapter();
        let device = rhi_provider.device();

        self.device = Some(device.clone());

        Self::log_gpu_info(
            device
                .deref()
                .query_interface::<dyn rhi::IDevice>()
                .unwrap(),
            adapter.deref(),
        );

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
        let surface_sender = SurfaceSender {
            window: window.clone(),
            sender: surface_send,
        };

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

        registry.core().resources.insert(RenderScene {
            scene: World::new(),
        });
        registry.core().resources.insert(renderer.build().unwrap());

        let mut egui_data = render_data.map(|v| EguiData {
            font_texture: EguiFontTexture::new(),
            render_data: v,
        });
        registry.core().schedule.add_system_to_stage(
            CoreStage::Render.into(),
            make_label!("render::render"),
            move |world: Res<WorldResource>,
                  mut renderer: ResMut<Renderer>,
                  mut render_scene: ResMut<RenderScene>| {
                device.garbage_collect().unwrap();

                let render_scene = &mut render_scene.scene;

                render_scene.remove_matching::<Has<DynamicObject>>();

                let query = world
                    .0
                    .query::<(Read<Transform>, Read<components::StaticMesh>)>();
                for (_id, (t, m)) in query {
                    render_scene.insert((
                        RenderTransform {
                            position: t.position,
                            rotation: t.rotation,
                            scale: t.scale,
                        },
                        StaticMesh {
                            vtx: Some(m.vtx),
                            idx: Some(m.idx),
                            material_instance: Some(m.material_instance),
                        },
                        DynamicObject,
                    ));
                }

                // Find the first camera object in the scene and make that the active camera.
                let mut query = world.0.query::<(Read<Transform>, Read<Camera>)>();
                let (_, (t, c)) = query.next().unwrap();

                let camera_entity = render_scene.insert((
                    RenderTransform {
                        position: t.position,
                        rotation: t.rotation,
                        scale: t.scale,
                    },
                    PerspectiveCamera {
                        vertical_fov: c.vertical_fov,
                        z_near: c.z_near,
                    },
                    DynamicObject,
                ));

                if let Some(e) = egui_data.as_mut() {
                    let render_data = e.render_data.take();

                    // Filter the deltas to only those that affect the font texture and upload
                    // a new font texture immediately.
                    let font_updates = render_data
                        .textures_delta
                        .set
                        .iter()
                        .filter(|(id, _)| *id == egui::TextureId::Managed(0))
                        .map(|(_, delta)| delta);
                    e.font_texture
                        .update_font_texture(&mut renderer, font_updates);

                    // Pass the egui commands and font texture that so our egui render pass
                    // can do its thing.
                    let _ = render_scene.insert_singleton(EguiPassContext {
                        font_handle: e.font_texture.font_handle.unwrap(),
                        render_data,
                    });
                }

                let options = DrawOptions {
                    force_rebuild_frame_graph: config.force_graph_rebuild,
                };
                renderer.draw_frame(&options, render_scene, camera_entity);
            },
        );

        // System to take the send events about the rendering surface into the renderer over the
        // channel that we gave it.
        registry
            .core()
            .schedule
            .add_exclusive_at_end_system_to_stage(
                CoreStage::InputCollection.into(),
                make_label!("render::send_surface_events"),
                move || {
                    surface_sender.pump();
                },
            );

        // System to update the transform history of entities so we get correct motion vectors. We
        // do this immediately after the main render job gets kicked off as we want to capture the
        // transforms immediately after the last render frame.
        registry
            .core()
            .schedule
            .add_exclusive_at_end_system_to_stage(
                CoreStage::Render.into(),
                make_label!("render::capture_previous_transform"),
                move |mut world: ResMut<WorldResource>| {
                    for (_id, (t, h)) in world
                        .0
                        .query_mut::<(Read<Transform>, Write<TransformHistory>)>()
                    {
                        h.previous = t.clone();
                    }
                },
            );
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

declare_interfaces!(PluginRender, [IPlugin]);

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

struct SurfaceSender {
    window: AnyArc<dyn IWindow>,
    sender: Sender<SurfaceNotification>,
}

impl SurfaceSender {
    fn pump(&self) {
        if self.window.resized() {
            let size = self.window.drawable_size();
            let size = rhi::Extent2D::new(size.0, size.1);
            self.sender
                .try_send(SurfaceNotification::Resized(size))
                .ok()
                .unwrap()
        }
    }
}

struct EguiRenderPlane {
    window: AnyArc<dyn IWindow>,
}

impl EguiRenderPlane {
    fn new(window: AnyArc<dyn IWindow>) -> Self {
        Self { window }
    }
}

impl IRenderPlane for EguiRenderPlane {
    fn init_resources(&mut self, _resource_builder: &mut ImmediateResourceBuilder) {
        // nothing
    }

    fn register_passes(
        &self,
        frame_graph: &mut FrameGraphBuilder<GraphArgs>,
        device: &dyn rhi::IDevice,
        pin_board: &aleph_pin_board::PinBoard,
        state_cache: &mut StateCache,
    ) -> RenderPlaneOutput {
        let pixels_per_point = self.window.current_display_scale();
        egui_draw::pass(
            frame_graph,
            device,
            pin_board,
            state_cache,
            pixels_per_point,
        )
    }
}

#[derive(Deserialize)]
struct Config {
    #[serde(rename = "renderAheadFrames")]
    pub render_ahead_frames: u32,

    #[serde(rename = "forceGraphRebuild")]
    pub force_graph_rebuild: bool,
}

impl Config {
    pub fn log(&self) {
        log::info!("render.renderAheadFrames = {}", self.render_ahead_frames);
        log::info!("render.forceGraphRebuild = {}", self.force_graph_rebuild);
    }
}

struct EguiData {
    font_texture: EguiFontTexture,
    render_data: AnyArc<dyn egui::IEguiRenderData>,
}

struct RenderScene {
    pub scene: World,
}
unsafe_impl_iobject!(RenderScene, "01924ac2-6c15-7362-964e-6bd6d632e4d2");
