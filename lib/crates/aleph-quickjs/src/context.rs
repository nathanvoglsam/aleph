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

use crate::Runtime;

#[derive(Clone)]
pub struct Context {
    /// We ref-count the context so we can't leave dangling contexts. Overhead is low as Context is
    /// pinned to a thread
    pub(crate) ctx: Rc<InnerContext>,
}

impl Context {
    pub fn get_raw(&self) -> NonNull<raw::JSContext> {
        self.ctx.ctx
    }
}

pub(crate) struct InnerContext {
    /// Hold a reference to the runtime to keep it alive as long as any contexts created from it are
    /// still live too.
    pub rt: Runtime,

    /// The context itself
    pub ctx: NonNull<raw::JSContext>,
}

impl Drop for InnerContext {
    fn drop(&mut self) {
        unsafe {
            raw::JS_FreeContext(self.ctx);
        }
    }
}
