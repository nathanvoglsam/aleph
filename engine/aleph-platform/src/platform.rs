//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use crate::frame_timer::FrameTimer;
use crate::keyboard::{Keyboard, KEYBOARD_EVENTS, KEYBOARD_STATE};
use crate::mouse::{Mouse, MOUSE_EVENTS};
use crate::window::{Window, WINDOW_EVENTS, WINDOW_STATE};
use aleph_settings::WindowSettings;
use sdl2::event::Event;
use std::cell::Cell;

pub enum PlatformBuildError {
    ///
    /// The SDL2 library failed to initialize outright. The reason produced by the sdl2 library is
    /// bundled with this variant.
    ///
    FailedToInitSDL2(String),

    ///
    /// The SDL2 library failed to initialize the video subsystem. The reason produced by the sdl2
    /// library is bundled with this variant.
    ///
    FailedToInitVideo(String),

    ///
    /// The SDL2 library failed to initialize the event subsystem. The reason produced by the sdl2
    /// library is bundled with this variant.
    ///
    FailedToInitEvent(String),

    ///
    /// The SDL2 library failed to initialize the event pump. The reason produced by the sdl2
    /// library is bundled with this variant.
    ///
    FailedToInitEventPump(String),

    ///
    /// The SDL2 library failed to initialize the timer subsystem. The reason produced by the sdl2
    /// library is bundled with this variant.
    ///
    FailedToInitTimer(String),
}

///
/// Struct that handles initializing the engine's windowing and input systems
///
pub struct PlatformBuilder {
    headless: bool,
}

impl PlatformBuilder {
    pub fn new() -> Self {
        Self { headless: false }
    }

    ///
    /// Set if the platform object should be created for a headless environment. This is useful for
    /// hosting the engine on a server that lacks a GPU or windowing system.
    ///
    /// A headless `Platform` will not initialize or handle anything to do with windowing or user
    /// input from a keyboard or mouse.
    ///
    pub fn headless(mut self, headless: bool) -> Self {
        self.headless = headless;
        self
    }

    ///
    /// Consumes the builder and constructs a new platform object
    ///
    pub fn build(self) -> Result<Platform, PlatformBuildError> {
        log::trace!("Initializing SDL2 Library");
        let sdl = sdl2::init().map_err(|v| PlatformBuildError::FailedToInitSDL2(v))?;

        log::trace!("Initializing SDL2 Event Subsystem");
        let event = sdl
            .event()
            .map_err(|v| PlatformBuildError::FailedToInitEvent(v))?;

        log::trace!("Initializing SDL2 Event Pump");
        let event_pump = sdl
            .event_pump()
            .map_err(|v| PlatformBuildError::FailedToInitEventPump(v))?;

        log::trace!("Initializing SDL2 Timer Subsystem");
        let timer = sdl
            .timer()
            .map_err(|v| PlatformBuildError::FailedToInitTimer(v))?;

        FrameTimer::init(&timer);

        // If we aren't running headless, init input handling, video system and create a window
        let (video, mouse_util, window) = if !self.headless {
            log::trace!("Initializing SDL2 Video Subsystem");
            let video = sdl
                .video()
                .map_err(|v| PlatformBuildError::FailedToInitVideo(v))?;

            log::trace!("Initializing SDL2 Mouse Util");
            let mouse_util = sdl.mouse();

            log::trace!("Initializing Window");
            let window = Window::init_window(&video, "ASD", &WindowSettings::default());

            Keyboard::init();
            Mouse::init();

            (Some(video), Some(mouse_util), Some(window))
        } else {
            (None, None, None)
        };

        let platform = Platform {
            headless: self.headless,
            sdl,
            video: Cell::new(video),
            event: Cell::new(Some(event)),
            event_pump: Cell::new(Some(event_pump)),
            mouse_util: Cell::new(mouse_util),
            timer: Cell::new(Some(timer)),
            window: Cell::new(window),
        };

        Ok(platform)
    }
}

