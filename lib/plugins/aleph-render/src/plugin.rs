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

use crate::dx12::{dxgi, AsWeakRef, D3D12Object};
use crate::renderer::EguiRenderer;
use crate::{dx12, dx12_alloc};
use aleph_gpu_dx12::{IAdapterExt, IDeviceExt, ISwapChainExt};
use interfaces::any;
use interfaces::gpu::{
    AdapterRequestOptions, ContextOptions, IContextProvider, PresentationMode,
    SwapChainConfiguration, TextureFormat,
};
use interfaces::platform::*;
use interfaces::plugin::*;
use interfaces::schedule::{CoreStage, IScheduleProvider};
use std::ops::Deref;

struct Data {
    window: any::AnyArc<dyn IWindow>,
    render_data: any::AnyArc<dyn egui::IEguiRenderData>,
    egui_provider: any::AnyArc<dyn egui::IEguiContextProvider>,
    device: dx12::Device,
    queue: dx12::CommandQueue,
    event: dx12::Event,
    fence: dx12::Fence,
    swap_chain: dxgi::SwapChain,
    buffers: Vec<dx12::Resource>,
    command_lists: Vec<dx12::GraphicsCommandList>,
    renderer: EguiRenderer,
}

pub struct PluginRender();

impl PluginRender {
    pub fn new() -> Self {
        Self()
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
        // Get the raw window handle from the window
        let window_provider = registry.get_interface::<dyn IWindowProvider>().unwrap();
        let window = window_provider.get_window().unwrap();
        let window_ref = window.deref();

        // Get our context provider for creating graphics API context
        let gpu_context_provider = registry.get_interface::<dyn IContextProvider>().unwrap();

        // Create our GPU context
        let options = ContextOptions {
            validation: true,
            debug: true,
        };
        let gpu_context = gpu_context_provider.make_context(&options).unwrap();

        // Create a surface for the window we want to render with
        let gpu_surface = gpu_context.create_surface(&window_ref).unwrap();

        // Get an adapter compatible with the requested surface
        let options = AdapterRequestOptions {
            surface: Some(gpu_surface.as_weak()),
            ..Default::default()
        };
        let gpu_adapter = gpu_context.request_adapter(&options).unwrap();

        // Create our device
        let gpu_device = gpu_adapter.request_device().unwrap();
        let gpu_device_ext = gpu_device.query_interface::<dyn IDeviceExt>().unwrap();

        let gpu_adapter_ext = gpu_adapter.query_interface::<dyn IAdapterExt>().unwrap();

        let adapter = gpu_adapter_ext.get_raw_handle().clone();
        let device = gpu_device_ext.get_raw_handle().clone();
        let queue = gpu_device_ext.get_raw_general_queue().unwrap().to_strong();

        // Get the render data slot for egui
        let render_data = registry
            .get_interface::<dyn egui::IEguiRenderData>()
            .unwrap();

        // Get the egui provider
        let egui_provider = registry
            .get_interface::<dyn egui::IEguiContextProvider>()
            .unwrap();

        aleph_log::info!("");
        Self::log_gpu_info(&adapter);
        aleph_log::info!("");

        let event = dx12::Event::new().unwrap();
        let fence = device.create_fence(0, dx12::FenceFlags::NONE).unwrap();

        let allocator_desc = dx12_alloc::AllocatorDesc::builder()
            .device(device.clone())
            .adapter(adapter)
            .build();
        let allocator = dx12_alloc::Allocator::new(&allocator_desc).unwrap();

        let drawable_size = window.drawable_size();
        let config = SwapChainConfiguration {
            usage: (),
            format: TextureFormat::Rgba8Unorm,
            width: drawable_size.0,
            height: drawable_size.1,
            present_mode: PresentationMode::Mailbox,
        };
        let gpu_swap_chain = gpu_surface
            .create_swap_chain(gpu_device.as_weak(), &config)
            .unwrap();
        let gpu_swap_chain_ext = gpu_swap_chain
            .query_interface::<dyn ISwapChainExt>()
            .unwrap();

        let swap_chain = gpu_swap_chain_ext.get_raw_handle().clone();

        let buffers = swap_chain.get_buffers(3).unwrap();
        buffers.iter().for_each(|v| {
            v.set_name("SwapChainImage").unwrap();
        });
        let command_lists: Vec<dx12::GraphicsCommandList> = (0..3)
            .into_iter()
            .map(|_| {
                device
                    .create_graphics_command_list(dx12::CommandListType::Direct)
                    .unwrap()
            })
            .collect();

        let renderer = EguiRenderer::new(
            device.clone(),
            allocator,
            &buffers,
            drawable_size.0,
            drawable_size.1,
        );

        let schedule_provider = registry.get_interface::<dyn IScheduleProvider>().unwrap();
        let schedule_cell = schedule_provider.get();
        let mut schedule = schedule_cell.get();

        let mut data = Data {
            window,
            render_data,
            egui_provider,
            device,
            queue,
            event,
            fence,
            swap_chain,
            buffers,
            command_lists,
            renderer,
        };
        schedule.add_exclusive_at_start_system_to_stage(
            &CoreStage::Render,
            "render::render",
            move || {
                let data = &mut data;
                let egui_ctx = data.egui_provider.get_context();

                data.fence.signal(0).unwrap();
                data.fence.set_event_on_completion(1, &data.event).unwrap();

                if data.window.resized() {
                    let dimensions = data.window.size();
                    unsafe {
                        data.buffers.clear();
                        data.swap_chain
                            .resize_buffers(
                                3,
                                dimensions.0,
                                dimensions.1,
                                dxgi::Format::Unknown,
                                dxgi::SwapChainFlags::NONE,
                                None,
                                &[data.queue.clone(), data.queue.clone(), data.queue.clone()],
                            )
                            .unwrap();
                        data.buffers = data.swap_chain.get_buffers(3).unwrap();
                        data.renderer.recreate_swap_resources(
                            &data.device,
                            &data.buffers,
                            dimensions,
                        );
                    }
                }

                unsafe {
                    let index = data.swap_chain.get_current_back_buffer_index();
                    let command_list = &mut data.command_lists[index as usize];
                    data.renderer.record_frame(
                        index as usize,
                        command_list,
                        &data.buffers,
                        &egui_ctx,
                        data.render_data.take(),
                    );

                    data.queue.execute_command_lists(&[command_list.as_weak()]);

                    data.swap_chain.present(0, 0).unwrap();

                    data.queue.signal(&data.fence, 1).unwrap();

                    data.event.wait(None).unwrap();
                }
            },
        );

        Box::new(Vec::new())
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
