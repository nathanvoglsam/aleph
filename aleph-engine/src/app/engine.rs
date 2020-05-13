//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use crate::app::{AppInfo, AppLogic, WindowSettings};
use sdl2::event::Event;

pub const ENGINE_NAME: &str = "AlephEngine";
pub const ENGINE_VERSION_STRING: &str = "0.1.0";
pub const ENGINE_VERSION_MAJOR: u32 = 0;
pub const ENGINE_VERSION_MINOR: u32 = 1;
pub const ENGINE_VERSION_PATCH: u32 = 0;
pub const ENGINE_VERSION_VK: u32 = erupt::make_version(
    ENGINE_VERSION_MAJOR,
    ENGINE_VERSION_MINOR,
    ENGINE_VERSION_PATCH,
);

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
        // =========================================================================================
        // Engine Initialization Starting
        // =========================================================================================

        // -----------------------------------------------------------------------------------------
        // Core Initialization
        // -----------------------------------------------------------------------------------------

        // First thing we do is initialize the log backend so everything can log from now on
        crate::logger::init();
        log::info!("Aleph Engine Starting");

        // Print info about the specific app to the log so we know what game and version we're on
        app_info.log_info();

        // Print engine info to the log so we know what engine version we're running on
        Engine::log_engine_info();

        // Print some system info to the log so we know what we were running on
        Engine::log_cpu_info();

        // Initialized the thread pools
        Engine::init_thread_pools();

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

        // -----------------------------------------------------------------------------------------
        // Graphics Initialization
        // -----------------------------------------------------------------------------------------

        // Load core vulkan functions for creating an instance
        let core_loader = crate::gpu::load_vulkan_core();

        // Create the vulkan instance
        let instance = crate::gpu::create_instance(&core_loader, &app_info, &window);

        let _instance_loader = crate::gpu::load_vulkan_instance(&core_loader, instance);

        // =========================================================================================
        // Engine Fully Initialized
        // =========================================================================================

        // Call the AppLogic on_init now that it is safe to do so
        log::trace!("Calling AppLogic::on_init");
        app.on_init();

        // Process the SDL2 events and store them into our own event queues for later use
        'game_loop: loop {
            if Engine::handle_pre_update(&mut window, &mut event_pump) {
                break 'game_loop;
            }

            app.on_update();
        }

        log::trace!("Calling AppLogic::on_exit");
        app.on_exit();
    }

    fn log_engine_info() {
        log::info!("=== Engine Info ===");
        log::info!("Name    : {}", ENGINE_NAME);
        log::info!("Version : {}", ENGINE_VERSION_STRING);
        log::info!("Arch    : {}", target::build::target_architecture().name());
        log::info!(
            "OS      : {}",
            target::build::target_platform().pretty_name()
        );
        log::info!(
            "Build   : {}",
            target::build::target_build_type().pretty_name()
        );
        log::info!("=== Engine Info ===");
    }

    fn handle_pre_update(
        mut window: &mut sdl2::video::Window,
        mut event_pump: &mut sdl2::EventPump,
    ) -> bool {
        // Get access to window state
        let mut window_state_lock = crate::app::WINDOW_STATE.write();

        let window_state = window_state_lock.as_mut();
        let window_state = window_state.expect("Window not initialized");

        crate::app::Window::process_window_requests(&mut window, window_state);

        if Self::handle_event_pump(&mut event_pump, window_state) {
            return true;
        }
        false
    }

    ///
    /// Internal function for handling the SDL2 event pump
    ///
    fn handle_event_pump(
        event_pump: &mut sdl2::EventPump,
        window_state: &mut crate::app::WindowState,
    ) -> bool {
        // Get access to window events
        let mut window_events_lock = crate::app::WINDOW_EVENTS.write();

        let window_events = window_events_lock.as_mut();
        let window_events = window_events.expect("Window not initialized");

        window_events.clear();

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
                _ => {}
            }
        }
        false
    }

    ///
    ///
    ///
    pub fn start_headless<T: Into<String>>(_: T, mut app: impl AppLogic) {
        // First thing we do is initialize the log backend so everything can log from now on
        crate::logger::init();
        log::info!("Aleph Engine Starting Headless");

        Engine::log_cpu_info();
        Engine::init_thread_pools();

        log::info!("Entering into AppLogic::on_init");
        app.on_init();
        log::info!("Exiting AppLogic::on_init");

        app.on_update();

        log::info!("Entering into AppLogic::on_exit");
        app.on_exit();
        log::info!("Exiting AppLogic::on_exit");
    }

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

    fn log_cpu_info() {
        log::info!("=== CPU INFO ===");
        log::info!("CPU Vendor    : {}", crate::cpuid::cpu_vendor());
        log::info!("CPU Brand     : {}", crate::cpuid::cpu_brand());
        log::info!("Physical CPUs : {}", num_cpus::get_physical());
        log::info!("Logical CPUs  : {}", num_cpus::get());
        log::info!("System RAM    : {}MB", sdl2::cpuinfo::system_ram());
        log::info!("=== CPU INFO ===");
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
}
