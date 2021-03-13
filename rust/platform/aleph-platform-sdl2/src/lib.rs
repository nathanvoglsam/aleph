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

extern crate aleph_interfaces as interfaces;

mod clipboard;
mod events;
mod frame_timer;
mod keyboard;
mod mouse;
mod window;

use crate::v2::frame_timer::FrameTimerImpl;
use interfaces::any::AnyArc;
use interfaces::platform::{
    Cursor, IClipboard, IClipboardProvider, IEvents, IEventsProvider, IFrameTimer,
    IFrameTimerProvider, IKeyboard, IKeyboardProvider, IMouse, IMouseProvider, IWindow,
    IWindowProvider,
};
use interfaces::plugin::stages::IMainInitStage;
use interfaces::plugin::{
    IInitResponse, IInterfaces, IPlugin, IPluginRegistrar, PluginDescription,
};
use sdl2::mouse::SystemCursor;
use std::any::TypeId;
use std::collections::HashMap;

pub struct PlatformSDL2 {
    sdl: Option<sdl2::Sdl>,
    video: Option<sdl2::VideoSubsystem>,
    event: Option<sdl2::EventSubsystem>,
    event_pump: Option<sdl2::EventPump>,
    mouse_util: Option<sdl2::mouse::MouseUtil>,
    timer: Option<sdl2::TimerSubsystem>,
    window: Option<sdl2::video::Window>,
    cursors: HashMap<Cursor, sdl2::mouse::Cursor>,
    provider: AnyArc<ProviderImpl>,
}

impl IPlugin for PlatformSDL2 {
    fn get_description(&self) -> PluginDescription {
        PluginDescription {
            name: "PlatformSDL2".to_string(),
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
        registrar.must_init_after::<dyn IMainInitStage>();
    }

    fn on_init(&mut self, _interfaces: &dyn IInterfaces) -> Box<dyn IInitResponse> {
        aleph_log::trace!("Initializing SDL2 Library");
        let sdl = sdl2::init().expect("Failed to initialize SDL2");

        aleph_log::trace!("Initializing SDL2 Event Subsystem");
        let event = sdl
            .event()
            .expect("Failed to initialize SDL2 event subsystem");

        aleph_log::trace!("Initializing SDL2 Event Pump");
        let event_pump = sdl
            .event_pump()
            .expect("Failed to initialize SDL2 event pump");

        aleph_log::trace!("Initializing SDL2 Timer Subsystem");
        let timer = sdl
            .timer()
            .expect("Failed to initialize SDL2 timer subsystem");

        let frame_timer = frame_timer::FrameTimerImpl::new(&timer);
        let frame_timer = AnyArc::new(frame_timer);

        // If we aren't running headless, init input handling, video system and create a window
        aleph_log::trace!("Initializing SDL2 Video Subsystem");
        let video = sdl
            .video()
            .expect("Failed to initialize SDL2 video subsystem");

        aleph_log::trace!("Initializing SDL2 Mouse Util");
        let mouse_util = sdl.mouse();

        aleph_log::trace!("Initializing Window");
        let window = Window::init_window(&video, &self.app_info.name, &self.settings.window);

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

        self.sdl = Some(sdl);
        self.video = Some(video);
        self.event = Some(event);
        self.event_pump = Some(event_pump);
        self.mouse_util = Some(mouse_util);
        self.timer = Some(timer);
        self.window = Some(window);
        self.cursors = cursors;
        self.provider.frame_timer = Some(frame_timer.clone());

        let response = vec![(
            TypeId::of::<dyn IFrameTimerProvider>(),
            self.provider.query_interface().unwrap(),
        )];
        Box::new(response)
    }

    fn on_update(&mut self, interfaces: &dyn IInterfaces) {
        unimplemented!()
    }

    fn on_exit(&mut self, interfaces: &dyn IInterfaces) {
        unimplemented!()
    }
}

interfaces::any::declare_interfaces!(PlatformSDL2, [IPlugin]);

pub struct ProviderImpl {
    pub frame_timer: Option<AnyArc<FrameTimerImpl>>,
}

impl IFrameTimerProvider for ProviderImpl {
    fn get_frame_timer(&self) -> Option<AnyArc<dyn IFrameTimer>> {
        self.frame_timer.map(|v| v.query_interface()).flatten()
    }
}

impl IWindowProvider for ProviderImpl {
    fn get_window(&self) -> Option<AnyArc<dyn IWindow>> {
        unimplemented!()
    }
}

impl IClipboardProvider for ProviderImpl {
    fn get_clipboard(&self) -> Option<AnyArc<dyn IClipboard>> {
        unimplemented!()
    }
}

impl IKeyboardProvider for ProviderImpl {
    fn get_keyboard(&self) -> Option<AnyArc<dyn IKeyboard>> {
        unimplemented!()
    }
}

impl IMouseProvider for ProviderImpl {
    fn get_mouse(&self) -> Option<AnyArc<dyn IMouse>> {
        unimplemented!()
    }
}

impl IEventsProvider for ProviderImpl {
    fn get_events(&self) -> Option<AnyArc<dyn IEvents>> {
        unimplemented!()
    }
}

interfaces::any::declare_interfaces!(
    ProviderImpl,
    [
        IFrameTimerProvider,
        IWindowProvider,
        IClipboardProvider,
        IKeyboardProvider,
        IMouseProvider,
        IEventsProvider
    ]
);

