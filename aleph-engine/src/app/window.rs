//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use crate::app::WindowSettings;
use once_cell::sync::Lazy;
use parking_lot::{Mutex, RwLock};
use sdl2::event::WindowEvent;
use sdl2::video::FullscreenType;

pub struct WindowState {
    pub(crate) title: String,
    pub(crate) current_width: u32,
    pub(crate) current_height: u32,
    pub(crate) windowed_width: u32,
    pub(crate) windowed_height: u32,
    pub(crate) fullscreen: bool,
}

pub enum WindowRequest {
    ChangeTitle(String),
    ChangeSize(u32, u32),
    ChangeWidth(u32),
    ChangeHeight(u32),
    GoFullscreen,
    GoWindowed,
    ToggleFullscreen,
}

pub static WINDOW_STATE: Lazy<RwLock<Option<WindowState>>> = Lazy::new(|| RwLock::new(None));

pub static WINDOW_REQUEST_QUEUE: Lazy<Mutex<Option<Vec<WindowRequest>>>> =
    Lazy::new(|| Mutex::new(None));

pub static WINDOW_EVENTS: Lazy<RwLock<Option<Vec<WindowEvent>>>> = Lazy::new(|| RwLock::new(None));

///
/// Global struct for the window
///
pub struct Window {}

impl Window {
    ///
    /// Is there a window currently loaded
    ///
    pub fn window_loaded() -> bool {
        WINDOW_STATE.read().is_some()
    }

    ///
    /// Internal function for initializing the video SDL subsystem
    ///
    pub(crate) fn init_video(sdl_ctx: &sdl2::Sdl) -> sdl2::VideoSubsystem {
        sdl_ctx
            .video()
            .expect("Failed to init SDL2 video subsystem")
    }

    ///
    /// Internal function for initializing the engine window
    ///
    pub(crate) fn init_window(
        video_ctx: &sdl2::VideoSubsystem,
        title: &str,
        window_settings: &WindowSettings,
    ) -> sdl2::video::Window {
        let window = video_ctx
            .window(title, window_settings.width, window_settings.height)
            .vulkan()
            .resizable()
            .build()
            .expect("Failed to create window");

        let window_state = WindowState {
            title: "".to_string(),
            current_width: window_settings.width,
            current_height: window_settings.height,
            windowed_width: window_settings.width,
            windowed_height: window_settings.height,
            fullscreen: false,
        };

        *WINDOW_STATE.write() = Some(window_state);
        *WINDOW_EVENTS.write() = Some(Vec::new());
        *WINDOW_REQUEST_QUEUE.lock() = Some(Vec::new());

        window
    }

    ///
    /// Internal function for handling internal reactions to window events then pushing them to the
    /// event queue
    ///
    pub(crate) fn process_window_event(
        window_state: &mut WindowState,
        window_events: &mut Vec<WindowEvent>,
        event: WindowEvent,
    ) {
        match &event {
            WindowEvent::Resized(width, height) => {
                window_state.current_width = *width as u32;
                window_state.current_height = *height as u32;
                log::trace!("Window resized by OS");
                log::trace!("Window Size: {}x{}", width, height);
            }
            WindowEvent::SizeChanged(_, _)
            | WindowEvent::Moved(_, _)
            | WindowEvent::Minimized
            | WindowEvent::Maximized
            | WindowEvent::Restored
            | WindowEvent::FocusGained
            | WindowEvent::FocusLost
            | WindowEvent::Close
            | WindowEvent::Enter
            | WindowEvent::Leave
            | WindowEvent::TakeFocus
            | WindowEvent::HitTest
            | WindowEvent::None
            | WindowEvent::Shown
            | WindowEvent::Hidden
            | WindowEvent::Exposed => {}
        }
        window_events.push(event);
    }

