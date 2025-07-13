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
use std::ffi::*;
use std::ptr::NonNull;
use std::rc::Rc;

use aleph_nstr::{NStr, nstr};
use raw::*;

use crate::{Context, NumberVariant, RefValue, Runtime};

#[test]
pub fn create_and_destroy_runtime_plus_context() {
    let runtime = Runtime::new().unwrap();
    let _context = runtime.new_context().unwrap();
}

#[test]
pub fn eval_script_int_add() {
    let runtime = Runtime::new().unwrap();
    let context = runtime.new_context().unwrap();

    let filename = nstr!("script.js");
    let script = nstr!("2 + 2");
    let result = context.eval(script, filename, JSEvalOptions::STRICT);
    let result = check_exception(&context, result);

    assert_eq!(result.get_tag(), JSTag::INT);
    assert_eq!(result.get_number(), Some(NumberVariant::Integer(4)));
}

#[test]
pub fn eval_script_float_add() {
    let runtime = Runtime::new().unwrap();
    let context = runtime.new_context().unwrap();

    let filename = nstr!("script.js");
    let script = nstr!("2.2 + 2.4");
    let result = context.eval(script, filename, JSEvalOptions::STRICT);
    let result = check_exception(&context, result);

    assert_eq!(result.get_tag(), JSTag::FLOAT64);
    assert_eq!(
        result.get_number(),
        Some(NumberVariant::Double(2.2f64 + 2.4f64))
    );
}

