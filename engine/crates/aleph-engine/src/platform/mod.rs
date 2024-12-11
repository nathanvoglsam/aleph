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
mod sdl_main_wrapper;
mod window;

pub use clipboard::Clipboard;
pub use events::Events;
pub use frame_timer::FrameTimer;
pub use gamepad::Gamepads;
pub use keyboard::Keyboard;
pub use mouse::Mouse;
pub use sdl_main_wrapper::intercept_main;
pub use window::Window;

use std::any::TypeId;
use std::cell::Cell;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;

use interfaces::any::{AnyArc, IAny};
use interfaces::label::make_label;
use interfaces::platform::{
    Cursor, Event, IClipboard, IEvents, IFrameTimer, IGamepads, IKeyboard, IMouse, IWindow,
    KeyboardEvent, MouseEvent, WindowEvent,
};
use interfaces::plugin::{IQuitHandle, IRegistryAccessor};
use interfaces::schedule::CoreStage;
use parking_lot::RwLockWriteGuard;
use sdl2::mouse::SystemCursor;

pub(crate) fn platform_interfaces() -> [TypeId; 7] {
    [
        TypeId::of::<dyn IClipboard>(),
        TypeId::of::<dyn IEvents>(),
        TypeId::of::<dyn IFrameTimer>(),
        TypeId::of::<dyn IGamepads>(),
        TypeId::of::<dyn IKeyboard>(),
        TypeId::of::<dyn IMouse>(),
        TypeId::of::<dyn IWindow>(),
    ]
}

use crate::platform::keyboard::KeyboardState;
use crate::platform::window::WindowState;
use crate::plugin_registry::RegistryAccessor;

#[derive(Clone)]
pub(crate) struct Objects {
    pub(crate) frame_timer: Option<AnyArc<FrameTimer>>,
    pub(crate) window: Option<AnyArc<Window>>,
    pub(crate) mouse: Option<AnyArc<Mouse>>,
    pub(crate) keyboard: Option<AnyArc<Keyboard>>,
    pub(crate) gamepads: Option<AnyArc<Gamepads>>,
    pub(crate) events: Option<AnyArc<Events>>,
    pub(crate) clipboard: Option<AnyArc<Clipboard>>,
}

pub(crate) struct PlatformSDL2 {
    sdl: Rc<Cell<Option<SdlObjects>>>,
}

impl PlatformSDL2 {
    pub(crate) fn new() -> Self {
        let sdl = SdlObjects {
            _ctx: None,
            video: None,
            _event: None,
            event_pump: None,
            mouse_util: None,
            timer: None,
            window: None,
            joystick: None,
            gamecontroller: None,
        };
        Self {
            sdl: Rc::new(Cell::new(Some(sdl))),
        }
    }
}

