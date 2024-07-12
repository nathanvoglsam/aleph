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
    ($name:literal) => {
        let _superluminal_guard = $crate::detail::Guard::new($name);
    };
    ($name:literal, $data:expr) => {
        let _superluminal_guard = $crate::detail::Guard::new_with_data($name, $data);
    };
}

#[macro_export]
macro_rules! register_thread {
    ($name:expr) => {
        $crate::superluminal_perf::set_current_thread_name($name);
    };
}

#[macro_export]
macro_rules! finish_frame {
    () => {
        // superluminal does not have a frame end function
    };
}

//
// RAII wrappers to support superluminal. These are public as they need to be callable from macros
// but are not intended for direct use.
//
#[doc(hidden)]
pub mod detail {
    pub struct Guard;

    // 0xFFFFFFFF means "use default color"
    const DEFAULT_SUPERLUMINAL_COLOR: u32 = 0xFFFFFFFF;

    impl Guard {
        #[inline(always)]
        pub fn new(name: &'static str) -> Self {
            superluminal_perf::begin_event(name);
            Guard
        }

        #[inline(always)]
        pub fn new_with_data(name: &'static str, data: &str) -> Self {
            superluminal_perf::begin_event_with_data(name, data, DEFAULT_SUPERLUMINAL_COLOR);
            Guard
        }
    }

    impl Drop for Guard {
        #[inline(always)]
        fn drop(&mut self) {
            superluminal_perf::end_event();
        }
    }
}
