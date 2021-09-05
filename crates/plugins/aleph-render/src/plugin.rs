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
use dx12::{dxgi, D3D12Object};
use interfaces::any;
use interfaces::platform::*;
use interfaces::plugin::*;

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

pub struct PluginRenderDX12 {
    data: Option<Data>,
}

impl PluginRenderDX12 {
    pub fn new() -> Self {
        Self { data: None }
    }
}

impl IPlugin for PluginRenderDX12 {
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
        registrar.depends_on::<dyn IWindowProvider>();
        registrar.must_init_after::<dyn IWindowProvider>();

        registrar.depends_on::<dyn egui::IEguiRenderData>();
        registrar.depends_on::<dyn egui::IEguiContextProvider>();
        registrar.must_init_after::<dyn egui::IEguiRenderData>();
        registrar.must_init_after::<dyn egui::IEguiContextProvider>();

        registrar.update_stage(UpdateStage::Render);
    }

    fn on_init(&mut self, registry: &dyn IRegistryAccessor) -> Box<dyn IInitResponse> {
        // Get the raw window handle from the window
        let window_provider = registry.get_interface::<dyn IWindowProvider>().unwrap();
        let window = window_provider.get_window().unwrap();
        let window_handle = window.raw_window_handle();

        // Get the render data slot for egui
        let render_data = registry
            .get_interface::<dyn egui::IEguiRenderData>()
            .unwrap();

        // Get the egui provider
        let egui_provider = registry
            .get_interface::<dyn egui::IEguiContextProvider>()
            .unwrap();

        log::trace!("Creating DXGIFactory");
        let mut dxgi_factory = dxgi::Factory::new(true).expect("Failed to create DXGI factory");

        log::trace!("Selecting DXGIAdatper");
        let dxgi_adapter = dxgi_factory
            .select_hardware_adapter(dx12::FeatureLevel::Level_11_0)
            .expect("Failed to find capable GPU");

        // Enable debug layers if requested
        let _debug = unsafe {
            setup_debug_layer(true, false);
        };

        log::trace!("Creating D3D12Device");
        let device = dx12::Device::new(Some(&dxgi_adapter), dx12::FeatureLevel::Level_11_0)
            .expect("Failed to create D3D12 device");

        //let _compiler = unsafe { dx12::DxcCompiler::new().unwrap() };
        //let _validator = unsafe { dx12::DxcValidator::new().unwrap() };

        aleph_log::info!("");
        Self::log_gpu_info(&dxgi_adapter);
        aleph_log::info!("");

        let allocator_desc = dx12_alloc::AllocatorDesc::builder()
            .device(device.clone())
            .adapter(dxgi_adapter.clone())
            .build();
        let allocator = dx12_alloc::Allocator::new(&allocator_desc).unwrap();

        let desc = dx12::CommandQueueDesc::builder()
            .queue_type(dx12::CommandListType::Direct)
            .priority(0)
            .build();
        let queue = device.create_command_queue(&desc).unwrap();

        let event = dx12::Event::new().unwrap();
        let fence = device.create_fence(0, dx12::FenceFlags::NONE).unwrap();

        let drawable_size = window.drawable_size();
        let desc = dxgi::SwapChainDesc1::builder()
            .width(drawable_size.0)
            .height(drawable_size.1)
            .format(dxgi::Format::R8G8B8A8Unorm)
            .buffer_count(3)
            .usage_flags(dxgi::UsageFlags::BACK_BUFFER)
            .usage_flags(dxgi::UsageFlags::RENDER_TARGET_OUTPUT)
            .build();
        let mut swap_chain = dxgi_factory
            .create_swap_chain(&queue, &window_handle, &desc)
            .unwrap();
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
            allocator.clone(),
            &buffers,
            drawable_size.0,
            drawable_size.1,
        );

        self.data = Some(Data {
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
        });

        Box::new(Vec::new())
    }

    fn on_render(&mut self, _registry: &dyn IRegistryAccessor) {
        let data = self.data.as_mut().unwrap();
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
                data.renderer
                    .recreate_swap_resources(&data.device, &data.buffers, dimensions);
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

            data.queue.execute_command_lists(&[&command_list]);

            data.swap_chain.present(0, 0).unwrap();

            data.queue.signal(&data.fence, 1).unwrap();

            data.event.wait(None).unwrap();
        }
    }
}

any::declare_interfaces!(PluginRenderDX12, [IPlugin]);

impl PluginRenderDX12 {
    ///
    /// Internal function for logging info about the CPU that is being used
    ///
    fn log_gpu_info(adapter: &dxgi::Adapter) {
        let info = adapter.get_adapter_desc().unwrap();

        let gpu_vendor = info.vendor_id_string().unwrap_or("Unknown");
        let gpu_name = info.description_string().unwrap_or("Unknown".to_string());
        let dvmem = info.dedicated_video_memory / 1_000_000;
        let dsmem = info.dedicated_system_memory / 1_000_000;
        let ssmem = info.shared_system_memory / 1_000_000;

        aleph_log::info!("=== GPU INFO ===");
        aleph_log::info!("GPU Vendor    : {}", gpu_vendor);
        aleph_log::info!("GPU Name      : {}", gpu_name);
        aleph_log::info!("Memory        : {}MB | {}MB | {}MB", dvmem, dsmem, ssmem)
    }
}

unsafe fn setup_debug_layer(gpu_debug: bool, gpu_validation: bool) -> Option<dx12::Debug> {
    if gpu_debug {
        log::trace!("D3D12 debug layers requested");
        if let Ok(debug) = dx12::Debug::new() {
            debug.enable_debug_layer();
            log::trace!("D3D12 debug layers enabled");
            if gpu_validation {
                log::trace!("D3D12 gpu validation requested");
                if debug.set_enable_gpu_validation(true).is_ok() {
                    log::trace!("D3D12 gpu validation enabled");
                } else {
                    log::trace!("D3D12 gpu validation not enabled");
                }
            }
            Some(debug)
        } else {
            None
        }
    } else {
        None
    }
}
