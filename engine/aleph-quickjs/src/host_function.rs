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

#[derive(Copy, Clone)]
pub struct GenericHostFn(pub(crate) raw::JSCFunctionFn);

impl GenericHostFn {
    #[doc(hidden)]
    pub unsafe fn new(v: raw::JSCFunctionFn) -> Self {
        Self(v)
    }
}

#[macro_export]
macro_rules! make_generic_host_fn {
    ($f:path) => {{
        use ::core::ffi::c_int;
        use ::core::mem::transmute;
        use ::core::ptr::NonNull;
        use $crate::raw::{JSContext, JSValue, JSValueConst};
        use $crate::{WeakContext, WeakValue};

        extern "C" fn __wrapper_fn(
            ctx: NonNull<JSContext>,
            this_val: JSValueConst,
            argc: c_int,
            argv: *mut JSValueConst,
        ) -> JSValue {
            let result = ::std::panic::catch_unwind(|| unsafe {
                let ctx = &ctx;
                let ctx: &WeakContext = transmute::<&NonNull<JSContext>, &WeakContext>(ctx);

                let this = &this_val;
                let this: &WeakValue = transmute::<&JSValueConst, &WeakValue>(this);

                let args: &[WeakValue] = if argc == 0 {
                    &[]
                } else {
                    let argv = argv.cast::<WeakValue>();
                    let argc = argc as usize;
                    std::slice::from_raw_parts(argv, argc)
                };

                let result = $f(ctx, this, args);
                result.detatch()
            });
            match result {
                ::core::result::Result::Ok(v) => v,
                ::core::result::Result::Err(_e) => unsafe {
                    raw::JS_ThrowPlainError(ctx, c"Native Panic".as_ptr())
                },
            }
        }
        unsafe { $crate::GenericHostFn::new(__wrapper_fn) }
    }};
}
