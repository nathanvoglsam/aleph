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

mod platform;

pub use clipboard::Clipboard;
pub use events::Event;
pub use events::Events;
pub use frame_timer::FrameTimer;
pub use keyboard::KeyDownEvent;
pub use keyboard::KeyUpEvent;
pub use keyboard::Keyboard;
pub use keyboard::KeyboardEvent;
pub use keyboard::KeyboardEvents;
pub use keyboard::KeyboardState;
pub use keyboard::KeyboardStateLock;
pub use keyboard::Keycode;
pub use keyboard::Mod;
pub use keyboard::Scancode;
pub use keyboard::TextInputEvent;
pub use mouse::Cursor;
pub use mouse::Mouse;
pub use mouse::MouseButton;
pub use mouse::MouseButtonDownEvent;
pub use mouse::MouseButtonUpEvent;
pub use mouse::MouseEvent;
pub use mouse::MouseEvents;
pub use mouse::MouseMotionEvent;
pub use mouse::MouseState;
pub use mouse::MouseWheelDirection;
pub use mouse::MouseWheelEvent;
pub use platform::Platform;
pub use platform::PlatformBuildError;
pub use platform::PlatformBuilder;
pub use window::Window;
pub use window::WindowEvent;
pub use window::WindowEvents;
