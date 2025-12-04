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

use std::mem::transmute;
use std::panic::{UnwindSafe, catch_unwind};
use std::ptr::NonNull;

use crate::{RefValue, WeakContext, WeakValue};

// ============================================================================================== //

#[derive(Copy, Clone)]
pub struct HostFn(pub(crate) raw::JSCFunctionFn);

pub type SignatureHostFn = fn(ctx: &WeakContext, this: &WeakValue, args: &[WeakValue]) -> RefValue;

impl HostFn {
    #[doc(hidden)]
    pub unsafe fn new(v: raw::JSCFunctionFn) -> Self {
        Self(v)
    }
}

#[macro_export]
macro_rules! make_host_fn {
    ($f:path) => {{
        extern "C" fn __wrapper_fn(
            ctx: ::core::ptr::NonNull<$crate::raw::JSContext>,
            this_val: $crate::raw::JSValueConst,
            argc: ::core::ffi::c_int,
            argv: *mut $crate::raw::JSValueConst,
        ) -> $crate::raw::JSValue {
            let _typecheck_f: $crate::SignatureHostFn = $f;
            unsafe {
                $crate::catch_unwind_and_throw_js_exception(ctx, || {
                    let ctx = $crate::context_arg(&ctx);
                    let this = $crate::this_val_arg(&this_val);
                    let args = $crate::value_list_arg(argc as usize, &argv);
                    let result = $f(ctx, this, args);
                    result.detatch()
                })
            }
        }
        unsafe { $crate::HostFn::new(__wrapper_fn) }
    }};
}

// ============================================================================================== //

#[derive(Copy, Clone)]
pub struct HostFnMagic(pub(crate) raw::JSCFunctionMagicFn);

pub type SignatureHostFnMagic =
    fn(ctx: &WeakContext, this: &WeakValue, args: &[WeakValue], magic: i32) -> RefValue;

impl HostFnMagic {
    #[doc(hidden)]
    pub unsafe fn new(v: raw::JSCFunctionMagicFn) -> Self {
        Self(v)
    }
}

#[macro_export]
macro_rules! make_host_fn_magic {
    ($f:path) => {{
        extern "C" fn __wrapper_fn(
            ctx: ::core::ptr::NonNull<$crate::raw::JSContext>,
            this_val: $crate::raw::JSValueConst,
            argc: ::core::ffi::c_int,
            argv: *mut $crate::raw::JSValueConst,
            magic: c_int,
        ) -> $crate::raw::JSValue {
            let _typecheck_f: $crate::SignatureHostFnMagic = $f;
            unsafe {
                $crate::catch_unwind_and_throw_js_exception(ctx, || {
                    let ctx = $crate::context_arg(&ctx);
                    let this = $crate::this_val_arg(&this_val);
                    let args = $crate::value_list_arg(argc as usize, &argv);
                    let result = $f(ctx, this, args, magic);
                    result.detatch()
                })
            }
        }
        unsafe { $crate::HostFnMagic::new(__wrapper_fn) }
    }};
}

// ============================================================================================== //

#[derive(Copy, Clone)]
pub struct HostFnData<const COUNT: usize>(pub(crate) raw::JSCFunctionDataFn);

pub type SignatureHostFnData<const COUNT: usize> = fn(
    ctx: &WeakContext,
    this: &WeakValue,
    args: &[WeakValue],
    magic: i32,
    data: &[WeakValue; COUNT],
) -> RefValue;

impl<const COUNT: usize> HostFnData<COUNT> {
    pub const LEN: usize = COUNT;

    #[doc(hidden)]
    pub unsafe fn new(v: raw::JSCFunctionDataFn) -> Self {
        Self(v)
    }
}

