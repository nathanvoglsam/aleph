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

use std::ops::Deref;

use crate::Context;

pub struct CtxString<'a> {
    ctx: &'a Context<'a>,
    v: *const str,
}

impl<'a> CtxString<'a> {
    /// Constructs a new [`CtxString`] from the given context and str.
    ///
    /// # Safety
    ///
    /// It is the caller's responsibility to ensure that the given string 'v' is live and was
    /// allocated from the given [`Context`]. [`CtxString`] will take ownership of the string and
    /// the [`Drop`] implementation will free the
    pub const unsafe fn from_ctx_and_str<'b>(ctx: &'a Context, v: &'b str) -> Self {
        Self {
            ctx,
            v: v as *const str,
        }
    }

    /// Leak the [`CtxString`] to get a bare string object. The lifetime is still tied to the
    /// context but you don't need to carry the [`Context`] reference around. The runtime will
    /// only reclaim the memory when the runtime is destroyed.
    ///
    /// For all intents and purposes this leaks the memory until you manually free the string.
    pub const fn leak(self) -> &'a str {
        let out = self.v;
        std::mem::forget(self);

        // Safety: This is safe as a [`CtxString`] is required to own the string slice. Leaking it
        //         gives it a lifetime tied to the context.
        unsafe { &*out }
    }
}

impl<'a> Drop for CtxString<'a> {
    fn drop(&mut self) {
        // Safety: It is unsafe to construct a 'CtxString' with the incorrect ctx and dead string
        //         so we can assume this is safe
        unsafe { self.ctx.free_c_str(&*self.v) }
    }
}

impl<'a> AsRef<str> for CtxString<'a> {
    #[inline]
    fn as_ref<'b>(&'b self) -> &'b str {
        // Safety: It is unsafe for a caller to construct a CtxString where this operation is unsafe
        unsafe { &*self.v }
    }
}

impl<'a> Deref for CtxString<'a> {
    type Target = str;

    #[inline]
    fn deref<'b>(&'b self) -> &'b Self::Target {
        self.as_ref()
    }
}
