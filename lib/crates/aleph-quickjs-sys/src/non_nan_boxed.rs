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

use crate::JSTag;
use std::ffi::*;
use std::hash::{Hash, Hasher};

#[repr(C)]
#[derive(Copy, Clone)]
union JSValueUnion {
    pub int32: i32,
    pub float64: f64,
    pub ptr: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct JSValue {
    u: JSValueUnion,
    tag: i64,
}

impl JSValue {
    pub(crate) const __NAN: JSValue = JSValue {
        u: JSValueUnion { float64: f64::NAN },
        tag: JSTag::FLOAT64.0 as i64,
    };

    pub(crate) const fn __new_val(tag: JSTag, v: i32) -> JSValue {
        let u = JSValueUnion { int32: v };
        JSValue {
            u,
            tag: tag.0 as i64,
        }
    }

    pub(crate) const fn __new_ptr(tag: JSTag, v: *mut c_void) -> JSValue {
        let u = JSValueUnion { ptr: v };
        JSValue {
            u,
            tag: tag.0 as i64,
        }
    }

    pub(crate) const fn __new_float64(d: f64) -> JSValue {
        JSValue {
            u: JSValueUnion { float64: d },
            tag: JSTag::FLOAT64.0 as i64,
        }
    }

    pub(crate) const fn __get_tag(&self) -> JSTag {
        JSTag(self.tag as c_int)
    }

    pub(crate) const fn __get_norm_tag(&self) -> JSTag {
        self.__get_tag()
    }

    pub(crate) const fn __get_int(&self) -> i32 {
        unsafe { self.u.int32 }
    }

    pub(crate) const fn __get_bool(&self) -> bool {
        self.__get_int() != 0
    }

    pub(crate) const fn __get_float64(&self) -> f64 {
        unsafe { self.u.float64 }
    }

    pub(crate) const fn __get_ptr(&self) -> *mut c_void {
        unsafe { self.u.ptr }
    }

    pub(crate) const fn __is_nan(&self) -> bool {
        if self.tag != (JSTag::FLOAT64.0 as i64) {
            false
        } else {
            let d = unsafe { self.u.float64 };
            let d: u64 = unsafe { std::mem::transmute(d) };
            (d & 0x7fffffffffffffff) > 0x7ff0000000000000
        }
    }
}

impl PartialEq for JSValue {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.tag == other.tag && unsafe { self.u.ptr == other.u.ptr }
    }
}

impl Eq for JSValue {}

impl Hash for JSValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            let v = self.u.ptr as usize;
            Hash::hash(&v, state);
        }
        Hash::hash(&self.tag, state);
    }
}
