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

use once_cell::sync::Lazy;
use parking_lot::RwLock;
use std::ops::Deref;

///
/// The global mouse state
///
pub static KEYBOARD_STATE: Lazy<RwLock<Option<KeyboardState>>> = Lazy::new(|| RwLock::new(None));

///
/// The global mouse events
///
pub static KEYBOARD_EVENTS: Lazy<RwLock<Option<Vec<KeyboardEvent>>>> =
    Lazy::new(|| RwLock::new(None));

///
/// Type alias for the sdl2 Keycode type
///
pub type Keycode = sdl2::keyboard::Keycode;

///
/// Type alias for the sdl2 Scancode type
///
pub type Scancode = sdl2::keyboard::Scancode;

///
/// Type alias for the sdl2 Mod type
///
pub type Mod = sdl2::keyboard::Mod;

pub struct KeyDownEvent {
    pub keycode: Keycode,
    pub scancode: Scancode,
    pub keymod: Mod,
    pub repeat: bool,
}

pub struct KeyUpEvent {
    pub keycode: Keycode,
    pub scancode: Scancode,
    pub keymod: Mod,
    pub repeat: bool,
}

pub struct TextInputEvent {
    pub text: String,
}

///
/// A mouse event
///
pub enum KeyboardEvent {
    KeyDown(KeyDownEvent),
    KeyUp(KeyUpEvent),
    TextInput(TextInputEvent),
}

///
/// Represents the state of the mouse this frame
///
pub struct KeyboardState {
    keys: [bool; 1024],
}

impl KeyboardState {
    pub fn keycode_down(&self, keycode: Keycode) -> bool {
        if let Some(scancode) = sdl2::keyboard::Scancode::from_keycode(keycode) {
            self.scancode_down(scancode)
        } else {
            false
        }
    }

    pub fn scancode_down(&self, scancode: Scancode) -> bool {
        self.keys[scancode as usize]
    }

    fn set_keycode(&mut self, scancode: Scancode, val: bool) {
        self.keys[scancode as usize] = val;
    }
}

///
/// A wrapper around a read guard on the underlying RwLock used to make the global keyboard events
/// list thread safe.
///
/// # Warning
///
/// Do not try and hold onto this between frames, it will deadlock the engine.
///
pub struct KeyboardStateLock {
    lock: parking_lot::RwLockReadGuard<'static, Option<KeyboardState>>,
}

impl Deref for KeyboardStateLock {
    type Target = KeyboardState;

    fn deref(&self) -> &Self::Target {
        self.lock.as_ref().unwrap()
    }
}

///
/// A wrapper around a read guard on the underlying RwLock used to make the global mouse events list
/// thread safe.
///
/// # Warning
///
/// Do not try and hold onto this between frames, it will deadlock the engine.
///
pub struct KeyboardEvents {
    lock: parking_lot::RwLockReadGuard<'static, Option<Vec<KeyboardEvent>>>,
}

impl Deref for KeyboardEvents {
    type Target = [KeyboardEvent];

    fn deref(&self) -> &Self::Target {
        self.lock.as_ref().unwrap().as_slice()
    }
}

///
/// A "namespace" struct similar to the `Engine` struct that is used to encapsulate the global mouse
/// state.
///
/// # Implementation Details
///
/// See the documentation for `Window`. This struct follows the same paradigm of queueing changes
/// for next frame as `Window`
///
pub struct Keyboard {}

impl Keyboard {
    ///
    /// Internal function for initializing the global mouse state
    ///
    pub(crate) fn init() {
        aleph_log::trace!("Initializing the Keyboard system");
        let keyboard_state = KeyboardState {
            keys: [false; 1024],
        };

        *KEYBOARD_STATE.write() = Some(keyboard_state);
        *KEYBOARD_EVENTS.write() = Some(Vec::new());
    }

    ///
    /// Internal function for handling the events produced by the OS
    ///
    pub(crate) fn process_keyboard_event(
        keyboard_events: &mut Vec<KeyboardEvent>,
        keyboard_state: &mut KeyboardState,
        event: sdl2::event::Event,
    ) {
        match event {
            sdl2::event::Event::KeyDown {
                keycode,
                scancode,
                keymod,
                repeat,
                ..
            } => {
                let event = KeyDownEvent {
                    keycode: keycode.unwrap(),
                    scancode: scancode.unwrap(),
                    keymod,
                    repeat,
                };
                keyboard_state.set_keycode(event.scancode, true);
                keyboard_events.push(KeyboardEvent::KeyDown(event));
            }
            sdl2::event::Event::KeyUp {
                keycode,
                scancode,
                keymod,
                repeat,
                ..
            } => {
                let event = KeyUpEvent {
                    keycode: keycode.unwrap(),
                    scancode: scancode.unwrap(),
                    keymod,
                    repeat,
                };
                keyboard_state.set_keycode(event.scancode, false);
                keyboard_events.push(KeyboardEvent::KeyUp(event));
            }
            sdl2::event::Event::TextInput { text, .. } => {
                let event = TextInputEvent { text };
                keyboard_events.push(KeyboardEvent::TextInput(event));
            }
            _ => {}
        }
    }

    ///
    /// Get the current state of the mouse, last updated at the beginning of the frame
    ///
    pub fn get_state() -> KeyboardStateLock {
        let keyboard_lock = KEYBOARD_STATE.read();
        KeyboardStateLock {
            lock: keyboard_lock,
        }
    }

    ///
    /// Get read only access to this frame's list of mouse events.
    ///
    /// # Warning
    ///
    /// This will lock a global RwLock so trying to hold on to this between frames will deadlock the
    /// engine.
    ///
    pub fn events() -> KeyboardEvents {
        let lock = KEYBOARD_EVENTS.read();
        KeyboardEvents { lock }
    }
}
