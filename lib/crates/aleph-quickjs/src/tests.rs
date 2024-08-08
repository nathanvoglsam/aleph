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

use crate::Runtime;
use aleph_nstr::nstr;
use raw::*;
use std::ffi::*;
use std::ptr::NonNull;

#[test]
pub fn create_and_destroy_runtime_plus_context() {
    unsafe {
        let runtime = Runtime::new().unwrap();
        let _ctx = runtime.new_context().unwrap();
    }
}

#[test]
pub fn eval_script_int_add() {
    let runtime = Runtime::new().unwrap();
    let ctx = runtime.new_context().unwrap();
    unsafe {
        let ctx = ctx.get_raw();

        let filename = nstr!("script.js");
        let script = nstr!("2 + 2");
        let result = JS_Eval(
            ctx,
            script.to_cstr_ptr(),
            script.len(),
            filename.to_cstr_ptr(),
            JSEvalType::GLOBAL,
        );
        assert_eq!(result.get_tag(), JSTag::INT);
        assert_eq!(result.get_int(), 4);
    }
}

#[test]
pub fn eval_script_float_add() {
    let runtime = Runtime::new().unwrap();
    let ctx = runtime.new_context().unwrap();
    unsafe {
        let ctx = ctx.get_raw();

        let filename = nstr!("script.js");
        let script = nstr!("2.2 + 2.4");
        let result = JS_Eval(
            ctx,
            script.to_cstr_ptr(),
            script.len(),
            filename.to_cstr_ptr(),
            JSEvalType::GLOBAL,
        );

        assert_eq!(result.get_tag(), JSTag::FLOAT64);
        assert_eq!(result.get_float64(), 2.2f64 + 2.4f64);
    }
}

#[test]
pub fn eval_script_call_c_func() {
    use std::sync::atomic::AtomicBool;
    use std::sync::atomic::Ordering;

    static CALLED: AtomicBool = AtomicBool::new(false);

    extern "C" fn func(
        _ctx: NonNull<JSContext>,
        this_val: JSValue,
        argc: c_int,
        argv: *mut JSValue,
    ) -> JSValue {
        unsafe {
            CALLED.store(true, Ordering::SeqCst);

            assert!(this_val.is_undefined());

            assert_eq!(argc, 1);

            let v = *argv;
            assert_eq!(v.get_tag(), JSTag::INT);
            assert_eq!(v.get_int(), 56);

            JSValue::new_i32(21)
        }
    }

    let runtime = Runtime::new().unwrap();
    let ctx = runtime.new_context().unwrap();

    unsafe {
        let ctx = ctx.get_raw();

        let global = JS_GetGlobalObject(ctx);
        assert!(global.is_object());

        let func_name = nstr!("call_me_maybe");
        let func_v = JS_NewCFunction(ctx, func, func_name.to_cstr_ptr(), 1);
        assert!(func_v.is_object());

        let result = JS_SetPropertyStr(ctx, global, func_name.to_cstr_ptr(), func_v);
        assert_ne!(result, -1);

        let filename = nstr!("script.js");
        let script = nstr!("call_me_maybe(56);");
        let result = JS_Eval(
            ctx,
            script.to_cstr_ptr(),
            script.len(),
            filename.to_cstr_ptr(),
            JSEvalType::GLOBAL,
        );

        assert!(CALLED.load(Ordering::SeqCst));
        assert_eq!(result.get_tag(), JSTag::INT);
        assert_eq!(result.get_int(), 21);
    }
}
