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

use std::ffi::c_void;
use std::marker::PhantomData;
use std::ptr::NonNull;

use aleph_alloc::BlinkAlloc;
use aleph_alloc::alloc::Allocator;
use aleph_alloc::instrumentation::{Instrumented, system};
use aleph_alloc::mallocator::Mallocator;
use ash::vk;

/// Takes an [Allocator] and returns a [vk::AllocationCallbacks] wrapper that adapts the rust
/// allocator into the
pub fn callbacks_from_rust_allocator<A: Allocator>(
    v: &BlinkAlloc<A>,
) -> vk::AllocationCallbacks<'_> {
    vk::AllocationCallbacks {
        p_user_data: NonNull::from(v).cast().as_ptr(),
        pfn_allocation: Some(allocation::<BlinkAlloc<A>>),
        pfn_reallocation: Some(reallocation::<BlinkAlloc<A>>),
        pfn_free: Some(free::<BlinkAlloc<A>>),
        pfn_internal_allocation: None,
        pfn_internal_free: None,
        _marker: PhantomData,
    }
}

unsafe extern "system" fn allocation<A: Allocator>(
    p_user_data: *mut c_void,
    size: usize,
    alignment: usize,
    _allocation_scope: vk::SystemAllocationScope,
) -> *mut c_void {
    unsafe {
        let allocator = NonNull::new_unchecked(p_user_data).cast::<A>().as_ref();
        let mallocator = Mallocator::new(allocator);
        mallocator.aligned_malloc(size, alignment)
    }
}

unsafe extern "system" fn reallocation<A: Allocator>(
    p_user_data: *mut c_void,
    p_original: *mut c_void,
    size: usize,
    alignment: usize,
    _allocation_scope: vk::SystemAllocationScope,
) -> *mut c_void {
    unsafe {
        let allocator = NonNull::new_unchecked(p_user_data).cast::<A>().as_ref();
        let mallocator = Mallocator::new(allocator);
        mallocator.aligned_realloc(p_original, size, alignment)
    }
}

unsafe extern "system" fn free<A: Allocator>(p_user_data: *mut c_void, p_memory: *mut c_void) {
    unsafe {
        let allocator = NonNull::new_unchecked(p_user_data).cast::<A>().as_ref();
        let mallocator = Mallocator::new(allocator);
        mallocator.free(p_memory)
    }
}

pub static GLOBAL: Option<&'static vk::AllocationCallbacks<'static>> = Some(&GLOBAL_CALLBACKS);

pub static GLOBAL_CALLBACKS: vk::AllocationCallbacks<'static> = vk::AllocationCallbacks {
    p_user_data: &GLOBAL_OBJECT as *const VulkanSystem as *mut VulkanSystem as *mut c_void,
    pfn_allocation: Some(allocation::<VulkanSystem>),
    pfn_reallocation: Some(reallocation::<VulkanSystem>),
    pfn_free: Some(free::<VulkanSystem>),
    pfn_internal_allocation: None,
    pfn_internal_free: None,
    _marker: PhantomData,
};

static GLOBAL_OBJECT: VulkanSystem = system();

pub struct Vulkan;
aleph_alloc::new_alloc_category!(Vulkan, "01991523-55ad-7942-a26e-477ae9cf712d");

pub type VulkanSystem = Instrumented<Vulkan>;
