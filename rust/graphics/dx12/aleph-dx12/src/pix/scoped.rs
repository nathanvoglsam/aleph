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
use crate::pix::Colour;
use crate::{CommandQueue, OpenGraphicsCommandList};
use raw::windows::win32::direct3d12::{ID3D12CommandQueue, ID3D12GraphicsCommandList};
use std::ffi::CStr;

enum Contexts<'a> {
    CommandQueue(&'a ID3D12CommandQueue),
    CommandList(&'a ID3D12GraphicsCommandList),
    None,
}

pub struct ScopedEvent<'a>(Contexts<'a>);

impl ScopedEvent<'static> {
    pub unsafe fn new(colour: Colour, text: &str) -> Self {
        begin_event(colour, text);
        Self(Contexts::None)
    }

    pub unsafe fn new_cstr(colour: Colour, text: &CStr) -> Self {
        begin_event_cstr(colour, text);
        Self(Contexts::None)
    }
}

impl<'a> ScopedEvent<'a> {
    pub(crate) unsafe fn for_queue(queue: &'a CommandQueue, colour: Colour, text: &str) -> Self {
        begin_event_on_queue(&queue.0, colour, text);
        Self(Contexts::CommandQueue(&queue.0))
    }

    pub(crate) unsafe fn for_queue_cstr(
        queue: &'a CommandQueue,
        colour: Colour,
        text: &CStr,
    ) -> Self {
        begin_event_cstr_on_queue(&queue.0, colour, text);
        Self(Contexts::CommandQueue(&queue.0))
    }

    pub(crate) unsafe fn for_list(
        list: &'a OpenGraphicsCommandList,
        colour: Colour,
        text: &str,
    ) -> Self {
        begin_event_on_list(&list.0, colour, text);
        Self(Contexts::CommandList(&list.0))
    }

    pub(crate) unsafe fn for_list_cstr(
        list: &'a OpenGraphicsCommandList,
        colour: Colour,
        text: &CStr,
    ) -> Self {
        begin_event_cstr_on_list(&list.0, colour, text);
        Self(Contexts::CommandList(&list.0))
    }
}

impl<'a> Drop for ScopedEvent<'a> {
    fn drop(&mut self) {
        unsafe {
            match self.0 {
                Contexts::CommandQueue(v) => end_event_on_queue(&v),
                Contexts::CommandList(v) => end_event_on_list(&v),
                Contexts::None => end_event(),
            }
        }
    }
}
