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
use std::fmt::{Debug, Formatter};
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
    pub fn eval(
        &self,
        script: &NStr,
        filename: &NStr,
        opts: raw::JSEvalFlags,
    ) -> Result<RefValue, Exception<'_>> {
        unsafe {
            let options = JSEvalOptions {
                eval_flags: opts,
                filename: filename.to_cstr_ptr(),
                line_num: 0,
                ..Default::default()
            };
            let v = raw::JS_Eval2(self.c, script.to_cstr_ptr(), script.len(), &options);
            self.maybe_exception(RefValue::new(v))
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
    pub fn get_exception(&self) -> Exception<'_> {
        unsafe {
            let v = raw::JS_GetException(self.c);
            Exception {
                v: RefValue::new(v),
                c: &self,
            }
        }
    }

    #[inline]
    pub fn new_atom(&self, s: &str) -> Option<Atom> {
        unsafe {
            let atom = raw::JS_NewAtomLen(self.c, s.as_ptr() as *const _, s.len())?;
            Some(Atom { v: atom })
        }
    }

    #[inline]
    pub fn new_string(&self, v: &str) -> Result<RefValue, Exception<'_>> {
        unsafe {
            let v = raw::JS_NewStringLen(self.c, v.as_ptr() as *const c_char, v.len());
            self.maybe_exception(RefValue::new(v))
        }
    }

    #[inline]
    pub fn new_object(&self) -> Result<RefValue, Exception<'_>> {
        unsafe {
            let v = raw::JS_NewObject(self.c);
            self.maybe_exception(RefValue::new(v))
        }
    }

    #[inline]
    pub fn new_host_function<const ARGS: usize>(
        &self,
        func: HostFn<ARGS>,
        name: &NStr,
    ) -> Result<RefValue, Exception<'_>> {
        unsafe {
            let v = raw::JS_NewCFunction2(
                self.c,
                func.0,
                name.to_cstr_ptr(),
                ARGS as c_int,
                raw::JSCFunctionEnum::GENERIC,
                0,
            );
            self.maybe_exception(RefValue::new(v))
        }
    }

    #[inline]
    pub fn new_host_function_magic<const ARGS: usize>(
        &self,
        func: HostFnMagic<ARGS>,
        name: &NStr,
        magic: c_int,
    ) -> Result<RefValue, Exception<'_>> {
        unsafe {
            let v = raw::JS_NewCFunction2(
                self.c,
                transmute(func.0),
                name.to_cstr_ptr(),
                ARGS as c_int,
                raw::JSCFunctionEnum::GENERIC_MAGIC,
                magic,
            );
            self.maybe_exception(RefValue::new(v))
        }
    }

    #[inline]
    pub fn new_host_function_data<const ARGS: usize, const DATAS: usize>(
        &self,
        func: HostFnData<ARGS, DATAS>,
        name: &NStr,
        magic: c_int,
        datas: &[ArgValue],
    ) -> Result<RefValue, Exception<'_>> {
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
            self.maybe_exception(RefValue::new(v))
        }
    }

    #[inline]
    pub fn new_host_function_map_float(
        &self,
        func: HostFnMapFloat,
        name: &NStr,
    ) -> Result<RefValue, Exception<'_>> {
        unsafe {
            let v = raw::JS_NewCFunction2(
                self.c,
                transmute(func.0),
                name.to_cstr_ptr(),
                host_fn_map_float_arg_num() as c_int,
                raw::JSCFunctionEnum::F_F,
                0,
            );
            self.maybe_exception(RefValue::new(v))
        }
    }

    #[inline]
    pub fn new_host_function_combine_float(
        &self,
        func: HostFnCombineFloat,
        name: &NStr,
    ) -> Result<RefValue, Exception<'_>> {
        unsafe {
            let v = raw::JS_NewCFunction2(
                self.c,
                transmute(func.0),
                name.to_cstr_ptr(),
                host_fn_combine_float_arg_num() as c_int,
                raw::JSCFunctionEnum::F_F_F,
                0,
            );
            self.maybe_exception(RefValue::new(v))
        }
    }

    #[inline]
    pub fn to_string(&self, v: &WeakValue) -> Result<RefValue, Exception<'_>> {
        unsafe {
            let v = raw::JS_ToString(self.c, v.0);
            self.maybe_exception(RefValue::new(v))
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
    pub fn get_property_str(&self, v: &WeakValue, prop: &str) -> Result<RefValue, Exception<'_>> {
        let a = self.new_atom(prop).ok_or(Exception::undefined(self))?;
        self.get_property(v, &a)
    }

    #[inline]
    pub fn get_property(&self, v: &WeakValue, prop: &Atom) -> Result<RefValue, Exception<'_>> {
        unsafe {
            let v = raw::JS_GetProperty(self.c, v.0, prop.v);
            self.maybe_exception(RefValue::new(v))
        }
    }

    #[inline]
    pub fn set_property_str(
        &self,
        v: &WeakValue,
        prop: &str,
        set: RefValue,
    ) -> Result<bool, Exception<'_>> {
        let a = self.new_atom(prop).ok_or(Exception::undefined(self))?;
        self.set_property(v, &a, set)
    }

    #[inline]
    pub fn set_property(
        &self,
        v: &WeakValue,
        prop: &Atom,
        set: RefValue,
    ) -> Result<bool, Exception<'_>> {
        unsafe {
            let set = set.detatch();
            let result = raw::JS_SetProperty(self.c, v.0, prop.v, set);
            self.check_exception(result, result != 0)
        }
    }

    #[inline]
    pub fn delete_property_str(&self, v: &WeakValue, prop: &str) -> Result<(), Exception<'_>> {
        let a = self.new_atom(prop).ok_or(Exception::undefined(self))?;
        self.delete_property(v, &a)
    }

    #[inline]
    pub fn delete_property(&self, v: &WeakValue, prop: &Atom) -> Result<(), Exception<'_>> {
        unsafe {
            let result = raw::JS_DeleteProperty(self.c, v.0, prop.v, 0); // TODO: flags
            self.check_exception(result, ())
        }
    }

    #[inline]
    pub fn seal_object(&self, v: &WeakValue) -> Result<(), Exception<'_>> {
        unsafe {
            let result = raw::JS_SealObject(self.c, v.0);
            self.check_exception(result, ())
        }
    }

    #[inline]
    pub fn freeze_object(&self, v: &WeakValue) -> Result<(), Exception<'_>> {
        unsafe {
            let result = raw::JS_FreezeObject(self.c, v.0);
            self.check_exception(result, ())
        }
    }

    #[inline]
    pub fn call(
        &self,
        f: &WeakValue,
        this: &WeakValue,
        args: &[ArgValue],
    ) -> Result<RefValue, Exception<'_>> {
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
            self.maybe_exception(RefValue::new(v))
        }
    }

    #[inline]
    pub fn get_own_property_names(
        &self,
        v: &WeakValue,
        opts: raw::JSGetPropertyNameOption,
    ) -> Result<OwnPropertyNames<'_>, Exception<'_>> {
        unsafe {
            let mut tab = std::ptr::null_mut();
            let mut len = 0;
            let result = raw::JS_GetOwnPropertyNames(self.c, &mut tab, &mut len, v.0, opts);
            self.check_exception(result, ())?;

            let slice = if len > 0 {
                let tab = NonNull::new(tab).unwrap();
                NonNull::slice_from_raw_parts(tab, len as usize)
            } else {
                NonNull::slice_from_raw_parts(NonNull::dangling(), 0)
            };

            Ok(OwnPropertyNames {
                ctx: self,
                props: slice,
            })
        }
    }

    pub fn to_json(&self, v: &WeakValue) -> Result<serde_json::Value, Exception<'_>> {
        let v = match v.get_tag() {
            JSTag::BIG_INT => unimplemented!(),
            JSTag::SYMBOL => unimplemented!(),
            JSTag::STRING => {
                let string = self.to_c_str(v).ok_or(Exception::undefined(self))?;
                let string = string.to_owned();
                serde_json::Value::String(string)
            }
            JSTag::MODULE => unimplemented!(),
            JSTag::FUNCTION_BYTECODE => unimplemented!(),
            JSTag::OBJECT => {
                if v.is_array() {
                    let opts = raw::JSGetPropertyNameOption::STRING_MASK
                        | raw::JSGetPropertyNameOption::ENUM_ONLY;
                    let props = self.get_own_property_names(v, opts)?;

                    let mut array = Vec::new();
                    for prop in props.iter() {
                        let atom = prop.atom.as_ref().ok_or(Exception::undefined(self))?;
                        let value = self.get_property(v, atom)?;
                        if let Ok(value) = self.to_json(&value) {
                            // let name = self.atom_to_c_str(atom)?;
                            array.push(value);
                        }
                    }

                    serde_json::Value::Array(array)
                } else {
                    let opts = raw::JSGetPropertyNameOption::STRING_MASK
                        | raw::JSGetPropertyNameOption::ENUM_ONLY;
                    let props = self.get_own_property_names(v, opts)?;

                    let mut object = serde_json::Map::new();
                    for prop in props.iter() {
                        let atom = prop.atom.as_ref().ok_or(Exception::undefined(self))?;
                        let value = self.get_property(v, atom)?;
                        if let Ok(value) = self.to_json(&value) {
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
            JSTag::UNDEFINED => return Err(Exception::undefined(self)),
            JSTag::UNINITIALIZED => unimplemented!(),
            JSTag::CATCH_OFFSET => unimplemented!(),
            JSTag::EXCEPTION => return Err(Exception::undefined(self)),
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
            JSTag::SHORT_BIG_INT => unimplemented!(),
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
        Ok(v)
    }

    #[inline]
    pub fn atom_to_value(&self, atom: &Atom) -> Result<RefValue, Exception<'_>> {
        unsafe {
            let v = raw::JS_AtomToValue(self.c, atom.v);
            self.maybe_exception(RefValue::new(v))
        }
    }

    #[inline]
    pub fn atom_to_string(&self, atom: &Atom) -> Result<RefValue, Exception<'_>> {
        unsafe {
            let v = raw::JS_AtomToString(self.c, atom.v);
            self.maybe_exception(RefValue::new(v))
        }
    }

    #[inline]
    pub fn atom_to_c_str(&self, atom: &Atom) -> Result<RuntimeString, Exception<'_>> {
        let string = self.atom_to_string(atom)?;
        self.to_c_str(&string).ok_or(Exception::undefined(self))
    }

    pub(crate) fn check_exception<T>(&self, r: c_int, v: T) -> Result<T, Exception<'_>> {
        if r < 0 {
            Err(self.get_exception())
        } else {
            Ok(v)
        }
    }

    pub(crate) fn maybe_exception(&self, v: RefValue) -> Result<RefValue, Exception<'_>> {
        if v.is_exception() {
            Err(self.get_exception())
        } else {
            Ok(v)
        }
    }
}

