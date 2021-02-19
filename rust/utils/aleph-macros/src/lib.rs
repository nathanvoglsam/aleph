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

use std::ffi::CStr;

#[inline]
#[allow(dead_code)]
pub unsafe fn str_to_cstr(string: &'static str) -> &'static CStr {
    CStr::from_ptr(string.as_ptr() as *const _)
}

#[macro_export]
macro_rules! cstr {
    ($strval:expr) => {
        unsafe { $crate::str_to_cstr(concat!($strval, "\0")) }
    };
}

#[repr(C)] // guarantee 'bytes' comes after '_align'
pub struct AlignedAs<Align, Bytes: ?Sized> {
    pub _align: [Align; 0],
    pub bytes: Bytes,
}

#[macro_export]
macro_rules! include_bytes_aligned_as {
    ($align_ty:ty, $path:literal) => {{
        // this assignment is made possible by CoerceUnsized
        static ALIGNED: &$crate::AlignedAs<$align_ty, [u8]> = &$crate::AlignedAs {
            _align: [],
            bytes: *include_bytes!($path),
        };

        &ALIGNED.bytes
    }};
}

///
/// Internal function used by `include_spirv_bytes` macro
///
#[inline]
#[allow(dead_code)]
pub fn spirv_bytes_map(bytes: &'static [u8]) -> (&'static [u8], &'static [u32]) {
    unsafe {
        let words = core::slice::from_raw_parts(bytes.as_ptr() as *const u32, bytes.len() / 4);
        (bytes, words)
    }
}

#[macro_export]
macro_rules! include_spirv_bytes {
    ($path:literal) => {{
        $crate::spirv_bytes_map($crate::include_bytes_aligned_as!(u32, $path))
    }};
}
