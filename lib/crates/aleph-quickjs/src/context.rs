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

use crate::{OwnPropertyNames, Runtime};
use aleph_nstr::NStr;

// TODO: JSValue are supposed to be ref counted. How should we handle this? Likely a newtype wrapper
//       with Rc semantics? Perhaps some newtype wrappers for pure value types like number which
//       aren't reference counted?
//
//       Problem is that freeing them requires access to the context. Which means we'd need to hold
//       on to a reference to the context for each JSValue. Ironically it would be easier with a
//       real GC and value rooting, at least as a caller and in this specific situation.

#[derive(Clone)]
pub struct Context {
    /// We ref-count the context so we can't leave dangling contexts. Overhead is low as Context is
    /// pinned to a thread
    pub(crate) ctx: Rc<InnerContext>,
}

impl Context {
    /// Direct wrapper over 'JS_Eval'.
    pub fn eval(&self, script: &NStr, filename: &NStr, opts: raw::JSEvalOptions) -> raw::JSValue {
        unsafe {
            raw::JS_Eval(
                self.ctx.ctx,
                script.to_cstr_ptr(),
                script.len(),
                filename.to_cstr_ptr(),
                opts,
            )
        }
    }

    pub fn get_global_object(&self) -> raw::JSValue {
        unsafe {
            raw::JS_GetGlobalObject(self.ctx.ctx)
        }
    }

    pub fn get_property_str(
        &self,
        this: raw::JSValue,
        prop: &NStr,
    ) -> raw::JSValue {
        unsafe {
            raw::JS_GetPropertyStr(self.ctx.ctx, this, prop.to_cstr_ptr())
        }
    }

    pub fn get_own_property_names(
        &self,
        obj: raw::JSValue,
        opts: raw::JSGetPropertyNameOption,
    ) -> OwnPropertyNames {
        unsafe {
            let mut tab = std::ptr::null_mut();
            let mut len = 0;
            let result = raw::JS_GetOwnPropertyNames(self.ctx.ctx, &mut tab, &mut len, obj, opts);
            if result < 0 {
                panic!("Don't know how to handle exceptions yet");
            }

            let slice = if len > 0 {
                let tab = NonNull::new(tab).unwrap();
                NonNull::slice_from_raw_parts(tab, len as usize)
            } else {
                NonNull::<[raw::JSPropertyEnum]>::slice_from_raw_parts(NonNull::dangling(), 0)
            };

            OwnPropertyNames {
                ctx: self.ctx.ctx,
                props: slice,
                _phantom: Default::default(),
            }
        }
    }

    pub fn get_exception(&self) -> raw::JSValue {
        unsafe { raw::JS_GetException(self.ctx.ctx) }
    }

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
