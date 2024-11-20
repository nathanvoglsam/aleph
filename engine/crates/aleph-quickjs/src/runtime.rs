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

use std::ptr::NonNull;
use std::rc::Rc;

use crate::context::InnerContext;
use crate::Context;

#[derive(Clone)]
pub struct Runtime(pub(crate) Rc<InnerRuntime>);

impl Runtime {
    #[inline]
    pub fn new() -> Option<Self> {
        unsafe {
            let rt = raw::JS_NewRuntime()?;
            let rt = InnerRuntime(rt);
            let rt = Rc::new(rt);
            Some(Self(rt))
        }
    }

    #[inline]
    pub fn to_raw(&self) -> NonNull<raw::JSRuntime> {
        self.0 .0
    }

    #[inline]
    pub fn new_context(&self) -> Option<Context> {
        unsafe {
            let ctx = raw::JS_NewContext(self.0 .0)?;
            let ctx = InnerContext {
                ctx,
                rt: self.clone(),
            };
            let ctx = Rc::new(ctx);
            Some(Context(ctx))
        }
    }

    #[inline]
    pub fn gc(&self) {
        unsafe {
            raw::JS_RunGC(self.0 .0);
        }
    }

    /// Query the memory usage from the runtime.
    #[inline]
    pub fn compute_memory_usage(&self) -> raw::JSMemoryUsage {
        unsafe {
            let mut usage = raw::JSMemoryUsage::default();
            raw::JS_ComputeMemoryUsage(self.0 .0, &mut usage);
            usage
        }
    }
}

pub(crate) struct InnerRuntime(pub(crate) NonNull<raw::JSRuntime>);

impl Drop for InnerRuntime {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            raw::JS_FreeRuntime(self.0);
        }
    }
}
