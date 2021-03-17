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

#[cfg(target_os = "windows")]
extern crate aleph_dx12 as dx12;

#[cfg(target_os = "windows")]
extern crate aleph_pix_raw as pix_raw;

#[cfg(target_os = "windows")]
extern crate aleph_windows_raw as windows_raw;

#[cfg(target_os = "windows")]
mod colour;

#[cfg(target_os = "windows")]
mod functions;

#[cfg(target_os = "windows")]
mod scoped;

#[cfg(target_os = "windows")]
mod exports {
    pub use super::colour::Colour;

    pub use super::functions::begin_event;
    pub use super::functions::begin_event_on_list;
    pub use super::functions::begin_event_on_queue;

    pub use super::functions::begin_event_cstr;
    pub use super::functions::begin_event_cstr_on_list;
    pub use super::functions::begin_event_cstr_on_queue;

    pub use super::functions::end_event;
    pub use super::functions::end_event_on_list;
    pub use super::functions::end_event_on_queue;

    pub use super::functions::set_marker;
    pub use super::functions::set_marker_on_list;
    pub use super::functions::set_marker_on_queue;

    pub use super::functions::set_marker_cstr;
    pub use super::functions::set_marker_cstr_on_list;
    pub use super::functions::set_marker_cstr_on_queue;

    pub use super::scoped::RecordScopedEvent;
    pub use super::scoped::ScopedEvent;
}

#[cfg(not(target_os = "windows"))]
mod exports {}

pub use exports::*;
