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

use std::alloc::{handle_alloc_error, Layout};
use std::cell::Cell;
use std::mem::{size_of, MaybeUninit};
use std::ptr::NonNull;

use aleph_rhi_api::*;
use blink_alloc::BlinkAlloc;

use crate::internal::descriptor_set::DescriptorSet;

pub struct DescriptorSetPool {
    /// Pool we allocate set objects from
    pub pool: BlinkAlloc,

    /// The maximum number of sets this pool can contain
    pub capacity: u32,

    /// The number of sets currently allocated into the pool
    pub num_sets: Cell<u32>,

    /// Free list of descriptors
    pub free_list: Cell<Vec<DescriptorSetHandle>>,
}

impl DescriptorSetPool {
    pub fn new(capacity: u32) -> Self {
        let pool = BlinkAlloc::with_chunk_size(size_of::<DescriptorSet>() * capacity as usize);
        let num_sets = Cell::new(0);
        let free_list = Cell::new(Vec::with_capacity(64));
        Self {
            pool,
            capacity,
            num_sets,
            free_list,
        }
    }

    /// Allocate the requested number of sets into the given array.
    ///
    /// Returns the number of sets that were taken from the free list. This is useful to know in
    /// the event the outer descriptor set allocator can reuse the arrays inside the set.
    pub fn allocate_sets(&self, sets: &mut [MaybeUninit<DescriptorSetHandle>]) -> Option<usize> {
        // First try and take from the set object free list
        let mut free_list = self.free_list.take();

        // We can't ever allocate more then 'capacity' sets so just immediately exit
        if sets.len() > self.capacity as usize {
            return None;
        }

        // Same if we're asking for more set objects than this pool can provide
        let projected_num_sets = self.num_sets.get() as usize + sets.len();
        if projected_num_sets > self.capacity as usize {
            return None;
        }

        if sets.len() <= free_list.len() {
            let num_from_free_list = sets.len();

            // If all the sets we need are in the free list then we just take that number off the
            // free list
            let start = free_list.len() - sets.len();
            let end = free_list.len();
            free_list
                .drain(start..end)
                .zip(sets)
                .for_each(|(set, dst)| {
                    dst.write(set);
                });

            self.free_list.set(free_list);

            self.num_sets.set(projected_num_sets as u32);
            Some(num_from_free_list)
        } else {
            // Otherwise we drain the entire free list into our output sets array and then we need
            // to allocate 'remaining_sets' from the pool
            let num_from_free_list = free_list.len();
            let remaining_sets = sets.len() - free_list.len();

            let first_sets = &mut sets[0..num_from_free_list];
            debug_assert_eq!(first_sets.len(), free_list.len());

            free_list.drain(..).zip(first_sets).for_each(|(set, dst)| {
                dst.write(set);
            });

            self.free_list.set(free_list);

            let remaining = &mut sets[num_from_free_list..];
            debug_assert_eq!(remaining.len(), remaining_sets);

            let layout = Layout::array::<DescriptorSet>(remaining_sets).unwrap();
            let new_sets = self.pool.allocate(layout);
            let new_sets = match new_sets {
                Ok(v) => v,
                Err(_v) => handle_alloc_error(layout),
            };
            let new_sets = unsafe {
                std::slice::from_raw_parts_mut(
                    new_sets.cast::<MaybeUninit<DescriptorSet>>().as_ptr(),
                    remaining_sets,
                )
            };

            new_sets.iter_mut().zip(remaining).for_each(|(set, dst)| {
                // Initialize the set object
                set.write(DescriptorSet {
                    _layout: NonNull::dangling(),
                    dynamic_constant_buffers: NonNull::from(&[]),
                    resource_allocation: Default::default(),
                    resource_handle_cpu: None,
                    resource_handle_gpu: None,
                    samplers: NonNull::from(&[]),
                });

                let handle = NonNull::from(set);
                let handle = unsafe { DescriptorSetHandle::from_raw(handle.cast()) };
                dst.write(handle);
            });

            self.num_sets.set(projected_num_sets as u32);
            Some(num_from_free_list)
        }
    }

    /// Free the requested number of sets
    pub fn free_sets(&self, v: &[DescriptorSetHandle]) {
        let mut free_list = self.free_list.take();

        self.num_sets.set(self.num_sets.get() - v.len() as u32);
        free_list.extend_from_slice(v);

        self.free_list.set(free_list);
    }

    pub unsafe fn reset_pool(&self) {
        self.num_sets.set(0);

        let mut free_list = self.free_list.take();
        free_list.clear();
        self.free_list.set(free_list);

        self.pool.reset_unchecked();
    }
}
