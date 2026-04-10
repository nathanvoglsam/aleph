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

mod clipboard;
mod events;
mod frame_timer;
mod gamepad;
mod keyboard;
mod mouse;
mod sdl_alloc_wrapper;
mod window;

use std::cell::Cell;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;

use aleph_alloc::instrumentation::Instrumented;
use api::label::make_label;
use api::platform::{
    AClipboard, AEvents, AFrameTimer, AGamepads, AKeyboard, AMouse, AWindow, Cursor, Event,
    KeyboardEvent, MouseEvent, WindowEvent,
};
use api::plugin::{
    IPlugin, IPluginRegistrar, IQuitHandle, IRegistryAccessor, PluginDescription, Provides,
};
use api::schedule::CoreStage;
pub use clipboard::Clipboard;
pub use events::Events;
pub use frame_timer::FrameTimer;
pub use gamepad::Gamepads;
pub use keyboard::Keyboard;
pub use mouse::Mouse;
use parking_lot::RwLockWriteGuard;
pub use sdl_alloc_wrapper::set_memory_functions;
use sdl3::mouse::SystemCursor;
pub use window::Window;

use crate::core::platform::keyboard::KeyboardState;
use crate::core::platform::window::WindowState;

#[derive(Clone)]
pub(crate) struct Objects {
    pub(crate) frame_timer: Option<Arc<FrameTimer>>,
    pub(crate) window: Option<Arc<Window>>,
    pub(crate) mouse: Option<Arc<Mouse>>,
    pub(crate) keyboard: Option<Arc<Keyboard>>,
    pub(crate) gamepads: Option<Arc<Gamepads>>,
    pub(crate) events: Option<Arc<Events>>,
    pub(crate) clipboard: Option<Arc<Clipboard>>,
}

pub(crate) struct CorePlatform {
    sdl: Rc<Cell<Option<SdlObjects>>>,
}

impl CorePlatform {
    pub(crate) fn new() -> Self {
        let sdl = SdlObjects {
            _ctx: None,
            video: None,
            _event: None,
            event_pump: None,
            mouse_util: None,
            window: None,
            joystick: None,
            gamepad: None,
        };
        Self {
            sdl: Rc::new(Cell::new(Some(sdl))),
        }
    }
}

impl IPlugin for CorePlatform {
    fn get_description(&self) -> PluginDescription {
        PluginDescription {
            name: "CorePlatform".to_string(),
            description: "Provides the basic platform interfaces".to_string(),
            major_version: 1,
            minor_version: 0,
            patch_version: 0,
        }
    }

    fn register(&mut self, registrar: &mut dyn IPluginRegistrar) {
        registrar.provides::<AFrameTimer>(Provides::Always);
        registrar.provides::<AWindow>(Provides::Always);
        registrar.provides::<AMouse>(Provides::Always);
        registrar.provides::<AKeyboard>(Provides::Always);
        registrar.provides::<AGamepads>(Provides::Always);
        registrar.provides::<AEvents>(Provides::Always);
        registrar.provides::<AClipboard>(Provides::Always);
    }

