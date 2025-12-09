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

use std::collections::HashSet;

use aleph_nstr::{NStr, nstr};
use raw::*;

use crate::{
    ArgValue, Context, NumberVariant, RefValue, Runtime, Value, WeakContext, WeakValue,
    make_host_fn, make_host_fn_combine_float, make_host_fn_map_float,
};

#[test]
pub fn create_and_destroy_runtime_plus_context() {
    std::thread::scope(|scope| {
        scope.spawn(move || {
            let runtime = Runtime::init_thread_runtime();
            let _context = runtime.new_context().unwrap();
        });
    });
}

#[test]
pub fn eval_script_int_add() {
    std::thread::scope(|scope| {
        scope.spawn(move || {
            let runtime = Runtime::init_thread_runtime();
            let context = runtime.new_context().unwrap();

            let filename = nstr!("script.js");
            let script = nstr!("2 + 2");
            let result = context.eval(script, filename, JSEvalFlags::STRICT);
            let result = check_exception(&context, result);

            assert_eq!(result.get_tag(), JSTag::INT);
            assert_eq!(result.get_number(), Some(NumberVariant::Integer(4)));
        });
    });
}

#[test]
pub fn eval_script_float_add() {
    std::thread::scope(|scope| {
        scope.spawn(move || {
            let runtime = Runtime::init_thread_runtime();
            let context = runtime.new_context().unwrap();

            let filename = nstr!("script.js");
            let script = nstr!("2.2 + 2.4");
            let result = context.eval(script, filename, JSEvalFlags::STRICT);
            let result = check_exception(&context, result);

            assert_eq!(result.get_tag(), JSTag::FLOAT64);
            assert_eq!(
                result.get_number(),
                Some(NumberVariant::Double(2.2f64 + 2.4f64))
            );
        });
    });
}

#[test]
pub fn eval_script_call_c_func() {
    use std::sync::atomic::{AtomicBool, Ordering};

    static CALLED: AtomicBool = AtomicBool::new(false);

    fn func(_ctx: &WeakContext, this: &WeakValue, [arg]: &[WeakValue; 1]) -> RefValue {
        CALLED.store(true, Ordering::SeqCst);

        assert!(this.is_undefined());

        assert_eq!(arg.get_number(), Some(NumberVariant::Integer(56)));

        Value::new_i32(21).upgrade()
    }

    std::thread::scope(|scope| {
        scope.spawn(move || {
            let runtime = Runtime::init_thread_runtime();
            let context = runtime.new_context().unwrap();

            let global = context.get_global_object();

            let func_name = nstr!("call_me_maybe");
            let func_v = context.new_host_function(make_host_fn!(func), func_name);
            assert!(func_v.is_object());

            let result = context.set_property_str(&global, func_name.to_str(), func_v.clone());
            assert_ne!(result, -1);

            let filename = nstr!("script.js");
            let script = nstr!("call_me_maybe(56);");
            let result = context.eval(script, filename, JSEvalFlags::STRICT);
            let result = check_exception(&context, result);

            assert!(CALLED.load(Ordering::SeqCst));
            assert_eq!(result.get_tag(), JSTag::INT);
            assert_eq!(result.get_number(), Some(NumberVariant::Integer(21)));
        });
    });
}

#[test]
pub fn eval_script_call_c_func_not_enough_args() {
    use std::sync::atomic::{AtomicBool, Ordering};

    static CALLED: AtomicBool = AtomicBool::new(false);

    fn func(_ctx: &WeakContext, this: &WeakValue, [present, missing]: &[WeakValue; 2]) -> RefValue {
        CALLED.store(true, Ordering::SeqCst);

        assert!(this.is_undefined());

        assert_eq!(present.get_number(), Some(NumberVariant::Integer(56)));
        assert!(missing.is_undefined());

        Value::new_i32(21).upgrade()
    }

    std::thread::scope(|scope| {
        scope.spawn(move || {
            let runtime = Runtime::init_thread_runtime();
            let context = runtime.new_context().unwrap();

            let global = context.get_global_object();

            let func_name = nstr!("call_me_maybe");
            let func_v = context.new_host_function(make_host_fn!(func), func_name);
            assert!(func_v.is_object());

            let result = context.set_property_str(&global, func_name.to_str(), func_v.clone());
            assert_ne!(result, -1);

            let filename = nstr!("script.js");
            let script = nstr!("call_me_maybe(56);");
            let result = context.eval(script, filename, JSEvalFlags::STRICT);
            let result = check_exception(&context, result);

            assert!(CALLED.load(Ordering::SeqCst));
            assert_eq!(result.get_tag(), JSTag::INT);
            assert_eq!(result.get_number(), Some(NumberVariant::Integer(21)));
        });
    });
}

