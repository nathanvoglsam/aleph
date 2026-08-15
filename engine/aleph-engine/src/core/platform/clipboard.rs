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

use std::ffi::{CStr, CString, c_void};
use std::marker::PhantomData;
use std::sync::Arc;

use api::platform::IClipboard;

///
/// Object that provides implementation of `IClipboard`
///
pub struct Clipboard(PhantomData<*const ()>);

impl Clipboard {
    /// Safety: It is the caller's responsibility to ensure they only construct a Clipboard object
    ///         on the main thread.
    pub(crate) unsafe fn new() -> Arc<Self> {
        Arc::new(Self(Default::default()))
    }
}

impl IClipboard for Clipboard {
    fn has(&self) -> bool {
        // Safety: These functions are only safe to call from the main thread. We enforce that via
        //         an unsafe constructor and denying Send+Sync on the Clipboard object.
        unsafe { sdl3::sys::clipboard::SDL_HasClipboardText() }
    }

    fn get(&self) -> Option<String> {
        // Safety: These functions are only safe to call from the main thread. We enforce that via
        //         an unsafe constructor and denying Send+Sync on the Clipboard object.
        unsafe {
            let buf = sdl3::sys::clipboard::SDL_GetClipboardText();

            if buf.is_null() {
                None
            } else {
                let cstr = CStr::from_ptr(buf as *const _);
                let cstr = cstr.to_str().ok()?;
                let string = cstr.to_string();

                sdl3::sys::stdinc::SDL_free(buf as *mut c_void);

                Some(string)
            }
        }
    }

    fn get_null_terminated(&self) -> Option<CString> {
        // Safety: These functions are only safe to call from the main thread. We enforce that via
        //         an unsafe constructor and denying Send+Sync on the Clipboard object.
        unsafe {
            let buf = sdl3::sys::clipboard::SDL_GetClipboardText();

            if buf.is_null() {
                None
            } else {
                let cstr = CStr::from_ptr(buf as *const _);
                let cstring = cstr.to_owned();

                sdl3::sys::stdinc::SDL_free(buf as *mut c_void);

                Some(cstring)
            }
        }
    }

    fn set(&self, value: &str) {
        // Safety: These functions are only safe to call from the main thread. We enforce that via
        //         an unsafe constructor and denying Send+Sync on the Clipboard object.
        unsafe {
            let mut string = value.to_string();
            string.push('\0');

            sdl3::sys::clipboard::SDL_SetClipboardText(string.as_ptr() as *const _);
        }
    }

    fn set_null_terminated(&self, value: &CStr) {
        // Safety: These functions are only safe to call from the main thread. We enforce that via
        //         an unsafe constructor and denying Send+Sync on the Clipboard object.
        unsafe {
            sdl3::sys::clipboard::SDL_SetClipboardText(value.as_ptr() as *const _);
        }
    }
}