    fn on_init(&mut self, registry: &mut dyn IRegistryAccessor) {
        log::info!("Initializing SDL3 Library");
        let sdl = sdl3::init().expect("Failed to initialize SDL3");

        log::info!("Initializing SDL3 Video Subsystem");
        let sdl_video = sdl
            .video()
            .expect("Failed to initialize SDL3 video subsystem");

        log::info!("Initializing SDL3 Event Subsystem");
        let sdl_event = sdl
            .event()
            .expect("Failed to initialize SDL3 event subsystem");

        log::info!("Initializing SDL3 Event Pump");
        let sdl_event_pump = sdl
            .event_pump()
            .expect("Failed to initialize SDL3 event pump");

        log::info!("Initializing SDL3 Joystick Subsystem");
        let sdl_joystick = sdl
            .joystick()
            .expect("Failed to initialize SDL3 joystick subsystem");

        log::info!("Initializing SDL3 Gamepad Subsystem");
        let sdl_gamepad = sdl
            .gamepad()
            .expect("Failed to initialize SDL3 controller subsystem");

        let sdl_mouse_util = sdl.mouse();

        // TODO: handle cursors better on platforms that lack cursors
        let cursors = if !cfg!(target_os = "ios") {
            init_cursor_map()
        } else {
            HashMap::new()
        };

        // Initialize all our implementations
        let frame_timer = FrameTimer::new();
        let mouse = Mouse::new();
        let (window, sdl_window) = Window::new(&sdl_video, "test");
        let keyboard = Keyboard::new();
        let gamepads = Gamepads::new();
        let events = Events::new();
        let clipboard = unsafe {
            // Safety: We always call this on the main thread currently.
            //         However, we don't guarantee it based on the interface. Might want to fix?
            Clipboard::new()
        };

        // Update our SDL3 handle storages with the created handles
        let mut sdl_o = self.sdl.take().unwrap();
        sdl_o._ctx = Some(sdl);
        sdl_o.video = Some(sdl_video);
        sdl_o._event = Some(sdl_event);
        sdl_o.event_pump = Some(sdl_event_pump);
        sdl_o.mouse_util = Some(sdl_mouse_util);
        sdl_o.window = Some(sdl_window);
        sdl_o.joystick = Some(sdl_joystick);
        sdl_o.gamepad = Some(sdl_gamepad);
        self.sdl.set(Some(sdl_o));

        // Update our provider with the newly created implementations
        let objects = Objects {
            frame_timer: Some(frame_timer),
            window: Some(window),
            mouse: Some(mouse),
            keyboard: Some(keyboard),
            gamepads: Some(gamepads),
            events: Some(events),
            clipboard: Some(clipboard),
        };

        let send_objects = objects.clone();
        let send_sdl = self.sdl.clone();
        let send_quit_handle = registry.quit_handle();
        registry
            .core()
            .schedule
            .add_exclusive_at_start_system_to_stage(
                CoreStage::InputCollection.into(),
                make_label!("platform_sdl3::input_collection"),
                move || {
                    let objects = &send_objects;
                    let sdl_cell = send_sdl.deref();
                    let quit_handle = send_quit_handle.deref();

                    let mut sdl = sdl_cell.take().unwrap();
                    {
                        Self::frame_timer(objects).unwrap().update();

                        Self::handle_requests(&cursors, &mut sdl, objects);
                        Self::handle_events(&mut sdl, objects, quit_handle);
                    }
                    sdl_cell.set(Some(sdl));
                },
            );

        objects.frame_timer.inspect(|v| {
            registry.provide(AFrameTimer(v.clone()));
        });
        objects.window.inspect(|v| {
            registry.provide(AWindow(v.clone()));
        });
        objects.mouse.inspect(|v| {
            registry.provide(AMouse(v.clone()));
        });
        objects.keyboard.inspect(|v| {
            registry.provide(AKeyboard(v.clone()));
        });
        objects.gamepads.inspect(|v| {
            registry.provide(AGamepads(v.clone()));
        });
        objects.events.inspect(|v| {
            registry.provide(AEvents(v.clone()));
        });
        objects.clipboard.inspect(|v| {
            registry.provide(AClipboard(v.clone()));
        });
    }

    fn on_shutdown(&mut self) {
        let mut sdl = self.sdl.take().unwrap();
        sdl.on_shutdown();
    }
}

impl CorePlatform {
    fn handle_requests(
        cursors: &HashMap<Cursor, sdl3::mouse::Cursor>,
        sdl: &mut SdlObjects,
        provider: &Objects,
    ) {
        let mut window = sdl.window.take().unwrap();
        let mouse_utils = sdl.mouse_util.take().unwrap();
        let mut window_state = Self::window_state(provider).unwrap();

        Self::mouse(provider)
            .unwrap()
            .process_mouse_requests(&window, &mouse_utils, cursors);
        Self::window(provider)
            .unwrap()
            .process_window_requests(&mut window, &mut window_state);

        drop(window_state);
        sdl.mouse_util = Some(mouse_utils);
        sdl.window = Some(window);
    }

