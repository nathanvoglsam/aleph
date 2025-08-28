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

// TODO: generational index + extra indirection for validating use-after-free for parameter blocks

use std::ptr::NonNull;

use aleph_any::{AnyArc, declare_interfaces};
use aleph_rhi_api::*;

use crate::internal::parameter_block::ParameterBlock;
use crate::{ValidationDevice, ValidationParameterBlockLayout};

pub struct ValidationDescriptorPool {
    pub(crate) _device: AnyArc<ValidationDevice>,
    pub(crate) _layout: AnyArc<ValidationParameterBlockLayout>,
    pub(crate) inner: Box<dyn IDescriptorPool>,
    pub(crate) pool_id: u64,
    pub(crate) block_objects: Vec<ParameterBlock>,
    pub(crate) free_list: Vec<ParameterBlockHandle>,
}

declare_interfaces!(ValidationDescriptorPool, [IDescriptorPool]);

crate::impl_platform_interface_passthrough!(ValidationDescriptorPool);

impl ValidationDescriptorPool {
    /// Checks if there is space to allocate a new set in the descriptor pool.
    ///
    /// # Warning
    ///
    /// This function assumes it is being called immediately prior to trying to allocate a set. As
    /// such it returns an OOM error instead of a simple bool.
    fn check_oom(&mut self) -> Result<(), DescriptorPoolAllocateError> {
        if self.block_objects.len() == self.block_objects.capacity() {
            Err(DescriptorPoolAllocateError::OutOfMemory)
        } else {
            Ok(())
        }
    }

    /// Constructs a [`ParameterBlock`] object for the parameter block with index 'block_index'.
    ///
    /// Specifically this creates the object that a [`ParameterBlockHandle`] is actually a pointer to.
    /// This will contain fully computed CPU and GPU handles to the resource and sampler
    /// `ID3D12DescriptorHeap` heaps. This should be called to initialize a [`ParameterBlock`] in the
    /// 'block_objects' pool.
    ///
    /// Once constructed the [`ParameterBlock`] should be immutable, as it will always refer to the
    /// descriptor memory for the set index it was constructed with.
    ///
    /// This function is expected to be used when allocating a new set out of `self.block_pool`
    /// instead of from the free list.
    fn create_block_object_for_block_index(
        &mut self,
        inner: ParameterBlockHandle,
    ) -> ParameterBlock {
        ParameterBlock {
            _magic_header: ParameterBlock::MAGIC_HEADER_VAL,
            _pool_id: self.pool_id,
            _layout: self._layout.clone(),
            inner,
        }
    }

    /// Constructs a [`ParameterBlockHandle`] for the set with index 'block_index'
    ///
    /// From an implementation stand point [`ParameterBlockHandle`] is just an opaque pointer. For
    /// this implementation it contains a pointer to a [`ParameterBlock`] instance. Specifically it
    /// contains a pointer to a set inside the `self.block_objects` array.
    ///
    /// This function is thus just a utility for getting the pointer to the [`ParameterBlock`] object
    /// with the requested index and converting that pointer into a [`ParameterBlockHandle`] so we can
    /// hand it out to callers on the other side of the API boundary.
    ///
    /// # Safety
    ///
    /// It is the caller's responsibility to check if 'block_index' is within the bounds of the
    /// `self.block_objects` array.
    ///
    /// It is also the caller's responsibility to ensure sound access through the pointer that
    /// the [ParameterBlockHandle] actually is.
    unsafe fn convert_block_index_to_handle(&self, block_index: usize) -> ParameterBlockHandle {
        // Get a pointer to the block_index'th element in the array and wrap it into a handle.
        //
        // We have to be careful to not construct a reference here to keep the code sound. From the
        // borrow checker's perspective creating the reference here (even though we don't
        // dereference it) would be unsound if we didn't synchronize with any api that would create
        // a reference from a handle.
        //
        // In practice this would mean any API that uses parameter blocks would be unsound if used
        // in parallel with this code by rust's soundness rules, which would be insane.
        //
        // By carefully using only pointers here it means we never create a reference to any of the
        // elements of the 'block_objects' array *except* inside other APIs by converting the handle
        // back to a pointer and converting to a reference.
        //
        // This won't solve use-after-free, or un-synchronized access to the underlying descriptor
        // memory. Thus any API that reads or writes from descriptors or parameter blocks is unsafe.
        let handle = self.block_objects.as_ptr().wrapping_add(block_index);
        let handle = handle as *mut ParameterBlock;

        // Safety: It's the caller's responsibility to ensure 'block_index' is in bounds. The pointer
        // is guaranteed to be valid if the index is in bounds.
        unsafe {
            let handle = NonNull::new_unchecked(handle).cast::<()>();

            // Safety: no actual unsafe code here, just a warning to make sure people don't use this
            //         unless they absolutely need to.
            ParameterBlockHandle::from_raw(handle)
        }
    }

