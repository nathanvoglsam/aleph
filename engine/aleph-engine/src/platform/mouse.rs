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

use interfaces::any::{AnyArc, declare_interfaces};
use interfaces::platform::{
    Cursor, Event, IMouse, IMouseEventsLock, MouseButton, MouseButtonDownEvent, MouseButtonUpEvent,
    MouseEvent, MouseMotionEvent, MouseState, MouseWheelDirection, MouseWheelEvent,
};
use parking_lot::{Mutex, RwLock, RwLockReadGuard};
use smallbox::space::S1;
use smallbox::{SmallBox, smallbox};

///
/// Internal enum for representing a mouse request
///
pub(crate) enum MouseRequest {
    SetPos(f32, f32),
    SetCursor(Cursor),
    ShowCursor,
    HideCursor,
}

///
/// The struct that provides the context for, and implements, `IKeyboard`
///
pub struct Mouse {
    /// The current state of the keyboard, as recorded at the beginning of a frame
    pub(crate) state: RwLock<MouseState>,

    /// The event list for the current frame
    pub(crate) events: RwLock<Vec<MouseEvent>>,

    /// The request queue that will be flushed and handled at the beginning of the frame after a
    /// request is made
    pub(crate) requests: Mutex<Vec<MouseRequest>>,
}

declare_interfaces!(Mouse, [IMouse]);

impl Mouse {
    pub(crate) fn new() -> AnyArc<Self> {
        let out = Self {
            state: RwLock::new(MouseState {
                pos: (0.0, 0.0),
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
    pub(crate) fn process_mouse_requests(
        &self,
        window: &sdl3::video::Window,
        mouse_utils: &sdl3::mouse::MouseUtil,
        cursors: &HashMap<Cursor, sdl3::mouse::Cursor>,
    ) {
        for request in self.requests.lock().drain(..) {
            match request {
                MouseRequest::SetPos(x, y) => {
                    log::trace!("Attempting to set new mouse position");
                    mouse_utils.warp_mouse_in_window(window, x, y);
                    log::trace!("Moved mouse to : {}, {}", x, y);
                }
                MouseRequest::SetCursor(cursor) => {
                    // Platforms with no cursor support (iOS) will not put any entries in the cursor
                    // table so we just skip setting the cursor there. Platforms that do have
                    // cursor support will always fill the table so this should always produce
                    // correct behaviour everywhere.
                    if let Some(cursor) = cursors.get(&cursor) {
                        cursor.set();
                    }
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
    pub(crate) fn process_mouse_event(
        &self,
        mouse_events: &mut Vec<MouseEvent>,
        all_events: &mut Vec<Event>,
        event: sdl3::event::Event,
    ) {
        match event {
            sdl3::event::Event::MouseMotion {
                mousestate,
                x,
                y,
                xrel,
                yrel,
                ..
            } => {
                let event = MouseMotionEvent {
                    mouse_state: MouseState {
                        pos: (mousestate.x() as f32, mousestate.y() as f32),
                        buttons: mousestate.to_sdl_state(),
                    },
                    x: x as f32,
                    y: y as f32,
                    x_rel: xrel as f32,
                    y_rel: yrel as f32,
                };
                let event = MouseEvent::MouseMotion(event);
                mouse_events.push(event.clone());
                all_events.push(Event::MouseEvent(event));
            }
            sdl3::event::Event::MouseButtonDown {
                mouse_btn,
                clicks,
                x,
                y,
                ..
            } => {
                let button = match mouse_btn {
                    sdl3::mouse::MouseButton::Unknown => return,
                    sdl3::mouse::MouseButton::Left => MouseButton::Left,
                    sdl3::mouse::MouseButton::Middle => MouseButton::Middle,
                    sdl3::mouse::MouseButton::Right => MouseButton::Right,
                    sdl3::mouse::MouseButton::X1 => MouseButton::X1,
                    sdl3::mouse::MouseButton::X2 => MouseButton::X2,
                };
                let event = MouseButtonDownEvent {
                    button,
                    clicks,
                    x: x as f32,
                    y: y as f32,
                };
                let event = MouseEvent::MouseButtonDown(event);
                mouse_events.push(event.clone());
                all_events.push(Event::MouseEvent(event));
            }
            sdl3::event::Event::MouseButtonUp {
                mouse_btn,
                clicks,
                x,
                y,
                ..
            } => {
                let button = match mouse_btn {
                    sdl3::mouse::MouseButton::Unknown => return,
                    sdl3::mouse::MouseButton::Left => MouseButton::Left,
                    sdl3::mouse::MouseButton::Middle => MouseButton::Middle,
                    sdl3::mouse::MouseButton::Right => MouseButton::Right,
                    sdl3::mouse::MouseButton::X1 => MouseButton::X1,
                    sdl3::mouse::MouseButton::X2 => MouseButton::X2,
                };
                let event = MouseButtonUpEvent {
                    button,
                    clicks,
                    x: x as f32,
                    y: y as f32,
                };
                let event = MouseEvent::MouseButtonUp(event);
                mouse_events.push(event.clone());
                all_events.push(Event::MouseEvent(event));
            }
            sdl3::event::Event::MouseWheel {
                x, y, direction, ..
            } => {
                let direction = match direction {
                    sdl3::mouse::MouseWheelDirection::Normal => MouseWheelDirection::Normal,
                    sdl3::mouse::MouseWheelDirection::Flipped => MouseWheelDirection::Flipped,
                    sdl3::mouse::MouseWheelDirection::Unknown(_) => return,
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
    pub(crate) fn update_state(&self, event_pump: &sdl3::EventPump) {
        let state = sdl3::mouse::MouseState::new(event_pump);

        *self.state.write().deref_mut() = MouseState {
            pos: (state.x() as f32, state.y() as f32),
            buttons: state.to_sdl_state(),
        };
    }
}

impl IMouse for Mouse {
    fn get_state(&self) -> MouseState {
        let lock = self.state.read();
        lock.deref().clone()
    }

    fn set_pos(&self, x: f32, y: f32) {
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

    fn events<'a>(&'a self) -> SmallBox<dyn IMouseEventsLock + 'a, S1> {
        smallbox!(MouseEventsLock(self.events.read()))
    }
}

///
/// Wrapper around RwLockReadGuard and implementation of `IKeyboardEventsLock`
///
pub struct MouseEventsLock<'a>(RwLockReadGuard<'a, Vec<MouseEvent>>);

impl<'a> IMouseEventsLock for MouseEventsLock<'a> {
    fn events(&self) -> &[MouseEvent] {
        self.0.as_slice()
    }
}
