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

use crate::renderer::EguiRenderer;
use interfaces::any::{declare_interfaces, AnyArc, QueryInterface};
use interfaces::gpu::*;
use interfaces::make_plugin_description_for_crate;
use interfaces::platform::*;
use interfaces::plugin::*;
use interfaces::schedule::{CoreStage, IScheduleProvider};
use std::ops::Deref;

struct Data {
    index: usize,
    window: AnyArc<dyn IWindow>,
    render_data: AnyArc<dyn egui::IEguiRenderData>,
    swap_chain: AnyArc<dyn ISwapChain>,
    renderer: EguiRenderer,
}

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
        registrar.depends_on::<dyn IContextProvider>();
        registrar.must_init_after::<dyn IContextProvider>();

        registrar.depends_on::<dyn IWindowProvider>();
        registrar.must_init_after::<dyn IWindowProvider>();

        registrar.depends_on::<dyn IScheduleProvider>();
        registrar.must_init_after::<dyn IScheduleProvider>();

        registrar.depends_on::<dyn egui::IEguiRenderData>();
        registrar.must_init_after::<dyn egui::IEguiRenderData>();
    }

    fn on_init(&mut self, registry: &dyn IRegistryAccessor) -> Box<dyn IInitResponse> {
        // Get the handle for the window
        let window = registry
            .get_interface::<dyn IWindowProvider>()
            .unwrap()
            .get_window()
            .unwrap();

        // Get the render data slot for egui and the egui provider
        let render_data = registry
            .get_interface::<dyn egui::IEguiRenderData>()
            .unwrap();

        // Get our context provider for creating graphics API context and create our GPU context
        let options = ContextOptions {
            validation: true,
            debug: true,
        };
        let gpu_context = registry
            .get_interface::<dyn IContextProvider>()
            .unwrap()
            .make_context(&options)
            .unwrap();

        // Create a surface for the window we want to render with
        let surface = gpu_context.create_surface(&window.deref()).unwrap();

        // Get an adapter compatible with the requested surface
        let options = AdapterRequestOptions {
            surface: Some(surface.deref()),
            allow_software_adapters: true,
            ..Default::default()
        };
        let adapter = gpu_context
            .request_adapter(&options)
            .expect("Find suitable adapter");

        // Create our device
        let device = adapter.request_device().unwrap();
        self.device = Some(device.clone());

        let queue = device.get_queue(QueueType::General).unwrap();

        log::info!("");
        Self::log_gpu_info(
            device.deref().query_interface::<dyn IDevice>().unwrap(),
            adapter.deref(),
        );
        log::info!("");

        let drawable_size = window.drawable_size();
        let config = SwapChainConfiguration {
            format: Format::Bgra8UnormSrgb,
            width: drawable_size.0,
            height: drawable_size.1,
            present_mode: PresentationMode::Mailbox,
            preferred_queue: QueueType::General,
        };
        let swap_chain = surface.create_swap_chain(device.deref(), &config).unwrap();

        assert!(swap_chain.present_supported_on_queue(QueueType::General));

        let renderer = EguiRenderer::new(device.clone(), drawable_size);

        let schedule_cell = registry
            .get_interface::<dyn IScheduleProvider>()
            .unwrap()
            .get();
        let mut schedule = schedule_cell.get();

        let mut data = Data {
            index: 0,
            window,
            render_data,
            swap_chain,
            renderer,
        };
        schedule.add_exclusive_at_start_system_to_stage(
            &CoreStage::Render,
            "render::render",
            move || {
                device.garbage_collect();

                let data = &mut data;

                if data.window.resized() {
                    let dimensions = data.window.size();
                    data.swap_chain.queue_resize(dimensions.0, dimensions.1);
                    data.renderer.recreate_swap_resources(dimensions);
                }

                unsafe {
                    data.index = (data.index + 1) % 3;
                    let acquired_image = data.swap_chain.acquire_image().unwrap();

                    let command_list = data.renderer.record_frame(
                        data.index,
                        acquired_image.deref(),
                        data.render_data.take(),
                    );

                    queue.submit_list(command_list).unwrap();
                    queue.present(data.swap_chain.deref()).unwrap();
                }
            },
        );

        Box::new(Vec::new())
    }

    fn on_exit(&mut self, _registry: &dyn IRegistryAccessor) {
        if let Some(device) = self.device.take() {
            // When existing we need to flush all still active GPU work and force a GC cycle to
            // release all references being held live by the resource tracking system. The resource
            // tracking system creates cycles in the object graph so if we don't clear them then
            // we'll leak GPU objects.
            device.wait_idle();
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
