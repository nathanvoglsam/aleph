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

use interfaces::any::{declare_interfaces, AnyArc};
use interfaces::platform::{Event, IEvents, IEventsLock};
use parking_lot::{RwLock, RwLockReadGuard};

///
/// The struct that provides the context for, and implements, `IEvents`
///
pub struct EventsImpl(pub RwLock<Vec<Event>>);

declare_interfaces!(EventsImpl, [IEvents]);

impl IEvents for EventsImpl {
    fn get<'a>(&'a self) -> Box<dyn IEventsLock + 'a> {
        let lock = EventsLockImpl(self.0.read());
        Box::new(lock)
    }
}

impl EventsImpl {
    pub fn new() -> AnyArc<Self> {
        let out = Self(RwLock::new(Vec::new()));
        AnyArc::new(out)
    }
}

///
/// Wrapper around RwLockReadGuard and implementation of `IEventsLock`
///
pub struct EventsLockImpl<'a>(pub RwLockReadGuard<'a, Vec<Event>>);

impl<'a> IEventsLock for EventsLockImpl<'a> {
    fn events(&self) -> &[Event] {
        self.0.as_slice()
    }
}