#[test]
pub fn eval_script_call_c_func_map_float() {
    fn func(v: f64) -> f64 {
        (v * 2.0) + 1.0
    }

    std::thread::scope(|scope| {
        scope.spawn(move || {
            let runtime = Runtime::init_thread_runtime();
            let context = runtime.new_context().unwrap();

            let global = context.get_global_object();

            let func_name = nstr!("call_me_maybe");
            let func_v =
                context.new_host_function_map_float(make_host_fn_map_float!(func), func_name);
            assert!(func_v.is_object());

            let result = context.set_property_str(&global, func_name.to_str(), func_v.clone());
            assert_ne!(result, -1);

            let filename = nstr!("script.js");
            let script = nstr!("function main() { return call_me_maybe(21); }");
            let result = context.eval(script, filename, JSEvalFlags::STRICT);
            let _result = check_exception(&context, result);

            let global = context.get_global_object();
            let js_func = context.get_property_str(&global, "main");
            let result = context.call(&js_func, &Value::UNDEFINED, &[]);
            let result = check_exception(&context, result);

            assert_eq!(result.get_number(), Some(NumberVariant::Integer(43)));
        });
    });
}

#[test]
pub fn eval_script_call_c_func_combine_float() {
    fn func(a: f64, b: f64) -> f64 {
        (a * b) + 1.0
    }

    std::thread::scope(|scope| {
        scope.spawn(move || {
            let runtime = Runtime::init_thread_runtime();
            let context = runtime.new_context().unwrap();

            let global = context.get_global_object();

            let func_name = nstr!("call_me_maybe");
            let func_v = context
                .new_host_function_combine_float(make_host_fn_combine_float!(func), func_name);
            assert!(func_v.is_object());

            let result = context.set_property_str(&global, func_name.to_str(), func_v.clone());
            assert_ne!(result, -1);

            let filename = nstr!("script.js");
            let script = nstr!("function main() { return call_me_maybe(21, 2); }");
            let result = context.eval(script, filename, JSEvalFlags::STRICT);
            let _result = check_exception(&context, result);

            let global = context.get_global_object();
            let js_func = context.get_property_str(&global, "main");
            let result = context.call(&js_func, &Value::UNDEFINED, &[]);
            let result = check_exception(&context, result);

            assert_eq!(result.get_number(), Some(NumberVariant::Integer(43)));
        });
    });
}

#[test]
pub fn eval_script_call_c_func_recursive() {
    fn func(ctx: &WeakContext, this: &WeakValue, [f, depth]: &[WeakValue; 2]) -> RefValue {
        assert!(this.is_undefined());

        let depth = depth.get_number().unwrap().normalize();

        if depth >= 5.0 {
            Value::new_f64(depth * depth).upgrade()
        } else {
            let new_depth = Value::new_f64(depth + 1.0);
            let args: [ArgValue; 2] = [f.as_arg(), new_depth.as_arg()];
            ctx.call(f, &Value::UNDEFINED, &args)
        }
    }

    std::thread::scope(|scope| {
        scope.spawn(move || {
            let runtime = Runtime::init_thread_runtime();
            let context = runtime.new_context().unwrap();

            let global = context.get_global_object();

            let func_name = nstr!("call_me_maybe");
            let func_v = context.new_host_function(make_host_fn!(func), func_name);
            assert!(func_v.is_object());

            let result = context.set_property_str(&global, func_name.to_str(), func_v.clone());
            assert_ne!(result, -1);

            let filename = nstr!("script.js");
            let script = nstr!("function main() { return call_me_maybe(call_me_maybe, 0); }");
            let result = context.eval(script, filename, JSEvalFlags::STRICT);
            let _result = check_exception(&context, result);

            let global = context.get_global_object();
            let js_func = context.get_property_str(&global, "main");
            let result = context.call(&js_func, &Value::UNDEFINED, &[]);
            let result = check_exception(&context, result);

            assert_eq!(result.get_number(), Some(NumberVariant::Double(25.0)));
        });
    });
}

