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

use crate::instrumentation::{IAllocationCategory, Uncategorized, global};
use allocator_api2::alloc::{AllocError, Allocator};
use std::alloc::{GlobalAlloc, Layout, System};
use std::ptr::NonNull;
use std::sync::atomic::Ordering;

/// Wrapper that is intended to be registered using `#[global_allocator]` to direct all un-tagged
/// allocations towards the [`Uncategorized`] memory category. This delegates to [`System`].
pub struct SystemUncategorized;

unsafe impl Allocator for SystemUncategorized {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        global::<Uncategorized>().allocate(layout)
    }

    fn allocate_zeroed(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        global::<Uncategorized>().allocate_zeroed(layout)
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        unsafe { global::<Uncategorized>().deallocate(ptr, layout) }
    }

    unsafe fn grow(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        unsafe { global::<Uncategorized>().grow(ptr, old_layout, new_layout) }
    }

    unsafe fn grow_zeroed(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        unsafe { global::<Uncategorized>().grow_zeroed(ptr, old_layout, new_layout) }
    }

    unsafe fn shrink(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        unsafe { global::<Uncategorized>().shrink(ptr, old_layout, new_layout) }
    }
}

unsafe impl GlobalAlloc for SystemUncategorized {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe {
            let out = System.alloc(layout);
            Self::add(layout.size());
            aleph_profile::emit_alloc_n(out, layout.size(), Uncategorized::NAME.to_cstr());
            out
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe {
            System.dealloc(ptr, layout);
            Self::sub(layout.size());
            aleph_profile::emit_free_n(ptr, Uncategorized::NAME.to_cstr());
        }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        unsafe {
            let out = System.alloc_zeroed(layout);
            Self::add(layout.size());
            aleph_profile::emit_alloc_n(out, layout.size(), Uncategorized::NAME.to_cstr());
            out
        }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        unsafe {
            let out = System.realloc(ptr, layout, new_size);
            Self::sub(layout.size());
            Self::add(new_size);
            aleph_profile::emit_free_n(ptr, Uncategorized::NAME.to_cstr());
            aleph_profile::emit_alloc_n(out, new_size, Uncategorized::NAME.to_cstr());
            out
        }
    }
}

impl SystemUncategorized {
    fn add(bytes: usize) {
        Uncategorized::info()
            .bytes_allocated
            .fetch_add(bytes, Ordering::Relaxed);
    }

    fn sub(bytes: usize) {
        Uncategorized::info()
            .bytes_allocated
            .fetch_sub(bytes, Ordering::Relaxed);
    }
}

/// This macro is a utility for registering a global allocator that automatically routes all
/// allocations to the [`Uncategorized`] category. It delegates allocating to [`System`].
#[macro_export]
macro_rules! register_global_allocator {
    () => {
        #[global_allocator]
        static GLOBAL_UNCAGORIZED: $crate::instrumentation::SystemUncategorized =
            $crate::instrumentation::SystemUncategorized;
    };
}
