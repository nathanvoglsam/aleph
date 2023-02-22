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

// TODO: generational index + extra indirection for validating use-after-free for descriptor sets

use crate::descriptor_set_layout::ValidationDescriptorSetLayout;
use crate::device::ValidationDevice;
use crate::internal::descriptor_set::DescriptorSet;
use interfaces::any::AnyArc;
use interfaces::gpu::*;
use std::ptr::NonNull;

pub struct ValidationDescriptorPool {
    pub(crate) _device: AnyArc<ValidationDevice>,
    pub(crate) _layout: AnyArc<ValidationDescriptorSetLayout>,
    pub(crate) inner: Box<dyn IDescriptorPool>,
    pub(crate) pool_id: u64,
    pub(crate) set_objects: Vec<DescriptorSet>,
    pub(crate) free_list: Vec<DescriptorSetHandle>,
}

interfaces::any::declare_interfaces!(ValidationDescriptorPool, [IDescriptorPool]);

crate::impl_platform_interface_passthrough!(ValidationDescriptorPool);

impl ValidationDescriptorPool {
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
    fn create_set_object_for_set_index(&mut self, inner: DescriptorSetHandle) -> DescriptorSet {
        DescriptorSet {
            _magic_header: DescriptorSet::MAGIC_HEADER_VAL,
            _pool_id: self.pool_id,
            _layout: self._layout.clone(),
            inner,
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
        let handle = handle as *mut DescriptorSet;

        // Safety: It's the caller's responsibility to ensure 'set_index' is in bounds. The pointer
        // is guaranteed to be valid if the index is in bounds.
        let handle = NonNull::new_unchecked(handle).cast::<()>();

        // Safety: no actual unsafe code here, just a warning to make sure people don't use this
        //         unless they absolutely need to.
        DescriptorSetHandle::from_raw(handle)
    }

    fn validate_set_handle(&self, set: &DescriptorSetHandle) {
        // Validate that a DescriptorSetHandle contains a correctly aligned pointer. This may help
        // catch when someone is passing in bad handles
        let align = core::mem::align_of::<DescriptorSet>();
        let set = set.clone();
        let set = unsafe { core::mem::transmute::<_, NonNull<DescriptorSet>>(set) };

        // This should also never happen in practice, but can help flag when people are doing
        // naughty bit casts and passing bad pointers in.
        if !align.is_power_of_two() {
            panic!("is_aligned_to: align is not a power-of-two");
        }
        assert_eq!(
            (set.as_ptr() as usize) & (align - 1),
            0,
            "DescriptorSetHandle contains badly-aligned pointer"
        );

        // If the pool is empty it's impossible for any handles to be from this particular pool.
        // This should never happen as we never allow empty descriptor pools.
        assert!(
            self.set_objects.is_empty(),
            "The DescriptorSet pool is empty, no handle can be valid"
        );

        let sets_base = self.set_objects.as_ptr();
        let sets_end = self
            .set_objects
            .as_ptr()
            .wrapping_add(self.set_objects.len());

        // This should never happen, but we check for completeness sake.
        assert!(
            sets_base < sets_end,
            "The DescriptorSet pool has overflowed the address space"
        );

        // Checks if the given descriptor set was allocated by this pool by checking if the pointer
        // comes from inside the set_objects array bounds.
        let set_ptr = set.as_ptr() as *const DescriptorSet;
        let set_oob = set_ptr < sets_base || set_ptr > sets_end;
        assert!(
            !set_oob,
            "The DescriptorSetHandle points outside of the pool, this handle is from another pool"
        );
    }
}

impl IDescriptorPool for ValidationDescriptorPool {
    fn allocate_set(&mut self) -> Result<DescriptorSetHandle, DescriptorPoolAllocateError> {
        // First try and grab something from the free list
        if let Some(handle) = self.free_list.pop() {
            return Ok(handle);
        }

        // We don't need to check OOM unless we're trying to allocate a new set object
        self.check_oom()?;

        let inner = self.inner.allocate_set()?;

        // Take the next free set, we create fresh set objects linearly
        let set_index = self.set_objects.len();
        let set = self.create_set_object_for_set_index(inner);
        self.set_objects.push(set);

        // Safety: set_index is guaranteed to be < set_objects.len() because it is created from
        // set_objects.len() immediately prior to pushing a new element in set_objects.
        let handle = unsafe {
            assert!(
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
            self.validate_set_handle(set);

            // Does further validation based on reading the set object itself
            DescriptorSet::validate(set, Some(self.pool_id));

            let inner: NonNull<()> = set.clone().into();
            let inner: NonNull<DescriptorSet> = inner.cast();
            let inner = inner.as_ref();

            // Validation is done, free the set.
            self.inner.free(&[inner.inner.clone()]);

            self.free_list.push(set.clone());
        }
    }

    unsafe fn reset(&mut self) {
        self.inner.reset()
    }
}
