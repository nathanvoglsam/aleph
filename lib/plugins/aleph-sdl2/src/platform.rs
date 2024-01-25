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

use std::any::TypeId;
use std::cell::Cell;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;

use interfaces::any::{AnyArc, IAny};
use interfaces::make_plugin_description_for_crate;
use interfaces::platform::{
    Cursor, Event, IClipboardProvider, IEventsProvider, IFrameTimerProvider, IKeyboardProvider,
    IMouseProvider, IWindowProvider, KeyboardEvent, MouseEvent, WindowEvent,
};
use interfaces::plugin::{
    IInitResponse, IPlugin, IPluginRegistrar, IQuitHandle, IRegistryAccessor, PluginDescription,
};
use interfaces::schedule::{CoreStage, IScheduleProvider};
use parking_lot::RwLockWriteGuard;
use sdl2::mouse::SystemCursor;

use crate::clipboard::ClipboardImpl;
use crate::events::EventsImpl;
use crate::frame_timer::FrameTimerImpl;
use crate::keyboard::{KeyboardImpl, KeyboardState};
use crate::mouse::MouseImpl;
use crate::provider::ProviderImpl;
use crate::window::{WindowImpl, WindowState};

pub struct PluginPlatformSDL2 {
    sdl: Rc<Cell<Option<SdlObjects>>>,
    provider: AnyArc<ProviderImpl>,
}

impl PluginPlatformSDL2 {
    pub fn new() -> Self {
        let main_ctx = unsafe { crate::sdl_main_wrapper::run_sdl_main() };

        let sdl = SdlObjects {
            _ctx: None,
            video: None,
            _event: None,
            event_pump: None,
            mouse_util: None,
            timer: None,
            window: None,
            main_ctx,
        };
        Self {
            sdl: Rc::new(Cell::new(Some(sdl))),
            provider: AnyArc::new(ProviderImpl {
                frame_timer: None,
                window: None,
                mouse: None,
                keyboard: None,
                events: None,
                clipboard: None,
            }),
        }
    }
}

impl Default for PluginPlatformSDL2 {
    fn default() -> Self {
        Self::new()
    }
}

impl IPlugin for PluginPlatformSDL2 {
    fn get_description(&self) -> PluginDescription {
        make_plugin_description_for_crate!()
    }

    fn register(&mut self, registrar: &mut dyn IPluginRegistrar) {
        registrar.must_init_after::<dyn IScheduleProvider>();
        registrar.depends_on::<dyn IScheduleProvider>();

        registrar.provides_interface::<dyn IFrameTimerProvider>();
        registrar.provides_interface::<dyn IWindowProvider>();
        registrar.provides_interface::<dyn IClipboardProvider>();
        registrar.provides_interface::<dyn IKeyboardProvider>();
        registrar.provides_interface::<dyn IMouseProvider>();
        registrar.provides_interface::<dyn IEventsProvider>();
    }