    ///
    /// Internal function for handling internal reactions to window events then pushing them to the
    /// event queue
    ///
    pub(crate) fn process_window_requests(
        window: &mut sdl2::video::Window,
        window_state: &mut WindowState,
    ) {
        let mut window_requests_lock = WINDOW_REQUEST_QUEUE.lock();
        let window_requests = window_requests_lock.as_mut();
        let window_requests = window_requests.expect("Window not initialized");

        for request in window_requests.drain(..) {
            match request {
                WindowRequest::ChangeTitle(title) => {
                    log::trace!("Attempting to change window title");
                    window_state.title = title;
                    window
                        .set_title(&window_state.title)
                        .expect("Failed to set window title");
                    log::trace!("Successfuly changed window title");
                    log::trace!("Window Title: {}", &window_state.title);
                }
                WindowRequest::ChangeSize(width, height) => {
                    log::trace!("Attempting to change window size");
                    window_state.current_width = width;
                    window_state.current_height = height;
                    window
                        .set_size(width, height)
                        .expect("Failed to resize window");
                    log::trace!("Successfuly changed window size");
                    log::trace!("Window Size: {}x{}", width, height);
                }
                WindowRequest::ChangeWidth(width) => {
                    log::trace!("Attempting to change window width");
                    window_state.current_width = width;
                    window
                        .set_size(width, window_state.current_height)
                        .expect("Failed to resize window");
                    log::trace!("Successfuly changed window width");
                    log::trace!("Window Size: {}x{}", width, window_state.current_height);
                }
                WindowRequest::ChangeHeight(height) => {
                    log::trace!("Attempting to change window height");
                    window_state.current_height = height;
                    window
                        .set_size(window_state.current_width, height)
                        .expect("Failed to resize window");
                    log::trace!("Successfuly changed window height");
                    log::trace!("Window Size: {}x{}", window_state.current_width, height);
                }
                WindowRequest::GoFullscreen => Window::handle_go_fullscreen(window, window_state),
                WindowRequest::GoWindowed => Window::handle_go_windowed(window, window_state),
                WindowRequest::ToggleFullscreen => {
                    if window_state.fullscreen {
                        Window::handle_go_windowed(window, window_state);
                    } else {
                        Window::handle_go_fullscreen(window, window_state);
                    }
                }
            }
        }
    }

    fn handle_go_fullscreen(window: &mut sdl2::video::Window, window_state: &mut WindowState) {
        log::trace!("Attempting to go fullscreen");
        window_state.windowed_width = window_state.current_width;
        window_state.windowed_height = window_state.current_height;

        window
            .set_fullscreen(FullscreenType::True)
            .expect("Failed to set window fullscreen");

        let display_mode = window
            .display_mode()
            .expect("Failed to get window display mode");

        window_state.fullscreen = true;
        window_state.current_width = display_mode.w as _;
        window_state.current_height = display_mode.h as _;
        log::trace!("Successfully went fullscreen");
        log::trace!(
            "Window Size: {}x{}",
            window_state.current_width,
            window_state.current_height
        );
    }

    fn handle_go_windowed(window: &mut sdl2::video::Window, window_state: &mut WindowState) {
        log::trace!("Attempting to go windowed mode");
        window_state.current_width = window_state.windowed_width;
        window_state.current_height = window_state.windowed_height;

        window_state.fullscreen = false;
        window
            .set_fullscreen(FullscreenType::Off)
            .expect("Failed to set window fullscreen");
        window
            .set_size(window_state.windowed_width, window_state.windowed_height)
            .expect("Failed to reset window size after leaving fullscreen");
        log::trace!("Successfully went windowed");
        log::trace!(
            "Window Size: {}x{}",
            window_state.current_width,
            window_state.current_height
        );
    }

    ///
    /// Returns the title for the window
    ///
    /// # Panic
    ///
    /// Panics if there is no window, such as if the engine is run headless
    ///
    pub fn title() -> String {
        WINDOW_STATE
            .read()
            .as_ref()
            .expect("Window not initialized")
            .title
            .clone()
    }

