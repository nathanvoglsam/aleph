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

use crate::keyboard::KeyboardEvent;
use crate::mouse::MouseEvent;
use crate::window::WindowEvent;
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use std::ops::Deref;

///
/// The events list
///
pub static ALL_EVENTS: Lazy<RwLock<Option<Vec<Event>>>> = Lazy::new(|| RwLock::new(None));

/// This represents the set of all types of events
#[derive(Clone)]
pub enum Event {
    KeyboardEvent(KeyboardEvent),
    MouseEvent(MouseEvent),
    WindowEvent(WindowEvent),
}

///
/// A wrapper around a read guard on the underlying RwLock used to make the global mouse events list
/// thread safe.
///
/// # Warning
///
/// Do not try and hold onto this between frames, it will deadlock the engine.
///
pub struct Events {
    lock: parking_lot::RwLockReadGuard<'static, Option<Vec<Event>>>,
}

impl Deref for Events {
    type Target = [Event];

    fn deref(&self) -> &Self::Target {
        self.lock.as_ref().unwrap().as_slice()
    }
}

impl Events {
    ///
    /// Internal function for initializing the global events state
    ///
    pub(crate) fn init() {
        aleph_log::trace!("Initializing the All Events system");
        *ALL_EVENTS.write() = Some(Vec::new());
    }

    ///
    /// Get read only access to this frame's list of all events.
    ///
    /// # Warning
    ///
    /// This will lock a global RwLock so trying to hold on to this between frames will deadlock the
    /// engine.
    ///
    pub fn get() -> Self {
        let lock = ALL_EVENTS.read();
        Events { lock }
    }
}
