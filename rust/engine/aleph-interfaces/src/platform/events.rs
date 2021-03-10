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

use crate::platform::{KeyboardEvent, MouseEvent, WindowEvent};
use any::*;

///
/// This interface should be used by plugins that wish to register themselves as the engine's
/// event provider. Anything that implements this should correctly handle creating and
/// destroying whatever is needed to access the system's event queue, and should be able to give out
/// an `AnyArc<IEvents>` to allow others to interface with the events system.
///
pub trait IEventsProvider: IAny {
    ///
    /// Returns an `AnyArc` that holds an `IEvents` interface.
    ///
    /// This will always return the same `IEvents` instance as `IEventsProvider` only supports
    /// handling a single events instance.
    ///
    /// A return value of `None` should signal that the functionality is not supported.
    ///
    fn get_events(&self) -> Option<AnyArc<dyn IEvents>>;
}

///
/// This interface represents the API expected of something that gives the engine access to a
/// device's event queue.
///
pub trait IEvents: IAny {
    fn get<'a>(&'a self) -> Box<dyn IEventsLock + 'a>;
}

///
/// This interface is used to provide access to the list of events for the current frame.
///
/// Some implementations may need to lock a mutex or read/write lock to provide access to the list
/// safely so this interface is passed to wrap the lock guard
///
pub trait IEventsLock {
    fn events(&self) -> &[Event];
}

/// This represents the set of all types of events
#[derive(Clone, Debug)]
pub enum Event {
    KeyboardEvent(KeyboardEvent),
    MouseEvent(MouseEvent),
    WindowEvent(WindowEvent),
}
