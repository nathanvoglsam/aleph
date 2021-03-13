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

use interfaces::any::declare_interfaces;
use interfaces::platform::{
    IKeyboard, IKeyboardEventsLock, IKeyboardStateLock, KeyCode, KeyboardEvent, ScanCode,
};
use parking_lot::{RwLock, RwLockReadGuard};

///
/// Represents the state of the keyboard this frame
///
pub struct KeyboardState {
    /// Array of boolean values that should be indexed with a `ScanCode` to see if that `ScanCode`
    /// is pressed.
    pub keys: [bool; ScanCode::MAX_VALUES],
}

///
/// The struct that provides the context for, and implements, `IKeyboard`
///
pub struct KeyboardImpl {
    /// The current state of the keyboard, as recorded at the beginning of a frame
    pub state: RwLock<KeyboardState>,

    /// The event list for the current frame
    pub events: RwLock<Vec<KeyboardEvent>>,
}

declare_interfaces!(KeyboardImpl, [IKeyboard]);

impl IKeyboard for KeyboardImpl {
    fn get_state<'a>(&'a self) -> Box<dyn IKeyboardStateLock + 'a> {
        let lock = KeyboardStateLockImpl(self.state.read());
        Box::new(lock)
    }

    fn events<'a>(&'a self) -> Box<dyn IKeyboardEventsLock + 'a> {
        let lock = KeyboardEventsLockImpl(self.events.read());
        Box::new(lock)
    }
}

///
/// Wrapper around RwLockReadGuard and implementation of `IKeyboardEventsLock`
///
pub struct KeyboardEventsLockImpl<'a>(pub RwLockReadGuard<'a, Vec<KeyboardEvent>>);

impl<'a> IKeyboardEventsLock for KeyboardEventsLockImpl<'a> {
    fn events(&self) -> &[KeyboardEvent] {
        self.0.as_slice()
    }
}

///
/// Wrapper around RwLockReadGuard and implementation of `IKeyboardStateLock`
///
pub struct KeyboardStateLockImpl<'a>(pub RwLockReadGuard<'a, KeyboardState>);

impl<'a> IKeyboardStateLock for KeyboardStateLockImpl<'a> {
    fn key_code_down(&self, _key_code: KeyCode) -> bool {
        unimplemented!()
    }

    fn scan_code_down(&self, scan_code: ScanCode) -> bool {
        self.0.keys[scan_code as usize]
    }
}
