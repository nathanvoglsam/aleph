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

use interfaces::any::{declare_interfaces, AnyArc};
use interfaces::platform::{
    Event, HasRawWindowHandle, IWindow, IWindowEventsLock, RawWindowHandle, WindowEvent,
};
use parking_lot::{Mutex, RwLock, RwLockReadGuard};
use std::sync::atomic::{AtomicBool, Ordering};

///
/// Does what it sends on the tin, holds the most recently collected state of the window. For more
/// info regarding "recently collected" see the documentation for `Window`
///
pub struct WindowState {
    /// The title/text in the window header
    pub title: String,

    /// The current width of the window on the desktop
    pub current_width: u32,

    /// The current height of the window on the desktop
    pub current_height: u32,

    /// The current width of the drawable surface
    pub drawable_width: u32,

    /// The current height of the drawable surface
    pub drawable_height: u32,

    /// The width of the window when not fullscreen
    pub windowed_width: u32,

    /// The height of the window when not fullscreen
    pub windowed_height: u32,

    /// Whether the window is currently fullscreen
    pub fullscreen: bool,

    /// Is the window currently focused
    pub focused: bool,

    /// Is the DPI of the window based on the display it is currently on
    pub current_dpi: f32,

    /// The window's window handle
    pub handle: RawWindowHandle,
}

unsafe impl Send for WindowState {}
unsafe impl Sync for WindowState {}

///
/// Represents the set of possible state change requests the window can perform
///
pub enum WindowRequest {
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
pub struct WindowImpl {
    /// The window state, as recorded at the beginning of a frame
    pub state: RwLock<WindowState>,

    /// The event list for the current frame
    pub events: RwLock<Vec<WindowEvent>>,

    /// The request queue that will be flushed and handled at the beginning of the frame after a
    /// request is made
    pub requests: Mutex<Vec<WindowRequest>>,

