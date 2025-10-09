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

use std::ffi::c_void;
use std::ptr::NonNull;

use any::*;
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use smallbox::SmallBox;
use smallbox::space::S1;

/// An enum of window events.
#[derive(Clone, Debug)]
pub enum WindowEvent {
    Shown,
    Hidden,
    Exposed,
    Moved(i32, i32),
    Resized(i32, i32),
    SizeChanged(i32, i32),
    Minimized,
    Maximized,
    Restored,
    Enter,
    Leave,
    FocusGained,
    FocusLost,
    Close,
    HitTest,
}

///
/// This interface should be implemented as the interface to an OS window.
///
pub trait IWindow: IAny + HasDisplayHandle + HasWindowHandle + Send + Sync + 'static {
    ///
    /// Returns whether the window has been resized since the last time this function was called.
    ///
    /// # Info
    ///
    /// This interface was created to provide a very simple, one shot function that can be called
    /// once per frame to check if the window has been resized since last time it was checked.
    ///
    /// If the window has been resized then this will return true once, and only once, until the
    /// window is resized again.
    ///
    /// # Warning
    ///
    /// This API will probably be useless to anyone other than the core engine implementers as the
    /// function will only yield the true result once per frame. The intended use for this API is
    /// for triggering a swap chain rebuild and this consumption based model makes the most sense
    /// for that use case.
    ///
    /// If you're using the engine, and not implementing it, then you should look at the
    /// `WindowEvents` API.
    ///
    fn resized(&self) -> bool;

    ///
    /// Returns the title for the window
    ///
    fn title(&self) -> String;

    ///
    /// Sets the title of the window
    ///
    /// Will only take affect at the beginning of the next frame
    ///
    fn set_title(&self, title: String);

    ///
    /// Return the width of the window
    ///
    fn width(&self) -> u32;

    ///
    /// Sets the width of the window
    ///
    /// Will only take affect at the beginning of the next frame
    ///
    fn set_width(&self, width: u32);

    ///
    /// Return the height of the window
    ///
    fn height(&self) -> u32;

    ///
    /// Sets the height of the window
    ///
    /// Will only take affect at the beginning of the next frame
    ///
    fn set_height(&self, height: u32);

    ///
    /// Returns the dimensions of the window on the desktop
    ///
    /// Basically just the result of calling both `Window::width` and `Window::height` but only
    /// locks the state mutex once
    ///
    fn size(&self) -> (u32, u32);

    ///
    /// Sets the size of the window
    ///
    /// Will only take affect at the beginning of the next frame
    ///
    fn set_size(&self, width: u32, height: u32);

    ///
    /// Returns the width of the drawable surface on the window
    ///
    fn drawable_width(&self) -> u32;

    ///
    /// Returns the height of the drawable surface on the window
    ///
    fn drawable_height(&self) -> u32;

    ///
    /// Returns the dimensions of the drawable surface on the window
    ///
    /// Basically just the result of calling both `Window::drawable_width` and
    /// `Window::drawable_height` but only locks the state mutex once
    ///
    fn drawable_size(&self) -> (u32, u32);

    ///
    /// Return if the window is currently fullscreen
    ///
    fn fullscreen(&self) -> bool;

    ///
    /// Return if the window is currently focused
    ///
    fn focused(&self) -> bool;

    ///
    /// Sets the window to fullscreen
    ///
    /// Will only take affect at the beginning of the next frame
    ///
    fn go_fullscreen(&self);

    ///
    /// Sets the window to windowed mode
    ///
    /// Will only take affect at the beginning of the next frame
    ///
    fn go_windowed(&self);

    ///
    /// Swaps between fullscreen or windowed
    ///
    /// Will only take affect at the beginning of the next frame
    ///
    fn toggle_fullscreen(&self);

    ///
    /// Returns the display scaling factor of the display the window is on.
    ///
    fn current_display_scale(&self) -> f32;

    ///
    /// Returns a scaling factor for mouse input coordinates that will scale the input coordinates
    /// from the platform's reported numbers into logical 'points' in the possibly scaled coordinate
    /// system.
    ///
    fn current_content_scale(&self) -> f32;

    ///
    /// Returns the 'CAMetalLayer' pointer for the window, if one exists.
    ///
    fn metal_layer(&self) -> Option<NonNull<c_void>>;

    ///
    /// Get read only access to this frame's list of window events.
    ///
    /// # Warning
    ///
    /// This will probably lock an RwLock so trying to hold on to this between frames will likely
    /// deadlock the engine.
    ///
    fn events<'a>(&'a self) -> SmallBox<dyn IWindowEventsLock + 'a, S1>;
}

///
/// This interface is used to provide access to the list of window events for the current frame.
///
/// Some implementations may need to lock a mutex or read/write lock to provide access to the list
/// safely so this interface is passed to wrap the lock guard
///
pub trait IWindowEventsLock {
    fn events(&self) -> &[WindowEvent];
}
