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
use std::ffi::{c_char, c_int};
use std::mem::transmute;
use std::ops::Deref;
use std::ptr::NonNull;

use aleph_nstr::NStr;
use raw::{JSEvalOptions, JSTag, JSValue};

use crate::{
    ArgValue, Atom, HostFn, HostFnCombineFloat, HostFnData, HostFnMagic, HostFnMapFloat,
    OwnPropertyNames, RefValue, Runtime, RuntimeString, WeakValue, host_fn_combine_float_arg_num,
    host_fn_map_float_arg_num,
};

pub struct Context {
    pub(crate) ctx: WeakContext,
    pub(crate) r: Runtime,
}

impl Clone for Context {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            ctx: WeakContext { c: self.ctx.c },
            r: self.r.clone(),
        }
    }
}

impl Deref for Context {
    type Target = WeakContext;

    fn deref(&self) -> &Self::Target {
        &self.ctx
    }
}

impl Drop for Context {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            raw::JS_FreeContext(self.ctx.c);
        }
    }
}

#[repr(transparent)]
pub struct WeakContext {
    pub(crate) c: NonNull<raw::JSContext>,
}

impl WeakContext {
    /// Trigger a manual garbage collection cycle.
    ///
    /// # Details
    ///
    /// This will trigger a GC cycle in the _runtime_ this context was created from. This may free
    /// memory for objects from other contexts too. The allocator is shared to contexts from the
    /// runtime.
    pub fn gc(&self) {
        unsafe {
            let rt = raw::JS_GetRuntime(self.c).unwrap_unchecked();
            raw::JS_RunGC(rt);
        }
    }

    /// Query the memory usage from the runtime the context was created from.
    #[inline]
    pub fn compute_memory_usage(&self) -> raw::JSMemoryUsage {
        unsafe {
            let rt = raw::JS_GetRuntime(self.c).unwrap_unchecked();
            let mut usage = raw::JSMemoryUsage::default();
            raw::JS_ComputeMemoryUsage(rt, &mut usage);
            usage
        }
    }