    /// A flag that is used to check whether the window has been resized
    pub resized: AtomicBool,
}

declare_interfaces!(WindowImpl, [IWindow]);

impl WindowImpl {
    ///
    /// Internal function for initializing the engine window
    ///
    pub fn new(
        video_ctx: &sdl2::VideoSubsystem,
        title: &str,
    ) -> (AnyArc<Self>, sdl2::video::Window) {
        const DEFAULT_WIDTH: u32 = 1280;
        const DEFAULT_HEIGHT: u32 = 800;

        let mut window = video_ctx.window(title, DEFAULT_WIDTH, DEFAULT_HEIGHT);
        window.resizable();
        window.allow_highdpi();

        // Android can only use Vulkan, and we need to set this flag to *use* Vulkan so I think this
        // is the best way to handle it. There shouldn't be any coupling between the RHI and'
        // windowing ideally but in practice this works and keeps the rest of the engine clean.
        //
        // It's not like you can use D3D12 on Android anyway.
        #[cfg(target_os = "android")]
        {
            window.vulkan();
        }

        #[cfg(target_os = "macos")]
        {
            use sdl2_sys::SDL_WindowFlags::SDL_WINDOW_METAL;
            // Add the SDL_WINDOW_METAL flag on macos as we need it for a graphics context
            window.set_window_flags(window.window_flags() | SDL_WINDOW_METAL as u32);
        }

        let mut window = window.build().expect("Failed to create window");

        let display_index = window.display_index().unwrap();
        let (_ddpi, hdpi, _vdpi) = video_ctx.display_dpi(display_index).unwrap();

        let drawable_size = window.drawable_size();

        let raw_window_handle = {
            #[cfg(not(target_os = "macos"))]
            {
                window.raw_window_handle()
            }

            #[cfg(target_os = "macos")]
            unsafe {
                use sdl2_sys::SDL_Metal_CreateView;

                let mut raw_window_handle = window.raw_window_handle();
                if let RawWindowHandle::AppKit(v) = &mut raw_window_handle {
                    v.ns_view = SDL_Metal_CreateView(window.raw());
                } else {
                    panic!("We only support MacOS window handles, not iOS");
                }
                raw_window_handle
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
            current_dpi: hdpi,
            handle: raw_window_handle,
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
    pub fn process_window_event(
        &self,
        video_ctx: &sdl2::VideoSubsystem,
        window_state: &mut WindowState,
        window_events: &mut Vec<WindowEvent>,
        all_events: &mut Vec<Event>,
        event: sdl2::event::WindowEvent,
    ) {
        match &event {
            sdl2::event::WindowEvent::Resized(width, height) => {
                self.resized.store(true, Ordering::Relaxed);
                window_state.current_width = *width as u32;
                window_state.current_height = *height as u32;
                log::trace!("Window resized by OS");
                log::trace!("Window Size: {}x{}", width, height);
            }
            sdl2::event::WindowEvent::FocusGained => {
                window_state.focused = true;
            }
            sdl2::event::WindowEvent::FocusLost => {
                window_state.focused = false;
            }
            sdl2::event::WindowEvent::SizeChanged(width, height) => {
                self.resized.store(true, Ordering::Relaxed);
                window_state.current_width = *width as u32;
                window_state.current_height = *height as u32;
                log::trace!("Window size changed");
                log::trace!("Window Size: {}x{}", width, height);
            }
            sdl2::event::WindowEvent::DisplayChanged(i) => {
                let (_ddpi, hdpi, _vdpi) = video_ctx.display_dpi(*i).unwrap();
                window_state.current_dpi = hdpi;
                log::trace!("Window display changed");
                log::trace!("Window Display: {} at {}dpi", i, hdpi);
            }
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
            | sdl2::event::WindowEvent::Exposed
            | sdl2::event::WindowEvent::ICCProfChanged => {},
        }

        let converted_event = match event {
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
            sdl2::event::WindowEvent::None => return,
            sdl2::event::WindowEvent::ICCProfChanged => return,
            sdl2::event::WindowEvent::DisplayChanged(_) => return,
        };
        window_events.push(converted_event.clone());
        all_events.push(Event::WindowEvent(converted_event));
    }

    ///
    /// Internal function for handling internal reactions to window events then pushing them to the
    /// event queue
    ///
    pub fn process_window_requests(
        &self,
        window: &mut sdl2::video::Window,
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

    fn handle_go_fullscreen(window: &mut sdl2::video::Window, window_state: &mut WindowState) {
        log::trace!("Attempting to go fullscreen");
        window_state.windowed_width = window_state.current_width;
        window_state.windowed_height = window_state.current_height;

        window
            .set_fullscreen(sdl2::video::FullscreenType::True)
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

    fn handle_go_windowed(window: &mut sdl2::video::Window, window_state: &mut WindowState) {
        log::trace!("Attempting to go windowed mode");
        window_state.current_width = window_state.windowed_width;
        window_state.current_height = window_state.windowed_height;

        window_state.fullscreen = false;
        window
            .set_fullscreen(sdl2::video::FullscreenType::Off)
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
    pub fn update_state(window: &mut sdl2::video::Window, window_state: &mut WindowState) {
        let drawable_size = window.vulkan_drawable_size();
        window_state.drawable_width = drawable_size.0;
        window_state.drawable_height = drawable_size.1;
    }
}

impl IWindow for WindowImpl {
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

    fn current_dpi(&self) -> f32 {
        self.state.read().current_dpi
    }

    fn current_display_scale(&self) -> f32 {
        let current_dpi = self.current_dpi();
        if cfg!(target_vendor="apple") {
            current_dpi / 72.0
        } else {
            current_dpi / 96.0
        }
    }

    fn events<'a>(&'a self) -> Box<dyn IWindowEventsLock + 'a> {
        let lock = WindowEventsLockImpl(self.events.read());
        Box::new(lock)
    }
}

unsafe impl HasRawWindowHandle for WindowImpl {
    fn raw_window_handle(&self) -> RawWindowHandle {
        self.state.read().handle
    }
}

impl Drop for WindowImpl {
    fn drop(&mut self) {
        #[cfg(target_os = "macos")]
        unsafe {
            use sdl2_sys::SDL_Metal_DestroyView;
            let state = self.state.write();

            if let RawWindowHandle::AppKit(v) = &state.handle {
                SDL_Metal_DestroyView(v.ns_view);
            } else {
                panic!("We only support MacOS window handles, not iOS");
            }
        }
    }
}

///
/// Wrapper around RwLockReadGuard and implementation of `IWindowEventsLock`
///
pub struct WindowEventsLockImpl<'a>(RwLockReadGuard<'a, Vec<WindowEvent>>);

impl<'a> IWindowEventsLock for WindowEventsLockImpl<'a> {
    fn events(&self) -> &[WindowEvent] {
        self.0.as_slice()
    }
}