    fn handle_events(sdl: &mut SdlObjects, provider: &Objects, quit_handle: &dyn IQuitHandle) {
        use sdl3::event::Event as SdlEvent;

        let mut event_pump = sdl.event_pump.take().unwrap();
        let mut sdl_window = sdl.window.take().unwrap();
        let sdl_gamepad = sdl.gamepad.take().unwrap();

        let sdl_display = sdl_window.get_display().expect("Failed to get display");

        let window = Self::window(provider).unwrap();
        let mouse = Self::mouse(provider).unwrap();
        let keyboard = Self::keyboard(provider).unwrap();
        let gamepads = Self::gamepads(provider).unwrap();

        let mut window_state = Self::window_state(provider).unwrap();
        let mut window_events = Self::window_events(provider).unwrap();
        let mut keyboard_state = Self::keyboard_state(provider).unwrap();
        let mut keyboard_events = Self::keyboard_events(provider).unwrap();
        let mut mouse_events = Self::mouse_events(provider).unwrap();
        let mut all_events = Self::all_events(provider).unwrap();

        let mut gamepads_map = gamepads.gamepads.borrow_mut();
        let mut gamepad_events = Vec::new();

        // Clear the events buffers of last frames events
        window_events.clear();
        mouse_events.clear();
        keyboard_events.clear();
        all_events.clear();

        // Clear the event pump and delegate the events to their event handlers
        for event in event_pump.poll_iter() {
            match event {
                SdlEvent::Quit { .. } => {
                    log::info!("Quit Event Received");
                    quit_handle.quit();
                }
                SdlEvent::Window { win_event, .. } => {
                    window.process_window_event(
                        &mut window_state,
                        &mut window_events,
                        &mut all_events,
                        win_event,
                    );
                }
                event @ SdlEvent::MouseButtonDown { .. }
                | event @ SdlEvent::MouseButtonUp { .. }
                | event @ SdlEvent::MouseMotion { .. }
                | event @ SdlEvent::MouseWheel { .. } => {
                    mouse.process_mouse_event(&mut mouse_events, &mut all_events, event);
                }
                event @ SdlEvent::KeyDown { .. }
                | event @ SdlEvent::KeyUp { .. }
                | event @ SdlEvent::TextInput { .. } => {
                    keyboard.process_keyboard_event(
                        &mut keyboard_events,
                        &mut keyboard_state,
                        &mut all_events,
                        event,
                    );
                }
                event @ SdlEvent::ControllerDeviceAdded { .. }
                | event @ SdlEvent::ControllerDeviceRemoved { .. }
                | event @ SdlEvent::ControllerDeviceRemapped { .. }
                | event @ SdlEvent::ControllerAxisMotion { .. }
                | event @ SdlEvent::ControllerButtonDown { .. }
                | event @ SdlEvent::ControllerButtonUp { .. } => {
                    gamepads.process_gamepad_event(&mut gamepads_map, &sdl_gamepad, &event);
                    gamepad_events.push(event);
                }
                _ => {}
            }
        }

        window_state.current_display_scale = sdl_window.display_scale();
        window_state.current_content_scale = sdl_display.get_content_scale().unwrap_or(1.0);

        // Update the mouse's state from the fresh sequence of events
        mouse.update_state(&event_pump);

        // Publish the active controller's state now that we've flushed the event pump
        gamepads.publish_active_state(&gamepads_map, &gamepad_events);

        Window::update_state(&mut sdl_window, &mut window_state);

        drop(window_state);
        drop(window_events);
        drop(keyboard_state);
        drop(keyboard_events);
        drop(mouse_events);
        drop(all_events);

        drop(gamepads_map);

        sdl.event_pump = Some(event_pump);
        sdl.window = Some(sdl_window);
        sdl.gamepad = Some(sdl_gamepad);
    }
}

