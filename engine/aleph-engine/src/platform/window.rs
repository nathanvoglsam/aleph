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

use std::ffi::c_void;
use std::ptr::NonNull;
use std::sync::atomic::{AtomicBool, Ordering};

use api::any::{AnyArc, declare_interfaces};
use api::platform::{
    Event, HasDisplayHandle, HasWindowHandle, IWindow, IWindowEventsLock, RawDisplayHandle,
    RawWindowHandle, WindowEvent,
};
use parking_lot::{Mutex, RwLock, RwLockReadGuard};
use raw_window_handle::{DisplayHandle, HandleError, WindowHandle};
use smallbox::space::S1;
use smallbox::{SmallBox, smallbox};

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

    /// Whether the window is currently fullscreen
    pub(crate) fullscreen: bool,

    /// Is the window currently focused
    pub(crate) focused: bool,

    /// Is the scaling of the window based on the display it is currently on
    pub(crate) current_display_scale: f32,

    /// Is the content scaling of the window based on the display it is currently on
    pub(crate) current_content_scale: f32,

    /// The window's display handle
    pub(crate) display_handle: RawDisplayHandle,

    /// The window's window handle
    pub(crate) window_handle: RawWindowHandle,

    /// The 'CAMetalLayer' pointer for the window, only available on Metal (Apple) platforms.
    pub(crate) metal_layer: Option<NonNull<c_void>>,
}

unsafe impl Send for WindowState {}
unsafe impl Sync for WindowState {}

///
/// Represents the set of possible state change requests the window can perform
///
pub(crate) enum WindowRequest {
    /// Request the window to update it's title
    ChangeTitle(String),

    /// Request the window change to the provided width and height
    ChangeSize(u32, u32),

    /// Request the window change to the provided width
    ChangeWidth(u32),

    /// Request the window change to the provided height
    ChangeHeight(u32),

    /// Request the window to become fullscreen
    GoFullscreen,

    /// Request the window to become windowed
    GoWindowed,

    /// Request the window to toggle between windowed and fullscreen
    ToggleFullscreen,
}

///
/// The struct that provides the context for, and implements, `IWindow`
///
pub struct Window {
    /// The window state, as recorded at the beginning of a frame
    pub(crate) state: RwLock<WindowState>,

    /// The event list for the current frame
    pub(crate) events: RwLock<Vec<WindowEvent>>,

    /// The request queue that will be flushed and handled at the beginning of the frame after a
    /// request is made
    pub(crate) requests: Mutex<Vec<WindowRequest>>,

    /// A flag that is used to check whether the window has been resized
    pub(crate) resized: AtomicBool,
}

declare_interfaces!(Window, [IWindow]);