impl PlatformSDL2 {
    pub(crate) fn on_init(&mut self, registry: &mut RegistryAccessor) {
        let quit_handle = registry.quit_handle.clone();
        ctrlc::set_handler(move || {
            println!();
            quit_handle.quit()
        })
        .expect("Failed to registr ctrl+c handler");

        #[cfg(windows)]
        {
            sdl2::hint::set("SDL_WINDOWS_DPI_SCALING", "1");
            // TODO: oh boy
            // SDL2's DPI scaling stuff on windows is _problematic_. When using the "DPI_SCALING"
            // mode SDL2 will apply all the scaling into logical points inside the backend before
            // giving them to us. That's fine, but unfortunately this is all done with integers.
            // Guess what happens when fractional scaling is turned on? If you bet truncated output
            // you'd be _very_ correct!!!!
            //
            // SDL3 is better and uses floats for the positions so you can get fractional positions
            // and scales. We _can_ work around this ourselves by using this alternative hint.
            //
            // sdl2::hint::set("SDL_WINDOWS_DPI_AWARENESS", "permonitorv2");
            //
            // This enables DPI awareness the same as the DPI_SCALING hint but doesn't apply any
            // scaling to the numbers we get from SDL. We have to do this ourselves, but critically
            // we get to convert to float and do the scaling right with fractional values.
            //
            // This can cause a few problems
            // - Artificially restricted input resolution as it will snap to integer mouse
            //   positions.
            // - Egui breaks as it can't cope with the rounding errors introduced by SDL2's scaling.
            //
            // Solving this should be possible by just looking at SDL2 and doing all the same
            // scaling but in floats. Mouse positions etc are fine but I'm not so sure about window
            // coordinates.
        }

        log::info!("Initializing SDL2 Library");
        let sdl = sdl2::init().expect("Failed to initialize SDL2");

        log::info!("Initializing SDL2 Timer Subsystem");
        let sdl_timer = sdl
            .timer()
            .expect("Failed to initialize SDL2 timer subsystem");

        log::info!("Initializing SDL2 Video Subsystem");
        let sdl_video = sdl
            .video()
            .expect("Failed to initialize SDL2 video subsystem");

        log::info!("Initializing SDL2 Event Subsystem");
        let sdl_event = sdl
            .event()
            .expect("Failed to initialize SDL2 event subsystem");

        log::info!("Initializing SDL2 Event Pump");
        let sdl_event_pump = sdl
            .event_pump()
            .expect("Failed to initialize SDL2 event pump");

        log::info!("Initializing SDL2 Joystick Subsystem");
        let sdl_joystick = sdl
            .joystick()
            .expect("Failed to initialize SDL2 joystick subsystem");

        log::info!("Initializing SDL2 Game Controller Subsystem");
        let sdl_gamecontroller = sdl
            .game_controller()
            .expect("Failed to initialize SDL2 controller subsystem");

        let sdl_mouse_util = sdl.mouse();

        // TODO: handle cursors better on platforms that lack cursors
        let cursors = if !cfg!(target_os = "ios") {
            init_cursor_map()
        } else {
            HashMap::new()
        };

        // Initialize all our implementations
        let frame_timer = FrameTimer::new(&sdl_timer);
        let mouse = Mouse::new();
        let (window, sdl_window) = Window::new(&sdl_video, "test");
        let keyboard = Keyboard::new();
        let gamepads = Gamepads::new();
        let events = Events::new();
        let clipboard = Clipboard::new();

        // Update our SDL2 handle storages with the created handles
        let mut sdl_o = self.sdl.take().unwrap();
        sdl_o._ctx = Some(sdl);
        sdl_o.video = Some(sdl_video);
        sdl_o._event = Some(sdl_event);
        sdl_o.event_pump = Some(sdl_event_pump);
        sdl_o.mouse_util = Some(sdl_mouse_util);
        sdl_o.timer = Some(sdl_timer);
        sdl_o.window = Some(sdl_window);
        sdl_o.joystick = Some(sdl_joystick);
        sdl_o.gamecontroller = Some(sdl_gamecontroller);
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
                make_label!("platform_sdl2::input_collection"),
                move || {
                    let objects = &send_objects;
                    let sdl_cell = send_sdl.deref();
                    let quit_handle = send_quit_handle.deref();

                    let mut sdl = sdl_cell.take().unwrap();
                    {
                        let timer = sdl.timer.take().unwrap();
                        Self::frame_timer(objects).unwrap().update(&timer);
                        sdl.timer = Some(timer);

                        Self::handle_requests(&cursors, &mut sdl, objects);
                        Self::handle_events(&mut sdl, objects, quit_handle);
                    }
                    sdl_cell.set(Some(sdl));
                },
            );

        objects.frame_timer.clone().inspect(|v| {
            registry.interfaces.insert(
                TypeId::of::<dyn IFrameTimer>(),
                AnyArc::map::<dyn IAny, _>(v.clone(), |v| v),
            );
        });
        objects.window.clone().inspect(|v| {
            registry.interfaces.insert(
                TypeId::of::<dyn IWindow>(),
                AnyArc::map::<dyn IAny, _>(v.clone(), |v| v),
            );
        });
        objects.mouse.clone().inspect(|v| {
            registry.interfaces.insert(
                TypeId::of::<dyn IMouse>(),
                AnyArc::map::<dyn IAny, _>(v.clone(), |v| v),
            );
        });
        objects.keyboard.clone().inspect(|v| {
            registry.interfaces.insert(
                TypeId::of::<dyn IKeyboard>(),
                AnyArc::map::<dyn IAny, _>(v.clone(), |v| v),
            );
        });
        objects.gamepads.clone().inspect(|v| {
            registry.interfaces.insert(
                TypeId::of::<dyn IGamepads>(),
                AnyArc::map::<dyn IAny, _>(v.clone(), |v| v),
            );
        });
        objects.events.clone().inspect(|v| {
            registry.interfaces.insert(
                TypeId::of::<dyn IEvents>(),
                AnyArc::map::<dyn IAny, _>(v.clone(), |v| v),
            );
        });
        objects.clipboard.clone().inspect(|v| {
            registry.interfaces.insert(
                TypeId::of::<dyn IClipboard>(),
                AnyArc::map::<dyn IAny, _>(v.clone(), |v| v),
            );
        });
    }

    pub(crate) fn on_shutdown(&mut self) {
        let mut sdl = self.sdl.take().unwrap();
        sdl.on_shutdown();
    }
}

