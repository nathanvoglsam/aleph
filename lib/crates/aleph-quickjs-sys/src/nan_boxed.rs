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

use std::ffi::*;
use std::mem::size_of;

use crate::JSTag;

const JS_FLOAT64_TAG_ADDEND: u64 = 0x7ff80000 - (JSTag::FIRST.0 as u64) + 1;

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct JSValue(pub u64);

impl JSValue {
    pub(crate) const __NAN: JSValue = JSValue(0x7ff8000000000000 - (JS_FLOAT64_TAG_ADDEND << 32));

    pub(crate) const fn __new_val(tag: JSTag, v: i32) -> JSValue {
        let tag = (tag.0 as u64) << 32;
        JSValue(tag | (v as u64))
    }

    pub(crate) const fn __new_ptr(tag: JSTag, v: *mut c_void) -> JSValue {
        assert!(size_of::<usize>() <= size_of::<JSValue>());
        let tag = (tag.0 as u64) << 32;
        let v = v as u64;
        JSValue(tag | v)
    }

    pub(crate) const fn __new_float64(d: f64) -> JSValue {
        // normalize NaN
        let v: u64 = unsafe { std::mem::transmute(d) };
        if (v & 0x7fffffffffffffff) > 0x7ff0000000000000 {
            Self::__NAN
        } else {
            JSValue(v - (JS_FLOAT64_TAG_ADDEND << 32))
        }
    }

    pub(crate) const fn __get_tag(&self) -> JSTag {
        JSTag((self.0 >> 32) as c_int)
    }

    pub(crate) const fn __get_norm_tag(&self) -> JSTag {
        let tag = self.__get_tag();
        if js_tag_is_float64(tag) {
            JSTag::FLOAT64
        } else {
            tag
        }
    }

    pub(crate) const fn __get_int(&self) -> i32 {
        self.0 as c_int
    }

    pub(crate) const fn __get_bool(&self) -> bool {
        self.0 != 0
    }

    pub(crate) const fn __get_float64(&self) -> f64 {
        let v = self.0;
        let v = v + (JS_FLOAT64_TAG_ADDEND << 32);
        return unsafe { std::mem::transmute(v) };
    }

    pub(crate) const fn __get_ptr(&self) -> *mut c_void {
        assert!(size_of::<usize>() <= size_of::<JSValue>());
        self.0 as usize as *mut c_void
    }

    pub(crate) const fn __is_nan(&self) -> bool {
        let tag = self.__get_tag();
        tag.0 == (Self::__NAN.0 >> 32) as c_int
    }
}

const fn js_tag_is_float64(tag: JSTag) -> bool {
    ((tag.0 - JSTag::FIRST.0) as c_uint) >= (JSTag::FLOAT64.0 - JSTag::FIRST.0) as c_uint
}
