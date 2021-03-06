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

use crate::clipboard::ClipboardImpl;
use crate::events::EventsImpl;
use crate::frame_timer::FrameTimerImpl;
use crate::keyboard::{KeyboardImpl, KeyboardState};
use crate::mouse::MouseImpl;
use crate::provider::ProviderImpl;
use crate::window::{WindowImpl, WindowState};
use interfaces::any::AnyArc;
use interfaces::platform::{
    Cursor, Event, IClipboardProvider, IEventsProvider, IFrameTimerProvider, IKeyboardProvider,
    IMouseProvider, IWindowProvider, KeyboardEvent, MouseEvent, WindowEvent,
};
use interfaces::plugin::{
    IInitResponse, IPlugin, IPluginRegistrar, IRegistryAccessor, PluginDescription, UpdateStage,
};
use parking_lot::RwLockWriteGuard;
use sdl2::mouse::SystemCursor;
use std::any::TypeId;
use std::collections::HashMap;
use std::ops::Deref;

pub struct PluginPlatformSDL2 {
    _sdl: Option<sdl2::Sdl>,
    _sdl_video: Option<sdl2::VideoSubsystem>,
    _sdl_event: Option<sdl2::EventSubsystem>,
    sdl_event_pump: Option<sdl2::EventPump>,
    sdl_mouse_util: Option<sdl2::mouse::MouseUtil>,
    sdl_timer: Option<sdl2::TimerSubsystem>,
    sdl_window: Option<sdl2::video::Window>,
    sdl_main_ctx: crate::sdl_main_wrapper::MainCtx,
    provider: AnyArc<ProviderImpl>,
    cursors: HashMap<Cursor, sdl2::mouse::Cursor>,
}

impl PluginPlatformSDL2 {
    pub fn new() -> Self {
        let sdl_main_ctx = unsafe { crate::sdl_main_wrapper::run_sdl_main() };

        Self {
            _sdl: None,
            _sdl_video: None,
            _sdl_event: None,
            sdl_event_pump: None,
            sdl_mouse_util: None,
            sdl_timer: None,
            sdl_window: None,
            sdl_main_ctx,
            provider: AnyArc::new(ProviderImpl {
                frame_timer: None,
                window: None,
                mouse: None,
                keyboard: None,
                events: None,
                clipboard: None,
            }),
            cursors: Default::default(),
        }
    }
}

impl Drop for PluginPlatformSDL2 {
    fn drop(&mut self) {
        self._sdl = None;
        self._sdl_video = None;
        self._sdl_event = None;
        self.sdl_event_pump = None;
        self.sdl_mouse_util = None;
        self.sdl_timer = None;
        self.sdl_window = None;

        unsafe { crate::sdl_main_wrapper::run_sdl_exit(&self.sdl_main_ctx) }
    }
}

impl IPlugin for PluginPlatformSDL2 {
    fn get_description(&self) -> PluginDescription {
        PluginDescription {
            name: "PluginPlatformSDL2".to_string(),
            description: "A platform abstraction layer implemented with SDL2".to_string(),
            major_version: 0,
            minor_version: 1,
            patch_version: 0,
        }
    }

    fn register(&mut self, registrar: &mut dyn IPluginRegistrar) {
        registrar.provides_interface::<dyn IFrameTimerProvider>();
        registrar.provides_interface::<dyn IWindowProvider>();
        registrar.provides_interface::<dyn IClipboardProvider>();
        registrar.provides_interface::<dyn IKeyboardProvider>();
        registrar.provides_interface::<dyn IMouseProvider>();
        registrar.provides_interface::<dyn IEventsProvider>();

        registrar.update_stage(UpdateStage::InputCollection);
    }

