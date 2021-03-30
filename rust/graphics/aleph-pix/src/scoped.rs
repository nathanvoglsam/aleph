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

use super::functions::*;
use crate::Colour;
use dx12::{CommandQueue, GraphicsCommandListRecorder};
use std::ffi::CStr;

pub struct ScopedEvent();

impl ScopedEvent {
    pub fn new(colour: Colour, text: &str) -> Self {
        unsafe {
            begin_event(colour, text);
        }
        Self()
    }

    pub fn new_cstr(colour: Colour, text: &CStr) -> Self {
        unsafe {
            begin_event_cstr(colour, text);
        }
        Self()
    }
}

impl Drop for ScopedEvent {
    fn drop(&mut self) {
        unsafe { end_event() }
    }
}

pub trait RecordScopedEvent {
    fn scoped_event(&mut self, colour: crate::Colour, text: &str, f: impl FnOnce(&mut Self));

    fn scoped_event_cstr(
        &mut self,
        colour: crate::Colour,
        text: &std::ffi::CStr,
        f: impl FnOnce(&mut Self),
    );
}

impl<'a> RecordScopedEvent for CommandQueue {
    fn scoped_event(&mut self, colour: Colour, text: &str, f: impl FnOnce(&mut Self)) {
        unsafe { for_queue(self, colour, text, f) }
    }

    fn scoped_event_cstr(&mut self, colour: Colour, text: &CStr, f: impl FnOnce(&mut Self)) {
        unsafe { for_queue_cstr(self, colour, text, f) }
    }
}

impl<'a> RecordScopedEvent for GraphicsCommandListRecorder<'a> {
    fn scoped_event(&mut self, colour: Colour, text: &str, f: impl FnOnce(&mut Self)) {
        unsafe { for_list(self, colour, text, f) }
    }

    fn scoped_event_cstr(&mut self, colour: Colour, text: &CStr, f: impl FnOnce(&mut Self)) {
        unsafe { for_list_cstr(self, colour, text, f) }
    }
}

pub unsafe fn for_queue<'a>(
    queue: &mut CommandQueue,
    colour: Colour,
    text: &str,
    f: impl FnOnce(&mut CommandQueue),
) {
    begin_event_on_queue(queue.as_raw_mut(), colour, text);
    f(queue);
    end_event_on_queue(queue.as_raw_mut());
}

pub unsafe fn for_queue_cstr<'a>(
    queue: &mut CommandQueue,
    colour: Colour,
    text: &CStr,
    f: impl FnOnce(&mut CommandQueue),
) {
    begin_event_cstr_on_queue(queue.as_raw_mut(), colour, text);
    f(queue);
    end_event_on_queue(queue.as_raw_mut());
}

pub unsafe fn for_list<'a>(
    list: &mut GraphicsCommandListRecorder<'a>,
    colour: Colour,
    text: &str,
    f: impl FnOnce(&mut GraphicsCommandListRecorder<'a>),
) {
    begin_event_on_list(list.as_raw_mut(), colour, text);
    f(list);
    end_event_on_list(list.as_raw_mut());
}

pub unsafe fn for_list_cstr<'a>(
    list: &mut GraphicsCommandListRecorder<'a>,
    colour: Colour,
    text: &CStr,
    f: impl FnOnce(&mut GraphicsCommandListRecorder<'a>),
) {
    begin_event_cstr_on_list(list.as_raw_mut(), colour, text);
    f(list);
    end_event_on_list(list.as_raw_mut());
}
