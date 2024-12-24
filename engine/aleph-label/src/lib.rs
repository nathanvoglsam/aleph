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
use std::fmt::{Debug, Display, Formatter};
use std::ops::Deref;

#[doc(hidden)]
pub use aleph_nstr::NStr;

#[macro_export]
macro_rules! make_label {
    ($v:literal) => {
        unsafe { $crate::Label::from_nstr($crate::NStr::new_str(concat!($v, "\0"))) }
    };
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Label(&'static NStr);

impl Label {
    /// # Safety
    /// It is the caller's responsibility to guarantee that the input string comes from a string
    /// literal.
    #[inline]
    pub const unsafe fn from_nstr(v: &'static NStr) -> Self {
        Self(v)
    }

    #[inline]
    pub const fn as_nstr(self) -> &'static NStr {
        self.0
    }
}

impl Deref for Label {
    type Target = NStr;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl Default for Label {
    #[inline(always)]
    fn default() -> Self {
        make_label!("")
    }
}

impl Debug for Label {
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self.to_str(), f)
    }
}

impl Display for Label {
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self.to_str(), f)
    }
}

impl aleph_profile::ProfileDataParam<'static> for Label {
    #[inline(always)]
    fn get_str(self) -> &'static str {
        self.0.to_str()
    }

    #[inline(always)]
    fn get_cstr(self) -> Option<&'static CStr> {
        Some(self.0.to_cstr())
    }

    #[inline(always)]
    fn get_cstring(self) -> CString {
        CString::from(self.0.to_cstr())
    }
}

impl<'a> aleph_profile::ProfileDataParam<'a> for &'a Label {
    #[inline(always)]
    fn get_str(self) -> &'a str {
        self.0.to_str()
    }

    #[inline(always)]
    fn get_cstr(self) -> Option<&'a CStr> {
        Some(self.0.to_cstr())
    }

    #[inline(always)]
    fn get_cstring(self) -> CString {
        CString::from(self.0.to_cstr())
    }
}
