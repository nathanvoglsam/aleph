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

use any::*;

///
/// This interface should be used by plugins that wish to register themselves as the engine's mouse
/// provider. Anything that implements this should correctly handle creating and destroying whatever
/// is needed to access the system's mouse, and should be able to give out an `AnyArc<IMouse>` to
/// allow others to retrieve information about and manipulate the mouse.
///
pub trait IMouseProvider: IAny {
    ///
    /// Returns an `AnyArc` that holds an `IMouse` interface.
    ///
    /// This will always return the same `IMouse` instance as `IMouseProvider` only supports
    /// handling a single mouse device.
    ///
    /// A return value of `None` should signal that the functionality is not supported.
    ///
    fn get_mouse(&self) -> Option<AnyArc<dyn IMouse>>;
}

///
/// This interface represents the API expected of something that gives the engine access to a
/// device's mouse.
///
pub trait IMouse: IAny {
    ///
    /// Get the current state of the mouse, last updated at the beginning of the frame
    ///
    fn get_state(&self) -> MouseState;

    ///
    /// Set the position of the mouse
    ///
    fn set_pos(&self, x: i32, y: i32);

    ///
    /// Sets the mouse cursor
    ///
    fn set_cursor(&self, cursor: Cursor);

    ///
    /// Makes the cursor visible
    ///
    fn show_cursor(&self);

    ///
    /// Makes the cursor invisible
    ///
    fn hide_cursor(&self);

    ///
    /// Get read only access to this frame's list of mouse events.
    ///
    /// # Warning
    ///
    /// This will probably lock an RwLock so trying to hold on to this between frames will likely
    /// deadlock the engine.
    ///
    fn events(&self) -> Box<dyn IMouseEventsLock>;
}

///
/// This interface is used to provide access to the list of mouse events for the current frame.
///
/// Some implementations may need to lock a mutex or read/write lock to provide access to the list
/// safely so this interface is passed to wrap the lock guard
///
pub trait IMouseEventsLock: IAny {
    fn events(&self) -> &[MouseEvent];
}

///
/// Represents the set of cursors available
///
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash)]
pub enum Cursor {
    Arrow,
    IBeam,
    SizeAll,
    SizeNS,
    SizeWE,
    SizeNESW,
    SizeNWSE,
    Hand,
    No,
}

/// A mouse event
#[derive(Clone, Debug)]
pub enum MouseEvent {
    MouseMotion(MouseMotionEvent),
    MouseButtonDown(MouseButtonDownEvent),
    MouseButtonUp(MouseButtonUpEvent),
    MouseWheel(MouseWheelEvent),
}

///
/// The different types of
///
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    X1,
    X2,
}

///
/// Mouse wheel direction
///
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum MouseWheelDirection {
    Normal,
    Flipped,
}

///
/// A mouse motion event
///
#[derive(Clone, Debug)]
pub struct MouseMotionEvent {
    pub mouse_state: MouseState,
    pub x: i32,
    pub y: i32,
    pub x_rel: i32,
    pub y_rel: i32,
}

///
/// A mouse button down event
///
#[derive(Clone, Debug)]
pub struct MouseButtonDownEvent {
    pub button: MouseButton,
    pub clicks: u8,
    pub x: i32,
    pub y: i32,
}

///
/// A mouse button up event
///
#[derive(Clone, Debug)]
pub struct MouseButtonUpEvent {
    pub button: MouseButton,
    pub clicks: u8,
    pub x: i32,
    pub y: i32,
}

///
/// A mouse wheel event
///
#[derive(Clone, Debug)]
pub struct MouseWheelEvent {
    pub x: i32,
    pub y: i32,
    pub direction: MouseWheelDirection,
}

///
/// Represents the state of the mouse this frame
///
#[derive(Clone, Debug)]
pub struct MouseState {
    pos: (i32, i32),
    buttons: u32,
}

impl MouseState {
    ///
    /// Get the position of the mouse
    ///
    pub fn pos(&self) -> (i32, i32) {
        self.pos
    }

    ///
    /// Is the left mouse button down
    ///
    pub fn left(&self) -> bool {
        self.button(1)
    }

    ///
    /// Is the middle mouse button down
    ///
    pub fn middle(&self) -> bool {
        self.button(2)
    }

    ///
    /// Is the right mouse button down
    ///
    pub fn right(&self) -> bool {
        self.button(3)
    }

    ///
    /// Is mouse button x1 (mouse button 4) down
    ///
    pub fn button_4(&self) -> bool {
        self.button(4)
    }

    ///
    /// Is mouse button x2 (mouse button 5) down
    ///
    pub fn button_5(&self) -> bool {
        self.button(5)
    }

    /// Lookup the `button`th state
    pub fn button(&self, button: u32) -> bool {
        if button == 0 {
            false
        } else {
            (self.buttons & 1 << (button - 1)) != 0
        }
    }
}