    fn on_init(&mut self, registry: &dyn IRegistryAccessor) -> Box<dyn IInitResponse> {
        let quit_handle = registry.quit_handle();
        ctrlc::set_handler(move || {
            println!();
            quit_handle.quit()
        })
        .expect("Failed to registr ctrl+c handler");

        #[cfg(windows)]
        {
            sdl2::hint::set("SDL_WINDOWS_DPI_SCALING", "1");
        }

        log::trace!("Initializing SDL2 Library");
        let sdl = sdl2::init().expect("Failed to initialize SDL2");

        log::trace!("Initializing SDL2 Timer Subsystem");
        let sdl_timer = sdl
            .timer()
            .expect("Failed to initialize SDL2 timer subsystem");

        log::trace!("Initializing SDL2 Video Subsystem");
        let sdl_video = sdl
            .video()
            .expect("Failed to initialize SDL2 video subsystem");

        log::trace!("Initializing SDL2 Event Subsystem");
        let sdl_event = sdl
            .event()
            .expect("Failed to initialize SDL2 event subsystem");

        log::trace!("Initializing SDL2 Event Pump");
        let sdl_event_pump = sdl
            .event_pump()
            .expect("Failed to initialize SDL2 event pump");

        let sdl_mouse_util = sdl.mouse();
        let cursors = init_cursor_map();

        // Initialize all our implementations
        let frame_timer = FrameTimerImpl::new(&sdl_timer);
        let mouse = MouseImpl::new();
        let (window, sdl_window) = WindowImpl::new(&sdl_video, "test");
        let keyboard = KeyboardImpl::new();
        let events = EventsImpl::new();
        let clipboard = ClipboardImpl::new();

        // Update our SDL2 handle storages with the created handles
        let mut sdl_o = self.sdl.take().unwrap();
        sdl_o._ctx = Some(sdl);
        sdl_o.video = Some(sdl_video);
        sdl_o._event = Some(sdl_event);
        sdl_o.event_pump = Some(sdl_event_pump);
        sdl_o.mouse_util = Some(sdl_mouse_util);
        sdl_o.timer = Some(sdl_timer);
        sdl_o.window = Some(sdl_window);
        self.sdl.set(Some(sdl_o));

        // Update our provider with the newly created implementations
        {
            let provider = AnyArc::get_mut(&mut self.provider).unwrap();
            provider.frame_timer = Some(frame_timer);
            provider.window = Some(window);
            provider.mouse = Some(mouse);
            provider.keyboard = Some(keyboard);
            provider.events = Some(events);
            provider.clipboard = Some(clipboard);
        }

        let schedule_provider = registry.get_interface::<dyn IScheduleProvider>().unwrap();
        let schedule_cell = schedule_provider.get();
        let mut schedule = schedule_cell.get();

        let send_provider = self.provider.clone();
        let send_sdl = self.sdl.clone();
        let send_quit_handle = registry.quit_handle();
        schedule.add_exclusive_at_start_system_to_stage(
            &CoreStage::InputCollection,
            "platform_sdl2::input_collection",
            move || {
                let provider = send_provider.deref();
                let sdl_cell = send_sdl.deref();
                let quit_handle = send_quit_handle.deref();

                let mut sdl = sdl_cell.take().unwrap();
                {
                    let timer = sdl.timer.take().unwrap();
                    Self::frame_timer(provider).unwrap().update(&timer);
                    sdl.timer = Some(timer);

                    Self::handle_requests(&cursors, &mut sdl, provider);
                    Self::handle_events(&mut sdl, provider, quit_handle);
                }
                sdl_cell.set(Some(sdl));
            },
        );

        // Provide our declared implementations to the plugin registry
        let response = vec![
            (
                TypeId::of::<dyn IFrameTimerProvider>(),
                AnyArc::map::<dyn IAny, _>(self.provider.clone(), |v| v),
            ),
            (
                TypeId::of::<dyn IWindowProvider>(),
                AnyArc::map::<dyn IAny, _>(self.provider.clone(), |v| v),
            ),
            (
                TypeId::of::<dyn IClipboardProvider>(),
                AnyArc::map::<dyn IAny, _>(self.provider.clone(), |v| v),
            ),
            (
                TypeId::of::<dyn IKeyboardProvider>(),
                AnyArc::map::<dyn IAny, _>(self.provider.clone(), |v| v),
            ),
            (
                TypeId::of::<dyn IMouseProvider>(),
                AnyArc::map::<dyn IAny, _>(self.provider.clone(), |v| v),
            ),
            (
                TypeId::of::<dyn IEventsProvider>(),
                AnyArc::map::<dyn IAny, _>(self.provider.clone(), |v| v),
            ),
        ];
        Box::new(response)
    }

    fn on_exit(&mut self, _registry: &dyn IRegistryAccessor) {
        let mut sdl = self.sdl.take().unwrap();

        sdl._ctx = None;
        sdl.video = None;
        sdl._event = None;
        sdl.event_pump = None;
        sdl.mouse_util = None;
        sdl.timer = None;
        sdl.window = None;

        unsafe { crate::sdl_main_wrapper::run_sdl_exit(&sdl.main_ctx) }
    }
}

