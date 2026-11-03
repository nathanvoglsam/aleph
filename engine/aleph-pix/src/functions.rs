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

use std::ffi::CStr;

#[cfg(feature = "enabled")]
use raw::PixShimFunctions;
use windows::Win32::Graphics::Direct3D12::{ID3D12CommandQueue, ID3D12GraphicsCommandList};

use crate::Colour;

///
/// The PIX shim DLL, loaded on first use. `None` if the DLL could not be loaded, in which case
/// every marker function is a no-op.
///
#[cfg(feature = "enabled")]
static PIX: std::sync::LazyLock<Option<PixShimFunctions>> =
    std::sync::LazyLock::new(|| unsafe { PixShimFunctions::load().ok() });

#[inline(always)]
pub unsafe fn begin_event_on_thread(colour: Colour, text: &CStr) {
    #[cfg(feature = "enabled")]
    unsafe {
        if let Some(pix) = PIX.as_ref() {
            (pix.begin_event_on_thread)(colour.into(), text.as_ptr())
        }
    }
    #[cfg(not(feature = "enabled"))]
    let _ = (colour, text);
}

#[inline(always)]
pub unsafe fn begin_event_on_list<'a, T: Into<&'a ID3D12GraphicsCommandList>>(
    context: T,
    colour: Colour,
    text: &CStr,
) {
    #[cfg(feature = "enabled")]
    unsafe {
        let context = context.into();
        if let Some(pix) = PIX.as_ref() {
            (pix.begin_event_on_command_list)(
                std::mem::transmute_copy(context),
                colour.into(),
                text.as_ptr(),
            )
        }
    }
    #[cfg(not(feature = "enabled"))]
    let _ = (context, colour, text);
}

#[inline(always)]
pub unsafe fn begin_event_on_queue<'a, T: Into<&'a ID3D12CommandQueue>>(
    context: T,
    colour: Colour,
    text: &CStr,
) {
    #[cfg(feature = "enabled")]
    unsafe {
        let context = context.into();
        if let Some(pix) = PIX.as_ref() {
            (pix.begin_event_on_command_queue)(
                std::mem::transmute_copy(context),
                colour.into(),
                text.as_ptr(),
            )
        }
    }
    #[cfg(not(feature = "enabled"))]
    let _ = (context, colour, text);
}

#[inline(always)]
pub unsafe fn begin_event_data_on_thread(colour: Colour, text: &CStr, data: &CStr) {
    #[cfg(feature = "enabled")]
    unsafe {
        if let Some(pix) = PIX.as_ref() {
            (pix.begin_event_data_on_thread)(colour.into(), text.as_ptr(), data.as_ptr())
        }
    }
    #[cfg(not(feature = "enabled"))]
    let _ = (colour, text, data);
}

#[inline(always)]
pub unsafe fn begin_event_data_on_list<'a, T: Into<&'a ID3D12GraphicsCommandList>>(
    context: T,
    colour: Colour,
    text: &CStr,
    data: &CStr,
) {
    #[cfg(feature = "enabled")]
    unsafe {
        let context = context.into();
        if let Some(pix) = PIX.as_ref() {
            (pix.begin_event_data_on_command_list)(
                std::mem::transmute_copy(context),
                colour.into(),
                text.as_ptr(),
                data.as_ptr(),
            )
        }
    }
    #[cfg(not(feature = "enabled"))]
    let _ = (context, colour, text, data);
}

#[inline(always)]
pub unsafe fn begin_event_data_on_queue<'a, T: Into<&'a ID3D12CommandQueue>>(
    context: T,
    colour: Colour,
    text: &CStr,
    data: &CStr,
) {
    #[cfg(feature = "enabled")]
    unsafe {
        let context = context.into();
        if let Some(pix) = PIX.as_ref() {
            (pix.begin_event_data_on_command_queue)(
                std::mem::transmute_copy(context),
                colour.into(),
                text.as_ptr(),
                data.as_ptr(),
            )
        }
    }
    #[cfg(not(feature = "enabled"))]
    let _ = (context, colour, text, data);
}

#[inline(always)]
pub unsafe fn set_marker_on_thread(colour: Colour, text: &CStr) {
    #[cfg(feature = "enabled")]
    unsafe {
        if let Some(pix) = PIX.as_ref() {
            (pix.set_marker_on_thread)(colour.into(), text.as_ptr())
        }
    }
    #[cfg(not(feature = "enabled"))]
    let _ = (colour, text);
}

