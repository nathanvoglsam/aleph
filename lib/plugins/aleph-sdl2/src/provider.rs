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

use interfaces::any::AnyArc;
use interfaces::platform::{
    IClipboard, IClipboardProvider, IEvents, IEventsProvider, IFrameTimer, IFrameTimerProvider,
    IKeyboard, IKeyboardProvider, IMouse, IMouseProvider, IWindow, IWindowProvider,
};

use crate::clipboard::ClipboardImpl;
use crate::events::EventsImpl;
use crate::frame_timer::FrameTimerImpl;
use crate::keyboard::KeyboardImpl;
use crate::mouse::MouseImpl;
use crate::window::WindowImpl;

pub struct ProviderImpl {
    pub frame_timer: Option<AnyArc<FrameTimerImpl>>,
    pub window: Option<AnyArc<WindowImpl>>,
    pub mouse: Option<AnyArc<MouseImpl>>,
    pub keyboard: Option<AnyArc<KeyboardImpl>>,
    pub events: Option<AnyArc<EventsImpl>>,
    pub clipboard: Option<AnyArc<ClipboardImpl>>,
}

impl IFrameTimerProvider for ProviderImpl {
    fn get_frame_timer(&self) -> Option<AnyArc<dyn IFrameTimer>> {
        self.frame_timer.as_ref().map(|v| {
            let v = v.clone();
            AnyArc::map::<dyn IFrameTimer, _>(v, |v| v)
        })
    }
}

impl IWindowProvider for ProviderImpl {
    fn get_window(&self) -> Option<AnyArc<dyn IWindow>> {
        self.window.as_ref().map(|v| {
            let v = v.clone();
            AnyArc::map::<dyn IWindow, _>(v, |v| v)
        })
    }
}

impl IClipboardProvider for ProviderImpl {
    fn get_clipboard(&self) -> Option<AnyArc<dyn IClipboard>> {
        self.clipboard.as_ref().map(|v| {
            let v = v.clone();
            AnyArc::map::<dyn IClipboard, _>(v, |v| v)
        })
    }
}

impl IKeyboardProvider for ProviderImpl {
    fn get_keyboard(&self) -> Option<AnyArc<dyn IKeyboard>> {
        self.keyboard.as_ref().map(|v| {
            let v = v.clone();
            AnyArc::map::<dyn IKeyboard, _>(v, |v| v)
        })
    }
}

impl IMouseProvider for ProviderImpl {
    fn get_mouse(&self) -> Option<AnyArc<dyn IMouse>> {
        self.mouse.as_ref().map(|v| {
            let v = v.clone();
            AnyArc::map::<dyn IMouse, _>(v, |v| v)
        })
    }
}

impl IEventsProvider for ProviderImpl {
    fn get_events(&self) -> Option<AnyArc<dyn IEvents>> {
        self.events.as_ref().map(|v| {
            let v = v.clone();
            AnyArc::map::<dyn IEvents, _>(v, |v| v)
        })
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