impl Window {
    ///
    /// Internal function for initializing the engine window
    ///
    pub(crate) fn new(
        video_ctx: &sdl3::VideoSubsystem,
        title: &str,
    ) -> (AnyArc<Self>, sdl3::video::Window) {
        const DEFAULT_WIDTH: u32 = 1280;
        const DEFAULT_HEIGHT: u32 = 800;

        let mut window = video_ctx.window(title, DEFAULT_WIDTH, DEFAULT_HEIGHT);
        window.resizable();
        window.high_pixel_density();

        #[cfg(any(target_os = "macos", target_os = "ios"))]
        {
            window.metal_view();
        }

        #[cfg(any(target_os = "ios"))]
        {
            window.fullscreen();
        }

        let mut window = window.build().expect("Failed to create window");
        let display = window.get_display().expect("Failed to get display");

        let scale = window.display_scale();
        let content_scale = display.get_content_scale().unwrap_or(1.0);

        let drawable_size = window.size_in_pixels();

        let (display_handle, window_handle, metal_layer) = {
            #[cfg(not(any(target_os = "macos", target_os = "ios")))]
            {
                (
                    window.display_handle().unwrap().as_raw(),
                    window.window_handle().unwrap().as_raw(),
                    None,
                )
            }

            #[cfg(any(target_os = "macos", target_os = "ios"))]
            unsafe {
                use raw_window_handle::{AppKitWindowHandle, UiKitWindowHandle};
                use sdl3::sys::metal::{SDL_Metal_CreateView, SDL_Metal_GetLayer};

                let window_handle = window.window_handle().unwrap();
                let (window_handle, metal_layer) = match window_handle.as_raw() {
                    RawWindowHandle::AppKit(_) => {
                        let ns_view = NonNull::new(SDL_Metal_CreateView(window.raw())).unwrap();
                        let metal_layer = NonNull::new(SDL_Metal_GetLayer(ns_view.as_ptr()));
                        let window_handle =
                            RawWindowHandle::AppKit(AppKitWindowHandle::new(ns_view));
                        (window_handle, metal_layer)
                    }
                    RawWindowHandle::UiKit(_) => {
                        let ui_view = NonNull::new(SDL_Metal_CreateView(window.raw())).unwrap();
                        let metal_layer = NonNull::new(SDL_Metal_GetLayer(ui_view.as_ptr()));
                        let window_handle = RawWindowHandle::UiKit(UiKitWindowHandle::new(ui_view));
                        (window_handle, metal_layer)
                    }
                    _ => {
                        panic!("Unexpected window handle for current platform!");
                    }
                };
                let display_handle = window.display_handle().unwrap().as_raw();

                (display_handle, window_handle, metal_layer)
            }
        };

        let window_state = WindowState {
            title: title.to_string(),
            current_width: DEFAULT_WIDTH,
            current_height: DEFAULT_HEIGHT,
            drawable_width: drawable_size.0,
            drawable_height: drawable_size.1,
            windowed_width: DEFAULT_WIDTH,
            windowed_height: DEFAULT_HEIGHT,
            fullscreen: false,
            focused: false,
            current_display_scale: scale,
            current_content_scale: content_scale,
            display_handle,
            window_handle,
            metal_layer,
        };

        let out = Self {
            state: RwLock::new(window_state),
            events: RwLock::new(Vec::new()),
            requests: Mutex::new(Vec::new()),
            resized: AtomicBool::new(false),
        };

        window.raise();

        (AnyArc::new(out), window)
    }

