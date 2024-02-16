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
use std::ffi::c_void;
use std::mem::align_of;
use std::mem::size_of;
use std::ptr::NonNull;

use allocator_api2::alloc::Allocator;
use ash::vk;

/// Takes an [Allocator] and returns a [vk::AllocationCallbacks] wrapper that adapts the rust
/// allocator into the
pub fn callbacks_from_rust_allocator<T: Allocator>(v: T) -> vk::AllocationCallbacks {
    let user_data = v.by_ref();
    let user_data = user_data as *const T as *mut T;
    vk::AllocationCallbacks {
        p_user_data: user_data.cast(),
        pfn_allocation: Some(allocation::<T>),
        pfn_reallocation: Some(reallocation::<T>),
        pfn_free: Some(free::<T>),
        pfn_internal_allocation: None,
        pfn_internal_free: None,
    }
}

unsafe extern "system" fn allocation<T: Allocator>(
    p_user_data: *mut c_void,
    size: usize,
    alignment: usize,
    _allocation_scope: vk::SystemAllocationScope,
) -> *mut c_void {
    let user_data = p_user_data.cast::<T>().cast_const();

    let alignment = alignment.min(align_of::<Layout>());
    let extra_capacity = size_of::<Layout>().max(alignment);
    let size = size + extra_capacity;

    let layout = Layout::from_size_align_unchecked(size, alignment);
    let result = T::allocate(user_data.as_ref().unwrap_unchecked(), layout);

    match result {
        Ok(v) => {
            let v = v.cast::<u8>();
            let object = v.as_ptr().add(extra_capacity);

            let layout_ptr = object.sub(size_of::<Layout>());
            layout_ptr.cast::<Layout>().write(layout);

            object.cast()
        }
        Err(_err) => std::ptr::null_mut(),
    }
}

unsafe extern "system" fn reallocation<T: Allocator>(
    p_user_data: *mut c_void,
    p_original: *mut c_void,
    size: usize,
    alignment: usize,
    allocation_scope: vk::SystemAllocationScope,
) -> *mut c_void {
    let new_ptr = allocation::<T>(p_user_data, size, alignment, allocation_scope).cast::<u8>();
    if !new_ptr.is_null() {
        // Pull the layout from the block directly behind the allocation pointer
        let ptr = p_original.cast::<Layout>().sub(1);
        let old_layout = ptr.read();

        // Copy old data to the new block, copying the smaller of either old size or new size so we
        // don't overrun the buffer bounds
        let src = p_original.cast::<u8>();
        let count = old_layout.size().min(size);
        new_ptr.copy_from(src, count);

        // Free the original block
        free::<T>(p_user_data, p_original);

        new_ptr.cast()
    } else {
        std::ptr::null_mut()
    }
}

unsafe extern "system" fn free<T: Allocator>(p_user_data: *mut c_void, p_memory: *mut c_void) {
    let user_data = p_user_data.cast::<T>().cast_const();

    // Pull the layout from the block directly behind the allocation pointer
    let ptr = p_memory.cast::<Layout>().sub(1);
    let layout = ptr.read();

    // Real pointer to send to the allocator is one 'extra_capacity' block back from the pointer we
    // give out so we need to get the real pointer back
    let alignment = layout.align().min(align_of::<Layout>());
    let extra_capacity = size_of::<Layout>().max(alignment);
    let real_ptr = p_memory.byte_sub(extra_capacity);

    T::deallocate(
        user_data.as_ref().unwrap_unchecked(),
        NonNull::new_unchecked(real_ptr).cast(),
        layout,
    );
}
