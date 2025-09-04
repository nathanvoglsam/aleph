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

#[macro_export]
macro_rules! scope {
    ($name:expr) => {
        let _cstr = concat!($name, "\0");
        let _cstr = unsafe { core::ffi::CStr::from_bytes_with_nul_unchecked(_cstr.as_bytes()) };
        let _superluminal_guard = $crate::detail::Guard::new(_cstr);
    };
    ($name:expr, $data:expr) => {
        let _cstr = concat!($name, "\0");
        let _cstr = unsafe { core::ffi::CStr::from_bytes_with_nul_unchecked(_cstr.as_bytes()) };
        let _superluminal_guard = $crate::detail::Guard::new_with_data(_cstr, $data);
    };
}

#[macro_export]
macro_rules! register_thread {
    ($name:expr) => {{
        #[cfg(target_os = "windows")]
        unsafe {
            use $crate::aleph_superluminal_sys as ffi;
            ffi::PerformanceAPI_SetCurrentThreadName_N(
                $name.as_ptr().cast::<i8>(),
                $name.len() as u16,
            );
        }
    }};
}

#[macro_export]
macro_rules! finish_frame {
    () => {
        // superluminal does not have a frame end function
        {}
    };
}

#[inline]
pub unsafe fn emit_alloc(_ptr: *mut u8, _size: usize) {
    // Intentional no-op
}

#[inline]
pub unsafe fn emit_free(_ptr: *mut u8) {
    // Intentional no-op
}

#[inline]
pub unsafe fn emit_alloc_n(_ptr: *mut u8, _size: usize, _name: &'static std::ffi::CStr) {
    // Intentional no-op
}

#[inline]
pub unsafe fn emit_free_n(_ptr: *mut u8, _name: &'static std::ffi::CStr) {
    // Intentional no-op
}

//
// RAII wrappers to support superluminal. These are public as they need to be callable from macros
// but are not intended for direct use.
//
#[doc(hidden)]
pub mod detail {
    use std::ffi::CStr;

    use aleph_superluminal_sys as ffi;

    use crate::ProfileDataParam;

    pub struct Guard;

    impl Guard {
        #[inline(always)]
        pub fn new(name: &'static CStr) -> Self {
            #[cfg(target_os = "windows")]
            unsafe {
                ffi::PerformanceAPI_BeginEvent(name.as_ptr(), std::ptr::null(), ffi::DEFAULT_COLOR);
            }
            Guard
        }

        #[inline(always)]
        pub fn new_with_data<'a>(name: &'static CStr, data: impl ProfileDataParam<'a>) -> Self {
            #[cfg(target_os = "windows")]
            if let Some(data) = data.get_cstr() {
                // If we can cheaply get the input as a cstr we're golden
                unsafe {
                    ffi::PerformanceAPI_BeginEvent(
                        name.as_ptr(),
                        data.as_ptr(),
                        ffi::DEFAULT_COLOR,
                    );
                }
            } else {
                // TODO: PERF - Could we get gains with stack allocation? Avoid hitting the global
                //              heap? Ideal for less overhead when profiling
                let data = data.get_cstring();
                unsafe {
                    ffi::PerformanceAPI_BeginEvent(
                        name.as_ptr(),
                        data.as_ptr(),
                        ffi::DEFAULT_COLOR,
                    );
                }
            }
            Guard
        }
    }

    impl Drop for Guard {
        #[inline(always)]
        fn drop(&mut self) {
            #[cfg(target_os = "windows")]
            unsafe {
                // PerformanceAPI_EndEvent returns a struct which is only used to prevent calls to it from being tail-call optimized.
                // We ignore the return value here so the caller of end_event doesn't need to deal with it.
                let _ = ffi::PerformanceAPI_EndEvent();
            }
        }
    }
}
