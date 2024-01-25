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

use std::any::TypeId;
use std::cell::UnsafeCell;
use std::ptr::NonNull;

use aleph_any::{declare_interfaces, AnyArc};
use aleph_rhi_api::*;
use windows::utils::{CPUDescriptorHandle, GPUDescriptorHandle};

use crate::descriptor_set_layout::DescriptorSetLayout;
use crate::device::Device;
use crate::internal::descriptor_arena::DescriptorArena;
use crate::internal::descriptor_set::DescriptorSet;

pub struct DescriptorPool {
    pub(crate) _device: AnyArc<Device>,
    pub(crate) _layout: AnyArc<DescriptorSetLayout>,

    /// The base address of the arena this pool allocates resource descriptors from
    pub(crate) resource_arena: Option<DescriptorArena>,

    /// Backing storage for all the descriptor set objects this pool gives out
    pub(crate) set_objects: Vec<UnsafeCell<DescriptorSet>>,

    /// Backing storage for all the set object's sampler slots
    pub(crate) sampler_buffer: Vec<Option<GPUDescriptorHandle>>,

    /// List of free handles
    pub(crate) free_list: Vec<DescriptorSetHandle>,
}

declare_interfaces!(DescriptorPool, [IDescriptorPool]);

impl IGetPlatformInterface for DescriptorPool {
    unsafe fn __query_platform_interface(&self, _target: TypeId, _out: *mut ()) -> Option<()> {
        None
    }
}

impl DescriptorPool {
    /// Checks if there is space to allocate a new set in the descriptor pool.
    ///
    /// # Warning
    ///
    /// This function assumes it is being called immediately prior to trying to allocate a set. As
    /// such it returns an OOM error instead of a simple bool.
    fn check_oom(&mut self) -> Result<(), DescriptorPoolAllocateError> {
        if self.set_objects.len() == self.set_objects.capacity() {
            Err(DescriptorPoolAllocateError::OutOfMemory)
        } else {
            Ok(())
        }
    }

    /// Constructs a [DescriptorSet] object for the descriptor set with index 'set_index'.
    ///
    /// Specifically this creates the object that a [DescriptorSetHandle] is actually a pointer to.
    /// This will contain fully computed CPU and GPU handles to the resource and sampler
    /// `ID3D12DescriptorHeap` heaps. This should be called to initialize a [DescriptorSet] in the
    /// 'set_objects' pool.
    ///
    /// Once constructed the [DescriptorSet] should be immutable, as it will always refer to the
    /// descriptor memory for the set index it was constructed with.
    ///
    /// This function is expected to be used when allocating a new set out of `self.set_pool`
    /// instead of from the free list.
    fn create_set_object_for_set_index(&mut self, set_index: u32) -> DescriptorSet {
        let (resource_handle_cpu, resource_handle_gpu) =
            Self::get_optional_handles_for_arena(self.resource_arena.as_ref(), set_index);

        let samplers = if !self._layout.sampler_tables.is_empty() {
            let idx = set_index as usize * self._layout.sampler_tables.len();
            Some(NonNull::from(&self.sampler_buffer[idx]))
        } else {
            None
        };

        DescriptorSet {
            _layout: self._layout.clone(),
            resource_handle_cpu,
            resource_handle_gpu,
            samplers,
            num_samplers: self._layout.sampler_tables.len(),
        }
    }

