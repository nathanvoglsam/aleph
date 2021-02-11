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

use std::os::raw::c_char;
use std::os::raw::c_void;

extern "C" {
    // NO CONTEXT PWCHAR
    pub fn SHIM_PIXBeginEvent_NW(color: u64, format_string: *const u16);
    pub fn SHIM_PIXBeginEvent_NC(color: u64, format_string: *const c_char);
    pub fn SHIM_PIXSetMarker_NW(color: u64, format_string: *const u16);
    pub fn SHIM_PIXSetMarker_NC(color: u64, formatString: *const c_char);
    pub fn SHIM_PIXBeginEvent_CW(context: *mut c_void, color: u64, format_string: *const u16);
    pub fn SHIM_PIXBeginEvent_CC(context: *mut c_void, color: u64, format_string: *const c_char);
    pub fn SHIM_PIXSetMarker_CW(context: *mut c_void, color: u64, format_string: *const u16);
    pub fn SHIM_PIXSetMarker_CC(context: *mut c_void, color: u64, format_string: *const c_char);
    pub fn SHIM_PIXEndEvent_N();
    pub fn SHIM_PIXEndEvent_C(context: *mut c_void);
}
