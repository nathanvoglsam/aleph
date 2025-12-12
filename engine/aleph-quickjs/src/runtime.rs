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

use std::alloc::Layout;
use std::cell::OnceCell;
use std::ffi::c_void;
use std::ptr::NonNull;
use std::rc::Rc;

use aleph_alloc::alloc::Allocator;
use aleph_alloc::mallocator::Mallocator;

use crate::{Context, WeakContext};

#[derive(Clone)]
pub struct Runtime(pub(crate) Rc<InnerRuntime>);

impl Runtime {
    /// Get a handle to the thread-local runtime on the calling thread, initializing the runtime if
    /// it has not already been created.
    ///
    /// This will create a 'default' runtime without any extra configuration. Use
    /// [`Runtime::init_thread_runtime_in`] to create a runtime that overrides the allocator
    /// functions.
    #[inline]
    pub fn init_thread_runtime() -> Self {
        let rt = THREAD_RUNTIME.with(|rt| {
            let rt = rt.get_or_init(|| unsafe {
                let rt = raw::JS_NewRuntime().unwrap();
                Rc::new(InnerRuntime(rt))
            });
            rt.clone()
        });
        Self(rt)
    }

    /// Get a handle to the thread-local runtime on the calling thread, initializing the runtime if
    /// it has not already been created.
    ///
    /// This function provides extended functionality over [`Runtime::init_thread_runtime`],
    /// allowing configuration of the runtime's malloc/free functions.
    ///
    /// # Warning
    ///
    /// The thread-local runtime is lazily initialized on first use. If another caller has already
    /// created the runtime with another function then this function will simply return the existing
    /// runtime _without configuring the allocator functions_. If you intend to use this interface
    /// it is recommended to call this _very_ early in your application lifecycle.
    #[inline]
    pub fn init_thread_runtime_in<Alloc: Allocator + Sized>(a: &'static Alloc) -> Self {
        extern "C" fn js_calloc<A: Allocator + Sized>(
            s: *mut c_void,
            count: usize,
            size: usize,
        ) -> *mut c_void {
            unsafe {
                let a = NonNull::new(s).unwrap_unchecked().cast::<A>().as_ref();
                let a = Mallocator::new(a);
                a.calloc(count, size)
            }
        }

        extern "C" fn js_malloc<A: Allocator + Sized>(s: *mut c_void, size: usize) -> *mut c_void {
            unsafe {
                let a = NonNull::new(s).unwrap_unchecked().cast::<A>().as_ref();
                let a = Mallocator::new(a);
                a.malloc(size)
            }
        }

        extern "C" fn js_free<A: Allocator + Sized>(s: *mut c_void, ptr: *mut c_void) {
            unsafe {
                let a = NonNull::new(s).unwrap_unchecked().cast::<A>().as_ref();
                let a = Mallocator::new(a);
                a.free(ptr)
            }
        }

        extern "C" fn js_realloc<A: Allocator + Sized>(
            s: *mut c_void,
            ptr: *mut c_void,
            size: usize,
        ) -> *mut c_void {
            unsafe {
                let a = NonNull::new(s).unwrap_unchecked().cast::<A>().as_ref();
                let a = Mallocator::new(a);
                a.realloc(ptr, size)
            }
        }

        extern "C" fn js_malloc_usable_size(ptr: *const c_void) -> usize {
            unsafe {
                let caller_layout = ptr.cast::<Layout>().sub(1).read();
                caller_layout.size()
            }
        }

        let rt = THREAD_RUNTIME.with(|rt| {
            let rt = rt.get_or_init(|| unsafe {
                let functions = raw::JSMallocFunctions {
                    js_calloc: Some(js_calloc::<Alloc>),
                    js_malloc: Some(js_malloc::<Alloc>),
                    js_free: Some(js_free::<Alloc>),
                    js_realloc: Some(js_realloc::<Alloc>),
                    js_malloc_usable_size: Some(js_malloc_usable_size),
                };
                let alloc = NonNull::from(a).cast().as_ptr();
                let rt = raw::JS_NewRuntime2(&functions, alloc).unwrap();
                Rc::new(InnerRuntime(rt))
            });
            rt.clone()
        });
        Self(rt)
    }

    /// Constructs a new [`Context`] inside this runtime.
    #[inline]
    pub fn new_context(&self) -> Option<Context> {
        unsafe {
            let ctx = raw::JS_NewContext(self.0.0)?;
            Some(Context {
                ctx: WeakContext { c: ctx },
                r: self.clone(),
            })
        }
    }

    /// Triggers a manual GC cycle on the runtime.
    #[inline]
    pub fn gc(&self) {
        unsafe {
            raw::JS_RunGC(self.0.0);
        }
    }

    /// Query the memory usage from the runtime.
    #[inline]
    pub fn compute_memory_usage(&self) -> raw::JSMemoryUsage {
        unsafe {
            let mut usage = raw::JSMemoryUsage::default();
            raw::JS_ComputeMemoryUsage(self.0.0, &mut usage);
            usage
        }
    }
}

pub(crate) fn with_runtime<F, R>(f: F) -> R
where
    F: FnOnce(&InnerRuntime) -> R,
{
    THREAD_RUNTIME.with(|rt| {
        let rt = rt.get_or_init(|| unsafe {
            let rt = raw::JS_NewRuntime().unwrap();
            Rc::new(InnerRuntime(rt))
        });
        f(&rt)
    })
}

pub(crate) struct InnerRuntime(pub(crate) NonNull<raw::JSRuntime>);

impl Drop for InnerRuntime {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            raw::JS_FreeRuntime(self.0);
        }
    }
}

thread_local! {
    static THREAD_RUNTIME: OnceCell<Rc<InnerRuntime>> = const { OnceCell::new() };
}
