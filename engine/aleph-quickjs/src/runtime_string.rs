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

use std::ffi::c_char;
use std::ops::Deref;

use crate::runtime::with_runtime;

pub struct RuntimeString {
    v: *const str,
}

impl RuntimeString {
    /// Constructs a new [`RuntimeString`] from the given str.
    ///
    /// # Safety
    ///
    /// It is the caller's responsibility to ensure that the given string 'v' is live and was
    /// allocated from the owning thread's [`Runtime`]. [`RuntimeString`] will take ownership of the
    /// string and the [`Drop`] implementation will free it.
    pub const unsafe fn from_ctx_and_str(v: &str) -> Self {
        Self { v: v as *const str }
    }
}

impl Drop for RuntimeString {
    fn drop(&mut self) {
        with_runtime(|rt| unsafe {
            raw::JS_FreeCStringRT(rt.0, (*self.v).as_ptr() as *const c_char)
        })
    }
}

impl AsRef<str> for RuntimeString {
    #[inline]
    fn as_ref(&self) -> &str {
        // Safety: It is unsafe for a caller to construct a CtxString where this operation is unsafe
        unsafe { &*self.v }
    }
}

impl Deref for RuntimeString {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}