impl CorePlatform {
    fn window_state(provider: &Objects) -> Option<RwLockWriteGuard<'_, WindowState>> {
        provider.window.as_ref().map(|v| v.state.write())
    }

    fn window_events(provider: &Objects) -> Option<RwLockWriteGuard<'_, Vec<WindowEvent>>> {
        provider.window.as_ref().map(|v| v.events.write())
    }

    fn keyboard_state(provider: &Objects) -> Option<RwLockWriteGuard<'_, KeyboardState>> {
        provider.keyboard.as_ref().map(|v| v.state.write())
    }

    fn keyboard_events(provider: &Objects) -> Option<RwLockWriteGuard<'_, Vec<KeyboardEvent>>> {
        provider.keyboard.as_ref().map(|v| v.events.write())
    }

    fn mouse_events(provider: &Objects) -> Option<RwLockWriteGuard<'_, Vec<MouseEvent>>> {
        provider.mouse.as_ref().map(|v| v.events.write())
    }

    fn all_events(provider: &Objects) -> Option<RwLockWriteGuard<'_, Vec<Event>>> {
        provider.events.as_ref().map(|v| v.deref().0.write())
    }

    fn mouse(provider: &Objects) -> Option<&Mouse> {
        provider.mouse.as_deref()
    }

    fn window(provider: &Objects) -> Option<&Window> {
        provider.window.as_deref()
    }

    fn keyboard(provider: &Objects) -> Option<&Keyboard> {
        provider.keyboard.as_deref()
    }

    fn gamepads(provider: &Objects) -> Option<&Gamepads> {
        provider.gamepads.as_deref()
    }

    fn frame_timer(provider: &Objects) -> Option<&FrameTimer> {
        provider.frame_timer.as_deref()
    }
}

fn init_cursor_map() -> HashMap<Cursor, sdl3::mouse::Cursor> {
    let mut cursors = HashMap::new();
    cursors.insert(
        Cursor::Arrow,
        sdl3::mouse::Cursor::from_system(SystemCursor::Arrow).unwrap(),
    );
    cursors.insert(
        Cursor::IBeam,
        sdl3::mouse::Cursor::from_system(SystemCursor::IBeam).unwrap(),
    );
    cursors.insert(
        Cursor::SizeAll,
        sdl3::mouse::Cursor::from_system(SystemCursor::SizeAll).unwrap(),
    );
    cursors.insert(
        Cursor::SizeNS,
        sdl3::mouse::Cursor::from_system(SystemCursor::SizeNS).unwrap(),
    );
    cursors.insert(
        Cursor::SizeWE,
        sdl3::mouse::Cursor::from_system(SystemCursor::SizeWE).unwrap(),
    );
    cursors.insert(
        Cursor::SizeNESW,
        sdl3::mouse::Cursor::from_system(SystemCursor::SizeNESW).unwrap(),
    );
    cursors.insert(
        Cursor::SizeNWSE,
        sdl3::mouse::Cursor::from_system(SystemCursor::SizeNWSE).unwrap(),
    );
    cursors.insert(
        Cursor::Hand,
        sdl3::mouse::Cursor::from_system(SystemCursor::Hand).unwrap(),
    );
    cursors.insert(
        Cursor::No,
        sdl3::mouse::Cursor::from_system(SystemCursor::No).unwrap(),
    );
    cursors
}

struct SdlObjects {
    _ctx: Option<sdl3::Sdl>,
    video: Option<sdl3::VideoSubsystem>,
    _event: Option<sdl3::EventSubsystem>,
    event_pump: Option<sdl3::EventPump>,
    mouse_util: Option<sdl3::mouse::MouseUtil>,
    window: Option<sdl3::video::Window>,
    joystick: Option<sdl3::JoystickSubsystem>,
    gamepad: Option<sdl3::GamepadSubsystem>,
}

impl SdlObjects {
    pub fn on_shutdown(&mut self) {
        self._ctx = None;
        self.video = None;
        self._event = None;
        self.event_pump = None;
        self.mouse_util = None;
        self.window = None;
        self.joystick = None;
        self.gamepad = None;
    }
}

pub struct Sdl;
aleph_alloc::new_alloc_category!(Sdl, "01993166-455d-7e71-9a81-022a9b388514");

pub type SdlSystem = Instrumented<Sdl>;
