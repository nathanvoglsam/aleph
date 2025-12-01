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
use raw::{JSEvalOptions, JSTag, JSValue};

use crate::opaque_box::{OpaqueBox, UntypedOpaqueBox};
use crate::runtime::ThreadLocalRuntime;
use crate::{ArgValue, Atom, CtxString, OwnPropertyNames, RefValue, WeakValue};

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
        ThreadLocalRuntime::gc();
    }

    /// Query the memory usage from the runtime the context was created from.
    #[inline]
    pub fn compute_memory_usage(&self) -> raw::JSMemoryUsage {
        ThreadLocalRuntime::compute_memory_usage()
    }

    /// Direct wrapper over 'JS_Eval'.
    #[inline]
    pub fn eval(&self, script: &NStr, filename: &NStr, opts: raw::JSEvalFlags) -> RefValue {
        unsafe {
            let options = JSEvalOptions {
                eval_flags: opts,
                filename: filename.to_cstr_ptr(),
                line_num: 0,
                ..Default::default()
            };
            let v = raw::JS_Eval2(self.0.ctx, script.to_cstr_ptr(), script.len(), &options);
            RefValue(v)
        }
    }

    /// Returns the global object [`Object`] for this context.
    #[inline]
    pub fn get_global_object(&self) -> RefValue {
        unsafe {
            let v = raw::JS_GetGlobalObject(self.0.ctx);
            RefValue(v)
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
            RefValue(v)
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
            RefValue(v)
        }
    }

    #[inline]
    pub fn new_object(&self) -> RefValue {
        unsafe {
            let v = raw::JS_NewObject(self.0.ctx);
            RefValue(v)
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
            RefValue(v)
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

    #[inline]
    pub fn to_string(&self, v: &WeakValue) -> RefValue {
        unsafe {
            let string = raw::JS_ToString(self.0.ctx, v.0);
            RefValue(string)
        }
    }

    #[inline]
    pub fn to_c_str(&self, v: &WeakValue) -> Option<CtxString> {
        unsafe {
            let mut len = 0;
            let cstr = raw::JS_ToCStringLen2(self.0.ctx, &mut len, v.0, false);

            if len == 0 || cstr.is_null() {
                None
            } else {
                let bytes = std::slice::from_raw_parts(cstr as *const u8, len);
                let string = str::from_utf8(bytes).unwrap_unchecked();
                Some(CtxString::from_ctx_and_str(self.clone(), string))
            }
        }
    }

    #[inline]
    pub fn get_property_str(&self, v: &WeakValue, prop: &str) -> RefValue {
        match self.new_atom(prop) {
            Some(a) => self.get_property(v, &a),
            _ => {
                panic!()
            }
        }
    }

    #[inline]
    pub fn get_property(&self, v: &WeakValue, prop: &Atom) -> RefValue {
        unsafe {
            let v = raw::JS_GetProperty(self.0.ctx, v.0, prop.v);
            RefValue(v)
        }
    }

    #[inline]
    pub fn set_property_str(&self, v: &WeakValue, prop: &str, set: RefValue) -> c_int {
        match self.new_atom(prop) {
            Some(a) => self.set_property(v, &a, set),
            _ => {
                panic!()
            }
        }
    }

    #[inline]
    pub fn set_property(&self, v: &WeakValue, prop: &Atom, set: RefValue) -> c_int {
        unsafe {
            let set = set.detatch();
            raw::JS_SetProperty(self.0.ctx, v.0, prop.v, set)
        }
    }

    #[inline]
    pub fn delete_property_str(&self, v: &WeakValue, prop: &str) -> c_int {
        match self.new_atom(prop) {
            Some(a) => self.delete_property(v, &a),
            _ => {
                panic!()
            }
        }
    }

    #[inline]
    pub fn delete_property(&self, v: &WeakValue, prop: &Atom) -> c_int {
        unsafe {
            raw::JS_DeleteProperty(self.0.ctx, v.0, prop.v, 0) // TODO: flags
        }
    }

    #[inline]
    pub fn call(&self, f: &WeakValue, this: &WeakValue, args: &[ArgValue]) -> RefValue {
        use std::mem::{align_of, size_of};
        unsafe {
            assert_eq!(size_of::<ArgValue>(), size_of::<JSValue>());
            assert_eq!(align_of::<JSValue>(), align_of::<JSValue>());

            let argc: c_int = args.len().try_into().unwrap();
            let argv: *mut JSValue = if !args.is_empty() {
                NonNull::from(args).cast().as_ptr()
            } else {
                std::ptr::null_mut()
            };

            let v = raw::JS_Call(self.0.ctx, f.0, this.0, argc, argv);
            RefValue(v)
        }
    }

    #[inline]
    pub fn get_own_property_names(
        &self,
        v: &WeakValue,
        opts: raw::JSGetPropertyNameOption,
    ) -> OwnPropertyNames {
        unsafe {
            let mut tab = std::ptr::null_mut();
            let mut len = 0;
            let result = raw::JS_GetOwnPropertyNames(self.0.ctx, &mut tab, &mut len, v.0, opts);
            if result < 0 {
                panic!("Don't know how to handle exceptions yet");
            }

            let slice = if len > 0 {
                let tab = NonNull::new(tab).unwrap();
                NonNull::slice_from_raw_parts(tab, len as usize)
            } else {
                NonNull::slice_from_raw_parts(NonNull::dangling(), 0)
            };

            OwnPropertyNames {
                ctx: self.clone(),
                props: slice,
            }
        }
    }

    pub fn to_json(&self, v: &WeakValue) -> Option<serde_json::Value> {
        let v = match v.get_tag() {
            JSTag::BIG_INT => unimplemented!(),
            JSTag::SYMBOL => unimplemented!(),
            JSTag::STRING => {
                let string = self.to_c_str(v)?;
                let string = string.to_owned();
                serde_json::Value::String(string)
            }
            JSTag::MODULE => unimplemented!(),
            JSTag::FUNCTION_BYTECODE => unimplemented!(),
            JSTag::OBJECT => {
                if v.is_array() {
                    let opts = raw::JSGetPropertyNameOption::STRING_MASK
                        | raw::JSGetPropertyNameOption::ENUM_ONLY;
                    let props = self.get_own_property_names(v, opts);

                    let mut array = Vec::new();
                    for prop in props.iter() {
                        let atom = prop.atom.as_ref()?;
                        let value = self.get_property(v, atom);
                        if let Some(value) = self.to_json(&value) {
                            // let name = atom.to_c_str()?;
                            array.push(value);
                        }
                    }

                    serde_json::Value::Array(array)
                } else {
                    let opts = raw::JSGetPropertyNameOption::STRING_MASK
                        | raw::JSGetPropertyNameOption::ENUM_ONLY;
                    let props = self.get_own_property_names(v, opts);

                    let mut object = serde_json::Map::new();
                    for prop in props.iter() {
                        let atom = prop.atom.as_ref()?;
                        let value = self.get_property(v, atom);
                        if let Some(value) = self.to_json(&value) {
                            let name = atom.to_c_str()?;
                            object.insert(name.to_string(), value);
                        }
                    }

                    serde_json::Value::Object(object)
                }
            }
            JSTag::BOOL => unsafe {
                let boolean = v.get_bool().unwrap_unchecked();
                serde_json::Value::Bool(boolean)
            },
            JSTag::NULL => serde_json::Value::Null,
            JSTag::UNDEFINED => return None,
            JSTag::UNINITIALIZED => unimplemented!(),
            JSTag::CATCH_OFFSET => unimplemented!(),
            JSTag::EXCEPTION => return None,
            JSTag::INT => unsafe {
                let number = v
                    .get_number()
                    .unwrap_unchecked()
                    .get_int()
                    .unwrap_unchecked();
                let number = serde_json::to_value(number).unwrap();
                assert!(number.is_i64() || number.is_u64());
                number
            },
            JSTag::FLOAT64 => unsafe {
                let number = v
                    .get_number()
                    .unwrap_unchecked()
                    .get_double()
                    .unwrap_unchecked();
                serde_json::to_value(number).unwrap()
            },
            _ => unimplemented!(),
        };
        Some(v)
    }
}

pub struct InnerContext {
    pub(crate) ctx: NonNull<raw::JSContext>,
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