/// Wrapper over a [`RefValue`] and a [`WeakContext`] that contains an exception value.
///
/// This wrapper provides a [`Debug`] implementation, enabling use in the [`Result::unwrap`] family
/// of functions. Use [`Exception::into_inner`] or the [`Into`] implementation to get to the
/// inner [`RefValue`] for more complex use cases.
///
/// # Why
///
/// Getting the exception message isn't free. It requires string conversions and needs a context
/// in which to do them. This wrapper defers the string conversion to the debug formatter so the
/// cost is opt-in.
pub struct Exception<'a> {
    v: RefValue,
    c: &'a WeakContext,
}

impl<'a> Exception<'a> {
    pub(crate) const fn undefined(c: &'a WeakContext) -> Self {
        Self {
            v: RefValue::new(JSValue::UNDEFINED),
            c,
        }
    }

    /// Get the context the exception was thrown by.
    pub const fn context(&self) -> &'a WeakContext {
        self.c
    }

    /// Unwrap and return the inner [`RefValue`].
    #[inline]
    pub fn into_inner(self) -> RefValue {
        self.v
    }
}

impl<'a> Into<RefValue> for Exception<'a> {
    #[inline]
    fn into(self) -> RefValue {
        self.v
    }
}

impl<'a> Debug for Exception<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // Early exit if the exception is undefined that avoids the cstring machinery
        if self.v.is_undefined() {
            return f.write_str("<undefined exception>");
        }

        if let Some(msg) = self.c.to_c_str(&self.v) {
            f.write_str(&msg)
        } else {
            f.write_str("<Unknown exception message>")
        }
    }
}

impl<'a> Into<RefValue> for Result<RefValue, Exception<'a>> {
    fn into(self) -> RefValue {
        self.unwrap_or_else(|e| e.v)
    }
}