///
///
///
pub struct Platform {
    headless: bool,
    sdl: sdl2::Sdl,
    video: Cell<Option<sdl2::VideoSubsystem>>,
    event: Cell<Option<sdl2::EventSubsystem>>,
    event_pump: Cell<Option<sdl2::EventPump>>,
    mouse_util: Cell<Option<sdl2::mouse::MouseUtil>>,
    timer: Cell<Option<sdl2::TimerSubsystem>>,
    window: Cell<Option<sdl2::video::Window>>,
}

impl Platform {
    ///
    /// Gets a builder for creating a new platform instance
    ///
    pub fn builder() -> PlatformBuilder {
        PlatformBuilder::new()
    }

    ///
    /// Gets whether the platform is running headless
    ///
    pub fn is_headless(&self) -> bool {
        self.headless
    }

    ///
    /// Updates anything that needs to be updated at the absolute very beginning of each frame.
    ///
    /// Currently this updates the FrameTimer with fresh timer values
    ///
    pub fn frame(&mut self) {
        // Get the timer subsystem
        let timer = self.timer.take().unwrap();

        // Mark a new frame for the frame timer
        FrameTimer::frame(&timer);

        // Put the timer back in its cell
        self.timer.set(Some(timer));
    }

    ///
    /// Processes any of the requests made of the platform systems from the previous frame
    ///
    pub fn process_requests(&mut self) {
        // This is a no-op in headless mode as a headless game instance can't get any window or
        // input requests as the systems aren't active
        if !self.headless {
            // Get access to window state
            let mut window_state_lock = WINDOW_STATE.write();
            let window_state = window_state_lock.as_mut().unwrap();

            let mut window = self.window.take().unwrap();
            let mouse_utils = self.mouse_util.take().unwrap();

            //imgui_ctx.update_mouse_pos_early();

            Mouse::process_mouse_requests(&window, &mouse_utils);
            Window::process_window_requests(&mut window, window_state);

            //drop(window_state_lock);
            //drop(keyboard_state_lock);
            //imgui_ctx.update_mouse_pos_late();
            //imgui_ctx.update_keyboard_input();

            self.mouse_util.set(Some(mouse_utils));
            self.window.set(Some(window));
        }
    }

    ///
    /// Processes the new events from the platform (window, input, etc) and update the state objects
    /// to propagate the changes
    ///
    pub fn process_events(&mut self) {
        // Get the event pump
        let mut event_pump = self.event_pump.take().unwrap();

        // When not running headless we need to handle window, keyboard and mouse events. If we're
        // running headless we only need to listen for a quit event
        if !self.headless {
            // Window state an events
            let mut window_state_lock = WINDOW_STATE.write();
            let window_state = window_state_lock.as_mut().unwrap();
            let mut window_events_lock = WINDOW_EVENTS.write();
            let window_events = window_events_lock.as_mut().unwrap();

            // Keyboard state an events
            let mut keyboard_state_lock = KEYBOARD_STATE.write();
            let keyboard_state = keyboard_state_lock.as_mut().unwrap();
            let mut keyboard_events_lock = KEYBOARD_EVENTS.write();
            let keyboard_events = keyboard_events_lock.as_mut().unwrap();

            // Mouse events
            let mut mouse_events_lock = MOUSE_EVENTS.write();
            let mouse_events = mouse_events_lock.as_mut().unwrap();

            // Clear the events buffers of last frames events
            window_events.clear();
            mouse_events.clear();
            keyboard_events.clear();

            // Clear the event pump and delegate the events to their event handlers
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => {
                        log::info!("Quit Event Received");
                        // TODO: Propagate quit out of the crate
                    }
                    Event::Window { win_event, .. } => {
                        Window::process_window_event(window_state, window_events, win_event);
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

            // Update the mouse's state from the fresh sequence of events
            Mouse::update_state(&event_pump);
        } else {
            // Clear the event pump and delegate the events to their event handlers
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => {
                        log::info!("Quit Event Received");
                        // TODO: Propagate quit out of the crate
                    }
                    _ => {}
                }
            }
        }

        // Return the event pump to its cell
        self.event_pump.set(Some(event_pump));
    }
}
