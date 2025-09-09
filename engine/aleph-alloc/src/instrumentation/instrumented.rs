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

use std::alloc::{GlobalAlloc, Layout, System};
use std::marker::PhantomData;
use std::ptr::NonNull;

use allocator_api2::alloc::{AllocError, Allocator};

use crate::instrumentation::{CategoryInfo, IAllocationCategory, add, emit_alloc, emit_free, sub};

/// An allocator wrapper type that will instrument all allocations made into it with the associated
/// category.
pub struct Instrumented<T: IAllocationCategory, A: Allocator = System> {
    inner: A,
    phantom: PhantomData<T>,
}

impl<T: IAllocationCategory, A: Allocator> Instrumented<T, A> {
    /// Wraps the given allocator into an [`Instrumented`] with the associated category.
    pub const fn new(inner: A) -> Self {
        Self {
            inner,
            phantom: PhantomData,
        }
    }

    /// Unwrap the [`Instrumented`] and get the inner allocator object.
    #[inline]
    pub fn into_inner(self) -> A {
        self.inner
    }
}

impl<T: IAllocationCategory, A: Allocator> From<A> for Instrumented<T, A> {
    fn from(inner: A) -> Self {
        Self::new(inner)
    }
}

impl<T: IAllocationCategory, A: Allocator + Default> Default for Instrumented<T, A> {
    fn default() -> Self {
        Self::new(A::default())
    }
}

unsafe impl<T: IAllocationCategory, A: Allocator> Allocator for Instrumented<T, A> {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        if !cfg!(feature = "instrumentation-enabled") {
            return self.inner.allocate(layout);
        }

        match self.inner.allocate(layout) {
            Ok(v) => unsafe {
                emit_alloc(
                    CategoryInfo::get::<T>(),
                    v.cast::<u8>().as_ptr(),
                    layout.size(),
                );
                add(CategoryInfo::get::<T>(), layout.size());
                Ok(v)
            },
            v @ Err(_) => v,
        }
    }

    fn allocate_zeroed(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        if !cfg!(feature = "instrumentation-enabled") {
            return self.inner.allocate_zeroed(layout);
        }

        match self.inner.allocate_zeroed(layout) {
            Ok(v) => unsafe {
                emit_alloc(
                    CategoryInfo::get::<T>(),
                    v.cast::<u8>().as_ptr(),
                    layout.size(),
                );
                add(CategoryInfo::get::<T>(), layout.size());
                Ok(v)
            },
            v @ Err(_) => v,
        }
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        unsafe {
            if !cfg!(feature = "instrumentation-enabled") {
                return self.inner.deallocate(ptr, layout);
            }

            sub(CategoryInfo::get::<T>(), layout.size());
            emit_free(CategoryInfo::get::<T>(), ptr.as_ptr());

            self.inner.deallocate(ptr, layout);
        }
    }

    unsafe fn grow(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        unsafe {
            if !cfg!(feature = "instrumentation-enabled") {
                return self.inner.grow(ptr, old_layout, new_layout);
            }

            sub(CategoryInfo::get::<T>(), old_layout.size());
            emit_free(CategoryInfo::get::<T>(), ptr.as_ptr());

            let out = self.inner.grow(ptr, old_layout, new_layout);
            handle_resized(CategoryInfo::get::<T>(), out, ptr, old_layout, new_layout)
        }
    }

    unsafe fn grow_zeroed(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        unsafe {
            if !cfg!(feature = "instrumentation-enabled") {
                return self.inner.grow_zeroed(ptr, old_layout, new_layout);
            }

            sub(CategoryInfo::get::<T>(), old_layout.size());
            emit_free(CategoryInfo::get::<T>(), ptr.as_ptr());

            let out = self.inner.grow_zeroed(ptr, old_layout, new_layout);
            handle_resized(CategoryInfo::get::<T>(), out, ptr, old_layout, new_layout)
        }
    }

    unsafe fn shrink(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        unsafe {
            if !cfg!(feature = "instrumentation-enabled") {
                return self.inner.shrink(ptr, old_layout, new_layout);
            }

            sub(CategoryInfo::get::<T>(), old_layout.size());
            emit_free(CategoryInfo::get::<T>(), ptr.as_ptr());

            let out = self.inner.shrink(ptr, old_layout, new_layout);
            handle_resized(CategoryInfo::get::<T>(), out, ptr, old_layout, new_layout)
        }
    }
}

unsafe impl<T: IAllocationCategory, A: Allocator + GlobalAlloc> GlobalAlloc for Instrumented<T, A> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe {
            if !cfg!(feature = "instrumentation-enabled") {
                return GlobalAlloc::alloc(&self.inner, layout);
            }

            let out = GlobalAlloc::alloc(&self.inner, layout);
            emit_alloc(CategoryInfo::get::<T>(), out, layout.size());
            add(CategoryInfo::get::<T>(), layout.size());
            out
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe {
            if !cfg!(feature = "instrumentation-enabled") {
                return GlobalAlloc::dealloc(&self.inner, ptr, layout);
            }

            sub(CategoryInfo::get::<T>(), layout.size());
            emit_free(CategoryInfo::get::<T>(), ptr);
            GlobalAlloc::dealloc(&self.inner, ptr, layout);
        }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        unsafe {
            if !cfg!(feature = "instrumentation-enabled") {
                return GlobalAlloc::alloc_zeroed(&self.inner, layout);
            }

            let out = GlobalAlloc::alloc_zeroed(&self.inner, layout);
            emit_alloc(CategoryInfo::get::<T>(), out, layout.size());
            add(CategoryInfo::get::<T>(), layout.size());
            out
        }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        unsafe {
            if !cfg!(feature = "instrumentation-enabled") {
                return GlobalAlloc::realloc(&self.inner, ptr, layout, new_size);
            }

            sub(CategoryInfo::get::<T>(), layout.size());
            emit_free(CategoryInfo::get::<T>(), ptr);

            let out = GlobalAlloc::realloc(&self.inner, ptr, layout, new_size);

            emit_alloc(CategoryInfo::get::<T>(), out, layout.size());
            add(CategoryInfo::get::<T>(), layout.size());

            out
        }
    }
}

#[inline(always)]
unsafe fn handle_resized(
    c: &'static CategoryInfo,
    result: Result<NonNull<[u8]>, AllocError>,
    ptr: NonNull<u8>,
    old_layout: Layout,
    new_layout: Layout,
) -> Result<NonNull<[u8]>, AllocError> {
    match result {
        Ok(v) => unsafe {
            let ptr = v.cast::<u8>();
            emit_alloc(c, ptr.as_ptr(), new_layout.size());
            add(c, new_layout.size());
            Ok(v)
        },
        v @ Err(_) => unsafe {
            emit_alloc(c, ptr.as_ptr(), old_layout.size());
            add(c, old_layout.size());
            v
        },
    }
}