#[inline(always)]
pub unsafe fn set_marker_on_list<'a, T: Into<&'a ID3D12GraphicsCommandList>>(
    context: T,
    colour: Colour,
    text: &CStr,
) {
    #[cfg(feature = "enabled")]
    unsafe {
        let context = context.into();
        if let Some(pix) = PIX.as_ref() {
            (pix.set_marker_on_command_list)(
                std::mem::transmute_copy(context),
                colour.into(),
                text.as_ptr(),
            )
        }
    }
    #[cfg(not(feature = "enabled"))]
    let _ = (context, colour, text);
}

#[inline(always)]
pub unsafe fn set_marker_on_queue<'a, T: Into<&'a ID3D12CommandQueue>>(
    context: T,
    colour: Colour,
    text: &CStr,
) {
    #[cfg(feature = "enabled")]
    unsafe {
        let context = context.into();
        if let Some(pix) = PIX.as_ref() {
            (pix.set_marker_on_command_queue)(
                std::mem::transmute_copy(context),
                colour.into(),
                text.as_ptr(),
            )
        }
    }
    #[cfg(not(feature = "enabled"))]
    let _ = (context, colour, text);
}

#[inline(always)]
pub unsafe fn set_marker_data_on_thread(colour: Colour, text: &CStr, data: &CStr) {
    #[cfg(feature = "enabled")]
    unsafe {
        if let Some(pix) = PIX.as_ref() {
            (pix.set_marker_data_on_thread)(colour.into(), text.as_ptr(), data.as_ptr())
        }
    }
    #[cfg(not(feature = "enabled"))]
    let _ = (colour, text, data);
}

#[inline(always)]
pub unsafe fn set_marker_data_on_list<'a, T: Into<&'a ID3D12GraphicsCommandList>>(
    context: T,
    colour: Colour,
    text: &CStr,
    data: &CStr,
) {
    #[cfg(feature = "enabled")]
    unsafe {
        let context = context.into();
        if let Some(pix) = PIX.as_ref() {
            (pix.set_marker_data_on_command_list)(
                std::mem::transmute_copy(context),
                colour.into(),
                text.as_ptr(),
                data.as_ptr(),
            )
        }
    }
    #[cfg(not(feature = "enabled"))]
    let _ = (context, colour, text, data);
}

#[inline(always)]
pub unsafe fn set_marker_data_on_queue<'a, T: Into<&'a ID3D12CommandQueue>>(
    context: T,
    colour: Colour,
    text: &CStr,
    data: &CStr,
) {
    #[cfg(feature = "enabled")]
    unsafe {
        let context = context.into();
        if let Some(pix) = PIX.as_ref() {
            (pix.set_marker_data_on_command_queue)(
                std::mem::transmute_copy(context),
                colour.into(),
                text.as_ptr(),
                data.as_ptr(),
            )
        }
    }
    #[cfg(not(feature = "enabled"))]
    let _ = (context, colour, text, data);
}

///
/// # Safety
///
/// FFI Call to `PIXEndEvent`
///
#[inline(always)]
pub unsafe fn end_event_on_thread() {
    #[cfg(feature = "enabled")]
    unsafe {
        if let Some(pix) = PIX.as_ref() {
            (pix.end_event_on_thread)()
        }
    }
}

///
/// # Safety
///
/// FFI Call to `PIXEndEvent`
///
#[inline(always)]
pub unsafe fn end_event_on_queue<'a, T: Into<&'a ID3D12CommandQueue>>(context: T) {
    #[cfg(feature = "enabled")]
    unsafe {
        if let Some(pix) = PIX.as_ref() {
            (pix.end_event_on_command_queue)(std::mem::transmute_copy(context.into()))
        }
    }
    #[cfg(not(feature = "enabled"))]
    let _ = context;
}

///
/// # Safety
///
/// FFI Call to `PIXEndEvent`
///
#[inline(always)]
pub unsafe fn end_event_on_list<'a, T: Into<&'a ID3D12GraphicsCommandList>>(context: T) {
    #[cfg(feature = "enabled")]
    unsafe {
        if let Some(pix) = PIX.as_ref() {
            (pix.end_event_on_command_list)(std::mem::transmute_copy(context.into()))
        }
    }
    #[cfg(not(feature = "enabled"))]
    let _ = context;
}