impl PluginPlatformSDL2 {
    fn handle_requests(
        cursors: &HashMap<Cursor, sdl2::mouse::Cursor>,
        sdl: &mut SdlObjects,
        provider: &ProviderImpl,
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

    fn handle_events(sdl: &mut SdlObjects, provider: &ProviderImpl, quit_handle: &dyn IQuitHandle) {
        let video_ctx = sdl.video.take().unwrap();
        let mut event_pump = sdl.event_pump.take().unwrap();
        let mut sdl_window = sdl.window.take().unwrap();

        let window = Self::window(provider).unwrap();
        let mouse = Self::mouse(provider).unwrap();
        let keyboard = Self::keyboard(provider).unwrap();

        let mut window_state = Self::window_state(provider).unwrap();
        let mut window_events = Self::window_events(provider).unwrap();
        let mut keyboard_state = Self::keyboard_state(provider).unwrap();
        let mut keyboard_events = Self::keyboard_events(provider).unwrap();
        let mut mouse_events = Self::mouse_events(provider).unwrap();
        let mut all_events = Self::all_events(provider).unwrap();

        // Clear the events buffers of last frames events
        window_events.clear();
        mouse_events.clear();
        keyboard_events.clear();
        all_events.clear();

        // Clear the event pump and delegate the events to their event handlers
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => {
                    log::info!("Quit Event Received");
                    quit_handle.quit();
                }
                sdl2::event::Event::Window { win_event, .. } => {
                    window.process_window_event(
                        &video_ctx,
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

        sdl.video = Some(video_ctx);
        sdl.event_pump = Some(event_pump);
        sdl.window = Some(sdl_window);
    }
}

impl PluginPlatformSDL2 {
    fn window_state(provider: &ProviderImpl) -> Option<RwLockWriteGuard<WindowState>> {
        provider.window.as_ref().map(|v| v.state.write())
    }

    fn window_events(provider: &ProviderImpl) -> Option<RwLockWriteGuard<Vec<WindowEvent>>> {
        provider.window.as_ref().map(|v| v.events.write())
    }

    fn keyboard_state(provider: &ProviderImpl) -> Option<RwLockWriteGuard<KeyboardState>> {
        provider.keyboard.as_ref().map(|v| v.state.write())
    }

    fn keyboard_events(provider: &ProviderImpl) -> Option<RwLockWriteGuard<Vec<KeyboardEvent>>> {
        provider.keyboard.as_ref().map(|v| v.events.write())
    }

    fn mouse_events(provider: &ProviderImpl) -> Option<RwLockWriteGuard<Vec<MouseEvent>>> {
        provider.mouse.as_ref().map(|v| v.events.write())
    }

    fn all_events(provider: &ProviderImpl) -> Option<RwLockWriteGuard<Vec<Event>>> {
        provider.events.as_ref().map(|v| v.deref().0.write())
    }

    fn mouse(provider: &ProviderImpl) -> Option<&MouseImpl> {
        provider.mouse.as_deref()
    }

    fn window(provider: &ProviderImpl) -> Option<&WindowImpl> {
        provider.window.as_deref()
    }

    fn keyboard(provider: &ProviderImpl) -> Option<&KeyboardImpl> {
        provider.keyboard.as_deref()
    }

    fn frame_timer(provider: &ProviderImpl) -> Option<&FrameTimerImpl> {
        provider.frame_timer.as_deref()
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

struct SdlObjects {
    _ctx: Option<sdl2::Sdl>,
    video: Option<sdl2::VideoSubsystem>,
    _event: Option<sdl2::EventSubsystem>,
    event_pump: Option<sdl2::EventPump>,
    mouse_util: Option<sdl2::mouse::MouseUtil>,
    timer: Option<sdl2::TimerSubsystem>,
    window: Option<sdl2::video::Window>,
    main_ctx: crate::sdl_main_wrapper::MainCtx,
}
