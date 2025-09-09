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

mod category_registry;
mod instrumented;
mod tagged;

pub use aleph_malloc_info::get_allocated_bytes;
pub use category_registry::*;
pub use instrumented::*;
pub use tagged::*;

/// Utility for getting an [`Allocator`] instance wrapping [`System`] with the given category.
pub const fn system<T: IAllocationCategory>() -> Instrumented<T> {
    Instrumented::new(allocator_api2::alloc::System)
}

/// Utility for getting an [`Allocator`] instance wrapping [`System`] with a [`Tagged`] allocator.
pub const fn system_tagged() -> Tagged {
    Tagged::new(allocator_api2::alloc::System)
}

/// The default allocation category.
pub struct Uncategorized;
crate::new_alloc_category!(Uncategorized, "01991417-00b8-7662-847e-369720bdb172");

/// This macro is a utility for registering a global [`Tagged`] allocator. It delegates allocating
/// to [`std::alloc::System`].
#[cfg(feature = "instrumentation-enabled")]
#[macro_export]
macro_rules! register_global_allocator {
    () => {
        #[global_allocator]
        static GLOBAL_TAGGED: $crate::instrumentation::Tagged =
            $crate::instrumentation::Tagged::new(::std::alloc::System);
    };
}

#[cfg(not(feature = "instrumentation-enabled"))]
#[macro_export]
macro_rules! register_global_allocator {
    () => {};
}

#[inline]
#[allow(unused)]
fn add(c: &'static CategoryInfo, bytes: usize) {
    use std::sync::atomic::Ordering;

    #[cfg(feature = "instrumentation-enabled")]
    c.bytes_allocated.fetch_add(bytes, Ordering::Relaxed);
}

#[inline]
#[allow(unused)]
fn sub(c: &'static CategoryInfo, bytes: usize) {
    use std::sync::atomic::Ordering;

    #[cfg(feature = "instrumentation-enabled")]
    c.bytes_allocated.fetch_sub(bytes, Ordering::Relaxed);
}

#[inline]
#[allow(unused)]
unsafe fn emit_alloc(c: &'static CategoryInfo, ptr: *mut u8, size: usize) {
    #[cfg(feature = "instrumentation-enabled")]
    unsafe {
        if c.id == Uncategorized::ID {
            aleph_profile::emit_alloc(ptr, size);
        } else {
            aleph_profile::emit_alloc_n(ptr, size, c.name.to_cstr());
        }
    }
}

#[inline]
#[allow(unused)]
unsafe fn emit_free(c: &'static CategoryInfo, ptr: *mut u8) {
    #[cfg(feature = "instrumentation-enabled")]
    unsafe {
        if c.id == Uncategorized::ID {
            aleph_profile::emit_free(ptr);
        } else {
            aleph_profile::emit_free_n(ptr, c.name.to_cstr());
        }
    }
}
