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

use std::cmp::Ordering;
use std::ffi::{CStr, CString};
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};

/// Shorthand utility for constructing a new [NStr] from a string literal. Assumes that no null
/// terminator was provided and inserts one on the caller's behalf
#[macro_export]
macro_rules! nstr {
    ($v:expr) => {
        $crate::NStr::new_str(concat!($v, "\0"))
    };
}

/// A utf8 string type that is a valid `str` but is guaranteed to contain a null
/// terminator at the end of the string. This type is guaranteed to be both a valid `str` and a
/// valid `CStr`, with accessors to convert to any of the compatible string types at zero cost.
///
/// These accessors are also const compatible.
pub struct NStr([u8]);

impl NStr {
    #[inline]
    pub const fn new_cstr(v: &CStr) -> &NStr {
        if let Some(v) = Self::from_cstr(v) {
            v
        } else {
            panic!("Attempting to construct NStr with invalid input!");
        }
    }

    #[inline]
    pub const fn new_str(v: &str) -> &NStr {
        if let Some(v) = Self::from_str(v) {
            v
        } else {
            panic!("Attempting to construct NStr with invalid input!");
        }
    }

    #[inline]
    pub const fn from_cstr(v: &CStr) -> Option<&NStr> {
        if let Ok(_s) = v.to_str() {
            // Safety: This is hoisted directly from the implementation from
            //         CStr::from_bytes_with_nul so if this isn't safe then we're hosed because
            //         neither is the standard library
            Some(unsafe { &*(v as *const CStr as *const NStr) })
        } else {
            None
        }
    }

    #[inline]
    pub const fn from_str(v: &str) -> Option<&NStr> {
        let bytes = v.as_bytes();
        if CStr::from_bytes_with_nul(bytes).is_ok() {
            // Safety: This is hoisted directly from the implementation from
            //         CStr::from_bytes_with_nul so if this isn't safe then we're hosed because
            //         neither is the standard library
            Some(unsafe { &*(bytes as *const [u8] as *const NStr) })
        } else {
            None
        }
    }

    #[inline]
    pub const fn from_bytes(v: &[u8]) -> Option<&NStr> {
        // Empty strings are invalid
        if v.is_empty() {
            return None;
        }

        if let Ok(v) = std::str::from_utf8(v) {
            Self::from_str(v)
        } else {
            None
        }
    }

    #[inline]
    pub const fn to_str(&self) -> &str {
        // Safety: It's illegal to construct a NStr that isn't a null terminated string so there
        //         will always be a zero byte to drop. Sometimes we will give out the empty string
        //         though, but that is 100% okay.
        let bytes = self.to_bytes();
        unsafe {
            let bytes = std::slice::from_raw_parts(bytes.as_ptr(), bytes.len() - 1);
            std::str::from_utf8_unchecked(bytes)
        }
    }

    #[inline]
    pub const fn to_cstr_ptr(&self) -> *const std::ffi::c_char {
        self.0.as_ptr() as *const std::ffi::c_char
    }

    #[inline]
    pub const fn to_str_with_nul(&self) -> &str {
        // Safety: This is hoisted directly from the implementation from
        //         CStr::from_bytes_with_nul so if this isn't safe then we're hosed because
        //         neither is the standard library.
        //
        //         It is illegal to construct an NStr that isn't also a valid string.
        unsafe { &*((&self.0) as *const [u8] as *const str) }
    }

    #[inline]
    pub const fn to_cstr(&self) -> &CStr {
        // Safety: This is hoisted directly from the implementation from
        //         CStr::from_bytes_with_nul so if this isn't safe then we're hosed because
        //         neither is the standard library
        //
        //         It is illegal to construct an NStr that isn't also a valid CStr.
        unsafe { &*((&self.0) as *const [u8] as *const CStr) }
    }

    #[inline]
    pub const fn to_bytes(&self) -> &[u8] {
        // Safety: This is hoisted directly from the implementation from
        //         CStr::from_bytes_with_nul so if this isn't safe then we're hosed because
        //         neither is the standard library
        //
        //         It is illegal to construct an NStr that isn't also a valid CStr.
        unsafe { &*((&self.0) as *const [u8]) }
    }

    /// Returns the length of the string, not including the null terminator.
    #[inline]
    pub const fn len(&self) -> usize {
        self.0.len() - 1
    }

    /// Returns true if the string has a length of 0
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<'a> aleph_profile::ProfileDataParam<'a> for &'a NStr {
    #[inline(always)]
    fn get_str(self) -> &'a str {
        self.to_str()
    }

    #[inline(always)]
    fn get_cstr(self) -> Option<&'a CStr> {
        Some(NStr::to_cstr(self))
    }

    #[inline(always)]
    fn get_cstring(self) -> CString {
        let cstr = NStr::to_cstr(self);
        CString::from(cstr)
    }
}

impl Debug for NStr {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self.to_str(), f)
    }
}

impl Display for NStr {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self.to_str(), f)
    }
}

impl<'a> Into<&'a str> for &'a NStr {
    #[inline]
    fn into(self) -> &'a str {
        self.to_str()
    }
}

impl<'a> Into<&'a CStr> for &'a NStr {
    #[inline]
    fn into(self) -> &'a CStr {
        self.to_cstr()
    }
}

impl PartialEq for NStr {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        let lhs = self.to_str();
        let rhs = other.to_str();
        str::eq(lhs, rhs)
    }
}

impl Eq for NStr {}

impl PartialOrd for NStr {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NStr {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        let lhs = self.to_str();
        let rhs = other.to_str();
        str::cmp(lhs, rhs)
    }
}

impl Hash for NStr {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        let this = self.to_str();
        Hash::hash(this, state)
    }
}

#[cfg(test)]
mod tests;