    ///
    /// Sets the title of the window
    ///
    /// Will only take affect at the beginning of the next frame
    ///
    /// # Panic
    ///
    /// Panics if there is no window, such as if the engine is run headless
    ///
    pub fn set_title(title: String) {
        WINDOW_REQUEST_QUEUE
            .lock()
            .as_mut()
            .expect("Window not initialized")
            .push(WindowRequest::ChangeTitle(title));
    }

    ///
    /// Return the width of the window
    ///
    /// # Panic
    ///
    /// Panics if there is no window, such as if the engine is run headless
    ///
    pub fn width() -> u32 {
        WINDOW_STATE
            .read()
            .as_ref()
            .expect("Window not initialized")
            .current_width
    }

    ///
    /// Sets the width of the window
    ///
    /// Will only take affect at the beginning of the next frame
    ///
    /// # Panic
    ///
    /// Panics if there is no window, such as if the engine is run headless
    ///
    pub fn set_width(width: u32) {
        WINDOW_REQUEST_QUEUE
            .lock()
            .as_mut()
            .expect("Window not initialized")
            .push(WindowRequest::ChangeWidth(width));
    }

    ///
    /// Return the height of the window
    ///
    /// # Panic
    ///
    /// Panics if there is no window, such as if the engine is run headless
    ///
    pub fn height() -> u32 {
        WINDOW_STATE
            .read()
            .as_ref()
            .expect("Window not initialized")
            .current_height
    }

    ///
    /// Sets the height of the window
    ///
    /// Will only take affect at the beginning of the next frame
    ///
    /// # Panic
    ///
    /// Panics if there is no window, such as if the engine is run headless
    ///
    pub fn set_height(height: u32) {
        WINDOW_REQUEST_QUEUE
            .lock()
            .as_mut()
            .expect("Window not initialized")
            .push(WindowRequest::ChangeHeight(height));
    }

    ///
    /// Sets the size of the window
    ///
    /// Will only take affect at the beginning of the next frame
    ///
    /// # Panic
    ///
    /// Panics if there is no window, such as if the engine is run headless
    ///
    pub fn set_size(width: u32, height: u32) {
        WINDOW_REQUEST_QUEUE
            .lock()
            .as_mut()
            .expect("Window not initialized")
            .push(WindowRequest::ChangeSize(width, height));
    }

    ///
    /// Return if the window is currently fullscreen
    ///
    /// # Panic
    ///
    /// Panics if there is no window, such as if the engine is run headless
    ///
    pub fn fullscreen() -> bool {
        WINDOW_STATE
            .read()
            .as_ref()
            .expect("Window not initialized")
            .fullscreen
    }

    ///
    /// Sets the window to fullscreen
    ///
    /// Will only take affect at the beginning of the next frame
    ///
    /// # Panic
    ///
    /// Panics if there is no window, such as if the engine is run headless
    ///
    pub fn go_fullscreen() {
        WINDOW_REQUEST_QUEUE
            .lock()
            .as_mut()
            .expect("Window not initialized")
            .push(WindowRequest::GoFullscreen);
    }

    ///
    /// Sets the window to windowed mode
    ///
    /// Will only take affect at the beginning of the next frame
    ///
    /// # Panic
    ///
    /// Panics if there is no window, such as if the engine is run headless
    ///
    pub fn go_windowed() {
        WINDOW_REQUEST_QUEUE
            .lock()
            .as_mut()
            .expect("Window not initialized")
            .push(WindowRequest::GoWindowed);
    }

    ///
    /// Swaps between fullscreen or windowed
    ///
    /// Will only take affect at the beginning of the next frame
    ///
    /// # Panic
    ///
    /// Panics if there is no window, such as if the engine is run headless
    ///
    pub fn toggle_fullscreen() {
        WINDOW_REQUEST_QUEUE
            .lock()
            .as_mut()
            .expect("Window not initialized")
            .push(WindowRequest::ToggleFullscreen);
    }
}
