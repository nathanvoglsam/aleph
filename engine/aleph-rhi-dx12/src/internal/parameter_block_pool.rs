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
use std::cell::Cell;
use std::mem::{MaybeUninit, size_of};
use std::ptr::NonNull;

use aleph_rhi_api::*;
use blink_alloc::BlinkAlloc;

use crate::internal::parameter_block::ParameterBlock;

pub struct ParameterBlockPool {
    /// Pool we allocate block objects from
    pub pool: BlinkAlloc,

    /// The maximum number of blocks this pool can contain
    pub capacity: u32,

    /// The number of blocks currently allocated into the pool
    pub num_blocks: Cell<u32>,

    /// Free list of descriptors
    pub free_list: Cell<Vec<ParameterBlockHandle>>,
}

impl ParameterBlockPool {
    pub fn new(capacity: u32) -> Self {
        let pool = BlinkAlloc::with_chunk_size(size_of::<ParameterBlock>() * capacity as usize);
        let num_blocks = Cell::new(0);
        let free_list = Cell::new(Vec::with_capacity(64));
        Self {
            pool,
            capacity,
            num_blocks,
            free_list,
        }
    }

    /// Allocate the requested number of blocks into the given array.
    ///
    /// Returns the number of blocks that were taken from the free list. This is useful to know in
    /// the event the outer descriptor block allocator can reuse the arrays inside the block.
    pub fn allocate_blocks(
        &self,
        blocks: &mut [MaybeUninit<ParameterBlockHandle>],
    ) -> Option<usize> {
        // First try and take from the block object free list
        let mut free_list = self.free_list.take();

        // We can't ever allocate more then 'capacity' blocks so just immediately exit
        if blocks.len() > self.capacity as usize {
            return None;
        }

        // Same if we're asking for more block objects than this pool can provide
        let projected_num_blocks = self.num_blocks.get() as usize + blocks.len();
        if projected_num_blocks > self.capacity as usize {
            return None;
        }

        if blocks.len() <= free_list.len() {
            let num_from_free_list = blocks.len();

            // If all the blocks we need are in the free list then we just take that number off the
            // free list
            let start = free_list.len() - blocks.len();
            let end = free_list.len();
            free_list
                .drain(start..end)
                .zip(blocks)
                .for_each(|(block, dst)| {
                    dst.write(block);
                });

            self.free_list.set(free_list);

            self.num_blocks.set(projected_num_blocks as u32);
            Some(num_from_free_list)
        } else {
            // Otherwise we drain the entire free list into our output blocks array and then we need
            // to allocate 'remaining_blocks' from the pool
            let num_from_free_list = free_list.len();
            let remaining_blocks = blocks.len() - free_list.len();

            let first_blocks = &mut blocks[0..num_from_free_list];
            debug_assert_eq!(first_blocks.len(), free_list.len());

            free_list
                .drain(..)
                .zip(first_blocks)
                .for_each(|(set, dst)| {
                    dst.write(set);
                });

            self.free_list.set(free_list);

            let remaining = &mut blocks[num_from_free_list..];
            debug_assert_eq!(remaining.len(), remaining_blocks);

            let layout = Layout::array::<ParameterBlock>(remaining_blocks).unwrap();
            let new_blocks = self.pool.allocate(layout);
            let new_blocks = match new_blocks {
                Ok(v) => v,
                Err(_v) => handle_alloc_error(layout),
            };
            let new_blocks = unsafe {
                std::slice::from_raw_parts_mut(
                    new_blocks.cast::<MaybeUninit<ParameterBlock>>().as_ptr(),
                    remaining_blocks,
                )
            };

            new_blocks
                .iter_mut()
                .zip(remaining)
                .for_each(|(block, dst)| {
                    // Initialize the block object
                    block.write(ParameterBlock {
                        _layout: NonNull::dangling(),
                        resource_allocation: Default::default(),
                        resource_handle_cpu: None,
                        resource_handle_gpu: None,
                        samplers: NonNull::from(&[]),
                    });

                    let handle = NonNull::from(block);
                    let handle = unsafe { ParameterBlockHandle::from_raw(handle.cast()) };
                    dst.write(handle);
                });

            self.num_blocks.set(projected_num_blocks as u32);
            Some(num_from_free_list)
        }
    }

    /// Free the requested number of blocks
    pub fn free_blocks(&self, v: &[ParameterBlockHandle]) {
        let mut free_list = self.free_list.take();

        self.num_blocks.set(self.num_blocks.get() - v.len() as u32);
        free_list.extend_from_slice(v);

        self.free_list.set(free_list);
    }

    pub unsafe fn reset_pool(&self) {
        unsafe {
            self.num_blocks.set(0);

            let mut free_list = self.free_list.take();
            free_list.clear();
            self.free_list.set(free_list);

            self.pool.reset_unchecked();
        }
    }
}
