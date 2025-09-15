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

use core::str;
use std::any::Any;
use std::ffi::{c_char, c_int, c_void};
use std::ptr::NonNull;
use std::rc::Rc;

use aleph_nstr::NStr;

use crate::opaque_box::{OpaqueBox, UntypedOpaqueBox};
use crate::{Atom, Object, RefValue, Runtime};

#[derive(Clone)]
pub struct Context(pub(crate) Rc<InnerContext>);

impl Context {
    /// Returns the inner [`raw::JSContext`].
    #[inline]
    pub fn to_raw(&self) -> NonNull<raw::JSContext> {
        self.0.ctx
    }

    /// Trigger a manual garbage collection cycle.
    ///
    /// # Details
    ///
    /// This will trigger a GC cycle in the _runtime_ this context was created from. This may free
    /// memory for objects from other contexts too. The allocator is shared to contexts from the
    /// runtime.
    pub fn gc(&self) {
        self.get_rt().gc();
    }

    /// Query the memory usage from the runtime the context was created from.
    #[inline]
    pub fn compute_memory_usage(&self) -> raw::JSMemoryUsage {
        self.get_rt().compute_memory_usage()
    }

    /// Direct wrapper over 'JS_Eval'.
    #[inline]
    pub fn eval(&self, script: &NStr, filename: &NStr, opts: raw::JSEvalOptions) -> RefValue {
        unsafe {
            let v = raw::JS_Eval(
                self.0.ctx,
                script.to_cstr_ptr(),
                script.len(),
                filename.to_cstr_ptr(),
                opts,
            );
            RefValue::from_raw(self, v)
        }
    }

    /// Returns the global object [`Object`] for this context.
    #[inline]
    pub fn get_global_object(&self) -> Object {
        unsafe {
            let v = raw::JS_GetGlobalObject(self.0.ctx);
            Object::from_raw(self, v).unwrap_unchecked()
        }
    }

    /// Take the current exception object, if one exists. Will return 'undefined' if there is no
    /// exception.
    ///
    /// This will take the exception from its slot inside the context, meaning it can only be
    /// fetched once. In the event an exception is thrown, back to back calls to this function
    /// will only yield the exception object for the first time with all future calls returning
    /// 'undefined'. This will continue to return 'undefined' until a new JS call fires another
    /// exception.
    #[inline]
    pub fn get_exception(&self) -> RefValue {
        unsafe {
            let v = raw::JS_GetException(self.0.ctx);
            RefValue::from_raw(self, v)
        }
    }

    #[inline]
    pub fn new_atom(&self, s: &str) -> Option<Atom> {
        unsafe {
            assert!(s.is_ascii());
            let atom = raw::JS_NewAtomLen(self.0.ctx, s.as_ptr() as *const _, s.len())?;
            Some(Atom {
                v: atom,
                c: self.clone(),
            })
        }
    }

    #[inline]
    pub fn new_string(&self, v: &str) -> RefValue {
        unsafe {
            let v = raw::JS_NewStringLen(self.0.ctx, v.as_ptr() as *const c_char, v.len());
            RefValue::from_raw(self, v)
        }
    }

    #[inline]
    pub fn new_object(&self) -> Object {
        unsafe {
            let v = raw::JS_NewObject(self.0.ctx);
            Object::from_raw(self, v).unwrap_unchecked()
        }
    }

    #[inline]
    pub fn new_c_function(
        &self,
        func: raw::JSCFunctionFn,
        name: &NStr,
        num_params: c_int,
    ) -> RefValue {
        unsafe {
            let v = raw::JS_NewCFunction(self.0.ctx, func, name.to_cstr_ptr(), num_params);
            RefValue::from_raw(self, v)
        }
    }

    #[inline]
    pub fn set_opaque<T: Any + Sized>(&self, v: T) {
        self.remove_opaque();

        unsafe {
            let opaque = OpaqueBox::new(v);
            raw::JS_SetContextOpaque(self.0.ctx, opaque.as_ptr() as *mut c_void);
        }
    }

    #[inline]
    pub fn get_opaque<T: Any + Sized>(&self) -> Option<&T> {
        unsafe {
            let old = raw::JS_GetContextOpaque(self.0.ctx);
            let old = NonNull::new(old);
            if let Some(old) = old {
                let old = old.cast::<UntypedOpaqueBox>().as_ref();
                old.try_to_typed::<T>().map(|v| &v.v)
            } else {
                None
            }
        }
    }

    #[inline]
    pub fn remove_opaque(&self) {
        unsafe {
            let old = raw::JS_GetContextOpaque(self.0.ctx);
            let old = NonNull::new(old);
            if let Some(old) = old {
                let old = old.cast::<UntypedOpaqueBox>();
                UntypedOpaqueBox::drop_inner(old);
            }

            raw::JS_SetContextOpaque(self.0.ctx, std::ptr::null_mut());
        }
    }
}

impl Context {
    /// Internal function used for getting the runtime the context was allocated from. The object
    /// should not be dropped.
    ///
    /// This is expected to be used to allow access to various functions on `Runtime` directly from
    /// the context object. Mainly to avoid code duplication.
    pub(crate) fn get_rt(&self) -> Runtime {
        self.0.rt.clone()
    }

    pub(crate) fn same_rt(&self, other: &Self) -> bool {
        self.0.rt.0.0 == other.0.rt.0.0
    }
}

pub struct InnerContext {
    pub(crate) ctx: NonNull<raw::JSContext>,
    pub(crate) rt: Runtime,
}

impl Drop for InnerContext {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            let old = raw::JS_GetContextOpaque(self.ctx);
            let old = NonNull::new(old);
            if let Some(old) = old {
                let old = old.cast::<UntypedOpaqueBox>();
                UntypedOpaqueBox::drop_inner(old);
            }

            raw::JS_FreeContext(self.ctx);
        }
    }
}
