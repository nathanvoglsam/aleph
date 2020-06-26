//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use crate::app::{AppLogic, FrameTimer, Imgui, Keyboard, Mouse, WindowSettings, Window};
use sdl2::event::Event;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use gpu::vulkan_core::erupt::vk1_0::{SemaphoreCreateInfoBuilder, Fence, Vk10DeviceLoaderExt};
use gpu::vulkan_core::GPUInfo;
use crate::app_info::AppInfo;
use crate::gpu::pipeline_cache::PipelineCache;

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
    pub fn start(app_info: AppInfo, mut app: impl AppLogic) {
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
        crate::logger::init();
        log::info!("Aleph Engine Starting");
        log::info!("");

        // Print info about the specific app to the log so we know what game and version we're on
        Engine::log_app_info(&app_info);
        log::info!("");

        // Print engine info to the log so we know what engine version we're running on
        Engine::log_engine_info();
        log::info!("");

        // Print some system info to the log so we know what we were running on
        Engine::log_cpu_info();
        log::info!("");

        // Initialize the thread pools
        Engine::init_thread_pools();
        log::info!("");

        // -----------------------------------------------------------------------------------------
        // SDL2 and Window Initialization
        // -----------------------------------------------------------------------------------------

        // Init SDL2
        log::trace!("Initializing SDL2");
        let sdl_ctx = sdl2::init().expect("Failed to initialize SDL2");

        // Init SDL2 video subsystem
        log::trace!("Initializing SDL2 Video Subsystem");
        let video_ctx = crate::app::Window::init_video(&sdl_ctx);

        // Init the window
        log::trace!("Initializing OS Window");
        let mut window =
            crate::app::Window::init_window(&video_ctx, &app_info.name, &WindowSettings::default());

        // Init the event pump
        log::trace!("Initializing Event Pump");
        let mut event_pump = sdl_ctx
            .event_pump()
            .expect("Failed to init SDL2 event pump");

        // Init SDL2 Mouse Utils
        log::trace!("Initializing Mouse Utils");
        let mouse_utils = sdl_ctx.mouse();

        log::trace!("Initializing SDL2 Timer Subsystem");
        let timer = sdl_ctx.timer().expect("Failed to init SDL2 Timer system");

        // Space out logging
        log::trace!("");

        // -----------------------------------------------------------------------------------------
        // Frame Timer Initialization
        // -----------------------------------------------------------------------------------------
        FrameTimer::init(&timer);

        // -----------------------------------------------------------------------------------------
        // Mouse Initialization
        // -----------------------------------------------------------------------------------------

        // Initialize the global mouse system
        Mouse::init();

        // -----------------------------------------------------------------------------------------
        // Keyboard Initialization
        // -----------------------------------------------------------------------------------------

        // Initialize the global mouse system
        Keyboard::init();

        // -----------------------------------------------------------------------------------------
        // ImGui Initialization
        // -----------------------------------------------------------------------------------------

        // Initialize imgui
        let mut imgui_ctx = Imgui::new();

        // -----------------------------------------------------------------------------------------
        // Graphics Initialization
        // -----------------------------------------------------------------------------------------

        // Load core vulkan functions for creating an instance
        let instance = gpu::core::InstanceBuilder::new()
            .debug(args.is_present("GPU_DEBUG") || args.is_present("GPU_VALIDATION"))
            .validation(args.is_present("GPU_VALIDATION"))
            .build(&window, &app_info);

        let device = gpu::core::DeviceBuilder::new().build(&instance);
        log::trace!("");

        Self::log_gpu_info(device.info());
        log::info!("");

        PipelineCache::init(&device);

        let allocator = gpu::alloc::Allocator::builder()
            .build(&device)
            .expect("Failed to build vulkan allocator");

        let mut swapchain = gpu::core::SwapchainBuilder::new()
            .vsync()
            .build(&device, Window::drawable_size());

        let _renderer = unsafe {
            gpu::render::Renderer::new(device.clone(), allocator.clone(), &swapchain)
        };

        let mut imgui_renderer = gpu::render::ImguiRenderer::new(
            imgui_ctx.context_mut().fonts(),
            device.clone(),
            allocator.clone(),
            &swapchain,
        );

        let create_info = SemaphoreCreateInfoBuilder::new();
        let acquire_semaphore = unsafe {
            device
                .loader()
                .create_semaphore(&create_info, None, None)
                .expect("Failed to create acquire semaphore")
        };
        let signal_semaphore = unsafe {
            device
                .loader()
                .create_semaphore(&create_info, None, None)
                .expect("Failed to create barrier semaphore")
        };

        // =========================================================================================
        // Engine Fully Initialized
        // =========================================================================================

        // Call the AppLogic on_init now that it is safe to do so
        log::trace!("Calling AppLogic::on_init");
        app.on_init();

        // Process the SDL2 events and store them into our own event queues for later use
        'game_loop: loop {
            // Check if the engine should shutdown
            if !Engine::keep_running() {
                break 'game_loop;
            }

            // Update the frame delta timer
            FrameTimer::frame(&timer);

            if Engine::handle_pre_update(&mut window, &mut event_pump, &mut imgui_ctx, &mouse_utils)
            {
                break 'game_loop;
            }

            let ui = imgui_ctx.frame(&mouse_utils);

            app.on_update(&ui);

            unsafe {
                device
                    .loader()
                    .device_wait_idle()
                    .expect("Failed to wait on device idle");
            }

            if swapchain.requires_rebuild() {
                let _ = swapchain.rebuild(Window::drawable_size());
                unsafe {
                    imgui_renderer.recreate_resources(&swapchain);
                }
            }

            let i = match swapchain.acquire_next(
                Duration::from_millis(10000),
                acquire_semaphore,
                Fence::null(),
            ) {
                Ok(v) => v,
                Err(_) => {
                    continue;
                }
            };

            unsafe {
                imgui_renderer.render_frame(
                    ui,
                    &swapchain,
                    acquire_semaphore,
                    signal_semaphore,
                    i as usize,
                );
            }

            swapchain.present(device.general_queue(), i as usize, &[signal_semaphore]);
        }

        log::trace!("Calling AppLogic::on_exit");
        app.on_exit();

        unsafe {
            device
                .loader()
                .device_wait_idle()
                .expect("Failed to wait on device idle");

            device.loader().destroy_semaphore(signal_semaphore, None);
            device.loader().destroy_semaphore(acquire_semaphore, None);
        }
    }

    ///
    /// A thread pool for use for long running tasks so long running tasks wont have short running
    /// tasks contend with workers
    ///
    pub fn long_running_pool() -> &'static rayon::ThreadPool {
        &crate::app::LONG_RUNNING_THREAD_POOL
            .get()
            .expect("Aleph not Initialized")
    }

    ///
    /// A thread pool for short running tasks so short running tasks wont contend for workers with
    /// long running tasks
    ///
    pub fn short_running_pool() -> &'static rayon::ThreadPool {
        &crate::app::SHORT_RUNNING_THREAD_POOL
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
        log::info!("=== Game Info ===");
        log::info!("Name    : {}", &app_info.name);
        log::info!(
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
        log::info!("=== Engine Info ===");
        log::info!("Name    : {}", app_info::engine_name());
        log::info!("Version : {}", app_info::engine_version_string());
        log::info!("Arch    : {}", target::build::target_architecture().name());
        log::info!(
            "OS      : {}",
            target::build::target_platform().pretty_name()
        );
        log::info!(
            "Build   : {}",
            target::build::target_build_type().pretty_name()
        );
    }

    ///
    /// Internal function for logging info about the CPU that is being used
    ///
    fn log_cpu_info() {
        log::info!("=== CPU INFO ===");
        log::info!("CPU Vendor    : {}", crate::cpuid::cpu_vendor());
        log::info!("CPU Brand     : {}", crate::cpuid::cpu_brand());
        log::info!("Physical CPUs : {}", num_cpus::get_physical());
        log::info!("Logical CPUs  : {}", num_cpus::get());
        log::info!("System RAM    : {}MB", sdl2::cpuinfo::system_ram());
    }

    ///
    /// Internal function for logging info about the CPU that is being used
    ///
    fn log_gpu_info(info: &GPUInfo) {
        log::info!("=== GPU INFO ===");
        log::info!("GPU Vendor    : {}", info.vendor_id.vendor_name());
        log::info!("GPU Name      : {}", &info.device_name);
        log::info!(
            "API Version   : {}.{}.{}",
            info.api_version_major,
            info.api_version_minor,
            info.api_version_patch
        )
    }

    ///
    /// Internal function for handling various events prior to the user part of the game loop
    ///
    fn handle_pre_update(
        window: &mut sdl2::video::Window,
        event_pump: &mut sdl2::EventPump,
        imgui_ctx: &mut Imgui,
        mouse_utils: &sdl2::mouse::MouseUtil,
    ) -> bool {
        // Get access to window state
        let mut window_state_lock = crate::app::WINDOW_STATE.write();

        let window_state = window_state_lock.as_mut();
        let window_state = window_state.expect("Window not initialized");

        // Get access to the keyboard state
        let mut keyboard_state_lock = crate::app::KEYBOARD_STATE.write();

        let keyboard_state = keyboard_state_lock.as_mut();
        let keyboard_state = keyboard_state.expect("Keyboard not initialized");

        imgui_ctx.update_mouse_pos_early();

        crate::app::Mouse::process_mouse_requests(window, mouse_utils);
        crate::app::Window::process_window_requests(window, window_state);

        if Self::handle_event_pump(event_pump, window_state, keyboard_state) {
            return true;
        }

        drop(window_state_lock);
        drop(keyboard_state_lock);

        imgui_ctx.update_mouse_pos_late();
        imgui_ctx.update_keyboard_input();

        false
    }

    ///
    /// Internal function for handling the SDL2 event pump
    ///
    fn handle_event_pump(
        event_pump: &mut sdl2::EventPump,
        window_state: &mut crate::app::WindowState,
        keyboard_state: &mut crate::app::KeyboardState,
    ) -> bool {
        // Get access to window events
        let mut window_events_lock = crate::app::WINDOW_EVENTS.write();
        let window_events = window_events_lock.as_mut();
        let window_events = window_events.expect("Window not initialized");

        let mut mouse_events_lock = crate::app::MOUSE_EVENTS.write();
        let mouse_events = mouse_events_lock.as_mut();
        let mouse_events = mouse_events.expect("Mouse system not initialized");

        let mut keyboard_events_lock = crate::app::KEYBOARD_EVENTS.write();
        let keyboard_events = keyboard_events_lock.as_mut();
        let keyboard_events = keyboard_events.expect("Mouse system not initialized");

        window_events.clear();
        mouse_events.clear();
        keyboard_events.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    log::info!("Quit Event Received");
                    return true;
                }
                Event::Window { win_event, .. } => {
                    crate::app::Window::process_window_event(
                        window_state,
                        window_events,
                        win_event,
                    );
                }
                event @ Event::MouseButtonDown { .. } => {
                    Mouse::process_mouse_event(mouse_events, event);
                }
                event @ Event::MouseButtonUp { .. } => {
                    Mouse::process_mouse_event(mouse_events, event);
                }
                event @ Event::MouseMotion { .. } => {
                    Mouse::process_mouse_event(mouse_events, event);
                }
                event @ Event::MouseWheel { .. } => {
                    Mouse::process_mouse_event(mouse_events, event);
                }
                event @ Event::KeyDown { .. } => {
                    Keyboard::process_keyboard_event(keyboard_events, keyboard_state, event);
                }
                event @ Event::KeyUp { .. } => {
                    Keyboard::process_keyboard_event(keyboard_events, keyboard_state, event);
                }
                event @ Event::TextInput { .. } => {
                    Keyboard::process_keyboard_event(keyboard_events, keyboard_state, event);
                }
                _ => {}
            }
        }

        crate::app::Mouse::update_state(event_pump);

        false
    }

    ///
    /// Internal function for initializing the global thread pools
    ///
    fn init_thread_pools() {
        let long_threads;
        let short_threads;

        match num_cpus::get() {
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
                let cpus = num_cpus::get();

                if (cpus / 4) < 1 {
                    long_threads = 1;
                } else {
                    long_threads = cpus / 4;
                }

                short_threads = cpus - long_threads;
            }
        }

        super::init_long_thread_pool(long_threads);
        log::info!(
            "Long Running thread pool initialized with {} threads",
            long_threads
        );

        super::init_short_thread_pool(short_threads);
        log::info!(
            "Short Running thread pool initialized with {} threads",
            short_threads
        );
    }
}