    fn on_init(&mut self, registry: &dyn IRegistryAccessor) -> Box<dyn IInitResponse> {
        let quit_handle = registry.quit_handle();
        ctrlc::set_handler(move || {
            println!();
            quit_handle.quit()
        })
        .expect("Failed to registr ctrl+c handler");

        aleph_log::trace!("Initializing SDL2 Library");
        let sdl = sdl2::init().expect("Failed to initialize SDL2");

        aleph_log::trace!("Initializing SDL2 Timer Subsystem");
        let sdl_timer = sdl
            .timer()
            .expect("Failed to initialize SDL2 timer subsystem");

        aleph_log::trace!("Initializing SDL2 Video Subsystem");
        let sdl_video = sdl
            .video()
            .expect("Failed to initialize SDL2 video subsystem");

        aleph_log::trace!("Initializing SDL2 Event Subsystem");
        let sdl_event = sdl
            .event()
            .expect("Failed to initialize SDL2 event subsystem");

        aleph_log::trace!("Initializing SDL2 Event Pump");
        let sdl_event_pump = sdl
            .event_pump()
            .expect("Failed to initialize SDL2 event pump");

        let sdl_mouse_util = sdl.mouse();
        self.cursors = init_cursor_map();

        // Initialize all our implementations
        let frame_timer = FrameTimerImpl::new(&sdl_timer);
        let mouse = MouseImpl::new();
        let (window, sdl_window) = WindowImpl::new(&sdl_video, "test");
        let keyboard = KeyboardImpl::new();
        let events = EventsImpl::new();
        let clipboard = ClipboardImpl::new();

        // Update our SDL2 handle storages with the created handles
        self._sdl = Some(sdl);
        self._sdl_video = Some(sdl_video);
        self._sdl_event = Some(sdl_event);
        self.sdl_event_pump = Some(sdl_event_pump);
        self.sdl_mouse_util = Some(sdl_mouse_util);
        self.sdl_timer = Some(sdl_timer);
        self.sdl_window = Some(sdl_window);

        // Update our provider with the newly created implementations
        let provider = AnyArc::get_mut(&mut self.provider).unwrap();
        provider.frame_timer = Some(frame_timer);
        provider.window = Some(window);
        provider.mouse = Some(mouse);
        provider.keyboard = Some(keyboard);
        provider.events = Some(events);
        provider.clipboard = Some(clipboard);

        // Provide our declared implementations to the plugin registry
        let response = vec![
            (
                TypeId::of::<dyn IFrameTimerProvider>(),
                AnyArc::into_any(self.provider.clone()),
            ),
            (
                TypeId::of::<dyn IWindowProvider>(),
                AnyArc::into_any(self.provider.clone()),
            ),
            (
                TypeId::of::<dyn IClipboardProvider>(),
                AnyArc::into_any(self.provider.clone()),
            ),
            (
                TypeId::of::<dyn IKeyboardProvider>(),
                AnyArc::into_any(self.provider.clone()),
            ),
            (
                TypeId::of::<dyn IMouseProvider>(),
                AnyArc::into_any(self.provider.clone()),
            ),
            (
                TypeId::of::<dyn IEventsProvider>(),
                AnyArc::into_any(self.provider.clone()),
            ),
        ];
        Box::new(response)
    }

    fn on_input_collection(&mut self, registry: &dyn IRegistryAccessor) {
        let timer = self.sdl_timer.take().unwrap();
        self.frame_timer().unwrap().update(&timer);
        self.sdl_timer = Some(timer);

        self.handle_requests(registry);
        self.handle_events(registry);
    }
}

impl PluginPlatformSDL2 {
    fn handle_requests(&mut self, _registry: &dyn IRegistryAccessor) {
        let mut window = self.sdl_window.take().unwrap();
        let mouse_utils = self.sdl_mouse_util.take().unwrap();
        let mut window_state = self.window_state().unwrap();

        self.mouse()
            .unwrap()
            .process_mouse_requests(&window, &mouse_utils, &self.cursors);
        self.window()
            .unwrap()
            .process_window_requests(&mut window, &mut window_state);

        drop(window_state);
        self.sdl_mouse_util = Some(mouse_utils);
        self.sdl_window = Some(window);
    }

