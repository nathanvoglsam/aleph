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
    ($name:literal, $data:literal) => {
        use $crate::ProfileDataParam;
        let _cstr = concat!($name, "\0");
        let _cstr = unsafe { core::ffi::CStr::from_bytes_with_nul_unchecked(_cstr.as_bytes()) };
        let _data = if let Ok(_data) = core::ffi::CStr::from_bytes_until_nul($data.as_bytes()) {
            _data
        } else {
            let _data = concat!($data, "\0");
            let _data = unsafe { core::ffi::CStr::from_bytes_with_nul_unchecked(_data.as_bytes()) };
            _data
        };
        let _pix_guard = $crate::detail::Guard::new_with_data(_cstr, _data);
    };
    ($name:literal) => {
        let _cstr = concat!($name, "\0");
        let _cstr = unsafe { core::ffi::CStr::from_bytes_with_nul_unchecked(_cstr.as_bytes()) };
        let _pix_guard = $crate::detail::Guard::new(_cstr);
    };
    ($name:literal, $data:expr) => {
        let _cstr = concat!($name, "\0");
        let _cstr = unsafe { core::ffi::CStr::from_bytes_with_nul_unchecked(_cstr.as_bytes()) };
        let _pix_guard = $crate::detail::Guard::new_with_data(_cstr, $data);
    };
}

#[macro_export]
macro_rules! register_thread {
    ($name:expr) => {};
}

#[macro_export]
macro_rules! finish_frame {
    () => {};
}

//
// RAII wrappers to support pix. These are public as they need to be callable from macros
// but are not intended for direct use.
//
#[doc(hidden)]
pub mod detail {
    use core::ffi::CStr;

    use aleph_pix::raw;

    use crate::ProfileDataParam;

    pub struct Guard;

    impl Guard {
        #[inline(always)]
        pub fn new(name: &'static CStr) -> Self {
            unsafe {
                raw::SHIM_PIXBeginEvent_N(aleph_pix::Colour::MAGENTA.into(), name.as_ptr());
            }
            Guard
        }

        #[inline(always)]
        pub fn new_with_data(name: &'static CStr, data: impl ProfileDataParam) -> Self {
            if let Some(data) = data.get_cstr() {
                // If we can cheaply get the input as a cstr we're golden
                unsafe {
                    raw::SHIM_PIXBeginEvent_N_D(
                        aleph_pix::Colour::MAGENTA.into(),
                        name.as_ptr(),
                        data.as_ptr(),
                    );
                }
                Guard
            } else {
                // TODO: PERF - Could we get gains with stack allocation? Avoid hitting the global
                //              heap? Ideal for less overhead when profiling
                let data = data.get_cstring();
                unsafe {
                    raw::SHIM_PIXBeginEvent_N_D(
                        aleph_pix::Colour::MAGENTA.into(),
                        name.as_ptr(),
                        data.as_ptr(),
                    );
                }
                Guard
            }
        }
    }

    impl Drop for Guard {
        #[inline(always)]
        fn drop(&mut self) {
            unsafe {
                raw::SHIM_PIXEndEvent_N();
            }
        }
    }
}
