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

use imgui::Ui;
use crate::AppLogic;
use app_info::AppInfo;
use platform::window::Window;
use platform::Platform;
use std::sync::atomic::{AtomicBool, Ordering};
use vulkan::pipeline_cache::PipelineCache;
use vulkan_core::erupt::vk1_0::Vk10DeviceLoaderExt;
use vulkan_core::GPUInfo;

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
        // -----------------------------------------------------------------------------------------
        // Read Command Line Switches
        // -----------------------------------------------------------------------------------------
        let name = app_info.name.clone();
        let version = app_info.version_string();
        let author = app_info.author.clone();
        let about = "Powered by AlephEngine";
        let args = clap::App::new(app_info.name.clone())
            .name(name)
            .version(version.as_str())
            .about(about)
            .author(&*author)
            .arg(
                clap::Arg::with_name("GPU_DEBUG")
                    .long("gpu-debug")
                    .short("d"),
            )
            .arg(
                clap::Arg::with_name("GPU_VALIDATION")
                    .long("gpu-validation")
                    .short("v"),
            )
            .get_matches();

        // -----------------------------------------------------------------------------------------
        // Core Initialization
        // -----------------------------------------------------------------------------------------

        // First thing we do is initialize the log backend so everything can log from now on
        aleph_logger::init();
        aleph_log::info!("Aleph Engine Starting");
        aleph_log::info!("");

        // Init the profiler and mark the first frame
        optick::next_frame();

        // Print info about the specific app to the log so we know what game and version we're on
        Engine::log_app_info(&app_info);
        aleph_log::info!("");

        // Print engine info to the log so we know what engine version we're running on
        Engine::log_engine_info();
        aleph_log::info!("");

        // Print some system info to the log so we know what we were running on
        Engine::log_cpu_info();
        aleph_log::info!("");

        // Initialize the thread pools
        Engine::init_thread_pools();
        aleph_log::info!("");

        // -----------------------------------------------------------------------------------------
        // SDL2 and Window Initialization
        // -----------------------------------------------------------------------------------------

        let mut platform = Platform::builder()
            .headless(false)
            .app_info(app_info.clone())
            .build()
            .expect("Failed to build platform layer");

        // -----------------------------------------------------------------------------------------
        // ImGui Initialization
        // -----------------------------------------------------------------------------------------

        // Initialize imgui
        let mut imgui_ctx = aleph_platform_imgui::Imgui::new();

        // -----------------------------------------------------------------------------------------
        // Graphics Initialization
        // -----------------------------------------------------------------------------------------

        // Load core vulkan functions for creating an instance
        let instance = vulkan_core::InstanceBuilder::new()
            .debug(args.is_present("GPU_DEBUG") || args.is_present("GPU_VALIDATION"))
            .validation(args.is_present("GPU_VALIDATION"))
            .build(&platform, &app_info);

        let device = vulkan_core::DeviceBuilder::new().build(&instance);

        Self::log_gpu_info(device.info());
        aleph_log::info!("");

        PipelineCache::init(&device);

        let allocator = vulkan_alloc::Allocator::builder()
            .build(&device)
            .expect("Failed to build vulkan allocator");

        let mut swapchain = vulkan_core::SwapchainBuilder::new()
            .vsync()
            .build(&device, Window::drawable_size());

        let mut renderer = unsafe {
            render::Renderer::new(
                device.clone(),
                allocator.clone(),
                &swapchain,
                imgui_ctx.context_mut(),
            )
        };

        // =========================================================================================
        // Engine Fully Initialized
        // =========================================================================================

        // Call the AppLogic on_init now that it is safe to do so
        aleph_log::trace!("Calling AppLogic::on_init");
        app.on_init();

        // Process the SDL2 events and store them into our own event queues for later use
        'game_loop: loop {
            // Mark a new frame for the optick profiler
            optick::next_frame();

            // Mark a new frame for the platform
            platform.frame();

            // ImGui pre-event update (this can emit requests so we need to handle them before we
            // call process_requests)
            imgui_ctx.update_mouse_pos_early();

            // Process requests and events
            platform.process_requests();
            platform.process_events(|| {
                Engine::exit();
            });

            // ImGui post-event update (this processes this frame's set of events so it must happen
            // after process_events an process_requests)
            imgui_ctx.update_mouse_pos_late();
            imgui_ctx.update_keyboard_input();

            // Check if the engine should shutdown. This will be updated by process_events so we
            // need to check after calling process_events
            if !Engine::keep_running() {
                break 'game_loop;
            }

            let ui = imgui_ctx.frame();

            {
                optick::event!("aleph_engine::AppLogic::on_update");
                app.on_update(&ui);
            }

            unsafe {
                let i = renderer.acquire_swap_image(&mut swapchain, Window::drawable_size());
                if i.is_none() {
                    continue;
                }
                let i = i.unwrap();
                renderer.render_frame(&mut swapchain, i, ui);
            }
        }

        aleph_log::trace!("Calling AppLogic::on_exit");
        app.on_exit();

        unsafe {
            device
                .loader()
                .device_wait_idle()
                .expect("Failed to wait on device idle");
        }
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
    fn log_gpu_info(info: &GPUInfo) {
        let gpu_vendor = info.vendor_id.vendor_name();
        let gpu_name = info.device_name.as_str();
        let maj = info.api_version_major;
        let min = info.api_version_minor;
        let pat = info.api_version_patch;
        aleph_log::info!("=== GPU INFO ===");
        aleph_log::info!("GPU Vendor    : {}", gpu_vendor);
        aleph_log::info!("GPU Name      : {}", gpu_name);
        aleph_log::info!("API Version   : {}.{}.{}", maj, min, pat)
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