    fn validate_block_handle(&self, set: ParameterBlockHandle) {
        // Validate that a ParameterBlockHandle contains a correctly aligned pointer. This may help
        // catch when someone is passing in bad handles
        let align = core::mem::align_of::<ParameterBlock>();
        let block = unsafe { core::mem::transmute::<_, NonNull<ParameterBlock>>(set) };

        // This should also never happen in practice, but can help flag when people are doing
        // naughty bit casts and passing bad pointers in.
        if !align.is_power_of_two() {
            panic!("is_aligned_to: align is not a power-of-two");
        }
        assert_eq!(
            (block.as_ptr() as usize) & (align - 1),
            0,
            "ParameterBlockHandle contains badly-aligned pointer"
        );

        // If the pool is empty it's impossible for any handles to be from this particular pool.
        // This should never happen as we never allow empty descriptor pools.
        assert!(
            self.block_objects.is_empty(),
            "The ParameterBlock pool is empty, no handle can be valid"
        );

        let blocks_base = self.block_objects.as_ptr();
        let blocks_end = self
            .block_objects
            .as_ptr()
            .wrapping_add(self.block_objects.len());

        // This should never happen, but we check for completeness sake.
        assert!(
            blocks_base < blocks_end,
            "The ParameterBlock pool has overflowed the address space"
        );

        // Checks if the given parameter block was allocated by this pool by checking if the pointer
        // comes from inside the block_objects array bounds.
        let block_ptr = block.as_ptr() as *const ParameterBlock;
        let block_oob = block_ptr < blocks_base || block_ptr > blocks_end;
        assert!(
            !block_oob,
            "The ParameterBlockHandle points outside of the pool, this handle is from another pool"
        );
    }
}

impl IDescriptorPool for ValidationDescriptorPool {
    fn allocate_block(&mut self) -> Result<ParameterBlockHandle, DescriptorPoolAllocateError> {
        // First try and grab something from the free list
        if let Some(handle) = self.free_list.pop() {
            return Ok(handle);
        }

        // We don't need to check OOM unless we're trying to allocate a new set object
        self.check_oom()?;

        let inner = self.inner.allocate_block()?;

        // Take the next free set, we create fresh set objects linearly
        let block_index = self.block_objects.len();
        let set = self.create_block_object_for_block_index(inner);
        self.block_objects.push(set);

        // Safety: block_index is guaranteed to be < block_objects.len() because it is created from
        // block_objects.len() immediately prior to pushing a new element in block_objects.
        let handle = unsafe {
            assert!(
                block_index < self.block_objects.len(),
                "'block_index' is out of bounds"
            );
            self.convert_block_index_to_handle(block_index)
        };

        Ok(handle)
    }

    unsafe fn free(&mut self, blocks: &[ParameterBlockHandle]) {
        unsafe {
            for &block in blocks {
                self.validate_block_handle(block);

                // Does further validation based on reading the block object itself
                ParameterBlock::validate(block, Some(self.pool_id));

                let inner: NonNull<()> = block.into();
                let inner: NonNull<ParameterBlock> = inner.cast();
                let inner = inner.as_ref();

                // Validation is done, free the block.
                self.inner.free(&[inner.inner]);

                self.free_list.push(block);
            }
        }
    }

    unsafe fn reset(&mut self) {
        unsafe {
            self.inner.reset();
            self.block_objects.clear();
            self.free_list.clear();
        }
    }
}