    ///
    /// Internal function for handling internal reactions to window events then pushing them to the
    /// event queue
    ///
    pub(crate) fn process_window_event(
        &self,
        window_state: &mut WindowState,
        window_events: &mut Vec<WindowEvent>,
        all_events: &mut Vec<Event>,
        event: sdl3::event::WindowEvent,
    ) {
        match &event {
            sdl3::event::WindowEvent::Resized(width, height) => {
                self.resized.store(true, Ordering::Relaxed);
                window_state.current_width = *width as u32;
                window_state.current_height = *height as u32;
                log::trace!("Window logical size changed: {}x{}", width, height);
            }
            sdl3::event::WindowEvent::FocusGained => {
                window_state.focused = true;
            }
            sdl3::event::WindowEvent::FocusLost => {
                window_state.focused = false;
            }
            sdl3::event::WindowEvent::PixelSizeChanged(width, height) => {
                self.resized.store(true, Ordering::Relaxed);
                window_state.current_width = *width as u32;
                window_state.current_height = *height as u32;
                log::trace!("Window pixel size changed: {}x{}", width, height);
            }
            sdl3::event::WindowEvent::DisplayChanged(v) => {
                log::trace!("Window display changed: {v}");
            }
            sdl3::event::WindowEvent::Moved(_, _)
            | sdl3::event::WindowEvent::Minimized
            | sdl3::event::WindowEvent::Maximized
            | sdl3::event::WindowEvent::Restored
            | sdl3::event::WindowEvent::CloseRequested
            | sdl3::event::WindowEvent::MouseEnter
            | sdl3::event::WindowEvent::MouseLeave
            | sdl3::event::WindowEvent::HitTest(_, _)
            | sdl3::event::WindowEvent::Shown
            | sdl3::event::WindowEvent::Hidden
            | sdl3::event::WindowEvent::Exposed
            | sdl3::event::WindowEvent::ICCProfChanged => {}
            sdl3::event::WindowEvent::None => {
                log::trace!("Got window event 'None'");
            }
        }

        let converted_event = match event {
            sdl3::event::WindowEvent::Shown => WindowEvent::Shown,
            sdl3::event::WindowEvent::Hidden => WindowEvent::Hidden,
            sdl3::event::WindowEvent::Exposed => WindowEvent::Exposed,
            sdl3::event::WindowEvent::Moved(x, y) => WindowEvent::Moved(x, y),
            sdl3::event::WindowEvent::Resized(x, y) => WindowEvent::Resized(x, y),
            sdl3::event::WindowEvent::PixelSizeChanged(x, y) => WindowEvent::SizeChanged(x, y),
            sdl3::event::WindowEvent::Minimized => WindowEvent::Minimized,
            sdl3::event::WindowEvent::Maximized => WindowEvent::Maximized,
            sdl3::event::WindowEvent::Restored => WindowEvent::Restored,
            sdl3::event::WindowEvent::MouseEnter => WindowEvent::Enter,
            sdl3::event::WindowEvent::MouseLeave => WindowEvent::Leave,
            sdl3::event::WindowEvent::FocusGained => WindowEvent::FocusGained,
            sdl3::event::WindowEvent::FocusLost => WindowEvent::FocusLost,
            sdl3::event::WindowEvent::CloseRequested => WindowEvent::Close,
            sdl3::event::WindowEvent::HitTest(_, _) => WindowEvent::HitTest,
            sdl3::event::WindowEvent::None => return,
            sdl3::event::WindowEvent::ICCProfChanged => return,
            sdl3::event::WindowEvent::DisplayChanged(_) => return,
        };
        window_events.push(converted_event.clone());
        all_events.push(Event::WindowEvent(converted_event));
    }

    ///
    /// Internal function for handling internal reactions to window events then pushing them to the
    /// event queue
    ///
    pub(crate) fn process_window_requests(
        &self,
        window: &mut sdl3::video::Window,
        window_state: &mut WindowState,
    ) {
        for request in self.requests.lock().drain(..) {
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
                WindowRequest::GoFullscreen => Self::handle_go_fullscreen(window, window_state),
                WindowRequest::GoWindowed => Self::handle_go_windowed(window, window_state),
                WindowRequest::ToggleFullscreen => {
                    if window_state.fullscreen {
                        Self::handle_go_windowed(window, window_state);
                    } else {
                        Self::handle_go_fullscreen(window, window_state);
                    }
                }
            }
        }
    }

    fn handle_go_fullscreen(window: &mut sdl3::video::Window, window_state: &mut WindowState) {
        log::trace!("Attempting to go fullscreen");
        window_state.windowed_width = window_state.current_width;
        window_state.windowed_height = window_state.current_height;

        window
            .set_fullscreen(true)
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
            window_state.current_height,
        );
    }

    fn handle_go_windowed(window: &mut sdl3::video::Window, window_state: &mut WindowState) {
        log::trace!("Attempting to go windowed mode");
        window_state.current_width = window_state.windowed_width;
        window_state.current_height = window_state.windowed_height;

        window_state.fullscreen = false;
        window
            .set_fullscreen(false)
            .expect("Failed to set window fullscreen");
        window
            .set_size(window_state.windowed_width, window_state.windowed_height)
            .expect("Failed to reset window size after leaving fullscreen");

        log::trace!("Successfully went windowed");
        log::trace!(
            "Window Size: {}x{}",
            window_state.current_width,
            window_state.current_height,
        );
    }

    ///
    /// Internal function for getting new mouse state from SDL2
    ///
    pub(crate) fn update_state(window: &mut sdl3::video::Window, window_state: &mut WindowState) {
        let drawable_size = window.size_in_pixels();
        window_state.drawable_width = drawable_size.0;
        window_state.drawable_height = drawable_size.1;
    }
}