#[test]
pub fn eval_script_get_property_names() {
    const SCRIPT: &'static NStr = nstr!(
        r#"
        var OUTPUT = {
            thingA: "Hello, World!",
            thingB: 56,
            thingC: {
                thingD: "Foo",
                bar: "baz"
            }
        };"#
    );

    let mut expected_names = HashSet::new();
    expected_names.insert("thingA");
    expected_names.insert("thingB");
    expected_names.insert("thingC");

    std::thread::scope(|scope| {
        scope.spawn(move || {
            let runtime = Runtime::init_thread_runtime();
            let context = runtime.new_context().unwrap();

            let filename = nstr!("script.js");
            let result = context.eval(SCRIPT, filename, JSEvalFlags::STRICT);
            let result = check_exception(&context, result);
            assert!(
                result.is_undefined(),
                "Expected 'undefined' got '{:?}'",
                result.get_tag()
            );

            let global = context.get_global_object();

            let result = context.get_property_str(&global, "OUTPUT");

            assert!(
                result.is_object(),
                "Expected 'object' got '{:?}'",
                result.get_tag()
            );

            let props = context.get_own_property_names(
                &result,
                JSGetPropertyNameOption::STRING_MASK | JSGetPropertyNameOption::ENUM_ONLY,
            );

            let props = props.iter();

            assert_eq!(props.len(), 3);
            for prop in props {
                let atom = prop.atom.as_ref().unwrap();
                let prop_name = context.atom_to_c_str(atom).unwrap();
                assert!(expected_names.remove(prop_name.as_ref()));
            }
        });
    });
}

#[test]
pub fn eval_script_to_serde() {
    const SCRIPT: &'static NStr = nstr!(
        r#"
        var OUTPUT = {
            thingA: "Hello, World!",
            thingB: 56.1,
            thingC: {
                thingD: "Foo",
                bar: "baz"
            }
        };"#
    );

    const JSON_EXPECTED: &'static str = r#"
        {
            "thingA": "Hello, World!",
            "thingB": 56.1,
            "thingC": {
                "thingD": "Foo",
                "bar": "baz"
            }
        }"#;

    std::thread::scope(|scope| {
        scope.spawn(move || {
            let runtime = Runtime::init_thread_runtime();
            let context = runtime.new_context().unwrap();

            //unsafe {
            let filename = nstr!("script.js");
            let result = context.eval(SCRIPT, filename, JSEvalFlags::STRICT);
            let result = check_exception(&context, result);
            assert!(
                result.is_undefined(),
                "Expected 'undefined' got '{:?}'",
                result.get_tag()
            );

            let global = context.get_global_object();

            let result = context.get_property_str(&global, "OUTPUT");

            assert!(
                result.is_object(),
                "Expected 'object' got '{:?}'",
                result.get_tag()
            );

            let result_json = context.to_json(&result).unwrap();
            let expected_json: serde_json::Value = serde_json::from_str(JSON_EXPECTED).unwrap();

            assert_eq!(result_json, expected_json);
            //}
        });
    });
}

