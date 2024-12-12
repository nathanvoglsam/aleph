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

use aleph_frame_graph::FrameGraphBuilder;
use aleph_math::ToSingle;
use aleph_pin_board::ScopedParamBoard;
use aleph_rhi_api::*;
use aleph_shader_db::ArchivedShaderDatabase;
use interfaces::any::{declare_interfaces, AnyArc, QueryInterface};
use interfaces::components::{self, Camera, Transform, TransformHistory};
use interfaces::label::make_label;
use interfaces::make_plugin_description_for_crate;
use interfaces::platform::*;
use interfaces::plugin::*;
use interfaces::rhi::IRhiProvider;
use interfaces::schedule::{CoreStage, WorldResource};
use interfaces::scheduler::{Res, ResMut};
use serde::Deserialize;

use aleph_renderer::pass::GraphArgs;
use aleph_renderer::{
    CameraInfo, DefaultRenderPlane, DefaultResources, DrawOptions, IRenderPlane, IRenderSurface,
    PerspectiveInfo, RenderPlaneOutput, RenderScene, RenderSceneParam, RenderTransform, Renderer,
    RendererBuilder, ShaderDatabaseAccessor, StateCache,
};

use crate::egui_draw::EguiPassContext;
use crate::egui_font_texture::EguiFontTexture;

pub struct PluginRender {
    device: Option<AnyArc<dyn IDevice>>,
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
            device.deref().query_interface::<dyn IDevice>().unwrap(),
            adapter.deref(),
        );

        let drawable_size = window.drawable_size();
        let swap_config = SwapChainConfiguration {
            format: Format::Bgra8UnormSrgb,
            width: drawable_size.0,
            height: drawable_size.1,
            present_mode: PresentationMode::Fifo,
            buffer_count: 3,
            present_queue: QueueType::General,
        };
        let swap_chain = surface
            .create_swap_chain(device.deref(), &swap_config)
            .unwrap();
        assert!(swap_chain.present_supported_on_queue(QueueType::General));

        let surface = RenderSurface {
            window: window.clone(),
            swap_chain,
        };

        // Try load the shader db, first from the immediate working directory and then from the
        // potential aleph project's .aleph/shaders directory.
        // TODO: we need a better way of managing and configuring where we get our shader db from
        let shader_db_bin = std::fs::read("shaders.shaderdb")
            .or_else(|_| std::fs::read(".aleph/shaders/shaders.shaderdb"))
            .unwrap()
            .leak(); // Leak so we get a static lifetime
        let shader_db = unsafe { rkyv::access_unchecked::<ArchivedShaderDatabase>(shader_db_bin) };
        shader_db.validate_header();
        let shader_db = ShaderDatabaseAccessor::new(device.as_ref(), shader_db);

        let mut renderer = RendererBuilder::new();
        renderer.device(device.clone());
        renderer.surface(surface);
        renderer.shader_db(shader_db);
        renderer.render_plane(DefaultRenderPlane::default());
        if render_data.is_some() {
            renderer.render_plane(EguiRenderPlane::new(window.clone()));
        }
        renderer.frames_in_flight(config.frames_in_flight as usize);

        registry.core().resources.insert(RenderScene::new());
        registry.core().resources.insert(renderer.build().unwrap());

        let mut egui_data = render_data.map(|v| EguiData {
            font_texture: EguiFontTexture::new(),
            render_data: v,
        });
        let mut board = ScopedParamBoard::new();
        registry.core().schedule.add_system_to_stage(
            CoreStage::Render.into(),
            make_label!("render::render"),
            move |world: Res<WorldResource>,
                  mut renderer: ResMut<Renderer>,
                  mut render_scene: ResMut<RenderScene>| {
                device.garbage_collect();

                render_scene.clear();

                let query = world.0.query::<(&Transform, &components::StaticMesh)>();
                for (_id, (t, m)) in query {
                    render_scene.push(
                        RenderTransform {
                            position: t.position,
                            rotation: t.rotation,
                            scale: t.scale,
                        },
                        aleph_renderer::StaticMesh {
                            vtx: m.vtx,
                            idx: m.idx,
                            colour_tex: m.colour_tex,
                            colour: m.colour,
                            metalness: m.metalness,
                            roughness: m.roughness,
                            metal_roughness_tex: m.metal_roughness_tex,
                            normal_map_tex: m.normal_map_tex,
                        },
                    );
                }

                board.scope(|board| {
                    board.publish::<RenderSceneParam>(&render_scene);

                    let mut query = world.0.query::<(&Transform, &Camera)>();
                    let (_, (t, c)) = query.next().unwrap();

                    let size = window.drawable_size();
                    let aspect = size.0 as f32 / size.1 as f32;
                    let camera_info = CameraInfo {
                        position: t.position.to_single(),
                        orientation: t.rotation,
                        projection: PerspectiveInfo::new(c.vertical_fov, c.z_near, aspect),
                    };
                    board.publish::<CameraInfo>(camera_info);

                    if let Some(e) = egui_data.as_mut() {
                        let render_data = e.render_data.take();

                        // Filter the deltas to only those that affect the font texture
                        let font_updates = render_data
                            .textures_delta
                            .set
                            .iter()
                            .filter(|(id, _)| *id == egui::TextureId::Managed(0))
                            .map(|(_, delta)| delta);
                        e.font_texture
                            .update_font_texture(&mut renderer, font_updates);

                        board.publish::<EguiPassContext>(EguiPassContext {
                            font_handle: e.font_texture.font_handle.unwrap(),
                            render_data,
                        });
                    }

                    unsafe {
                        let options = DrawOptions {
                            force_rebuild_frame_graph: config.force_graph_rebuild,
                        };
                        renderer.draw_next_frame(&options, board);
                    }
                });
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
                    for (_id, (t, h)) in world.0.query_mut::<(&Transform, &mut TransformHistory)>()
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
            device.wait_idle();
            device.garbage_collect();
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
    fn log_gpu_info(device: &dyn IDevice, adapter: &dyn IAdapter) {
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

struct RenderSurface {
    window: AnyArc<dyn IWindow>,
    swap_chain: AnyArc<dyn ISwapChain>,
}

impl IRenderSurface for RenderSurface {
    fn get_render_extent(&self) -> Extent2D {
        let size = self.window.drawable_size();
        Extent2D::new(size.0, size.1)
    }

    fn get_swap_chain(&self) -> &dyn ISwapChain {
        self.swap_chain.as_ref()
    }

    fn needs_rebuild(&self) -> bool {
        self.window.resized()
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
    fn register_passes(
        &self,
        frame_graph: &mut FrameGraphBuilder<GraphArgs>,
        device: &dyn IDevice,
        pin_board: &aleph_pin_board::PinBoard,
        state_cache: &mut StateCache,
        _default_resources: &DefaultResources,
    ) -> RenderPlaneOutput {
        let pixels_per_point = self.window.current_display_scale();
        crate::egui_draw::pass(
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
    #[serde(rename = "framesInFlight")]
    pub frames_in_flight: u32,

    #[serde(rename = "forceGraphRebuild")]
    pub force_graph_rebuild: bool,
}

impl Config {
    pub fn log(&self) {
        log::info!("render.framesInFlight = {}", self.frames_in_flight);
        log::info!("render.forceGraphRebuild = {}", self.force_graph_rebuild);
    }
}

struct EguiData {
    font_texture: EguiFontTexture,
    render_data: AnyArc<dyn egui::IEguiRenderData>,
}
