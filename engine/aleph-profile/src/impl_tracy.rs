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

// This crate is a fork of aclysma/profiling
//
// MIT License
//
// Copyright (c) 2020 Philip Degarmo and other contributors
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

use tracy_client::sys;

#[macro_export]
macro_rules! scope {
    ($name:expr) => {
        // Note: callstack_depth is 0 since this has significant overhead
        let _tracy_span = $crate::tracy_client::span!($name, 0);
    };
    ($name:expr, $data:expr) => {
        // Note: callstack_depth is 0 since this has significant overhead
        let _tracy_span = $crate::tracy_client::span!($name, 0);
        {
            use $crate::ProfileDataParam;
            _tracy_span.emit_text(ProfileDataParam::get_str($data));
        }
    };
}

#[macro_export]
macro_rules! register_thread {
    ($name:expr) => {
        $crate::tracy_client::Client::running()
            .expect("register_thread! without a running tracy_client::Client")
            .set_thread_name($name);
    };
}

#[macro_export]
macro_rules! finish_frame {
    () => {
        $crate::tracy_client::Client::running()
            .expect("finish_frame! without a running tracy_client::Client")
            .frame_mark();
    };
}

#[inline]
pub fn message(message: &str) {
    tracy_client::Client::start().message(message, CALLSTACK_DEPTH_MSG);
}

#[inline]
pub fn color_message(message: &str, rgba: u32) {
    tracy_client::Client::start().color_message(message, rgba, CALLSTACK_DEPTH_MSG);
}

#[inline]
pub unsafe fn emit_alloc(ptr: *mut u8, size: usize) {
    tracy_client::Client::start();

    let ptr = ptr as *const u8 as *const std::os::raw::c_void;
    if CALLSTACK_DEPTH_MEM != 0 {
        unsafe {
            tracy_client::sys::___tracy_emit_memory_alloc_callstack(
                ptr,
                size,
                CALLSTACK_DEPTH_MEM,
                1,
            )
        }
    } else {
        unsafe { tracy_client::sys::___tracy_emit_memory_alloc(ptr, size, 1) }
    }
}

#[inline]
pub unsafe fn emit_free(ptr: *mut u8) {
    tracy_client::Client::start();

    let ptr = ptr as *const u8 as *const std::os::raw::c_void;
    if CALLSTACK_DEPTH_MEM != 0 {
        unsafe {
            tracy_client::sys::___tracy_emit_memory_free_callstack(ptr, CALLSTACK_DEPTH_MEM, 1)
        }
    } else {
        unsafe { tracy_client::sys::___tracy_emit_memory_free(ptr, 1) }
    }
}

#[inline]
pub unsafe fn emit_alloc_n(ptr: *mut u8, size: usize, name: &'static std::ffi::CStr) {
    tracy_client::Client::start();

    let ptr = ptr as *const u8 as *const std::os::raw::c_void;
    if CALLSTACK_DEPTH_MEM != 0 {
        unsafe {
            tracy_client::sys::___tracy_emit_memory_alloc_callstack_named(
                ptr,
                size,
                CALLSTACK_DEPTH_MEM,
                1,
                name.as_ptr(),
            )
        }
    } else {
        unsafe { tracy_client::sys::___tracy_emit_memory_alloc_named(ptr, size, 1, name.as_ptr()) }
    }
}

#[inline]
pub unsafe fn emit_free_n(ptr: *mut u8, name: &'static std::ffi::CStr) {
    tracy_client::Client::start();

    let ptr = ptr as *const u8 as *const std::os::raw::c_void;
    if CALLSTACK_DEPTH_MEM != 0 {
        unsafe {
            tracy_client::sys::___tracy_emit_memory_free_callstack_named(
                ptr,
                CALLSTACK_DEPTH_MEM,
                1,
                name.as_ptr(),
            )
        }
    } else {
        unsafe { tracy_client::sys::___tracy_emit_memory_free_named(ptr, 1, name.as_ptr()) }
    }
}

const CALLSTACK_DEPTH_MEM: i32 = clamp_depth_mem(62);

const fn clamp_depth_mem(v: i32) -> i32 {
    if cfg!(feature = "memory-callstacks") {
        if cfg!(windows) {
            match v {
                v if v < 0 => 0,
                v if v > 62 => 62,
                v => v,
            }
        } else {
            v
        }
    } else if cfg!(feature = "memory-callsites") {
        1
    } else {
        0
    }
}

const CALLSTACK_DEPTH_MSG: i32 = clamp_depth_msg(62);

const fn clamp_depth_msg(v: i32) -> i32 {
    if cfg!(feature = "message-callstacks") {
        if cfg!(windows) {
            match v {
                v if v < 0 => 0,
                v if v > 62 => 62,
                v => v,
            }
        } else {
            v
        }
    } else if cfg!(feature = "message-callsites") {
        1
    } else {
        0
    }
}
