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
use std::marker::PhantomData;
use std::ptr::NonNull;
use std::sync::atomic::Ordering;

use allocator_api2::alloc::{AllocError, Allocator, Global};

use crate::instrumentation::IAllocationCategory;

/// An allocator wrapper type that will instrument all allocations made into it with the associated
/// category.
pub struct Instrumented<T: IAllocationCategory, A: Allocator = Global> {
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

    #[inline]
    fn add(bytes: usize) {
        T::info()
            .bytes_allocated
            .fetch_add(bytes, Ordering::Relaxed);
    }

    #[inline]
    fn sub(bytes: usize) {
        T::info()
            .bytes_allocated
            .fetch_sub(bytes, Ordering::Relaxed);
    }

    #[inline]
    unsafe fn handle_resized(
        result: Result<NonNull<[u8]>, AllocError>,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        match result {
            Ok(v) => unsafe {
                let ptr = v.cast::<u8>();
                Self::add(new_layout.size());
                aleph_profile::emit_alloc_n(ptr.as_ptr(), new_layout.size(), T::NAME.to_cstr());
                Ok(v)
            },
            v @ Err(_) => unsafe {
                Self::add(old_layout.size());
                aleph_profile::emit_alloc_n(ptr.as_ptr(), new_layout.size(), T::NAME.to_cstr());
                v
            },
        }
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
        Self::add(layout.size());
        match self.inner.allocate(layout) {
            Ok(v) => unsafe {
                aleph_profile::emit_alloc_n(
                    v.cast::<u8>().as_ptr(),
                    layout.size(),
                    T::NAME.to_cstr(),
                );
                Ok(v)
            },
            v @ Err(_) => v,
        }
    }

    fn allocate_zeroed(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        Self::add(layout.size());
        match self.inner.allocate_zeroed(layout) {
            Ok(v) => unsafe {
                aleph_profile::emit_alloc_n(
                    v.cast::<u8>().as_ptr(),
                    layout.size(),
                    T::NAME.to_cstr(),
                );
                Ok(v)
            },
            v @ Err(_) => v,
        }
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        unsafe {
            aleph_profile::emit_free_n(ptr.as_ptr(), T::NAME.to_cstr());
            self.inner.deallocate(ptr, layout);
        }
        Self::sub(layout.size());
    }

    unsafe fn grow(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        unsafe {
            aleph_profile::emit_free_n(ptr.as_ptr(), T::NAME.to_cstr());
            Self::sub(old_layout.size());
            let out = self.inner.grow(ptr, old_layout, new_layout);
            Self::handle_resized(out, ptr, old_layout, new_layout)
        }
    }

    unsafe fn grow_zeroed(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        unsafe {
            aleph_profile::emit_free_n(ptr.as_ptr(), T::NAME.to_cstr());
            Self::sub(old_layout.size());
            let out = self.inner.grow_zeroed(ptr, old_layout, new_layout);
            Self::handle_resized(out, ptr, old_layout, new_layout)
        }
    }

    unsafe fn shrink(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        unsafe {
            aleph_profile::emit_free_n(ptr.as_ptr(), T::NAME.to_cstr());
            Self::sub(old_layout.size());
            let out = self.inner.shrink(ptr, old_layout, new_layout);
            Self::handle_resized(out, ptr, old_layout, new_layout)
        }
    }
}

/// Alias for 'Instrumented<T, Global>'.
pub type InstrumentedGlobal<T> = Instrumented<T, Global>;