impl IWindow for Window {
    fn resized(&self) -> bool {
        self.resized.swap(false, Ordering::Relaxed)
    }

    fn title(&self) -> String {
        self.state.read().title.clone()
    }

    fn set_title(&self, title: String) {
        self.requests.lock().push(WindowRequest::ChangeTitle(title));
    }

    fn width(&self) -> u32 {
        self.state.read().current_height
    }

    fn set_width(&self, width: u32) {
        self.requests.lock().push(WindowRequest::ChangeWidth(width));
    }

    fn height(&self) -> u32 {
        self.state.read().current_height
    }

    fn set_height(&self, height: u32) {
        self.requests
            .lock()
            .push(WindowRequest::ChangeHeight(height));
    }

    fn size(&self) -> (u32, u32) {
        let state = self.state.read();
        (state.current_width, state.current_height)
    }

    fn set_size(&self, width: u32, height: u32) {
        self.requests
            .lock()
            .push(WindowRequest::ChangeSize(width, height));
    }

    fn drawable_width(&self) -> u32 {
        self.state.read().drawable_width
    }

    fn drawable_height(&self) -> u32 {
        self.state.read().drawable_height
    }

    fn drawable_size(&self) -> (u32, u32) {
        let state = self.state.read();
        (state.drawable_width, state.drawable_height)
    }

    fn fullscreen(&self) -> bool {
        self.state.read().fullscreen
    }

    fn focused(&self) -> bool {
        self.state.read().focused
    }

    fn go_fullscreen(&self) {
        self.requests.lock().push(WindowRequest::GoFullscreen);
    }

    fn go_windowed(&self) {
        self.requests.lock().push(WindowRequest::GoWindowed);
    }

    fn toggle_fullscreen(&self) {
        self.requests.lock().push(WindowRequest::ToggleFullscreen);
    }

    fn current_display_scale(&self) -> f32 {
        let state = self.state.read();
        state.current_display_scale
    }

    fn current_content_scale(&self) -> f32 {
        let state = self.state.read();
        state.current_content_scale
    }

    fn metal_layer(&self) -> Option<NonNull<c_void>> {
        let state = self.state.read();
        state.metal_layer
    }

    fn events<'a>(&'a self) -> SmallBox<dyn IWindowEventsLock + 'a, S1> {
        smallbox!(WindowEventsLock(self.events.read()))
    }
}

impl HasDisplayHandle for Window {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        let handle = self.state.read().display_handle;
        unsafe { Ok(DisplayHandle::borrow_raw(handle)) }
    }
}

impl HasWindowHandle for Window {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        let handle = self.state.read().window_handle;
        unsafe { Ok(WindowHandle::borrow_raw(handle)) }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        #[cfg(any(target_os = "macos", target_os = "ios"))]
        unsafe {
            use sdl3::sys::metal::SDL_Metal_DestroyView;
            let state = self.state.write();

            match &state.window_handle {
                RawWindowHandle::UiKit(v) => {
                    SDL_Metal_DestroyView(v.ui_view.as_ptr());
                }
                RawWindowHandle::AppKit(v) => {
                    SDL_Metal_DestroyView(v.ns_view.as_ptr());
                }
                _ => {
                    panic!("Unsupported window handle type");
                }
            }
        }
    }
}

///
/// Wrapper around RwLockReadGuard and implementation of `IWindowEventsLock`
///
pub struct WindowEventsLock<'a>(RwLockReadGuard<'a, Vec<WindowEvent>>);

impl<'a> IWindowEventsLock for WindowEventsLock<'a> {
    fn events(&self) -> &[WindowEvent] {
        self.0.as_slice()
    }
}
