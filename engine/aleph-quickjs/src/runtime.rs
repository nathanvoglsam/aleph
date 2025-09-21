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
use std::any::Any;
use std::ffi::c_void;
use std::ptr::NonNull;
use std::rc::Rc;

use aleph_alloc::alloc::Allocator;
use aleph_alloc::mallocator::Mallocator;

use crate::Context;
use crate::context::InnerContext;
use crate::opaque_box::{OpaqueBox, UntypedOpaqueBox};

#[derive(Clone)]
pub struct Runtime(pub(crate) Rc<InnerRuntime>);

impl Runtime {
    #[inline]
    pub fn new() -> Option<Self> {
        unsafe {
            let rt = raw::JS_NewRuntime()?;
            let rt = InnerRuntime(rt);
            let rt = Rc::new(rt);
            Some(Self(rt))
        }
    }

    #[inline]
    pub fn new_in<Alloc: Allocator + Sized>(a: &'static Alloc) -> Option<Self> {
        unsafe {
            extern "C" fn js_malloc<A: Allocator + Sized>(
                s: *mut raw::JSMallocState,
                size: usize,
            ) -> *mut c_void {
                unsafe {
                    let s = s.as_ref().unwrap_unchecked();
                    let a = NonNull::new(s.opaque)
                        .unwrap_unchecked()
                        .cast::<A>()
                        .as_ref();
                    let a = Mallocator::new(a);
                    a.malloc(size)
                }
            }

            extern "C" fn js_free<A: Allocator + Sized>(
                s: *mut raw::JSMallocState,
                ptr: *mut c_void,
            ) {
                unsafe {
                    let s = s.as_ref().unwrap_unchecked();
                    let a = NonNull::new(s.opaque)
                        .unwrap_unchecked()
                        .cast::<A>()
                        .as_ref();
                    let a = Mallocator::new(a);
                    a.free(ptr)
                }
            }

            extern "C" fn js_realloc<A: Allocator + Sized>(
                s: *mut raw::JSMallocState,
                ptr: *mut c_void,
                size: usize,
            ) -> *mut c_void {
                unsafe {
                    let s = s.as_ref().unwrap_unchecked();
                    let a = NonNull::new(s.opaque)
                        .unwrap_unchecked()
                        .cast::<A>()
                        .as_ref();
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

            let functions = raw::JSMallocFunctions {
                js_malloc: Some(js_malloc::<Alloc>),
                js_free: Some(js_free::<Alloc>),
                js_realloc: Some(js_realloc::<Alloc>),
                js_malloc_usable_size: Some(js_malloc_usable_size),
            };
            let alloc = NonNull::from(a).cast().as_ptr();
            let rt = raw::JS_NewRuntime2(&functions, alloc)?;
            let rt = InnerRuntime(rt);
            let rt = Rc::new(rt);
            Some(Self(rt))
        }
    }

    #[inline]
    pub fn to_raw(&self) -> NonNull<raw::JSRuntime> {
        self.0.0
    }

    #[inline]
    pub fn new_context(&self) -> Option<Context> {
        unsafe {
            let ctx = raw::JS_NewContext(self.0.0)?;
            let ctx = InnerContext {
                ctx,
                rt: self.clone(),
            };
            let ctx = Rc::new(ctx);
            Some(Context(ctx))
        }
    }

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

    #[inline]
    pub fn set_opaque<T: Any + Sized>(&self, v: T) {
        self.remove_opaque();

        unsafe {
            let opaque = OpaqueBox::new(v);
            raw::JS_SetRuntimeOpaque(self.0.0, opaque.as_ptr() as *mut c_void);
        }
    }

    #[inline]
    pub fn get_opaque<T: Any + Sized>(&self) -> Option<&T> {
        unsafe {
            let old = raw::JS_GetRuntimeOpaque(self.0.0);
            let old = NonNull::new(old);
            if let Some(old) = old {
                let old = old.cast::<UntypedOpaqueBox>().as_ref();
                old.try_to_typed::<T>().map(|v| &v.v)
            } else {
                None
            }
        }
    }

    #[inline]
    pub fn remove_opaque(&self) {
        unsafe {
            let old = raw::JS_GetRuntimeOpaque(self.0.0);
            let old = NonNull::new(old);
            if let Some(old) = old {
                let old = old.cast::<UntypedOpaqueBox>();
                UntypedOpaqueBox::drop_inner(old);
            }

            raw::JS_SetRuntimeOpaque(self.0.0, std::ptr::null_mut());
        }
    }
}

pub(crate) struct InnerRuntime(pub(crate) NonNull<raw::JSRuntime>);

impl Drop for InnerRuntime {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            let old = raw::JS_GetRuntimeOpaque(self.0);
            let old = NonNull::new(old);
            if let Some(old) = old {
                let old = old.cast::<UntypedOpaqueBox>();
                UntypedOpaqueBox::drop_inner(old);
            }

            raw::JS_FreeRuntime(self.0);
        }
    }
}
