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

/// # Safety
///
/// It's the caller's responsibility to ensure that the string is null terminated.
#[inline]
#[allow(dead_code)]
#[doc(hidden)]
pub unsafe fn str_to_cstr(string: &'static str) -> &'static std::ffi::CStr {
    std::mem::transmute(string)
}

#[macro_export]
macro_rules! cstr {
    ($strval:expr) => {{
        fn caster(string: &'static str) -> &'static std::ffi::CStr {
            unsafe { $crate::macros::str_to_cstr(string) }
        }
        caster(concat!($strval, "\0"))
    }};
}

/// # Safety
///
/// It's the caller's responsibility to ensure that the string is null terminated.
#[inline]
#[allow(dead_code)]
#[doc(hidden)]
pub unsafe fn str_to_cstr_raw(string: &'static str) -> *const std::os::raw::c_char {
    string.as_ptr() as *const _
}

#[macro_export]
macro_rules! cstr_ptr {
    ($strval:expr) => {{
        fn caster(string: &'static str) -> *const std::os::raw::c_char {
            unsafe { $crate::macros::str_to_cstr_raw(string) }
        }
        caster(concat!($strval, "\0"))
    }};
}
