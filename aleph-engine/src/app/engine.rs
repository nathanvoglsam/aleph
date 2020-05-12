//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use crate::app::{AppInfo, AppLogic, WindowSettings};
use erupt::vk1_0::Vk10CoreLoaderExt;
use sdl2::event::Event;
use std::ffi::{CStr, CString};

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
/// The entry point for Aleph.
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
        log::info!("=== Game Info ===");
        log::info!("Name    : {}", &app_info.name);
        log::info!(
            "Version : {}.{}.{}",
            app_info.major,
            app_info.minor,
            app_info.patch
        );
        log::info!("=== Game Info ===");
        log::info!("=== Engine Info ===");
        log::info!("Name    : {}", ENGINE_NAME);
        log::info!("Version : {}", ENGINE_VERSION_STRING);
        log::info!("=== Engine Info ===");

        // Print some system info to the log so we know what we were running on
        Engine::log_cpu_info();

        // Initialized the thread pools
        Engine::init_thread_pools();

        // -----------------------------------------------------------------------------------------
        // SDL2 and Window Initialization
        // -----------------------------------------------------------------------------------------

        log::trace!("Initializing SDL2");
        let sdl_ctx = sdl2::init().expect("Failed to initialize SDL2");

        log::trace!("Initializing SDL2 Video Subsystem");
        // Init SDL2 video subsystem
        let video_ctx = crate::app::Window::init_video(&sdl_ctx);

        log::trace!("Initializing OS Window");
        // Init the window
        let mut window =
            crate::app::Window::init_window(&video_ctx, &app_info.name, &WindowSettings::default());

        log::trace!("Initializing Event Pump");
        // Init the event pump
        let mut event_pump = sdl_ctx
            .event_pump()
            .expect("Failed to init SDL2 event pump");

        // -----------------------------------------------------------------------------------------
        // Graphics Initialization
        // -----------------------------------------------------------------------------------------

        // Load core vulkan functions
        log::trace!("Initializing Vulkan Core Loader");
        let mut core_loader =
            erupt::CoreLoader::new().expect("Failed to create Vulkan core loader");

        // Load vulkan 1.0 core functions
        log::trace!("Initializing Vulkan 1.0");
        core_loader.load_vk1_0().expect("Failed to load Vulkan 1.0");
        log::trace!("Vulkan Loaded");

        // Print vulkan version
        let api_version = core_loader.instance_version();
        log::info!("=== VULKAN INFO ===");
        log::info!(
            "Version: {}.{}.{}",
            erupt::version_major(api_version),
            erupt::version_minor(api_version),
            erupt::version_patch(api_version)
        );
        log::info!("=== VULKAN INFO ===");

        // Fill out ApplicationInfo for creating a vulkan instance
        let app_name_cstr = CString::new(app_info.name.as_str()).unwrap();
        let app_version = erupt::make_version(app_info.major, app_info.minor, app_info.patch);
        let engine_name = unsafe { CStr::from_ptr(erupt::cstr!("AlephEngine")) };
        let api_version = erupt::make_version(1, 0, 0);
        let app_info = erupt::vk1_0::ApplicationInfoBuilder::new()
            .application_name(&app_name_cstr)
            .application_version(app_version)
            .engine_name(engine_name)
            .engine_version(ENGINE_VERSION_VK)
            .api_version(api_version);

        let mut extensions = erupt::utils::surface::enumerate_required_extensions(&window)
            .expect("Failed to get required vulkan surface extensions");
        extensions.push(erupt::extensions::ext_debug_utils::EXT_DEBUG_UTILS_EXTENSION_NAME);

        let mut layers = Vec::new();
        layers.push(erupt::cstr!("VK_LAYER_LUNARG_standard_validation"));

        // Fill out InstanceCreateInfo for creating a vulkan instance
        let create_info = erupt::vk1_0::InstanceCreateInfoBuilder::new()
            .application_info(&app_info)
            .enabled_extension_names(&extensions)
            .enabled_layer_names(&layers);

        // Construct the vulkan instance
        log::info!("Creating Vulkan instance");
        let instance = unsafe {
            let instance = core_loader.create_instance(&create_info, None, None);
            let instance = instance.expect("Failed to create Vulkan instance");
            instance
        };

        // Load the vulkan function pointers
        log::info!("Loading Vulkan functions");
        let mut instance_loader = erupt::InstanceLoader::new(&core_loader, instance)
            .expect("Failed to initialize Vulkan instance loader");
        instance_loader
            .load_vk1_0()
            .expect("Failed to load vulkan functions");

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
