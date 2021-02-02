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

use crate::events::Event;
use aleph_settings::WindowSettings;
use once_cell::sync::Lazy;
use parking_lot::{Mutex, RwLock};
use sdl2::video::FullscreenType;
use std::ops::Deref;
use std::sync::atomic::{AtomicBool, Ordering};

/// An enum of window events.
#[derive(Clone)]
pub enum WindowEvent {
    Shown,
    Hidden,
    Exposed,
    Moved(i32, i32),
    Resized(i32, i32),
    SizeChanged(i32, i32),
    Minimized,
    Maximized,
    Restored,
    Enter,
    Leave,
    FocusGained,
    FocusLost,
    Close,
    TakeFocus,
    HitTest,
}

impl From<sdl2::event::WindowEvent> for WindowEvent {
    fn from(v: sdl2::event::WindowEvent) -> Self {
        match v {
            sdl2::event::WindowEvent::Shown => WindowEvent::Shown,
            sdl2::event::WindowEvent::Hidden => WindowEvent::Hidden,
            sdl2::event::WindowEvent::Exposed => WindowEvent::Exposed,
            sdl2::event::WindowEvent::Moved(x, y) => WindowEvent::Moved(x, y),
            sdl2::event::WindowEvent::Resized(x, y) => WindowEvent::Resized(x, y),
            sdl2::event::WindowEvent::SizeChanged(x, y) => WindowEvent::SizeChanged(x, y),
            sdl2::event::WindowEvent::Minimized => WindowEvent::Minimized,
            sdl2::event::WindowEvent::Maximized => WindowEvent::Maximized,
            sdl2::event::WindowEvent::Restored => WindowEvent::Restored,
            sdl2::event::WindowEvent::Enter => WindowEvent::Enter,
            sdl2::event::WindowEvent::Leave => WindowEvent::Leave,
            sdl2::event::WindowEvent::FocusGained => WindowEvent::FocusGained,
            sdl2::event::WindowEvent::FocusLost => WindowEvent::FocusLost,
            sdl2::event::WindowEvent::Close => WindowEvent::Close,
            sdl2::event::WindowEvent::TakeFocus => WindowEvent::TakeFocus,
            sdl2::event::WindowEvent::HitTest => WindowEvent::HitTest,
            sdl2::event::WindowEvent::None => panic!("Unsupported event"),
        }
    }
}

///
/// Does what it sends on the tin, holds the most recently collected state of the window. For more
/// info regarding "recently collected" see the documentation for `Window`
///
pub(crate) struct WindowState {
    /// The title/text in the window header
    pub(crate) title: String,

    /// The current width of the window on the desktop
    pub(crate) current_width: u32,

    /// The current height of the window on the desktop
    pub(crate) current_height: u32,

    /// The current width of the drawable surface
    pub(crate) drawable_width: u32,

    /// The current height of the drawable surface
    pub(crate) drawable_height: u32,

    /// The width of the window when not fullscreen
    pub(crate) windowed_width: u32,

    /// The height of the window when not fullscreen
    pub(crate) windowed_height: u32,

    pub(crate) refresh_rate: u32,

    /// Whether the window is currently fullscreen
    pub(crate) fullscreen: bool,

    /// Is the window currently focused
    pub(crate) focused: bool,
}

///
/// Represents the set of possible state change requests the window can perform
///
pub(crate) enum WindowRequest {
    ChangeTitle(String),
    ChangeSize(u32, u32),
    ChangeWidth(u32),
    ChangeHeight(u32),
    GoFullscreen,
    GoWindowed,
    ToggleFullscreen,
}

pub(crate) static WINDOW_STATE: Lazy<RwLock<Option<WindowState>>> = Lazy::new(|| RwLock::new(None));

pub(crate) static WINDOW_REQUEST_QUEUE: Lazy<Mutex<Option<Vec<WindowRequest>>>> =
    Lazy::new(|| Mutex::new(None));

pub(crate) static WINDOW_EVENTS: Lazy<RwLock<Option<Vec<WindowEvent>>>> =
    Lazy::new(|| RwLock::new(None));

