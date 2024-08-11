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

use std::ffi::c_int;
use std::marker::PhantomData;
use std::mem::ManuallyDrop;
use std::ptr::NonNull;

use crate::{
    Atom, DupRawValue, GetRawValue, Object, OwnPropertyNames, RefValue, Runtime, ToRefValue,
};

use aleph_nstr::NStr;

pub struct Context<'a> {
    pub(crate) ctx: NonNull<raw::JSContext>,
    pub(crate) _phantom: PhantomData<&'a Runtime>,
}

impl<'a> Context<'a> {
    /// Returns the inner [`raw::JSContext`].
    pub const fn to_raw(&self) -> NonNull<raw::JSContext> {
        self.ctx
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
                self.ctx,
                script.to_cstr_ptr(),
                script.len(),
                filename.to_cstr_ptr(),
                opts,
            );
            RefValue::from_raw(v)
        }
    }

    /// Returns the global object [`Object`] for this context.
    ///
    /// # Memory
    ///
    /// Each call to [`Context::get_global_object`] will increment the reference count to the global
    /// object. Make sure to free the value, otherwise you will leak memory.
    #[inline]
    pub fn get_global_object(&self) -> Object {
        unsafe {
            let v = raw::JS_GetGlobalObject(self.ctx);
            Object::from_raw(v).unwrap_unchecked()
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
            let v = raw::JS_GetException(self.ctx);
            RefValue::from_raw(v)
        }
    }

    /// # Safety
    ///
    /// It is the caller's responsibility to ensure the JS value given is live and allocated
    /// from this context.
    #[inline]
    pub unsafe fn get_property_str(&self, this: &impl GetRawValue, prop: &NStr) -> RefValue {
        unsafe {
            if let Some(a) = self.new_atom(prop) {
                let v = self.get_property(this, &a);
                self.free_atom(a);
                v
            } else {
                panic!()
            }
        }
    }

    /// # Safety
    ///
    /// It is the caller's responsibility to ensure the JS value and atom is live and allocated
    /// from this context.
    #[inline]
    pub unsafe fn get_property(&self, this: &impl GetRawValue, prop: &Atom) -> RefValue {
        let this = this.get_raw_value();
        unsafe {
            let v = raw::JS_GetProperty(self.ctx, this, prop.to_raw());
            RefValue::from_raw(v)
        }
    }

    /// # Safety
    ///
    /// It is the caller's responsibility to ensure the JS values given are live and allocated
    /// from this context.
    #[inline]
    pub unsafe fn set_property_str(
        &self,
        this: &impl GetRawValue,
        prop: &NStr,
        v: &impl DupRawValue,
    ) -> c_int {
        unsafe {
            if let Some(a) = self.new_atom(prop) {
                let result = self.set_property(this, &a, v);
                self.free_atom(a);
                result
            } else {
                panic!()
            }
        }
    }

    /// # Safety
    ///
    /// It is the caller's responsibility to ensure the JS values and atom are live and allocated
    /// from this context.
    #[inline]
    pub unsafe fn set_property(
        &self,
        this: &impl GetRawValue,
        prop: &Atom,
        v: &impl DupRawValue,
    ) -> c_int {
        let this = this.get_raw_value();
        let v = v.dup_raw_value();
        unsafe { raw::JS_SetProperty(self.ctx, this, prop.to_raw(), v) }
    }

    /// # Safety
    ///
    /// It is the caller's responsibility to ensure the JS values given are live and allocated
    /// from this context.
    #[inline]
    pub unsafe fn delete_property_str(&self, this: &impl GetRawValue, prop: &NStr) -> c_int {
        unsafe {
            if let Some(a) = self.new_atom(prop) {
                let result = self.delete_property(this, &a);
                self.free_atom(a);
                result
            } else {
                panic!()
            }
        }
    }

    /// # Safety
    ///
    /// It is the caller's responsibility to ensure the JS values given are live and allocated
    /// from this context.
    #[inline]
    pub unsafe fn delete_property(&self, this: &impl GetRawValue, prop: &Atom) -> c_int {
        let this = this.get_raw_value();
        unsafe {
            raw::JS_DeleteProperty(self.ctx, this, prop.to_raw(), 0) // TODO: flags
        }
    }

    /// # Safety
    ///
    /// It is the caller's responsibility to ensure the [`raw::JSValue`] given is live and allocated
    /// from this context.
    #[inline]
    pub unsafe fn get_own_property_names(
        &self,
        obj: &impl GetRawValue,
        opts: raw::JSGetPropertyNameOption,
    ) -> OwnPropertyNames {
        let obj = obj.get_raw_value();
        unsafe {
            let mut tab = std::ptr::null_mut();
            let mut len = 0;
            let result = raw::JS_GetOwnPropertyNames(self.ctx, &mut tab, &mut len, obj, opts);
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
                ctx: self.ctx,
                props: slice,
                _phantom: Default::default(),
            }
        }
    }

    #[inline]
    pub fn new_atom(&self, s: &NStr) -> Option<Atom> {
        unsafe {
            let atom = raw::JS_NewAtomLen(self.ctx, s.to_cstr_ptr(), s.len());
            atom.map(|v| Atom::from_raw(v))
        }
    }

    #[inline]
    pub fn new_object(&self) -> Object {
        unsafe {
            let v = raw::JS_NewObject(self.ctx);
            Object::from_raw(v).unwrap_unchecked()
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
            let v = raw::JS_NewCFunction(self.ctx, func, name.to_cstr_ptr(), num_params);
            RefValue::from_raw(v)
        }
    }

    /// An explicit destructor for JS value wrappers. This is similar to the [`Drop`]
    /// implementations on our wrapper functions but will _immediately_ free the value if the ref
    /// count reaches zero. This will not trigger a GC cycle, rather the individual value will be
    /// explicitly freed.
    ///
    /// # Safety
    ///
    /// It is the caller's responsibility to ensure that the given [`RefValue`] was allocated from
    /// this context.
    #[inline]
    pub unsafe fn free_value<'b>(&self, v: impl ToRefValue<'b>) {
        let v = v.to_ref_value();

        v.to_raw().free_value(self.ctx);

        // Prevent double-freeing the 'RefValue'
        std::mem::forget(v)
    }

    /// # Safety
    ///
    /// It is the caller's responsibility to ensure that the given [`Atom`] is live and was
    /// allocated from this context.
    #[inline]
    pub unsafe fn free_atom<'b>(&self, a: Atom) {
        let v = a.to_raw();
        raw::JS_FreeAtom(self.ctx, v);
    }

    /// # Safety
    ///
    /// It is the caller's responsibility to ensure that the given [`Atom`] is live and was
    /// allocated from this context.
    #[inline]
    pub unsafe fn dup_atom(&self, a: &Atom) -> Option<Atom> {
        unsafe {
            let atom = raw::JS_DupAtom(self.ctx, a.to_raw());
            atom.map(|v| Atom::from_raw(v))
        }
    }
}

impl<'a> Context<'a> {
    /// Internal function used for getting the runtime the context was allocated from. The object
    /// should not be dropped.
    ///
    /// This is expected to be used to allow access to various functions on `Runtime` directly from
    /// the context object. Mainly to avoid code duplication.
    fn get_rt(&self) -> ManuallyDrop<Runtime> {
        unsafe {
            let rt = raw::JS_GetRuntime(self.ctx).unwrap_unchecked();
            ManuallyDrop::new(Runtime(rt))
        }
    }
}

impl<'a> Drop for Context<'a> {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            raw::JS_FreeContext(self.ctx);
        }
    }
}
