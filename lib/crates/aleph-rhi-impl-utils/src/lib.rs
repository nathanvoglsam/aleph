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

#[doc(hidden)]
pub extern crate aleph_any;

#[doc(hidden)]
pub extern crate aleph_rhi_api;

use std::any::TypeId;
use std::ffi::{c_char, CStr};

pub mod bump_cell;
pub mod conv;
pub mod macros;
pub mod manually_drop;
pub mod unwrap;

pub unsafe fn try_clone_value_into_slot<T: Clone + Sized + 'static>(
    src: &T,
    out: *mut (),
    expecting: TypeId,
) -> Option<()> {
    if expecting == TypeId::of::<T>() {
        let out = out as *mut T;
        out.write(src.clone());

        Some(())
    } else {
        None
    }
}

#[inline(always)]
pub unsafe fn str_from_ptr<'a>(v: *const c_char) -> &'a str {
    CStr::from_ptr(v).to_str().unwrap()
}