#[macro_export]
macro_rules! make_host_fn_data {
    ($f:path, $len: literal) => {{
        extern "C" fn __wrapper_fn(
            ctx: ::core::ptr::NonNull<$crate::raw::JSContext>,
            this_val: $crate::raw::JSValueConst,
            argc: ::core::ffi::c_int,
            argv: *mut $crate::raw::JSValueConst,
            magic: c_int,
            data: *mut $crate::raw::JSValueConst,
        ) -> $crate::raw::JSValue {
            let _typecheck_len: usize = $len;
            let _typecheck_f: $crate::SignatureHostFnData<$len> = $f;
            unsafe {
                $crate::catch_unwind_and_throw_js_exception(ctx, || {
                    let ctx = $crate::context_arg(&ctx);
                    let this = $crate::this_val_arg(&this_val);
                    let args = $crate::value_list_arg(argc as usize, &argv);
                    let data = $crate::value_list_arg($len, &data);
                    let result = $f(ctx, this, args, magic);
                    result.detatch()
                })
            }
        }
        unsafe { $crate::HostFnData::<$len>::new(__wrapper_fn) }
    }};
}

// ============================================================================================== //

#[derive(Copy, Clone)]
pub struct HostFnMapFloat(pub(crate) raw::JSFFFn);

pub type SignatureHostFnMapFloat = fn(f64) -> f64;

impl HostFnMapFloat {
    #[doc(hidden)]
    pub unsafe fn new(v: raw::JSFFFn) -> Self {
        Self(v)
    }
}

#[macro_export]
macro_rules! make_host_fn_map_float {
    ($f:path) => {{
        extern "C" fn __wrapper_fn(v: f64) -> f64 {
            let _typecheck_f: $crate::SignatureHostFnMapFloat = $f;
            $f(v)
        }
        unsafe { $crate::HostFnMapFloat::new(__wrapper_fn) }
    }};
}

// ============================================================================================== //

#[derive(Copy, Clone)]
pub struct HostFnCombineFloat(pub(crate) raw::JSFFFFn);

pub type SignatureHostFnCombineFloat = fn(f64, f64) -> f64;

impl HostFnCombineFloat {
    #[doc(hidden)]
    pub unsafe fn new(v: raw::JSFFFFn) -> Self {
        Self(v)
    }
}

#[macro_export]
macro_rules! make_host_fn_combine_float {
    ($f:path) => {{
        extern "C" fn __wrapper_fn(a: f64, b: f64) -> f64 {
            let _typecheck_f: $crate::SignatureHostFnCombineFloat = $f;
            $f(a, b)
        }
        unsafe { $crate::HostFnCombineFloat::new(__wrapper_fn) }
    }};
}

// ============================================================================================== //

/// Internal function for host function wrappers
#[doc(hidden)]
pub unsafe fn context_arg(ctx: &NonNull<raw::JSContext>) -> &WeakContext {
    unsafe { transmute::<&NonNull<raw::JSContext>, &WeakContext>(ctx) }
}

/// Internal function for host function wrappers
#[doc(hidden)]
pub unsafe fn this_val_arg(this_val: &raw::JSValueConst) -> &WeakValue {
    unsafe { transmute::<&raw::JSValueConst, &WeakValue>(this_val) }
}

/// Internal function for host function wrappers
#[doc(hidden)]
pub unsafe fn value_list_arg(len: usize, ptr: &*mut raw::JSValueConst) -> &[WeakValue] {
    unsafe {
        if len == 0 {
            &[]
        } else {
            std::slice::from_raw_parts(ptr.cast::<WeakValue>(), len)
        }
    }
}

/// Internal function to translate a rust panic into a JS exception.
#[doc(hidden)]
pub unsafe fn catch_unwind_and_throw_js_exception(
    ctx: NonNull<raw::JSContext>,
    f: impl (FnOnce() -> raw::JSValue) + UnwindSafe,
) -> raw::JSValue {
    match catch_unwind(f) {
        Ok(v) => v,
        Err(_e) => unsafe { raw::JS_ThrowPlainError(ctx, c"Native Panic".as_ptr()) },
    }
}