impl PlatformSDL2 {
    fn handle_requests(
        cursors: &HashMap<Cursor, sdl2::mouse::Cursor>,
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
        use sdl2::event::Event as SdlEvent;

        let video_ctx = sdl.video.take().unwrap();
        let mut event_pump = sdl.event_pump.take().unwrap();
        let mut sdl_window = sdl.window.take().unwrap();
        let controller = sdl.gamecontroller.take().unwrap();
        let joystick = sdl.joystick.take().unwrap();

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
                        &video_ctx,
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
                    gamepads.process_gamepad_event(
                        &mut gamepads_map,
                        &joystick,
                        &controller,
                        &event,
                    );
                    gamepad_events.push(event);
                }
                _ => {}
            }
        }

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

        sdl.video = Some(video_ctx);
        sdl.event_pump = Some(event_pump);
        sdl.window = Some(sdl_window);
        sdl.gamecontroller = Some(controller);
        sdl.joystick = Some(joystick);
    }
}

impl PlatformSDL2 {
    fn window_state(provider: &Objects) -> Option<RwLockWriteGuard<WindowState>> {
        provider.window.as_ref().map(|v| v.state.write())
    }

    fn window_events(provider: &Objects) -> Option<RwLockWriteGuard<Vec<WindowEvent>>> {
        provider.window.as_ref().map(|v| v.events.write())
    }

    fn keyboard_state(provider: &Objects) -> Option<RwLockWriteGuard<KeyboardState>> {
        provider.keyboard.as_ref().map(|v| v.state.write())
    }

    fn keyboard_events(provider: &Objects) -> Option<RwLockWriteGuard<Vec<KeyboardEvent>>> {
        provider.keyboard.as_ref().map(|v| v.events.write())
    }

    fn mouse_events(provider: &Objects) -> Option<RwLockWriteGuard<Vec<MouseEvent>>> {
        provider.mouse.as_ref().map(|v| v.events.write())
    }

    fn all_events(provider: &Objects) -> Option<RwLockWriteGuard<Vec<Event>>> {
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

fn init_cursor_map() -> HashMap<Cursor, sdl2::mouse::Cursor> {
    let mut cursors = HashMap::new();
    cursors.insert(
        Cursor::Arrow,
        sdl2::mouse::Cursor::from_system(SystemCursor::Arrow).unwrap(),
    );
    cursors.insert(
        Cursor::IBeam,
        sdl2::mouse::Cursor::from_system(SystemCursor::IBeam).unwrap(),
    );
    cursors.insert(
        Cursor::SizeAll,
        sdl2::mouse::Cursor::from_system(SystemCursor::SizeAll).unwrap(),
    );
    cursors.insert(
        Cursor::SizeNS,
        sdl2::mouse::Cursor::from_system(SystemCursor::SizeNS).unwrap(),
    );
    cursors.insert(
        Cursor::SizeWE,
        sdl2::mouse::Cursor::from_system(SystemCursor::SizeWE).unwrap(),
    );
    cursors.insert(
        Cursor::SizeNESW,
        sdl2::mouse::Cursor::from_system(SystemCursor::SizeNESW).unwrap(),
    );
    cursors.insert(
        Cursor::SizeNWSE,
        sdl2::mouse::Cursor::from_system(SystemCursor::SizeNWSE).unwrap(),
    );
    cursors.insert(
        Cursor::Hand,
        sdl2::mouse::Cursor::from_system(SystemCursor::Hand).unwrap(),
    );
    cursors.insert(
        Cursor::No,
        sdl2::mouse::Cursor::from_system(SystemCursor::No).unwrap(),
    );
    cursors
}

struct SdlObjects {
    _ctx: Option<sdl2::Sdl>,
    video: Option<sdl2::VideoSubsystem>,
    _event: Option<sdl2::EventSubsystem>,
    event_pump: Option<sdl2::EventPump>,
    mouse_util: Option<sdl2::mouse::MouseUtil>,
    timer: Option<sdl2::TimerSubsystem>,
    window: Option<sdl2::video::Window>,
    joystick: Option<sdl2::JoystickSubsystem>,
    gamecontroller: Option<sdl2::GameControllerSubsystem>,
}

impl SdlObjects {
    pub fn on_shutdown(&mut self) {
        self._ctx = None;
        self.video = None;
        self._event = None;
        self.event_pump = None;
        self.mouse_util = None;
        self.timer = None;
        self.window = None;
        self.joystick = None;
        self.gamecontroller = None;
    }
}
