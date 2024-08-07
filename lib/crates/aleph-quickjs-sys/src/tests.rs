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

use crate::*;

fn script_str(v: &str) -> (*const c_char, usize) {
    let cstr = CStr::from_bytes_with_nul(v.as_bytes()).unwrap();
    (cstr.as_ptr(), v.len() - 1)
}

#[test]
pub fn create_and_destroy_runtime_plus_context() {
    unsafe {
        let runtime = JS_NewRuntime().unwrap();
        let ctx = JS_NewContext(runtime).unwrap();
        JS_FreeContext(ctx);
        JS_FreeRuntime(runtime);
    }
}

#[test]
pub fn eval_script_int_add() {
    unsafe {
        let runtime = JS_NewRuntime().unwrap();
        let ctx = JS_NewContext(runtime).unwrap();

        let filename = "script.js\0".as_ptr() as *const c_char;
        let (script, len) = script_str("2 + 2\0");

        let result = JS_Eval(ctx, script, len, filename, JSEvalType::GLOBAL);
        assert_eq!(result.get_tag(), JSTag::INT);
        assert_eq!(result.get_int(), 4);

        JS_FreeContext(ctx);
        JS_FreeRuntime(runtime);
    }
}

#[test]
pub fn eval_script_float_add() {
    unsafe {
        let runtime = JS_NewRuntime().unwrap();
        let ctx = JS_NewContext(runtime).unwrap();

        let filename = "script.js\0".as_ptr() as *const c_char;
        let (script, len) = script_str("2.2 + 2.4\0");
        let result = JS_Eval(ctx, script, len, filename, JSEvalType::GLOBAL);

        assert_eq!(result.get_tag(), JSTag::FLOAT64);
        assert_eq!(result.get_float64(), 2.2f64 + 2.4f64);

        JS_FreeContext(ctx);
        JS_FreeRuntime(runtime);
    }
}

#[test]
pub fn eval_script_call_c_func() {
    use std::sync::atomic::AtomicBool;
    use std::sync::atomic::Ordering;

    static CALLED: AtomicBool = AtomicBool::new(false);

    extern "C" fn func(_ctx: NonNull<JSContext>, this_val: JSValue, argc: c_int, argv: *mut JSValue) -> JSValue {
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

    unsafe {
        let runtime = JS_NewRuntime().unwrap();
        let ctx = JS_NewContext(runtime).unwrap();

        let global = JS_GetGlobalObject(ctx);
        assert!(global.is_object());

        let func_name = "call_me_maybe\0".as_ptr() as *const c_char;
        let func_v = JS_NewCFunction(ctx, func, func_name, 1);
        assert!(func_v.is_object());

        let result = JS_SetPropertyStr(ctx, global, func_name, func_v);
        assert_ne!(result, -1);

        let filename = "script.js\0".as_ptr() as *const c_char;
        let (script, len) = script_str("call_me_maybe(56);\0");
        let result = JS_Eval(ctx, script, len, filename, JSEvalType::GLOBAL);

        assert!(CALLED.load(Ordering::SeqCst));
        assert_eq!(result.get_tag(), JSTag::INT);
        assert_eq!(result.get_int(), 21);

        JS_FreeContext(ctx);
        JS_FreeRuntime(runtime);
    }
}
