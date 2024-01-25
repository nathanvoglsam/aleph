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

use aleph_rhi_api::*;
use interfaces::any::{declare_interfaces, AnyArc, QueryInterface};
use interfaces::make_plugin_description_for_crate;
use interfaces::platform::*;
use interfaces::plugin::*;
use interfaces::rhi::IRhiProvider;
use interfaces::schedule::{CoreStage, IScheduleProvider};

use crate::renderer::EguiRenderer;

struct Data {
    index: usize,
    should_resize: bool,
    window: AnyArc<dyn IWindow>,
    render_data: AnyArc<dyn egui::IEguiRenderData>,
    swap_chain: AnyArc<dyn ISwapChain>,
    renderer: EguiRenderer,
    swap_images: Vec<AnyArc<dyn ITexture>>,
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
        registrar.depends_on::<dyn IWindowProvider>();
        registrar.must_init_after::<dyn IWindowProvider>();

        registrar.depends_on::<dyn IRhiProvider>();
        registrar.must_init_after::<dyn IRhiProvider>();

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

        let rhi_provider = registry.get_interface::<dyn IRhiProvider>().unwrap();
        let surface = rhi_provider.surface().unwrap();
        let adapter = rhi_provider.adapter();
        let device = rhi_provider.device();

        self.device = Some(device.clone());

        let queue = device.get_queue(QueueType::General).unwrap();

        Self::log_gpu_info(
            device.deref().query_interface::<dyn IDevice>().unwrap(),
            adapter.deref(),
        );

        let drawable_size = window.drawable_size();
        let config = SwapChainConfiguration {
            format: Format::Bgra8UnormSrgb,
            width: drawable_size.0,
            height: drawable_size.1,
            present_mode: PresentationMode::Mailbox,
            buffer_count: 3,
            present_queue: QueueType::General,
        };
        let swap_chain = surface.create_swap_chain(device.deref(), &config).unwrap();

        let config = swap_chain.get_config();
        let mut swap_images: Vec<_> = (0..config.buffer_count).map(|_| None).collect();
        swap_chain.get_images(&mut swap_images);
        let swap_images: Vec<_> = swap_images.into_iter().map(|v| v.unwrap()).collect();
        let back_buffer_desc = swap_images[0].desc();

        assert!(swap_chain.present_supported_on_queue(QueueType::General));

        let pixels_per_point = window.current_display_scale();
        let renderer = EguiRenderer::new(device.clone(), &back_buffer_desc, pixels_per_point);

        let schedule_cell = registry
            .get_interface::<dyn IScheduleProvider>()
            .unwrap()
            .get();
        let mut schedule = schedule_cell.get();

        let mut data = Data {
            index: 0,
            should_resize: false,
            window,
            render_data,
            swap_chain,
            renderer,
            swap_images,
        };
        schedule.add_exclusive_at_start_system_to_stage(
            &CoreStage::Render,
            "render::render",
            move || {
                device.garbage_collect();

                let data = &mut data;

                if data.window.resized() || data.should_resize {
                    data.swap_images.clear();
                    let drawable_size = data.window.drawable_size();
                    let new_config = data
                        .swap_chain
                        .rebuild(Some(Extent2D::new(drawable_size.0, drawable_size.1)))
                        .unwrap();

                    let mut swap_images: Vec<_> =
                        (0..new_config.buffer_count).map(|_| None).collect();
                    data.swap_chain.get_images(&mut swap_images);
                    data.swap_images = swap_images.into_iter().map(|v| v.unwrap()).collect();

                    let back_buffer_desc = data.swap_images[0].desc();
                    let pixels_per_point = data.window.current_display_scale();
                    data.renderer
                        .rebuild_after_resize(&back_buffer_desc, pixels_per_point);

                    data.should_resize = false;
                }

                unsafe {
                    data.index = (data.index + 1) % 2;
                    let acquire_semaphore =
                        data.renderer.frames[data.index].acquire_semaphore.clone();
                    let present_semaphore =
                        data.renderer.frames[data.index].present_semaphore.clone();
                    let fence = data.renderer.frames[data.index].done_fence.clone();

                    assert_eq!(
                        device.wait_fences(&[fence.as_ref()], true, u32::MAX),
                        FenceWaitResult::Complete
                    );
                    device.reset_fences(&[fence.as_ref()]);

                    let acquired_index = match data.swap_chain.acquire_next_image(&AcquireDesc {
                        signal_semaphore: acquire_semaphore.as_ref(),
                    }) {
                        Ok(i) => i,
                        Err(ImageAcquireError::SubOptimal(i)) => {
                            data.should_resize = true;
                            i
                        }
                        v @ _ => v.unwrap(),
                    };
                    let acquired_image = data.swap_images[acquired_index as usize].clone();

                    let command_list = data.renderer.record_frame(
                        data.index,
                        acquired_image.deref(),
                        data.render_data.take(),
                    );

                    queue
                        .submit(&QueueSubmitDesc {
                            command_lists: &[Some(command_list).into()],
                            wait_semaphores: &[acquire_semaphore.as_ref()],
                            signal_semaphores: &[present_semaphore.as_ref()],
                            fence: Some(fence.as_ref()),
                        })
                        .unwrap();
                    queue
                        .present(&QueuePresentDesc {
                            swap_chain: data.swap_chain.as_ref(),
                            image_index: acquired_index,
                            wait_semaphores: &[present_semaphore.as_ref()],
                        })
                        .unwrap();
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
            device.garbage_collect();
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