#[test]
pub fn runtime_deferred_gc_free() {
    //!
    //! This test is in a sense just testing the QuickJS runtime, but it exists for an important
    //! reason.
    //!
    //! Our safe wrapper depends on the semantics seen below to be valid for ref-counted objects.
    //! We provide an Rc like interface for JS objects. It is important that we're allowed to set
    //! the ref-count to zero without freeing the object without breaking things. This means we can
    //! allow the value wrappers to use Clone and Drop like Rc does to increment/decrement the value
    //! ref-count.
    //!
    //! The expectation is that freeing objects used like this will be deferred to the next garbage
    //! collection cycle. This means we'll use more memory and generate more garbage but the API
    //! will be __much__ nicer to use. If performance is important enough then explicit freeing APIs
    //! are also available but the API is unsafe and a bit harder to use.
    //!

    std::thread::scope(|scope| {
        scope.spawn(move || {
            let runtime = Runtime::init_thread_runtime();
            let context = runtime.new_context().unwrap();

            unsafe {
                // Capture the baseline allocation stats so we know how many objects are live before we
                // do anything.
                let baseline = context.compute_memory_usage();

                // Create an object and assert we have exclusive ownership of it
                let obj1 = context.new_object();
                assert_eq!(obj1.get_ref_count(), Some(1));

                // Capture the allocation stats after we've created a single object. We should have exactly
                // one more object and the stats should reflect this
                let created = context.compute_memory_usage();
                assert_eq!(baseline.obj_count + 1, created.obj_count);

                // Increment the refcount and assert we have 2 ref counts now and that both objects have
                // the same ref count.
                let obj2 = obj1.clone();
                assert_eq!(obj1.get_ref_count(), Some(2));
                assert_eq!(obj2.get_ref_count(), Some(2));

                // Incrementing should never make more objects so make sure.
                let increment = context.compute_memory_usage();
                assert_eq!(created.obj_count, increment.obj_count);

                // Manually decrement the reference count for each object
                let raw1 = obj1.to_raw();
                let raw2 = obj2.to_raw();
                drop(obj1);
                drop(obj2);

                // Both should now have a reference count of zero
                assert_eq!(raw1.get_ref_count(), Some(0));
                assert_eq!(raw2.get_ref_count(), Some(0));

                // We've manually decremented the ref-count without freeing. It should now be zero but the
                // object is still alive as a zombie until the GC runs when it will actually be freed.
                let decrement = context.compute_memory_usage();
                assert_eq!(created.obj_count, decrement.obj_count);

                // Trigger a manual GC and hope for the best?
                //
                // What we want to know is if it's valid for us to decrement the refcount from zero from
                // outside of the runtime with out actually freeing the object and then let the GC clean
                // the dead objects up later.
                //
                // This would make lifetime management for our safe wrappers significantly easier as we
                // could just treat values like rust's Rc types.
                runtime.gc();

                // Our object should have been collected now. It had already hit a ref-count of zero before
                // but we opted not to free it then and instead defer it to the garbage collector to find
                // and free.
                let collected = context.compute_memory_usage();
                assert_eq!(baseline.obj_count, collected.obj_count);
            }
        });
    });
}

#[test]
pub fn set_object_property_ref_count_behavior() {
    std::thread::scope(|scope| {
        scope.spawn(move || {
            let runtime = Runtime::init_thread_runtime();
            let context = runtime.new_context().unwrap();

            let root = context.new_object();
            let leaf = context.new_object();

            assert_eq!(root.get_ref_count(), Some(1));
            assert_eq!(leaf.get_ref_count(), Some(1));

            let result = context.set_property_str(&root, "leaf", leaf.clone());
            assert!(result >= 0);

            assert_eq!(root.get_ref_count(), Some(1));
            assert_eq!(leaf.get_ref_count(), Some(2));

            let result = context.delete_property_str(&root, "leaf");
            assert!(result >= 0);

            assert_eq!(root.get_ref_count(), Some(1));
            assert_eq!(leaf.get_ref_count(), Some(1));
        });
    });
}

#[test]
pub fn string_with_internal_null_character() {
    std::thread::scope(|scope| {
        scope.spawn(move || {
            let runtime = Runtime::init_thread_runtime();
            let context = runtime.new_context().unwrap();

            let string = "String with a \0 character in the middle";

            let v = context.new_string(string);
            assert!(v.is_string());

            let got_string = context.to_c_str(&v).unwrap();
            assert_eq!(string, got_string.as_ref());
        });
    });
}

fn check_exception<'a>(ctx: &'a Context, v: RefValue) -> RefValue {
    if v.is_exception() {
        let exception = ctx.get_exception();
        let message = ctx
            .to_c_str(&exception)
            .expect("Failed to get exception message");
        let message_str = message.as_ref();
        panic!("Unhandled JS Exception: {}", message_str);
    } else {
        v
    }
}
