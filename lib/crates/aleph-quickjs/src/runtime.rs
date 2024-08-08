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
pub struct Runtime {
    /// We ref-count the runtime so we can't leave dangling contexts. Overhead is low as Runtime is
    /// pinned to a thread
    pub(crate) rt: Rc<InnerRuntime>,
}

impl Runtime {
    pub fn new() -> Option<Self> {
        unsafe {
            let rt = raw::JS_NewRuntime()?;
            Some(Self {
                rt: Rc::new(InnerRuntime { rt }),
            })
        }
    }

    pub fn new_context(&self) -> Option<Context> {
        unsafe {
            let ctx = raw::JS_NewContext(self.rt.rt)?;
            Some(Context {
                ctx: Rc::new(InnerContext {
                    rt: self.clone(),
                    ctx,
                }),
            })
        }
    }

    pub fn get_raw(&self) -> NonNull<raw::JSRuntime> {
        self.rt.rt
    }
}

pub(crate) struct InnerRuntime {
    /// The inner runtime pointer
    pub rt: NonNull<raw::JSRuntime>,
}

impl Drop for InnerRuntime {
    fn drop(&mut self) {
        unsafe {
            raw::JS_FreeRuntime(self.rt);
        }
    }
}