    fn handle_events(&mut self, registry: &dyn IRegistryAccessor) {
        let mut event_pump = self.sdl_event_pump.take().unwrap();
        let mut sdl_window = self.sdl_window.take().unwrap();

        let window = self.window().unwrap();
        let mouse = self.mouse().unwrap();
        let keyboard = self.keyboard().unwrap();

        let mut window_state = self.window_state().unwrap();
        let mut window_events = self.window_events().unwrap();
        let mut keyboard_state = self.keyboard_state().unwrap();
        let mut keyboard_events = self.keyboard_events().unwrap();
        let mut mouse_events = self.mouse_events().unwrap();
        let mut all_events = self.all_events().unwrap();

        // Clear the events buffers of last frames events
        window_events.clear();
        mouse_events.clear();
        keyboard_events.clear();
        all_events.clear();

        // Clear the event pump and delegate the events to their event handlers
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => {
                    aleph_log::info!("Quit Event Received");
                    registry.quit_handle().quit();
                }
                sdl2::event::Event::Window { win_event, .. } => {
                    window.process_window_event(
                        &mut window_state,
                        &mut window_events,
                        &mut all_events,
                        win_event,
                    );
                }
                event @ sdl2::event::Event::MouseButtonDown { .. } => {
                    mouse.process_mouse_event(&mut mouse_events, &mut all_events, event);
                }
                event @ sdl2::event::Event::MouseButtonUp { .. } => {
                    mouse.process_mouse_event(&mut mouse_events, &mut all_events, event);
                }
                event @ sdl2::event::Event::MouseMotion { .. } => {
                    mouse.process_mouse_event(&mut mouse_events, &mut all_events, event);
                }
                event @ sdl2::event::Event::MouseWheel { .. } => {
                    mouse.process_mouse_event(&mut mouse_events, &mut all_events, event);
                }
                event @ sdl2::event::Event::KeyDown { .. } => {
                    keyboard.process_keyboard_event(
                        &mut keyboard_events,
                        &mut keyboard_state,
                        &mut all_events,
                        event,
                    );
                }
                event @ sdl2::event::Event::KeyUp { .. } => {
                    keyboard.process_keyboard_event(
                        &mut keyboard_events,
                        &mut keyboard_state,
                        &mut all_events,
                        event,
                    );
                }
                event @ sdl2::event::Event::TextInput { .. } => {
                    keyboard.process_keyboard_event(
                        &mut keyboard_events,
                        &mut keyboard_state,
                        &mut all_events,
                        event,
                    );
                }
                _ => {}
            }
        }

        // Update the mouse's state from the fresh sequence of events
        mouse.update_state(&event_pump);

        WindowImpl::update_state(&mut sdl_window, &mut window_state);

        drop(window_state);
        drop(window_events);
        drop(keyboard_state);
        drop(keyboard_events);
        drop(mouse_events);
        drop(all_events);

        self.sdl_event_pump = Some(event_pump);
        self.sdl_window = Some(sdl_window);
    }
}

impl PluginPlatformSDL2 {
    fn window_state(&self) -> Option<RwLockWriteGuard<WindowState>> {
        self.provider.window.as_ref().map(|v| v.state.write())
    }

    fn window_events(&self) -> Option<RwLockWriteGuard<Vec<WindowEvent>>> {
        self.provider.window.as_ref().map(|v| v.events.write())
    }

    fn keyboard_state(&self) -> Option<RwLockWriteGuard<KeyboardState>> {
        self.provider.keyboard.as_ref().map(|v| v.state.write())
    }

    fn keyboard_events(&self) -> Option<RwLockWriteGuard<Vec<KeyboardEvent>>> {
        self.provider.keyboard.as_ref().map(|v| v.events.write())
    }

    fn mouse_events(&self) -> Option<RwLockWriteGuard<Vec<MouseEvent>>> {
        self.provider.mouse.as_ref().map(|v| v.events.write())
    }

    fn all_events(&self) -> Option<RwLockWriteGuard<Vec<Event>>> {
        self.provider.events.as_ref().map(|v| v.deref().0.write())
    }

    fn mouse(&self) -> Option<&MouseImpl> {
        self.provider.mouse.as_ref().map(|v| v.deref())
    }

    fn window(&self) -> Option<&WindowImpl> {
        self.provider.window.as_ref().map(|v| v.deref())
    }

    fn keyboard(&self) -> Option<&KeyboardImpl> {
        self.provider.keyboard.as_ref().map(|v| v.deref())
    }

    fn frame_timer(&self) -> Option<&FrameTimerImpl> {
        self.provider.frame_timer.as_ref().map(|v| v.deref())
    }
}

interfaces::any::declare_interfaces!(PluginPlatformSDL2, [IPlugin]);

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
