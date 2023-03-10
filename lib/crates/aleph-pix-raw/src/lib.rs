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
    /// This is an internal utility provided to allow the rust code to query whether it's linked to the actual PIX
    /// library or the dummy noop implementation. Useful for disabling the marker code when it won't do anything.
    ///
    /// Returns: 1 if PIX is available, 0 if NOOP implementation
    pub fn SHIM_IsLibraryAvailable() -> u32;

    pub fn SHIM_PIXBeginEvent_N(colour: u64, string: *const c_char);
    pub fn SHIM_PIXSetMarker_N(colour: u64, string: *const c_char);
    pub fn SHIM_PIXBeginEvent_CL(context: *mut c_void, colour: u64, string: *const c_char);
    pub fn SHIM_PIXBeginEvent_CQ(context: *mut c_void, colour: u64, string: *const c_char);
    pub fn SHIM_PIXSetMarker_CL(context: *mut c_void, colour: u64, string: *const c_char);
    pub fn SHIM_PIXSetMarker_CQ(context: *mut c_void, colour: u64, string: *const c_char);
    pub fn SHIM_PIXEndEvent_N();
    pub fn SHIM_PIXEndEvent_CL(context: *mut c_void);
    pub fn SHIM_PIXEndEvent_CQ(context: *mut c_void);
}
