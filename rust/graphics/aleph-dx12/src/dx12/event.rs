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

use std::num::NonZeroIsize;
use windows_raw::win32::system_services::{
    CreateEventW, WaitForMultipleObjects, WaitForSingleObject, BOOL, HANDLE, INFINITE, PWSTR,
};
use windows_raw::win32::windows_programming::CloseHandle;

#[repr(transparent)]
pub struct Event(pub(crate) NonZeroIsize);

impl Event {
    pub fn new() -> Option<Self> {
        let event = unsafe {
            CreateEventW(
                std::ptr::null_mut(),
                BOOL::from(false),
                BOOL::from(false),
                PWSTR(std::ptr::null_mut()),
            )
        };

        NonZeroIsize::new(event.0).map(|v| Self(v))
    }

    pub fn wait(&self, timeout: Option<u32>) -> u32 {
        if let Some(timeout) = timeout {
            unsafe { WaitForSingleObject(HANDLE(self.0.get()), timeout) }
        } else {
            unsafe { WaitForSingleObject(HANDLE(self.0.get()), INFINITE) }
        }
    }
}

impl Drop for Event {
    fn drop(&mut self) {
        unsafe {
            assert_ne!(
                CloseHandle(HANDLE(self.0.get())),
                windows_raw::BOOL::from(false)
            );
        }
    }
}

pub trait WaitAll {
    fn wait_all(&self, timeout: Option<u32>) -> u32;

    fn wait_any(&self, timeout: Option<u32>) -> u32;
}

impl WaitAll for [Event] {
    fn wait_all(&self, timeout: Option<u32>) -> u32 {
        if let Some(timeout) = timeout {
            unsafe {
                WaitForMultipleObjects(
                    self.len() as _,
                    self.as_ptr() as *const _,
                    BOOL::from(true),
                    timeout,
                )
            }
        } else {
            unsafe {
                WaitForMultipleObjects(
                    self.len() as _,
                    self.as_ptr() as *const _,
                    BOOL::from(true),
                    INFINITE,
                )
            }
        }
    }

    fn wait_any(&self, timeout: Option<u32>) -> u32 {
        if let Some(timeout) = timeout {
            unsafe {
                WaitForMultipleObjects(
                    self.len() as _,
                    self.as_ptr() as *const _,
                    BOOL::from(false),
                    timeout,
                )
            }
        } else {
            unsafe {
                WaitForMultipleObjects(
                    self.len() as _,
                    self.as_ptr() as *const _,
                    BOOL::from(false),
                    INFINITE,
                )
            }
        }
    }
}
