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

use crate::dx12::dxgi;
use crate::renderer::EguiRenderer;
use aleph_gpu_dx12::{IAdapterExt, IDeviceExt, ISwapTextureExt};
use interfaces::any;
use interfaces::gpu::{
    AdapterRequestOptions, ContextOptions, IContextProvider, IDevice, ISwapChain, PresentationMode,
    QueueType, SwapChainConfiguration, TextureFormat,
};
use interfaces::platform::*;
use interfaces::plugin::*;
use interfaces::ref_ptr::RefPtr;
use interfaces::schedule::{CoreStage, IScheduleProvider};
use std::ops::Deref;

struct Data {
    index: usize,
    window: any::AnyArc<dyn IWindow>,
    render_data: any::AnyArc<dyn egui::IEguiRenderData>,
    egui_provider: any::AnyArc<dyn egui::IEguiContextProvider>,
    swap_chain: RefPtr<dyn ISwapChain>,
    renderer: EguiRenderer,
}

pub struct PluginRender {
    device: Option<RefPtr<dyn IDeviceExt>>,
}

impl PluginRender {
    pub fn new() -> Self {
        Self { device: None }
    }
}

impl IPlugin for PluginRender {
    fn get_description(&self) -> PluginDescription {
        PluginDescription {
            name: "PluginRenderDX12".to_string(),
            description: "A render plugin implemented with dx12".to_string(),
            major_version: env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap(),
            minor_version: env!("CARGO_PKG_VERSION_MINOR").parse().unwrap(),
            patch_version: env!("CARGO_PKG_VERSION_PATCH").parse().unwrap(),
        }
    }

    fn register(&mut self, registrar: &mut dyn IPluginRegistrar) {
        registrar.depends_on::<dyn IContextProvider>();
        registrar.must_init_after::<dyn IContextProvider>();

        registrar.depends_on::<dyn IWindowProvider>();
        registrar.must_init_after::<dyn IWindowProvider>();

        registrar.depends_on::<dyn IScheduleProvider>();
        registrar.must_init_after::<dyn IScheduleProvider>();

        registrar.depends_on::<dyn egui::IEguiRenderData>();
        registrar.depends_on::<dyn egui::IEguiContextProvider>();
        registrar.must_init_after::<dyn egui::IEguiRenderData>();
        registrar.must_init_after::<dyn egui::IEguiContextProvider>();
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
        let egui_provider = registry
            .get_interface::<dyn egui::IEguiContextProvider>()
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
            surface: Some(surface.as_weak()),
            ..Default::default()
        };
        let adapter = gpu_context
            .request_adapter(&options)
            .unwrap()
            .query_interface::<dyn IAdapterExt>()
            .unwrap();

        // Create our device
        let device = adapter
            .request_device()
            .unwrap()
            .query_interface::<dyn IDeviceExt>()
            .unwrap();
        self.device = Some(device.clone());

        aleph_log::info!("");
        Self::log_gpu_info(&adapter.get_raw_handle());
        aleph_log::info!("");

        let drawable_size = window.drawable_size();
        let config = SwapChainConfiguration {
            format: TextureFormat::Bgra8UnormSrgb,
            width: drawable_size.0,
            height: drawable_size.1,
            present_mode: PresentationMode::Mailbox,
            preferred_queue: QueueType::General,
        };
        let device_handle = device.query_interface::<dyn IDevice>().unwrap();
        let swap_chain = surface
            .create_swap_chain(device_handle.as_weak(), &config)
            .unwrap();

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
            egui_provider,
            swap_chain,
            renderer,
        };
        schedule.add_exclusive_at_start_system_to_stage(
            &CoreStage::Render,
            "render::render",
            move || {
                device.garbage_collect();

                let data = &mut data;
                let egui_ctx = data.egui_provider.get_context();

                if data.window.resized() {
                    let dimensions = data.window.size();
                    data.swap_chain.queue_resize(dimensions.0, dimensions.1);
                    data.renderer.recreate_swap_resources(dimensions);
                }

                unsafe {
                    data.index = (data.index + 1) % 3;
                    let acquired_image = data.swap_chain.acquire_image().unwrap();
                    let image = acquired_image.image();
                    let image_ext = image.query_interface::<dyn ISwapTextureExt>().unwrap();

                    let command_list = data.renderer.record_frame(
                        data.index,
                        image_ext.get_raw_handle(),
                        acquired_image.image(),
                        image_ext.get_raw_rtv(),
                        &egui_ctx,
                        data.render_data.take(),
                    );

                    device.general_queue_submit_list(command_list).unwrap();
                    device.general_queue_present(acquired_image).unwrap();
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

any::declare_interfaces!(PluginRender, [IPlugin]);

impl PluginRender {
    ///
    /// Internal function for logging info about the CPU that is being used
    ///
    fn log_gpu_info(adapter: &dxgi::Adapter) {
        let info = adapter.get_adapter_desc().unwrap();

        let gpu_vendor = info.vendor_id_string();
        let gpu_name = info
            .description_string()
            .unwrap_or_else(|| "Unknown".to_string());
        let dvmem = info.dedicated_video_memory / 1_000_000;
        let dsmem = info.dedicated_system_memory / 1_000_000;
        let ssmem = info.shared_system_memory / 1_000_000;

        aleph_log::info!("=== GPU INFO ===");
        aleph_log::info!("GPU Vendor    : {}", gpu_vendor);
        aleph_log::info!("GPU Name      : {}", gpu_name);
        aleph_log::info!("Memory        : {}MB | {}MB | {}MB", dvmem, dsmem, ssmem)
    }
}
