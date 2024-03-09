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

use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

use interfaces::any::{declare_interfaces, AnyArc};
use interfaces::platform::{
    Cursor, Event, IMouse, IMouseEventsLock, MouseButton, MouseButtonDownEvent, MouseButtonUpEvent,
    MouseEvent, MouseMotionEvent, MouseState, MouseWheelDirection, MouseWheelEvent,
};
use parking_lot::{Mutex, RwLock, RwLockReadGuard};

///
/// Internal enum for representing a mouse request
///
pub enum MouseRequest {
    SetPos(i32, i32),
    SetCursor(Cursor),
    ShowCursor,
    HideCursor,
}

///
/// The struct that provides the context for, and implements, `IKeyboard`
///
pub struct MouseImpl {
    /// The current state of the keyboard, as recorded at the beginning of a frame
    pub state: RwLock<MouseState>,

    /// The event list for the current frame
    pub events: RwLock<Vec<MouseEvent>>,

    /// The request queue that will be flushed and handled at the beginning of the frame after a
    /// request is made
    pub requests: Mutex<Vec<MouseRequest>>,
}

declare_interfaces!(MouseImpl, [IMouse]);

impl MouseImpl {
    pub fn new() -> AnyArc<Self> {
        let out = Self {
            state: RwLock::new(MouseState {
                pos: (0, 0),
                buttons: 0,
            }),
            events: RwLock::new(Vec::new()),
            requests: Mutex::new(Vec::new()),
        };
        AnyArc::new(out)
    }

    ///
    /// Internal function for handling requests made in the last frame
    ///
    pub fn process_mouse_requests(
        &self,
        window: &sdl2::video::Window,
        mouse_utils: &sdl2::mouse::MouseUtil,
        cursors: &HashMap<Cursor, sdl2::mouse::Cursor>,
    ) {
        for request in self.requests.lock().drain(..) {
            match request {
                MouseRequest::SetPos(x, y) => {
                    log::trace!("Attempting to set new mouse position");
                    mouse_utils.warp_mouse_in_window(window, x, y);
                    log::trace!("Moved mouse to : {}, {}", x, y);
                }
                MouseRequest::SetCursor(cursor) => {
                    let cursor = cursors.get(&cursor).unwrap();
                    cursor.set();
                }
                MouseRequest::ShowCursor => {
                    mouse_utils.show_cursor(true);
                }
                MouseRequest::HideCursor => {
                    mouse_utils.show_cursor(false);
                }
            }
        }
    }

    ///
    /// Internal function for handling the events produced by the OS
    ///
    pub fn process_mouse_event(
        &self,
        mouse_events: &mut Vec<MouseEvent>,
        all_events: &mut Vec<Event>,
        event: sdl2::event::Event,
    ) {
        match event {
            sdl2::event::Event::MouseMotion {
                mousestate,
                x,
                y,
                xrel,
                yrel,
                ..
            } => {
                let event = MouseMotionEvent {
                    mouse_state: MouseState {
                        pos: (mousestate.x(), mousestate.y()),
                        buttons: mousestate.to_sdl_state(),
                    },
                    x,
                    y,
                    x_rel: xrel,
                    y_rel: yrel,
                };
                let event = MouseEvent::MouseMotion(event);
                mouse_events.push(event.clone());
                all_events.push(Event::MouseEvent(event));
            }
            sdl2::event::Event::MouseButtonDown {
                mouse_btn,
                clicks,
                x,
                y,
                ..
            } => {
                let button = match mouse_btn {
                    sdl2::mouse::MouseButton::Unknown => return,
                    sdl2::mouse::MouseButton::Left => MouseButton::Left,
                    sdl2::mouse::MouseButton::Middle => MouseButton::Middle,
                    sdl2::mouse::MouseButton::Right => MouseButton::Right,
                    sdl2::mouse::MouseButton::X1 => MouseButton::X1,
                    sdl2::mouse::MouseButton::X2 => MouseButton::X2,
                };
                let event = MouseButtonDownEvent {
                    button,
                    clicks,
                    x,
                    y,
                };
                let event = MouseEvent::MouseButtonDown(event);
                mouse_events.push(event.clone());
                all_events.push(Event::MouseEvent(event));
            }
            sdl2::event::Event::MouseButtonUp {
                mouse_btn,
                clicks,
                x,
                y,
                ..
            } => {
                let button = match mouse_btn {
                    sdl2::mouse::MouseButton::Unknown => return,
                    sdl2::mouse::MouseButton::Left => MouseButton::Left,
                    sdl2::mouse::MouseButton::Middle => MouseButton::Middle,
                    sdl2::mouse::MouseButton::Right => MouseButton::Right,
                    sdl2::mouse::MouseButton::X1 => MouseButton::X1,
                    sdl2::mouse::MouseButton::X2 => MouseButton::X2,
                };
                let event = MouseButtonUpEvent {
                    button,
                    clicks,
                    x,
                    y,
                };
                let event = MouseEvent::MouseButtonUp(event);
                mouse_events.push(event.clone());
                all_events.push(Event::MouseEvent(event));
            }
            sdl2::event::Event::MouseWheel {
                x, y, direction, ..
            } => {
                let direction = match direction {
                    sdl2::mouse::MouseWheelDirection::Normal => MouseWheelDirection::Normal,
                    sdl2::mouse::MouseWheelDirection::Flipped => MouseWheelDirection::Flipped,
                    sdl2::mouse::MouseWheelDirection::Unknown(_) => return,
                };
                let event = MouseWheelEvent { x, y, direction };
                let event = MouseEvent::MouseWheel(event);
                mouse_events.push(event.clone());
                all_events.push(Event::MouseEvent(event));
            }
            _ => {}
        }
    }

    ///
    /// Internal function for getting new mouse state from SDL2
    ///
    pub fn update_state(&self, event_pump: &sdl2::EventPump) {
        let state = sdl2::mouse::MouseState::new(event_pump);

        *self.state.write().deref_mut() = MouseState {
            pos: (state.x(), state.y()),
            buttons: state.to_sdl_state(),
        };
    }
}

impl IMouse for MouseImpl {
    fn get_state(&self) -> MouseState {
        let lock = self.state.read();
        lock.deref().clone()
    }

    fn set_pos(&self, x: i32, y: i32) {
        self.requests.lock().push(MouseRequest::SetPos(x, y));
    }

    fn set_cursor(&self, cursor: Cursor) {
        self.requests.lock().push(MouseRequest::SetCursor(cursor));
    }

    fn show_cursor(&self) {
        self.requests.lock().push(MouseRequest::ShowCursor);
    }

    fn hide_cursor(&self) {
        self.requests.lock().push(MouseRequest::HideCursor);
    }

    fn events<'a>(&'a self) -> Box<dyn IMouseEventsLock + 'a> {
        let lock = MouseEventsLockImpl(self.events.read());
        Box::new(lock)
    }
}

///
/// Wrapper around RwLockReadGuard and implementation of `IKeyboardEventsLock`
///
pub struct MouseEventsLockImpl<'a>(pub RwLockReadGuard<'a, Vec<MouseEvent>>);

impl<'a> IMouseEventsLock for MouseEventsLockImpl<'a> {
    fn events(&self) -> &[MouseEvent] {
        self.0.as_slice()
    }
}
