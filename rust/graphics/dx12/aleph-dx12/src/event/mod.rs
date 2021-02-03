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

use crate::raw::windows::win32::system_services::{
    CreateEventW, WaitForMultipleObjects, WaitForSingleObject, HANDLE, INFINITE,
};
use crate::raw::windows::win32::windows_programming::CloseHandle;

pub struct EventBuilder {
    manual_reset: bool,
    start_signalled: bool,
}

impl EventBuilder {
    pub fn new() -> EventBuilder {
        EventBuilder {
            manual_reset: false,
            start_signalled: false,
        }
    }

    pub fn manual_reset(mut self, manual_reset: bool) -> Self {
        self.manual_reset = manual_reset;
        self
    }

    pub fn start_signalled(mut self, start_signalled: bool) -> Self {
        self.start_signalled = start_signalled;
        self
    }

    pub fn build(self) -> Option<Event> {
        let event = unsafe {
            CreateEventW(
                std::ptr::null_mut(),
                self.manual_reset.into(),
                self.start_signalled.into(),
                std::ptr::null(),
            )
        };

        if event.0 != 0 {
            Some(Event(event))
        } else {
            None
        }
    }
}

#[repr(transparent)]
pub struct Event(HANDLE);

impl Event {
    pub fn builder() -> EventBuilder {
        EventBuilder::new()
    }

    pub fn wait(&self, timeout: Option<u32>) -> u32 {
        if let Some(timeout) = timeout {
            unsafe { WaitForSingleObject(self.0, timeout) }
        } else {
            unsafe { WaitForSingleObject(self.0, INFINITE) }
        }
    }

    pub fn raw(&self) -> HANDLE {
        self.0
    }
}

impl Drop for Event {
    fn drop(&mut self) {
        unsafe {
            assert_ne!(CloseHandle(self.0), false.into());
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
                    true.into(),
                    timeout,
                )
            }
        } else {
            unsafe {
                WaitForMultipleObjects(
                    self.len() as _,
                    self.as_ptr() as *const _,
                    true.into(),
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
                    false.into(),
                    timeout,
                )
            }
        } else {
            unsafe {
                WaitForMultipleObjects(
                    self.len() as _,
                    self.as_ptr() as *const _,
                    false.into(),
                    INFINITE,
                )
            }
        }
    }
}