pub(crate) static WINDOW_RESIZED: AtomicBool = AtomicBool::new(false);

///
/// A wrapper around a read guard on the underlying RwLock used to make the global window events
/// list thread safe.
///
/// # Warning
///
/// Do not try and hold onto this between frames, it will deadlock the engine.
///
pub struct WindowEvents {
    lock: parking_lot::RwLockReadGuard<'static, Option<Vec<WindowEvent>>>,
}

impl Deref for WindowEvents {
    type Target = [WindowEvent];

    fn deref(&self) -> &Self::Target {
        self.lock.as_ref().unwrap().as_slice()
    }
}

///
/// A "namespace struct" similar to the `Engine` struct that is used to encapsulate the global
/// window. A game is almost never ever going to need more than one window unless you're doing some
/// super wacky stuff so rather than trying to support we embrace it and offer a cleaner interface.
///
/// The main drawback is that getting multiple windows working is going to be really janky to use
/// but I think the benefits of having such a convenient thread-safe interface is worth it.
///
/// Another thing to be aware of is that almost every function publicly exposed here will panic if
/// there is no window initialized. This can cause issues if running headless, hence the
/// `window_loaded` function to gate it. You're not going to be touching the window much anyway
/// outside of UI or input code so that probably shouldn't be shipping in a headless executable
/// anyway. The engine will initialize it for you so unless you're doing something weird it wont be
/// a problem regardless.
///
/// # Implementation Details
///
/// The window is only safe to directly touch on the main thread because reasons™. The window
/// interface is thread-safe and accessible from any thread. To allow this the interface never
/// directly touches the window.
///
/// Underlying the window interface is a global request queue and a global window state object. Both
/// are gated behind locks. To change the window with one of the functions such as `set_title` a
/// request is pushed onto the request queue. The change will then actually take place on the next
/// frame.
///
/// At the beginning of each frame the main thread will consume the queued events and perform the
/// changes requested then update the global window state.
///
/// Because of this the state readable from this interface will be one frame out of date. The result
/// of a function like `set_title` will only be visible through the `title` function will only be
/// visible on the frame after `set_title` is called.
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
    /// Internal function for initializing the engine window
    ///
    pub(crate) fn init_window(
        video_ctx: &sdl2::VideoSubsystem,
        title: &str,
        window_settings: &WindowSettings,
    ) -> sdl2::video::Window {
        let mut window = video_ctx
            .window(title, window_settings.width, window_settings.height)
            .resizable()
            .build()
            .expect("Failed to create window");

        let drawable_size = window.vulkan_drawable_size();

        let display_mode = window.display_mode().unwrap();

        let window_state = WindowState {
            title: "".to_string(),
            current_width: window_settings.width,
            current_height: window_settings.height,
            drawable_width: drawable_size.0,
            drawable_height: drawable_size.1,
            windowed_width: window_settings.width,
            windowed_height: window_settings.height,
            refresh_rate: display_mode.refresh_rate as _,
            fullscreen: false,
            focused: false,
        };

        *WINDOW_STATE.write() = Some(window_state);
        *WINDOW_EVENTS.write() = Some(Vec::new());
        *WINDOW_REQUEST_QUEUE.lock() = Some(Vec::new());

        window.raise();

        window
    }

    ///
    /// Internal function for handling internal reactions to window events then pushing them to the
    /// event queue
    ///
    pub(crate) fn process_window_event(
        window_state: &mut WindowState,
        window_events: &mut Vec<WindowEvent>,
        all_events: &mut Vec<Event>,
        event: sdl2::event::WindowEvent,
    ) {
        match &event {
            sdl2::event::WindowEvent::Resized(width, height) => {
                WINDOW_RESIZED.store(true, Ordering::Relaxed);
                window_state.current_width = *width as u32;
                window_state.current_height = *height as u32;
                aleph_log::trace!("Window resized by OS");
                aleph_log::trace!("Window Size: {}x{}", width, height);
            }
            sdl2::event::WindowEvent::FocusGained => {
                window_state.focused = true;
            }
            sdl2::event::WindowEvent::FocusLost => {
                window_state.focused = false;
            }
            sdl2::event::WindowEvent::SizeChanged(width, height) => {
                WINDOW_RESIZED.store(true, Ordering::Relaxed);
                window_state.current_width = *width as u32;
                window_state.current_height = *height as u32;
                aleph_log::trace!("Window size changed");
                aleph_log::trace!("Window Size: {}x{}", width, height);
            },
            sdl2::event::WindowEvent::Moved(_, _)
            | sdl2::event::WindowEvent::Minimized
            | sdl2::event::WindowEvent::Maximized
            | sdl2::event::WindowEvent::Restored
            | sdl2::event::WindowEvent::Close
            | sdl2::event::WindowEvent::Enter
            | sdl2::event::WindowEvent::Leave
            | sdl2::event::WindowEvent::TakeFocus
            | sdl2::event::WindowEvent::HitTest
            | sdl2::event::WindowEvent::None
            | sdl2::event::WindowEvent::Shown
            | sdl2::event::WindowEvent::Hidden
            | sdl2::event::WindowEvent::Exposed => {}
        }
        window_events.push(event.into());
        all_events.push(Event::WindowEvent(event.into()));
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
                    aleph_log::trace!("Attempting to change window title");
                    window_state.title = title;
                    window
                        .set_title(&window_state.title)
                        .expect("Failed to set window title");
                    aleph_log::trace!("Successfuly changed window title");
                    aleph_log::trace!("Window Title: {}", &window_state.title);
                }
                WindowRequest::ChangeSize(width, height) => {
                    aleph_log::trace!("Attempting to change window size");
                    window_state.current_width = width;
                    window_state.current_height = height;
                    window
                        .set_size(width, height)
                        .expect("Failed to resize window");
                    aleph_log::trace!("Successfuly changed window size");
                    aleph_log::trace!("Window Size: {}x{}", width, height);
                }
                WindowRequest::ChangeWidth(width) => {
                    aleph_log::trace!("Attempting to change window width");
                    window_state.current_width = width;
                    window
                        .set_size(width, window_state.current_height)
                        .expect("Failed to resize window");
                    aleph_log::trace!("Successfuly changed window width");
                    aleph_log::trace!("Window Size: {}x{}", width, window_state.current_height);
                }
                WindowRequest::ChangeHeight(height) => {
                    aleph_log::trace!("Attempting to change window height");
                    window_state.current_height = height;
                    window
                        .set_size(window_state.current_width, height)
                        .expect("Failed to resize window");
                    aleph_log::trace!("Successfuly changed window height");
                    aleph_log::trace!("Window Size: {}x{}", window_state.current_width, height);
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

        let drawable_size = window.vulkan_drawable_size();
        window_state.drawable_width = drawable_size.0;
        window_state.drawable_height = drawable_size.1;
    }

    fn handle_go_fullscreen(window: &mut sdl2::video::Window, window_state: &mut WindowState) {
        aleph_log::trace!("Attempting to go fullscreen");
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
        window_state.refresh_rate = display_mode.refresh_rate as _;
        aleph_log::trace!("Successfully went fullscreen");
        aleph_log::trace!(
            "Window Size: {}x{} at {}hz",
            window_state.current_width,
            window_state.current_height,
            window_state.refresh_rate
        );
    }

    fn handle_go_windowed(window: &mut sdl2::video::Window, window_state: &mut WindowState) {
        aleph_log::trace!("Attempting to go windowed mode");
        window_state.current_width = window_state.windowed_width;
        window_state.current_height = window_state.windowed_height;

        window_state.fullscreen = false;
        window
            .set_fullscreen(FullscreenType::Off)
            .expect("Failed to set window fullscreen");
        window
            .set_size(window_state.windowed_width, window_state.windowed_height)
            .expect("Failed to reset window size after leaving fullscreen");

        let display_mode = window.display_mode().unwrap();
        window_state.refresh_rate = display_mode.refresh_rate as _;

        aleph_log::trace!("Successfully went windowed");
        aleph_log::trace!(
            "Window Size: {}x{} at {}hz",
            window_state.current_width,
            window_state.current_height,
            window_state.refresh_rate
        );
    }

    ///
    /// Internal function for getting new mouse state from SDL2
    ///
    pub(crate) fn update_state(window: &mut sdl2::video::Window, window_state: &mut WindowState) {
        let display_mode = window.display_mode().unwrap();
        window_state.refresh_rate = display_mode.refresh_rate as _;
    }

    ///
    /// Returns whether the window has been resized since the last time this function was called.
    ///
    /// # Info
    ///
    /// This interface was created to provide a very simple, one shot function that can be called
    /// once per frame to check if the window has been resized since last time it was checked.
    ///
    /// If the window has been resized then this will return true once, and only once, until the
    /// window is resized against. Calling this function resets the WINDOW_RESIZED flag
    ///
    /// This function is a very thin wrapper around a global `AtomicBool` that gets flagged by the
    /// event processing phase if it finds a window resizing event.
    ///
    /// # Warning
    ///
    /// This API will probably be useless to anyone other than the core engine implementers as the
    /// function will only yield the true result once per frame. The intended use for this API is
    /// for triggering a swap chain rebuild and this consumption based model makes the most sense
    /// for that use case.
    ///
    /// If you're using the engine, and not implementing it, then you should look at the
    /// `WindowEvents` API.
    ///
    pub fn resized() -> bool {
        WINDOW_RESIZED.swap(false, Ordering::Relaxed)
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
    /// Returns the dimensions of the window on the desktop
    ///
    /// Basically just the result of calling both `Window::width` and `Window::height` but only
    /// locks the state mutex once
    ///
    /// # Panic
    ///
    /// Panics if there is no window, such as if the engine is run headless
    ///
    pub fn size() -> (u32, u32) {
        let window_state_lock = WINDOW_STATE.read();
        let window_state = window_state_lock.as_ref().expect("Window not initialized");
        (window_state.current_width, window_state.current_height)
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
    /// Returns the width of the drawable surface on the window
    ///
    /// # Panic
    ///
    /// Panics if there is no window, such as if the engine is run headless
    ///
    pub fn drawable_width() -> u32 {
        WINDOW_STATE
            .read()
            .as_ref()
            .expect("Window not initialized")
            .drawable_width
    }

    ///
    /// Returns the height of the drawable surface on the window
    ///
    /// # Panic
    ///
    /// Panics if there is no window, such as if the engine is run headless
    ///
    pub fn drawable_height() -> u32 {
        WINDOW_STATE
            .read()
            .as_ref()
            .expect("Window not initialized")
            .drawable_height
    }

    ///
    /// Returns the dimensions of the drawable surface on the window
    ///
    /// Basically just the result of calling both `Window::drawable_width` and
    /// `Window::drawable_height` but only locks the state mutex once
    ///
    /// # Panic
    ///
    /// Panics if there is no window, such as if the engine is run headless
    ///
    pub fn drawable_size() -> (u32, u32) {
        let window_state_lock = WINDOW_STATE.read();
        let window_state = window_state_lock.as_ref().expect("Window not initialized");
        (window_state.drawable_width, window_state.drawable_height)
    }

    ///
    /// Returns the current refresh rate of the window
    ///
    /// # Panic
    ///
    /// Panics if there is no window, such as if the engine is run headless
    ///
    pub fn refresh_rate() -> u32 {
        let window_state_lock = WINDOW_STATE.read();
        let window_state = window_state_lock.as_ref().expect("Window not initialized");
        window_state.refresh_rate
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
    /// Return if the window is currently focused
    ///
    /// # Panic
    ///
    /// Panics if there is no window, such as if the engine is run headless
    ///
    pub fn focused() -> bool {
        WINDOW_STATE
            .read()
            .as_ref()
            .expect("Window not initialized")
            .focused
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

    ///
    /// Get read only access to this frame's list of window events.
    ///
    /// # Warning
    ///
    /// This will lock a global RwLock so trying to hold on to this between frames will deadlock the
    /// engine.
    ///
    pub fn events() -> WindowEvents {
        let lock = WINDOW_EVENTS.read();
        WindowEvents { lock }
    }
}