    /// Constructs a [DescriptorSetHandle] for the set with index 'set_index'
    ///
    /// From an implementation stand point [DescriptorSetHandle] is just an opaque pointer. For
    /// this implementation it contains a pointer to a [DescriptorSet] instance. Specifically it
    /// contains a pointer to a set inside the `self.set_objects` array.
    ///
    /// This function is thus just a utility for getting the pointer to the [DescriptorSet] object
    /// with the requested index and converting that pointer into a [DescriptorSetHandle] so we can
    /// hand it out to callers on the other side of the API boundary.
    ///
    /// # Safety
    ///
    /// It is the caller's responsibility to check if 'set_index' is within the bounds of the
    /// `self.set_objects` array.
    ///
    /// It is also the caller's responsibility to ensure sound access through the pointer that
    /// the [DescriptorSetHandle] actually is.
    unsafe fn convert_set_index_to_handle(&self, set_index: usize) -> DescriptorSetHandle {
        // Get a pointer to the set_index'th element in the array and wrap it into a handle.
        //
        // We have to be careful to not construct a reference here to keep the code sound. From the
        // borrow checker's perspective creating the reference here (even though we don't
        // dereference it) would be unsound if we didn't synchronize with any api that would create
        // a reference from a handle.
        //
        // In practice this would mean any API that uses descriptor sets would be unsound if used
        // in parallel with this code by rust's soundness rules, which would be insane.
        //
        // By carefully using only pointers here it means we never create a reference to any of the
        // elements of the 'set_objects' array *except* inside other APIs by converting the handle
        // back to a pointer and converting to a reference.
        //
        // This won't solve use-after-free, or un-synchronized access to the underlying descriptor
        // memory. Thus any API that reads or writes from descriptors or descriptor sets is unsafe.
        let handle = self.set_objects.as_ptr().wrapping_add(set_index);
        let handle = handle as *mut UnsafeCell<DescriptorSet>;

        // Safety: It's the caller's responsibility to ensure 'set_index' is in bounds. The pointer
        // is guaranteed to be valid if the index is in bounds.
        let handle = NonNull::new_unchecked(handle).cast::<()>();

        // Safety: no actual unsafe code here, just a warning to make sure people don't use this
        //         unless they absolutely need to.
        DescriptorSetHandle::from_raw(handle)
    }

    fn get_optional_handles_for_arena(
        arena: Option<&DescriptorArena>,
        set_index: u32,
    ) -> (Option<CPUDescriptorHandle>, Option<GPUDescriptorHandle>) {
        if let Some(arena) = arena {
            let (cpu, gpu) = arena.get_handles_for_set_index(set_index);
            (Some(cpu), Some(gpu))
        } else {
            (None, None)
        }
    }
}

impl IDescriptorPool for DescriptorPool {
    fn allocate_set(&mut self) -> Result<DescriptorSetHandle, DescriptorPoolAllocateError> {
        // First try and grab something from the free list
        if let Some(handle) = self.free_list.pop() {
            return Ok(handle);
        }

        // We don't need to check OOM unless we're trying to allocate a new set object
        self.check_oom()?;

        // Take the next free set, we create fresh set objects linearly
        let set_index = self.set_objects.len();
        let set = self.create_set_object_for_set_index(set_index as u32);
        self.set_objects.push(UnsafeCell::new(set));

        // Safety: set_index is guaranteed to be < set_objects.len() because it is created from
        // set_objects.len() immediately prior to pushing a new element in set_objects.
        let handle = unsafe {
            debug_assert!(
                set_index < self.set_objects.len(),
                "'set_index' is out of bounds"
            );
            self.convert_set_index_to_handle(set_index)
        };

        Ok(handle)
    }

    fn allocate_sets(
        &mut self,
        num_sets: usize,
    ) -> Result<Vec<DescriptorSetHandle>, DescriptorPoolAllocateError> {
        let mut sets = Vec::with_capacity(num_sets);
        for _ in 0..num_sets {
            sets.push(self.allocate_set()?);
        }
        Ok(sets)
    }

    unsafe fn free(&mut self, sets: &[DescriptorSetHandle]) {
        for set in sets {
            self.free_list.push(set.clone());
        }
    }

    unsafe fn reset(&mut self) {
        self.free_list.clear();
        self.set_objects.clear();
    }
}

impl Drop for DescriptorPool {
    fn drop(&mut self) {
        if let Some(arena) = self.resource_arena.as_ref() {
            // Safety:
            // It's not possible to use the DescriptorPool, and thus the Arena, again as we're in
            // the drop implementation.
            //
            // We can't prevent user's further up the callstack from trying to use descriptors from
            // the pool (and arena) after calling this. This is reflected in all APIs that use them
            // being unsafe. We still leave preventing user-after-free to the caller.
            unsafe {
                arena.release_allocation_to_heap(self._device.descriptor_heaps.gpu_view_heap());
            }
        }
    }
}
