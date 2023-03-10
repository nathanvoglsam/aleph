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
use std::ffi::CStr;
use windows::Win32::Graphics::Direct3D12::{ID3D12CommandQueue, ID3D12GraphicsCommandList};

///
/// # Safety
///
/// FFI Call to `PIXBeginEvent` and `PIXEndEvent`
///
#[inline(always)]
pub unsafe fn scoped_for_queue<'a, T: Into<&'a ID3D12CommandQueue>>(
    queue: T,
    colour: Colour,
    text: &str,
    f: impl FnOnce(&'a ID3D12CommandQueue),
) {
    let queue = queue.into();
    begin_event_on_queue(queue, colour, text);
    f(queue);
    end_event_on_queue(queue);
}

///
/// # Safety
///
/// FFI Call to `PIXBeginEvent` and `PIXEndEvent`
///
#[inline(always)]
pub unsafe fn scoped_for_queue_cstr<'a, T: Into<&'a ID3D12CommandQueue>>(
    queue: T,
    colour: Colour,
    text: &CStr,
    f: impl FnOnce(&'a ID3D12CommandQueue),
) {
    let queue = queue.into();
    begin_event_cstr_on_queue(queue, colour, text);
    f(queue);
    end_event_on_queue(queue);
}

///
/// # Safety
///
/// FFI Call to `PIXBeginEvent` and `PIXEndEvent`
///
#[inline(always)]
pub unsafe fn scoped_for_list<'a, T: Into<&'a ID3D12GraphicsCommandList>>(
    list: T,
    colour: Colour,
    text: &str,
    f: impl FnOnce(&'a ID3D12GraphicsCommandList),
) {
    let list = list.into();
    begin_event_on_list(list, colour, text);
    f(list);
    end_event_on_list(list);
}

///
/// # Safety
///
/// FFI Call to `PIXBeginEvent` and `PIXEndEvent`
///
#[inline(always)]
pub unsafe fn scoped_for_list_cstr<'a, T: Into<&'a ID3D12GraphicsCommandList>>(
    list: T,
    colour: Colour,
    text: &CStr,
    f: impl FnOnce(&'a ID3D12GraphicsCommandList),
) {
    let list = list.into();
    begin_event_cstr_on_list(list, colour, text);
    f(list);
    end_event_on_list(list);
}
