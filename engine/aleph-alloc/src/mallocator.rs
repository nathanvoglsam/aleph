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

use std::alloc::{Layout, handle_alloc_error};
use std::ffi::c_void;
use std::ptr::NonNull;

use allocator_api2::alloc::Allocator;

/// Allocator wrapper that provides a malloc like interface that redirects into the inner allocator
/// instance.
///
/// # Warning
///
/// This has per-allocation overhead of 2 usize values. Because 'free' and friends do not require
/// the caller give the [`Layout`] of the allocation being freed we have to tag one on to each
/// allocation so we can recover the layout from just the pointer.
///
/// As such this should really only be used for instrumenting allocations in development builds and
/// should be stripped out for final release builds.
pub struct Mallocator<A: Allocator>(A);

impl<A: Allocator> Mallocator<A> {
    /// Constructs a new [`Mallocator`] wrapping the given allocator.
    pub const fn new(a: A) -> Self {
        Self(a)
    }

    /// Unwrap the [`Mallocator`] and get the inner allocator object.
    #[inline]
    pub fn into_inner(self) -> A {
        self.0
    }

    /// Implementation of 'malloc' wrapped over the given allocator
    pub unsafe extern "C" fn malloc(&self, size: usize) -> *mut c_void {
        unsafe {
            if size == 0 {
                return std::ptr::null_mut();
            }
            self.aligned_malloc(size, Self::align_for(size))
        }
    }

    /// Implementation of 'malloc' wrapped over the given allocator
    pub unsafe extern "C" fn aligned_malloc(&self, size: usize, align: usize) -> *mut c_void {
        unsafe {
            if size == 0 {
                return std::ptr::null_mut();
            }

            // Combine the layout of the requested allocation with the tag we have to attach so we
            // can recover the size/align for the deallocation.
            let caller_layout = Layout::from_size_align_unchecked(size, align);
            let (tagged_layout, offset) = Layout::new::<Layout>()
                .extend(caller_layout)
                .unwrap_unchecked();

            let ptr = match self.0.allocate(tagged_layout) {
                Ok(v) => v,
                Err(_) => handle_alloc_error(tagged_layout),
            }
            .cast::<c_void>();

            // This will always be aligned because the alignment of 'ptr' will always be
            // at least as aligned as Layout. The ptr is either more aligned and the space for
            // 'layout' is padded, or exactly aligned as it is tightly packed to the boundary
            // of the 'layout' tag.
            let ptr = ptr.byte_add(offset);
            ptr.cast::<Layout>().sub(1).write(caller_layout);
            ptr.as_ptr()
        }
    }

    /// Implementation of 'calloc' wrapped over the given allocator
    pub unsafe extern "C" fn calloc(&self, nmemb: usize, size: usize) -> *mut c_void {
        unsafe {
            if size == 0 || nmemb == 0 {
                return std::ptr::null_mut();
            }

            let caller_layout =
                Layout::from_size_align_unchecked(size * nmemb, Self::align_for(nmemb));
            let (tagged_layout, offset) = Layout::new::<Layout>()
                .extend(caller_layout)
                .unwrap_unchecked();

            let ptr = match self.0.allocate_zeroed(tagged_layout) {
                Ok(v) => v,
                Err(_) => handle_alloc_error(tagged_layout),
            }
            .cast::<c_void>();

            let ptr = ptr.byte_add(offset);
            ptr.cast::<Layout>().sub(1).write(caller_layout);
            ptr.as_ptr()
        }
    }

    /// Implementation of 'realloc' wrapped over the given allocator
    pub unsafe extern "C" fn realloc(&self, mem: *mut c_void, size: usize) -> *mut c_void {
        unsafe { self.aligned_realloc(mem, size, Self::align_for(size)) }
    }

    pub unsafe extern "C" fn aligned_realloc(
        &self,
        mem: *mut c_void,
        size: usize,
        align: usize,
    ) -> *mut c_void {
        unsafe {
            let mem = match NonNull::new(mem) {
                None => return self.malloc(size),
                Some(v) => v,
            };

            if size == 0 {
                self.free(mem.as_ptr());
                return std::ptr::null_mut();
            }

            let old_caller_layout = mem.cast::<Layout>().sub(1).read();
            let (old_tagged_layout, old_offset) = Layout::new::<Layout>()
                .extend(old_caller_layout)
                .unwrap_unchecked();

            let new_caller_layout = Layout::from_size_align_unchecked(size, align);
            let (new_tagged_layout, new_offset) = Layout::new::<Layout>()
                .extend(new_caller_layout)
                .unwrap_unchecked();

            let real_ptr = mem.byte_sub(old_offset);
            let result = if new_tagged_layout.size() < old_tagged_layout.size() {
                self.0
                    .shrink(real_ptr.cast(), old_tagged_layout, new_tagged_layout)
            } else {
                self.0
                    .grow(real_ptr.cast(), old_tagged_layout, new_tagged_layout)
            };

            let new_ptr = match result {
                Ok(v) => v,
                Err(_) => handle_alloc_error(new_tagged_layout),
            }
            .cast::<c_void>()
            .byte_add(new_offset);

            new_ptr.cast::<Layout>().sub(1).write(new_caller_layout);

            new_ptr.as_ptr()
        }
    }

    /// Implementation of 'free' wrapped over the given allocator
    pub unsafe extern "C" fn free(&self, mem: *mut c_void) {
        unsafe {
            // Ignore null pointers
            let mem = match NonNull::new(mem) {
                None => return,
                Some(v) => v,
            };

            let caller_layout = mem.cast::<Layout>().sub(1).read();
            let (tagged_layout, offset) = Layout::new::<Layout>()
                .extend(caller_layout)
                .unwrap_unchecked();

            let real_ptr = mem.byte_sub(offset);
            self.0.deallocate(real_ptr.cast(), tagged_layout);
        }
    }

    const fn align_for(size: usize) -> usize {
        // TODO: is this good enough?
        let align = size.next_power_of_two();
        if size.next_power_of_two() > 16 {
            16
        } else {
            align
        }
    }
}

impl<A: Allocator> From<A> for Mallocator<A> {
    fn from(inner: A) -> Self {
        Self::new(inner)
    }
}

impl<A: Allocator + Default> Default for Mallocator<A> {
    fn default() -> Self {
        Self::new(A::default())
    }
}
