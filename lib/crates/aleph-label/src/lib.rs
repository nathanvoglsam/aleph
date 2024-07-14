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

use std::ffi::{CStr, CString};

#[macro_export]
macro_rules! make_label {
    ($v:literal) => {
        unsafe { $crate::Label::new(concat!($v, "\0")) }
    };
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Label(&'static str);

impl Label {
    /// Constructs a new [Label2] instance from the given string.
    ///
    /// # Safety
    ///
    /// The given string __must__ be static string literal. A string allocated on the heap and
    /// leaked is _not_ safe to use and violates assumptions users of [Label2] are allowed to make.
    ///
    /// The label must also be null-terminated. An empty string is encoded by a single null byte.
    ///
    /// # Why
    ///
    /// These labels may be passed to profiling instrumentation that requires string literals to
    /// be used.
    #[inline]
    pub const unsafe fn new(v: &'static str) -> Self {
        assert!(v.len() >= 1);
        match CStr::from_bytes_with_nul(v.as_bytes()) {
            Ok(_) => {}
            Err(_e) => {
                panic!("Label is not null terminated!");
            }
        }
        Self(v)
    }

    #[inline]
    pub const fn to_str(self) -> &'static str {
        // Safety: It's illegal to construct a Label that isn't a null terminated string so there
        //         will always be a zero byte to drop. Sometimes we will give out the empty string
        //         though, but that is 100% okay.
        unsafe {
            let bytes = self.0.as_bytes();
            let bytes = CStr::from_bytes_with_nul_unchecked(bytes);

            match bytes.to_str() {
                Ok(v) => v,
                Err(_) => {
                    unreachable!()
                }
            }
        }
    }

    #[inline]
    pub const fn to_str_with_nul(self) -> &'static str {
        self.0
    }

    #[inline]
    pub const fn to_cstr(self) -> &'static CStr {
        // Safety: It's illegal to construct a Label that isn't a valid CStr
        unsafe { CStr::from_bytes_with_nul_unchecked(self.0.as_bytes()) }
    }
}

impl Default for Label {
    #[inline(always)]
    fn default() -> Self {
        make_label!("")
    }
}

impl std::fmt::Debug for Label {
    #[inline(always)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self.to_str(), f)
    }
}

impl std::fmt::Display for Label {
    #[inline(always)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.to_str(), f)
    }
}

impl aleph_profile::ProfileDataParam<'static> for Label {
    #[inline(always)]
    fn as_str(self) -> &'static str {
        self.to_str()
    }

    #[inline(always)]
    fn as_cstr(self) -> Option<&'static CStr> {
        Some(Label::to_cstr(self))
    }

    #[inline(always)]
    fn to_cstr(self) -> CString {
        let cstr = Label::to_cstr(self);
        CString::from(cstr)
    }
}

impl<'a> aleph_profile::ProfileDataParam<'a> for &'a Label {
    #[inline(always)]
    fn as_str(self) -> &'a str {
        self.to_str()
    }

    #[inline(always)]
    fn as_cstr(self) -> Option<&'a CStr> {
        Some(Label::to_cstr(*self))
    }

    #[inline(always)]
    fn to_cstr(self) -> CString {
        let cstr = Label::to_cstr(*self);
        CString::from(cstr)
    }
}
