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

use crate::{AnyArc, IAny};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

struct Test(Arc<AtomicUsize>);

impl Drop for Test {
    fn drop(&mut self) {
        self.0.fetch_add(20, Ordering::Relaxed);
    }
}

trait ITest: IAny {
    fn test_fn(&self);
}

trait ITestOther: IAny {
    fn test_fn_other(&self);
}

impl ITest for Test {
    fn test_fn(&self) {
        self.0.fetch_add(1, Ordering::Relaxed);
    }
}

impl ITestOther for Test {
    fn test_fn_other(&self) {
        self.0.fetch_add(5, Ordering::Relaxed);
    }
}

crate::declare_interfaces!(Test, [ITest, ITestOther]);

#[test]
pub fn arc_test_1() {
    // Our counter for running the test
    let counter = Arc::new(AtomicUsize::default());

    // Wrap our counter in an AnyArc
    let test = Test(counter.clone());
    let mut test = AnyArc::new(test);

    // Get our interface casted to another interface
    let mut test_other = test.query_interface::<dyn ITestOther>().unwrap();

    test.test_fn();
    assert_eq!(counter.load(Ordering::Relaxed), 1);

    test_other.test_fn_other();
    assert_eq!(counter.load(Ordering::Relaxed), 6);

    test.test_fn();
    assert_eq!(counter.load(Ordering::Relaxed), 7);

    test_other.test_fn_other();
    assert_eq!(counter.load(Ordering::Relaxed), 12);

    assert!(AnyArc::get_mut(&mut test).is_none());

    drop(test);
    assert_eq!(counter.load(Ordering::Relaxed), 12);

    assert!(AnyArc::get_mut(&mut test_other).is_some());

    drop(test_other);
    assert_eq!(counter.load(Ordering::Relaxed), 32);
}