#[test]
pub fn eval_script_call_c_func() {
    use std::sync::atomic::{AtomicBool, Ordering};

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
    let context = runtime.new_context().unwrap();

    let global = context.get_global_object();

    let func_name = nstr!("call_me_maybe");
    let func_v = context.new_c_function(func, func_name, 1);
    assert!(func_v.is_object());

    let result = global.set_property_str(func_name.to_str(), &func_v);
    assert_ne!(result, -1);

    let filename = nstr!("script.js");
    let script = nstr!("call_me_maybe(56);");
    let result = context.eval(script, filename, JSEvalOptions::STRICT);
    let result = check_exception(&context, result);

    assert!(CALLED.load(Ordering::SeqCst));
    assert_eq!(result.get_tag(), JSTag::INT);
    assert_eq!(result.get_number(), Some(NumberVariant::Integer(21)));
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

    let runtime = Runtime::new().unwrap();
    let context = runtime.new_context().unwrap();

    let filename = nstr!("script.js");
    let result = context.eval(SCRIPT, filename, JSEvalOptions::STRICT);
    let result = check_exception(&context, result);
    assert!(
        result.is_undefined(),
        "Expected 'undefined' got '{:?}'",
        result.get_tag()
    );

    let global = context.get_global_object();

    let result = global.get_property_str("OUTPUT");

    assert!(
        result.is_object(),
        "Expected 'object' got '{:?}'",
        result.get_tag()
    );
    let result = result.to_object().ok().unwrap();

    let props = result.get_own_property_names(
        JSGetPropertyNameOption::STRING_MASK | JSGetPropertyNameOption::ENUM_ONLY,
    );

    let props = props.iter();

    assert_eq!(props.len(), 3);
    for prop in props {
        let atom = prop.atom.as_ref().unwrap();
        let prop_name = atom.to_c_str().unwrap();
        assert!(expected_names.remove(prop_name.as_ref()));
    }
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

    let runtime = Runtime::new().unwrap();
    let context = runtime.new_context().unwrap();

    //unsafe {
    let filename = nstr!("script.js");
    let result = context.eval(SCRIPT, filename, JSEvalOptions::STRICT);
    let result = check_exception(&context, result);
    assert!(
        result.is_undefined(),
        "Expected 'undefined' got '{:?}'",
        result.get_tag()
    );

    let global = context.get_global_object();

    let result = global.get_property_str("OUTPUT");

    assert!(
        result.is_object(),
        "Expected 'object' got '{:?}'",
        result.get_tag()
    );
    let result = result.to_object().ok().unwrap();

    let result_json = result.to_json().unwrap();
    let expected_json: serde_json::Value = serde_json::from_str(JSON_EXPECTED).unwrap();

    assert_eq!(result_json, expected_json);
    //}
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
    let runtime = Runtime::new().unwrap();
    let context = runtime.new_context().unwrap();

    unsafe {
        // Capture the baseline allocation stats so we know how many objects are live before we
        // do anything.
        let baseline = context.compute_memory_usage();

        // Create an object and assert we have exclusive ownership of it
        let obj1 = context.new_object();
        assert_eq!(obj1.get_ref_count(), 1);

        // Capture the allocation stats after we've created a single object. We should have exactly
        // one more object and the stats should reflect this
        let created = context.compute_memory_usage();
        assert_eq!(baseline.obj_count + 1, created.obj_count);

        // Increment the refcount and assert we have 2 ref counts now and that both objects have
        // the same ref count.
        let obj2 = obj1.clone();
        assert_eq!(obj1.get_ref_count(), 2);
        assert_eq!(obj2.get_ref_count(), 2);

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
}

#[test]
pub fn set_object_property_ref_count_behavior() {
    let runtime = Runtime::new().unwrap();
    let context = runtime.new_context().unwrap();

    let root = context.new_object();
    let leaf = context.new_object();

    assert_eq!(root.get_ref_count(), 1);
    assert_eq!(leaf.get_ref_count(), 1);

    let result = root.set_property_str("leaf", &leaf);
    assert!(result >= 0);

    assert_eq!(root.get_ref_count(), 1);
    assert_eq!(leaf.get_ref_count(), 2);

    let result = root.delete_property_str("leaf");
    assert!(result >= 0);

    assert_eq!(root.get_ref_count(), 1);
    assert_eq!(leaf.get_ref_count(), 1);
}

#[test]
pub fn string_with_internal_null_character() {
    let runtime = Runtime::new().unwrap();
    let context = runtime.new_context().unwrap();

    let string = "String with a \0 character in the middle";

    let v = context.new_string(string);
    assert!(v.is_string());

    let got_string = v.to_c_str().unwrap();
    assert_eq!(string, got_string.as_ref());
}

#[test]
pub fn runtime_opaque_test() {
    let runtime = Runtime::new().unwrap();

    let o1 = Rc::new(123i32);
    let o2 = Rc::new(456u64);

    assert_eq!(Rc::strong_count(&o1), 1);
    assert_eq!(Rc::strong_count(&o2), 1);
    assert!(runtime.get_opaque::<Rc<i32>>().is_none());
    assert!(runtime.get_opaque::<Rc<u64>>().is_none());

    runtime.set_opaque(o1.clone());

    assert_eq!(Rc::strong_count(&o1), 2);
    assert_eq!(Rc::strong_count(&o2), 1);
    assert!(runtime.get_opaque::<Rc<i32>>().is_some());
    assert!(runtime.get_opaque::<Rc<u64>>().is_none());

    runtime.set_opaque(o2.clone());

    assert_eq!(Rc::strong_count(&o1), 1);
    assert_eq!(Rc::strong_count(&o2), 2);
    assert!(runtime.get_opaque::<Rc<i32>>().is_none());
    assert!(runtime.get_opaque::<Rc<u64>>().is_some());

    runtime.remove_opaque();

    assert_eq!(Rc::strong_count(&o1), 1);
    assert_eq!(Rc::strong_count(&o2), 1);
    assert!(runtime.get_opaque::<Rc<i32>>().is_none());
    assert!(runtime.get_opaque::<Rc<u64>>().is_none());
}

#[test]
pub fn runtime_opaque_drop_test() {
    let runtime = Runtime::new().unwrap();

    let o1 = Rc::new(420i32);

    assert_eq!(Rc::strong_count(&o1), 1);
    assert!(runtime.get_opaque::<Rc<i32>>().is_none());

    runtime.set_opaque(o1.clone());

    assert_eq!(Rc::strong_count(&o1), 2);
    assert!(runtime.get_opaque::<Rc<i32>>().is_some());

    drop(runtime);

    assert_eq!(Rc::strong_count(&o1), 1);
}

#[test]
pub fn context_opaque_test() {
    let runtime = Runtime::new().unwrap();
    let context = runtime.new_context().unwrap();

    let o1 = Rc::new(123i32);
    let o2 = Rc::new(456u64);

    assert_eq!(Rc::strong_count(&o1), 1);
    assert_eq!(Rc::strong_count(&o2), 1);
    assert!(context.get_opaque::<Rc<i32>>().is_none());
    assert!(context.get_opaque::<Rc<u64>>().is_none());

    context.set_opaque(o1.clone());

    assert_eq!(Rc::strong_count(&o1), 2);
    assert_eq!(Rc::strong_count(&o2), 1);
    assert!(context.get_opaque::<Rc<i32>>().is_some());
    assert!(context.get_opaque::<Rc<u64>>().is_none());

    context.set_opaque(o2.clone());

    assert_eq!(Rc::strong_count(&o1), 1);
    assert_eq!(Rc::strong_count(&o2), 2);
    assert!(context.get_opaque::<Rc<i32>>().is_none());
    assert!(context.get_opaque::<Rc<u64>>().is_some());

    context.remove_opaque();

    assert_eq!(Rc::strong_count(&o1), 1);
    assert_eq!(Rc::strong_count(&o2), 1);
    assert!(context.get_opaque::<Rc<i32>>().is_none());
    assert!(context.get_opaque::<Rc<u64>>().is_none());
}

#[test]
pub fn context_opaque_drop_test() {
    let runtime = Runtime::new().unwrap();
    let context = runtime.new_context().unwrap();

    let o1 = Rc::new(420i32);

    assert_eq!(Rc::strong_count(&o1), 1);
    assert!(context.get_opaque::<Rc<i32>>().is_none());

    context.set_opaque(o1.clone());

    assert_eq!(Rc::strong_count(&o1), 2);
    assert!(context.get_opaque::<Rc<i32>>().is_some());

    drop(context);

    assert_eq!(Rc::strong_count(&o1), 1);
}

fn check_exception<'a>(ctx: &'a Context, v: RefValue) -> RefValue {
    if v.is_exception() {
        let exception = ctx.get_exception();
        let message = exception
            .to_c_str()
            .expect("Failed to get exception message");
        let message_str = message.as_ref();
        panic!("Unhandled JS Exception: {}", message_str);
    } else {
        v
    }
}
