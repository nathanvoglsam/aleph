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

use crate::dx12::FeatureLevel;
use crate::dx12::{dxgi, D3D12Object};
use crate::pix::{Colour, ScopedEvent};
use crate::platform::{Platform, Window};
use app_info::AppInfo;
use egui::PaintJobs;
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicBool, Ordering};

static ENGINE_KEEP_RUNNING: AtomicBool = AtomicBool::new(true);

///
/// A "namespace" struct that wraps a bunch of global stace into a struct for aesthetic and
/// convenience purposes.
///
/// Also serves as the engine's entry point with the `start` function.
///
pub struct Engine {}

impl Engine {
    ///
    /// This function is **THE** entry point for Aleph. It all begins here. This manages setting up
    /// a window or drawing surface, setting up input, asset systems, rendering, scripting, etc.
    ///
    /// Once everything is set up it hands
    ///
    pub fn start(app_info: AppInfo, mut app: impl crate::AppLogic) {
        // Initialize COM with MTA
        #[cfg(target_os = "windows")]
        dx12::initialize_mta().unwrap();

        #[cfg(target_os = "windows")]
        dx12::name_thread_as_main_thread();

        // -----------------------------------------------------------------------------------------
        // Read Command Line Switches
        // -----------------------------------------------------------------------------------------
        //let name = app_info.name.clone();
        //let version = app_info.version_string();
        //let author = app_info.author.clone();
        //let about = "Powered by AlephEngine";
        //let args = clap::App::new(app_info.name.clone())
        //    .name(name)
        //    .version(version.as_str())
        //    .about(about)
        //    .author(&*author)
        //    .arg(
        //        clap::Arg::with_name("GPU_DEBUG")
        //            .long("gpu-debug")
        //            .short("d"),
        //    )
        //    .arg(
        //        clap::Arg::with_name("GPU_VALIDATION")
        //            .long("gpu-validation")
        //            .short("v"),
        //    )
        //    .get_matches();

        // -----------------------------------------------------------------------------------------
        // Core Initialization
        // -----------------------------------------------------------------------------------------

        // First thing we do is initialize the log backend so everything can log from now on
        aleph_logger::init();
        aleph_log::info!("");
        aleph_log::info!("Aleph Engine Starting");

        // Print info about the specific app to the log so we know what game and version we're on
        aleph_log::info!("");
        Engine::log_app_info(&app_info);
        aleph_log::info!("");

        // Print engine info to the log so we know what engine version we're running on
        aleph_log::info!("");
        Engine::log_engine_info();
        aleph_log::info!("");

        // Print some system info to the log so we know what we were running on
        aleph_log::info!("");
        Engine::log_cpu_info();
        aleph_log::info!("");

        // Initialize the thread pools
        aleph_log::info!("");
        Engine::init_thread_pools();
        aleph_log::info!("");

        // -----------------------------------------------------------------------------------------
        // SDL2 and Window Initialization
        // -----------------------------------------------------------------------------------------

        Platform::builder()
            .headless(false)
            .app_info(app_info.clone())
            .build(move |platform| {
                let mut platform = platform.expect("Failed to build platform layer");

                // ---------------------------------------------------------------------------------
                // Egui Initialization
                // ---------------------------------------------------------------------------------

                // Initialize egui
                let mut egui_ctx = egui::CtxRef::default();
                egui_ctx.set_fonts(egui_font_definitions(true));

                // ---------------------------------------------------------------------------------
                // Graphics Initialization
                // ---------------------------------------------------------------------------------

                log::trace!("Creating DXGIFactory");
                let mut dxgi_factory =
                    dxgi::Factory::new(true).expect("Failed to create DXGI factory");

                log::trace!("Selecting DXGIAdatper");
                let dxgi_adapter = dxgi_factory
                    .select_hardware_adapter(dx12::FeatureLevel::Level_11_0)
                    .expect("Failed to find capable GPU");

                // Enable debug layers if requested
                let _debug = unsafe {
                    setup_debug_layer(true, false);
                };

                log::trace!("Creating D3D12Device");
                let device = dx12::Device::new(Some(&dxgi_adapter), FeatureLevel::Level_11_0)
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

                let drawable_size = Window::drawable_size();
                let desc = dxgi::SwapChainDesc1::builder()
                    .width(drawable_size.0)
                    .height(drawable_size.1)
                    .format(dxgi::Format::R8G8B8A8Unorm)
                    .buffer_count(3)
                    .usage_flags(dxgi::UsageFlags::BACK_BUFFER)
                    .usage_flags(dxgi::UsageFlags::RENDER_TARGET_OUTPUT)
                    .build();
                let mut swapchain = dxgi_factory
                    .create_swap_chain(&queue, &platform, &desc)
                    .unwrap();
                let mut buffers = swapchain.get_buffers(3).unwrap();
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

                let mut renderer = render::EguiRenderer::new(
                    device.clone(),
                    allocator.clone(),
                    &buffers,
                    drawable_size.0,
                    drawable_size.1,
                );

                // =================================================================================
                // Engine Fully Initialized
                // =================================================================================

                // Call the AppLogic on_init now that it is safe to do so
                aleph_log::trace!("Calling AppLogic::on_init");
                app.on_init();

                // Process the SDL2 events and store them into our own event queues for later use
                'game_loop: loop {
                    let _pix = ScopedEvent::new(Colour::CYAN, "Frame");

                    // Mark a new frame for the platform
                    platform.frame();

                    // Process requests and events
                    platform.process_requests();
                    platform.process_events(|| {
                        Engine::exit();
                    });

                    // Check if the engine should shutdown. This will be updated by process_events
                    // so we need to check after calling process_events
                    if !Engine::keep_running() {
                        break 'game_loop;
                    }

                    // Collect input and begin new Egui frame
                    let new_input = aleph_platform_egui::get_egui_input();
                    egui_ctx.begin_frame(new_input);

                    app.on_update(&egui_ctx);

                    fence.signal(0).unwrap();
                    fence.set_event_on_completion(1, &event).unwrap();

                    if Window::resized() {
                        let dimensions = Window::size();
                        unsafe {
                            buffers.clear();
                            swapchain
                                .resize_buffers(
                                    3,
                                    dimensions.0,
                                    dimensions.1,
                                    dxgi::Format::Unknown,
                                    dxgi::SwapChainFlags::NONE,
                                    None,
                                    &[&queue, &queue, &queue],
                                )
                                .unwrap();
                            buffers = swapchain.get_buffers(3).unwrap();
                            renderer.recreate_swap_resources(&device, &buffers, dimensions);
                        }
                    }

                    // End the egui frame
                    let (output, shapes) = egui_ctx.end_frame();
                    let jobs: PaintJobs = egui_ctx.tessellate(shapes);
                    aleph_platform_egui::process_egui_output(output);

                    unsafe {
                        let index = swapchain.get_current_back_buffer_index();
                        let command_list = &command_lists[index as usize];
                        renderer.record_frame(
                            index as usize,
                            command_list,
                            &buffers,
                            &egui_ctx,
                            jobs,
                        );

                        let mut queue_recorder = queue.record();

                        let mut submission_builder = dx12::SubmissionBuilder::new();
                        submission_builder.add(command_list);
                        queue_recorder.execute_command_lists(&submission_builder);

                        swapchain.present(0, 0).unwrap();

                        queue_recorder.signal(&fence, 1).unwrap();
                        event.wait(None);
                    }
                }

                aleph_log::trace!("Calling AppLogic::on_exit");
                app.on_exit();
            });
    }

    ///
    /// A thread pool for use for long running tasks so long running tasks wont have short running
    /// tasks contend with workers
    ///
    pub fn long_running_pool() -> &'static rayon::ThreadPool {
        &crate::thread_pools::LONG_RUNNING_THREAD_POOL
            .get()
            .expect("Aleph not Initialized")
    }

    ///
    /// A thread pool for short running tasks so short running tasks wont contend for workers with
    /// long running tasks
    ///
    pub fn short_running_pool() -> &'static rayon::ThreadPool {
        &crate::thread_pools::SHORT_RUNNING_THREAD_POOL
            .get()
            .expect("Aleph not Initialized")
    }

    ///
    /// Requests the engine to shutdown (at the earliest convenience)
    ///
    /// This enqueues a shutdown. The current frame of execution *will* finish and shutdown will
    /// begin after the frame has completed and before the next frame *would* have started
    ///
    pub fn exit() {
        ENGINE_KEEP_RUNNING.store(false, Ordering::Relaxed);
    }

    ///
    /// Internal function for getting the KEEP_ENGINE_RUNNING value
    ///
    fn keep_running() -> bool {
        ENGINE_KEEP_RUNNING.load(Ordering::Relaxed)
    }

    ///
    /// Internal function for logging info about the game
    ///
    fn log_app_info(app_info: &AppInfo) {
        aleph_log::info!("=== Game Info ===");
        aleph_log::info!("Name    : {}", &app_info.name);
        aleph_log::info!(
            "Version : {}.{}.{}",
            app_info.major,
            app_info.minor,
            app_info.patch
        );
    }

    ///
    /// Internal function for logging info about the engine
    ///
    fn log_engine_info() {
        let engine_name = app_info::engine_name();
        let engine_version = app_info::engine_version_string();
        let arch = aleph_target::build::target_architecture().name();
        let os = aleph_target::build::target_platform().pretty_name();
        let build = aleph_target::build::target_build_type().pretty_name();
        aleph_log::info!("=== Engine Info ===");
        aleph_log::info!("Name    : {}", engine_name);
        aleph_log::info!("Version : {}", engine_version);
        aleph_log::info!("Arch    : {}", arch);
        aleph_log::info!("OS      : {}", os);
        aleph_log::info!("Build   : {}", build);
    }

    ///
    /// Internal function for logging info about the CPU that is being used
    ///
    fn log_cpu_info() {
        let cpu_vendor = aleph_cpu_info::cpu_vendor();
        let cpu_brand = aleph_cpu_info::cpu_brand();
        let physical_cpus = aleph_cpu_info::physical_core_count();
        let logical_cpus = aleph_cpu_info::logical_core_count();
        let system_ram = Platform::system_ram();
        aleph_log::info!("=== CPU INFO ===");
        aleph_log::info!("CPU Vendor    : {}", cpu_vendor);
        aleph_log::info!("CPU Brand     : {}", cpu_brand);
        aleph_log::info!("Physical CPUs : {}", physical_cpus);
        aleph_log::info!("Logical CPUs  : {}", logical_cpus);
        aleph_log::info!("System RAM    : {}MB", system_ram);
    }

    ///
    /// Internal function for logging info about the CPU that is being used
    ///
    fn log_gpu_info(adapter: &dxgi::Adapter) {
        let info = adapter.get_adapter_desc().unwrap();

        let gpu_vendor = if info.vendor_id == 0x10DE {
            "NVIDIA"
        } else if info.vendor_id == 0x1002 {
            "AMD"
        } else if info.vendor_id == 0x8086 {
            "INTEL"
        } else {
            "Unknown"
        };
        let gpu_name = String::from_utf16(&info.description).unwrap();
        let dvmem = info.dedicated_video_memory / 1_000_000;
        let dsmem = info.dedicated_system_memory / 1_000_000;
        let ssmem = info.shared_system_memory / 1_000_000;

        aleph_log::info!("=== GPU INFO ===");
        aleph_log::info!("GPU Vendor    : {}", gpu_vendor);
        aleph_log::info!("GPU Name      : {}", gpu_name);
        aleph_log::info!("Memory        : {}MB | {}MB | {}MB", dvmem, dsmem, ssmem)
    }

    ///
    /// Internal function for initializing the global thread pools
    ///
    fn init_thread_pools() {
        let long_threads;
        let short_threads;

        match aleph_cpu_info::logical_core_count() {
            2 => {
                long_threads = 1;
                short_threads = 1;
            }
            4 => {
                long_threads = 1;
                short_threads = 3;
            }
            6 => {
                long_threads = 2;
                short_threads = 4;
            }
            8 => {
                long_threads = 2;
                short_threads = 6;
            }
            10 => {
                long_threads = 4;
                short_threads = 6;
            }
            12 => {
                long_threads = 4;
                short_threads = 8;
            }
            14 => {
                long_threads = 4;
                short_threads = 10;
            }
            16 => {
                long_threads = 4;
                short_threads = 12;
            }
            _ => {
                let cpus = aleph_cpu_info::logical_core_count();

                if (cpus / 4) < 1 {
                    long_threads = 1;
                } else {
                    long_threads = cpus / 4;
                }

                short_threads = cpus - long_threads;
            }
        }

        crate::thread_pools::init_long_thread_pool(long_threads);
        aleph_log::info!(
            "Long Running thread pool initialized with {} threads",
            long_threads
        );

        crate::thread_pools::init_short_thread_pool(short_threads);
        aleph_log::info!(
            "Short Running thread pool initialized with {} threads",
            short_threads
        );
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

fn egui_font_definitions(jetbrains: bool) -> egui::FontDefinitions {
    use aleph_embedded_data as data;

    let mut font_data = BTreeMap::new();
    let mut fonts_for_family = BTreeMap::new();

    let jetbrains_mono_name = "JetbrainsMono";
    let jetbrains_mono = data::fonts::jetbrains_mono_regular();
    let cascadia_code_name = "CascadiaCode";
    let cascadia_code = data::fonts::cascadia_code();
    let noto_sans_name = "NotoSans-Regular";
    let noto_sans = data::fonts::noto_sans_regular();
    let noto_emoji_name = "NotoEmoji-Regular";
    let noto_emoji = data::fonts::noto_emoji_regular();
    let emoji_icons_name = "emoji-icon-font";
    let emoji_icons = data::fonts::emoji_icon_font();

    let monospace_name = if jetbrains {
        font_data.insert(
            jetbrains_mono_name.to_owned(),
            Cow::Borrowed(jetbrains_mono),
        );
        jetbrains_mono_name
    } else {
        font_data.insert(cascadia_code_name.to_owned(), Cow::Borrowed(cascadia_code));
        cascadia_code_name
    };
    font_data.insert(noto_sans_name.to_owned(), Cow::Borrowed(noto_sans));
    font_data.insert(noto_emoji_name.to_owned(), Cow::Borrowed(noto_emoji));
    font_data.insert(emoji_icons_name.to_owned(), Cow::Borrowed(emoji_icons));

    fonts_for_family.insert(
        egui::FontFamily::Monospace,
        vec![
            monospace_name.to_owned(),
            noto_sans_name.to_owned(),
            noto_emoji_name.to_owned(),
            emoji_icons_name.to_owned(),
        ],
    );
    fonts_for_family.insert(
        egui::FontFamily::Proportional,
        vec![
            noto_sans_name.to_owned(),
            noto_emoji_name.to_owned(),
            emoji_icons_name.to_owned(),
        ],
    );

    let mut family_and_size = BTreeMap::new();
    family_and_size.insert(
        egui::TextStyle::Small,
        (egui::FontFamily::Proportional, 14.0),
    );
    family_and_size.insert(
        egui::TextStyle::Body,
        (egui::FontFamily::Proportional, 17.0),
    );
    family_and_size.insert(
        egui::TextStyle::Button,
        (egui::FontFamily::Proportional, 18.0),
    );
    family_and_size.insert(
        egui::TextStyle::Heading,
        (egui::FontFamily::Proportional, 22.0),
    );
    family_and_size.insert(
        egui::TextStyle::Monospace,
        (egui::FontFamily::Monospace, 14.0),
    );

    egui::FontDefinitions {
        font_data,
        fonts_for_family,
        family_and_size,
    }
}