    /// Returns the inner [`raw::JSContext`].
    #[inline]
    pub fn to_raw(&self) -> NonNull<raw::JSContext> {
        self.c
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
            let v = raw::JS_Eval2(self.c, script.to_cstr_ptr(), script.len(), &options);
            RefValue::new(v)
        }
    }

    /// Returns the global object [`RefValue`] for this context.
    #[inline]
    pub fn get_global_object(&self) -> RefValue {
        unsafe {
            let v = raw::JS_GetGlobalObject(self.c);
            RefValue::new(v)
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
            let v = raw::JS_GetException(self.c);
            RefValue::new(v)
        }
    }

    #[inline]
    pub fn new_atom(&self, s: &str) -> Option<Atom> {
        unsafe {
            assert!(s.is_ascii());
            let atom = raw::JS_NewAtomLen(self.c, s.as_ptr() as *const _, s.len())?;
            Some(Atom { v: atom })
        }
    }

    #[inline]
    pub fn new_string(&self, v: &str) -> RefValue {
        unsafe {
            let v = raw::JS_NewStringLen(self.c, v.as_ptr() as *const c_char, v.len());
            RefValue::new(v)
        }
    }

    #[inline]
    pub fn new_object(&self) -> RefValue {
        unsafe {
            let v = raw::JS_NewObject(self.c);
            RefValue::new(v)
        }
    }

    #[inline]
    pub fn new_host_function<const ARGS: usize>(
        &self,
        func: HostFn<ARGS>,
        name: &NStr,
    ) -> RefValue {
        unsafe {
            let v = raw::JS_NewCFunction2(
                self.c,
                func.0,
                name.to_cstr_ptr(),
                ARGS as c_int,
                raw::JSCFunctionEnum::GENERIC,
                0,
            );
            RefValue::new(v)
        }
    }

    #[inline]
    pub fn new_host_function_magic<const ARGS: usize>(
        &self,
        func: HostFnMagic<ARGS>,
        name: &NStr,
        magic: c_int,
    ) -> RefValue {
        unsafe {
            let v = raw::JS_NewCFunction2(
                self.c,
                transmute(func.0),
                name.to_cstr_ptr(),
                ARGS as c_int,
                raw::JSCFunctionEnum::GENERIC_MAGIC,
                magic,
            );
            RefValue::new(v)
        }
    }

    #[inline]
    pub fn new_host_function_data<const ARGS: usize, const DATAS: usize>(
        &self,
        func: HostFnData<ARGS, DATAS>,
        name: &NStr,
        magic: c_int,
        datas: &[ArgValue],
    ) -> RefValue {
        unsafe {
            assert_eq!(DATAS, datas.len());
            let len: c_int = datas.len().try_into().unwrap();
            let datas = datas.as_ptr() as *const _ as *mut raw::JSValueConst;
            let v = raw::JS_NewCFunctionData2(
                self.c,
                func.0,
                name.to_cstr_ptr(),
                ARGS as c_int,
                magic,
                len,
                datas,
            );
            RefValue::new(v)
        }
    }

    #[inline]
    pub fn new_host_function_map_float(&self, func: HostFnMapFloat, name: &NStr) -> RefValue {
        unsafe {
            let v = raw::JS_NewCFunction2(
                self.c,
                transmute(func.0),
                name.to_cstr_ptr(),
                host_fn_map_float_arg_num() as c_int,
                raw::JSCFunctionEnum::F_F,
                0,
            );
            RefValue::new(v)
        }
    }

    #[inline]
    pub fn new_host_function_combine_float(
        &self,
        func: HostFnCombineFloat,
        name: &NStr,
    ) -> RefValue {
        unsafe {
            let v = raw::JS_NewCFunction2(
                self.c,
                transmute(func.0),
                name.to_cstr_ptr(),
                host_fn_combine_float_arg_num() as c_int,
                raw::JSCFunctionEnum::F_F_F,
                0,
            );
            RefValue::new(v)
        }
    }

    #[inline]
    pub fn to_string(&self, v: &WeakValue) -> RefValue {
        unsafe {
            let string = raw::JS_ToString(self.c, v.0);
            RefValue::new(string)
        }
    }

    #[inline]
    pub fn to_c_str(&self, v: &WeakValue) -> Option<RuntimeString> {
        unsafe {
            let mut len = 0;
            let cstr = raw::JS_ToCStringLen2(self.c, &mut len, v.0, false);

            if len == 0 || cstr.is_null() {
                None
            } else {
                let bytes = std::slice::from_raw_parts(cstr as *const u8, len);
                let string = str::from_utf8(bytes).unwrap_unchecked();
                Some(RuntimeString::from_ctx_and_str(string))
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
            let v = raw::JS_GetProperty(self.c, v.0, prop.v);
            RefValue::new(v)
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
            raw::JS_SetProperty(self.c, v.0, prop.v, set)
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
            raw::JS_DeleteProperty(self.c, v.0, prop.v, 0) // TODO: flags
        }
    }

    #[inline]
    pub fn seal_object(&self, v: &WeakValue) -> c_int {
        unsafe { raw::JS_SealObject(self.c, v.0) }
    }

    #[inline]
    pub fn freeze_object(&self, v: &WeakValue) -> c_int {
        unsafe { raw::JS_FreezeObject(self.c, v.0) }
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

            let v = raw::JS_Call(self.c, f.0, this.0, argc, argv);
            RefValue::new(v)
        }
    }

    #[inline]
    pub fn get_own_property_names(
        &self,
        v: &WeakValue,
        opts: raw::JSGetPropertyNameOption,
    ) -> OwnPropertyNames<'_> {
        unsafe {
            let mut tab = std::ptr::null_mut();
            let mut len = 0;
            let result = raw::JS_GetOwnPropertyNames(self.c, &mut tab, &mut len, v.0, opts);
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
                ctx: self,
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
                            // let name = self.atom_to_c_str(atom)?;
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
                            let name = self.atom_to_c_str(atom)?;
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

    #[inline]
    pub fn atom_to_value(&self, atom: &Atom) -> RefValue {
        unsafe {
            let string = raw::JS_AtomToValue(self.c, atom.v);
            RefValue::new(string)
        }
    }

    #[inline]
    pub fn atom_to_string(&self, atom: &Atom) -> RefValue {
        unsafe {
            let string = raw::JS_AtomToString(self.c, atom.v);
            RefValue::new(string)
        }
    }

    #[inline]
    pub fn atom_to_c_str(&self, atom: &Atom) -> Option<RuntimeString> {
        let string = self.atom_to_string(atom);
        if !string.is_exception() {
            self.to_c_str(&string)
        } else {
            None
        }
    }
}
