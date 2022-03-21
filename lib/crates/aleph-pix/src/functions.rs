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

#![allow(clippy::missing_safety_doc)]

use crate::Colour;
use pix_raw::*;
use std::ffi::{CStr, CString};
use windows::Win32::Graphics::Direct3D12::{ID3D12CommandQueue, ID3D12GraphicsCommandList};

pub unsafe fn begin_event(colour: Colour, text: &str) {
    let text = CString::new(text).unwrap();
    SHIM_PIXBeginEvent_N(colour.into(), text.as_ptr())
}

pub unsafe fn begin_event_cstr(colour: Colour, text: &CStr) {
    SHIM_PIXBeginEvent_N(colour.into(), text.as_ptr())
}

pub unsafe fn begin_event_on_queue(context: &ID3D12CommandQueue, colour: Colour, text: &str) {
    let text = CString::new(text).unwrap();
    SHIM_PIXBeginEvent_CQ(
        std::mem::transmute_copy(context),
        colour.into(),
        text.as_ptr(),
    )
}

pub unsafe fn begin_event_cstr_on_queue(context: &ID3D12CommandQueue, colour: Colour, text: &CStr) {
    SHIM_PIXBeginEvent_CQ(
        std::mem::transmute_copy(context),
        colour.into(),
        text.as_ptr(),
    )
}

pub unsafe fn begin_event_on_list(context: &ID3D12GraphicsCommandList, colour: Colour, text: &str) {
    let text = CString::new(text).unwrap();
    SHIM_PIXBeginEvent_CL(
        std::mem::transmute_copy(context),
        colour.into(),
        text.as_ptr(),
    )
}

pub unsafe fn begin_event_cstr_on_list(
    context: &ID3D12GraphicsCommandList,
    colour: Colour,
    text: &CStr,
) {
    SHIM_PIXBeginEvent_CL(
        std::mem::transmute_copy(context),
        colour.into(),
        text.as_ptr(),
    )
}

pub unsafe fn end_event() {
    SHIM_PIXEndEvent_N();
}

pub unsafe fn end_event_on_queue(context: &ID3D12CommandQueue) {
    SHIM_PIXEndEvent_CQ(std::mem::transmute_copy(context));
}

pub unsafe fn end_event_on_list(context: &ID3D12GraphicsCommandList) {
    SHIM_PIXEndEvent_CL(std::mem::transmute_copy(context));
}

pub unsafe fn set_marker(colour: Colour, text: &str) {
    let text = CString::new(text).unwrap();
    SHIM_PIXSetMarker_N(colour.into(), text.as_ptr());
}

pub unsafe fn set_marker_cstr(colour: Colour, text: &CStr) {
    SHIM_PIXSetMarker_N(colour.into(), text.as_ptr());
}

pub unsafe fn set_marker_on_queue(context: &ID3D12CommandQueue, colour: Colour, text: &str) {
    let text = CString::new(text).unwrap();
    SHIM_PIXSetMarker_CQ(
        std::mem::transmute_copy(context),
        colour.into(),
        text.as_ptr(),
    );
}

pub unsafe fn set_marker_cstr_on_queue(context: &ID3D12CommandQueue, colour: Colour, text: &CStr) {
    SHIM_PIXSetMarker_CQ(
        std::mem::transmute_copy(context),
        colour.into(),
        text.as_ptr(),
    );
}

pub unsafe fn set_marker_on_list(context: &ID3D12GraphicsCommandList, colour: Colour, text: &str) {
    let text = CString::new(text).unwrap();
    SHIM_PIXSetMarker_CL(
        std::mem::transmute_copy(context),
        colour.into(),
        text.as_ptr(),
    );
}

pub unsafe fn set_marker_cstr_on_list(
    context: &ID3D12GraphicsCommandList,
    colour: Colour,
    text: &CStr,
) {
    SHIM_PIXSetMarker_CL(
        std::mem::transmute_copy(context),
        colour.into(),
        text.as_ptr(),
    );
}
