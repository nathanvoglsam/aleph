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

use std::cell::Cell;
use std::ptr::NonNull;

use aleph_any::{AnyArc, declare_interfaces};
use aleph_rhi_api::*;

use crate::internal::parameter_block::ParameterBlock;
use crate::internal::unwrap;
use crate::{ValidationDevice, ValidationParameterBlockLayout};

pub struct ValidationDescriptorArena {
    pub(crate) _device: AnyArc<ValidationDevice>,
    pub(crate) inner: Box<dyn IDescriptorArena>,
    pub(crate) pool_id: u64,
    pub(crate) block_objects: Cell<Vec<ParameterBlock>>,
    pub(crate) free_list: Cell<Vec<ParameterBlockHandle>>,
}

declare_interfaces!(ValidationDescriptorArena, [IDescriptorArena]);

crate::impl_platform_interface_passthrough!(ValidationDescriptorArena);

impl ValidationDescriptorArena {
    /// Checks if there is space to allocate a new block in the descriptor pool.
    ///
    /// # Warning
    ///
    /// This function assumes it is being called immediately prior to trying to allocate a set. As
    /// such it returns an OOM error instead of a simple bool.
    fn check_oom(&self) -> Result<(), DescriptorArenaAllocateError> {
        let block_objects = self.block_objects.take();
        let out = if block_objects.len() == block_objects.capacity() {
            Err(DescriptorArenaAllocateError::OutOfMemory)
        } else {
            Ok(())
        };
        self.block_objects.set(block_objects);
        out
    }

    /// Constructs a [`ParameterBlock`] object for the parameter block with index 'block_index'.
    ///
    /// Specifically this creates the object that a [ParameterBlockHandle] is actually a pointer to.
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
        &self,
        inner: ParameterBlockHandle,
        layout: AnyArc<ValidationParameterBlockLayout>,
    ) -> ParameterBlock {
        ParameterBlock {
            _magic_header: ParameterBlock::MAGIC_HEADER_VAL,
            _pool_id: self.pool_id,
            _layout: layout,
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
        let block_objects = self.block_objects.take();
        let handle = block_objects.as_ptr().wrapping_add(block_index);
        let handle = handle as *mut ParameterBlock;
        self.block_objects.set(block_objects);

        // Safety: It's the caller's responsibility to ensure 'block_index' is in bounds. The pointer
        // is guaranteed to be valid if the index is in bounds.
        unsafe {
            let handle = NonNull::new_unchecked(handle).cast::<()>();

            // Safety: no actual unsafe code here, just a warning to make sure people don't use this
            //         unless they absolutely need to.
            ParameterBlockHandle::from_raw(handle)
        }
    }

    fn validate_block_handle(&self, block: ParameterBlockHandle) {
        // Validate that a ParameterBlockHandle contains a correctly aligned pointer. This may help
        // catch when someone is passing in bad handles
        let align = align_of::<ParameterBlock>();
        let block = ParameterBlock::ptr_from_handle(block);

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

        let block_objects = self.block_objects.take();

        // If the pool is empty it's impossible for any handles to be from this particular pool.
        // This should never happen as we never allow empty descriptor pools.
        assert!(
            !block_objects.is_empty(),
            "The ParameterBlock pool is empty, no handle can be valid"
        );

        let blocks_base = block_objects.as_ptr();
        let blocks_end = block_objects.as_ptr().wrapping_add(block_objects.len());

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

        self.block_objects.set(block_objects);
    }
}

impl IDescriptorArena for ValidationDescriptorArena {
    fn allocate_block(
        &self,
        layout: &dyn IParameterBlockLayout,
    ) -> Result<ParameterBlockHandle, DescriptorArenaAllocateError> {
        let layout = unwrap::parameter_block_layout(layout);
        assert!(
            !layout
                .desc()
                .flags
                .contains(ParameterBlockFlags::PUSH_DESCRIPTOR),
            "Allocating from an IDescriptorArena using a IParameterBlockLayout with the 'PUSH_DESCRIPTOR' flag is not allowed"
        );

        // First try and grab something from the free list
        let mut free_list = self.free_list.take();
        if let Some(handle) = free_list.pop() {
            self.free_list.set(free_list);
            return Ok(handle);
        }
        self.free_list.set(free_list);

        // We don't need to check OOM unless we're trying to allocate a new set object
        self.check_oom()?;

        let inner = self.inner.allocate_block(layout.inner.as_ref())?;

        // Take the next free set, we create fresh set objects linearly
        let mut block_objects = self.block_objects.take();
        let block_index = block_objects.len();
        let set = self.create_block_object_for_block_index(inner, layout.this.upgrade().unwrap());
        block_objects.push(set);

        // Safety: block_index is guaranteed to be < block_objects.len() because it is created from
        // block_objects.len() immediately prior to pushing a new element in block_objects.
        let handle = unsafe {
            assert!(
                block_index < block_objects.len(),
                "'block_index' is out of bounds"
            );
            self.block_objects.set(block_objects);
            self.convert_block_index_to_handle(block_index)
        };

        Ok(handle)
    }

    unsafe fn free(&self, blocks: &[ParameterBlockHandle]) {
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

                let mut free_list = self.free_list.take();
                free_list.push(block);
                self.free_list.set(free_list);
            }
        }
    }

    unsafe fn reset(&self) {
        unsafe {
            self.inner.reset();
        }

        let mut block_objects = self.block_objects.take();
        block_objects.clear();
        self.block_objects.set(block_objects);

        let mut free_list = self.free_list.take();
        free_list.clear();
        self.free_list.set(free_list);
    }
}
