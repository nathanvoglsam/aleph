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
use std::any::TypeId;
use std::mem::MaybeUninit;
use std::ptr::NonNull;

use aleph_alloc::offset_allocator::OffsetAllocator;
use aleph_any::{AnyArc, declare_interfaces};
use aleph_rhi_api::*;
use aleph_rhi_impl_utils::parameter_block_pool::{IBlockFactory, ParameterBlockPool};
use allocator_api2::alloc::{Allocator, Global};
use blink_alloc::BlinkAlloc;
use windows::utils::GPUDescriptorHandle;

use crate::device::Device;
use crate::internal::descriptor_chunk::DescriptorChunk;
use crate::internal::parameter_block::ParameterBlock;
use crate::internal::unwrap;
use crate::parameter_block_layout::ParameterBlockLayout;

pub struct DescriptorArenaLinear {
    pub(crate) _device: AnyArc<Device>,
    pub(crate) resource_arena: DescriptorChunk,
    pub(crate) pool: ParameterBlockPool<LinearBlockFactory>,
}

declare_interfaces!(DescriptorArenaLinear, [IDescriptorArena]);

impl IGetPlatformInterface for DescriptorArenaLinear {
    unsafe fn __query_platform_interface(&self, _target: TypeId, _out: *mut ()) -> Option<()> {
        None
    }
}

impl IDescriptorArena for DescriptorArenaLinear {
    fn allocate_block(
        &self,
        layout: &dyn IParameterBlockLayout,
    ) -> Result<ParameterBlockHandle, DescriptorAllocateError> {
        let layout = unwrap::parameter_block_layout(layout);

        let mut blocks: [MaybeUninit<_>; 1] = [MaybeUninit::uninit(); 1];
        self.pool
            .allocate_blocks((Some(&self.resource_arena), layout), &mut blocks)?;

        unsafe {
            let block = blocks[0].assume_init();
            Ok(block)
        }
    }

    fn allocate_blocks(
        &self,
        layout: &dyn IParameterBlockLayout,
        num_blocks: usize,
    ) -> Result<Box<[ParameterBlockHandle]>, DescriptorAllocateError> {
        let layout = unwrap::parameter_block_layout(layout);

        let mut blocks = Box::new_uninit_slice(num_blocks);
        self.pool
            .allocate_blocks((Some(&self.resource_arena), layout), &mut blocks)?;

        let blocks = Box::leak(blocks);
        let blocks = NonNull::from(blocks);
        let blocks =
            NonNull::slice_from_raw_parts(blocks.cast::<ParameterBlockHandle>(), blocks.len());
        unsafe { Ok(Box::from_raw(blocks.as_ptr())) }
    }

    unsafe fn free(&self, _blocks: &[ParameterBlockHandle]) {
        unreachable!("It is illegal to call 'free' on a 'linear' descriptor arena");
    }

    unsafe fn reset(&self) {
        unsafe { self.pool.reset_pool() }
    }
}

impl Drop for DescriptorArenaLinear {
    fn drop(&mut self) {
        // Safety:
        // It's not possible to use the DescriptorArena, and thus the Chunk, again as we're in
        // the drop implementation.
        //
        // We can't prevent user's further up the callstack from trying to use descriptors from
        // the pool (and arena) after calling this. This is reflected in all APIs that use them
        // being unsafe. We still leave preventing user-after-free to the caller.
        unsafe {
            self.resource_arena
                .release_allocation_to_heap(self._device.descriptor_heaps.gpu_view_heap());
        }
    }
}

pub struct LinearBlockFactory {
    /// Bump allocator offset into 'resource_arena'.
    pub next_resource_index: usize,

    /// A bump arena used to allocate the backing buffers for the sampler and dynamic cb arrays
    /// inside the set objects.
    pub arena: BlinkAlloc,
}

unsafe impl IBlockFactory for LinearBlockFactory {
    type Param<'a> = (Option<&'a DescriptorChunk>, &'a ParameterBlockLayout);
    type T = ParameterBlock;

    fn init_blocks<'a>(
        &mut self,
        p: Self::Param<'a>,
        blocks: &mut [MaybeUninit<Self::T>],
    ) -> Result<(), DescriptorAllocateError> {
        let (resource_arena, block_layout) = p;
        let num_resources = block_layout.compiled.resources.num_resources() as usize;
        let num_samplers = block_layout.compiled.samplers.num_samplers() as usize;

        debug_assert!(
            !(resource_arena.is_none() && num_resources != 0),
            "The resource layout needs a resource arena, but none was given!"
        );

        // Check if the given arena has enough space to serve the requested number of resources.
        if let Some(resource_arena) = resource_arena {
            let total_num_resources = num_resources * blocks.len();
            let end = self.next_resource_index + total_num_resources;
            if end > resource_arena.num_descriptors as usize {
                return Err(DescriptorAllocateError::OutOfMemory);
            }
        }

        for block in blocks {
            // Create the sampler array, only if the layout requires them.
            let samplers = if num_samplers == 0 {
                NonNull::slice_from_raw_parts(NonNull::dangling(), 0)
            } else {
                let layout = Layout::array::<Option<GPUDescriptorHandle>>(num_samplers).unwrap();
                let result = self.arena.allocate_zeroed(layout);
                let result = match result {
                    Ok(v) => v,
                    Err(_) => handle_alloc_error(layout),
                };
                NonNull::slice_from_raw_parts(result.cast(), num_samplers)
            };

            // Get the sub-slice of the resource arena that we've allocated for this parameter
            // block.
            let (cpu, gpu) = if let Some(resource_arena) = resource_arena {
                let (cpu, gpu) =
                    resource_arena.get_handles_for_index(self.next_resource_index as u32);
                self.next_resource_index += num_resources;
                (Some(cpu), Some(gpu))
            } else {
                (None, None)
            };

            let new = ParameterBlock {
                _layout: NonNull::from(block_layout),
                resource_allocation: Default::default(), // Never used here
                resource_handle_cpu: cpu,
                resource_handle_gpu: gpu,
                samplers,
            };
            block.write(new);
        }
        Ok(())
    }

    fn reuse_blocks(
        &mut self,
        _p: Self::Param<'_>,
        _blocks: impl Iterator<Item = NonNull<Self::T>>,
    ) -> Result<(), DescriptorAllocateError> {
        // Intentional no-op
        Ok(())
    }

    fn free_blocks(&mut self, _blocks: impl Iterator<Item = NonNull<Self::T>>) {
        // Intentional no-op
    }

    fn reset_blocks(&mut self, blocks: &mut [Self::T]) {
        for block in blocks {
            // block.resource_allocation is unused in this use case
            block.resource_handle_cpu = None;
            block.resource_handle_gpu = None;
            block.samplers = NonNull::slice_from_raw_parts(NonNull::dangling(), 0);
        }
        self.next_resource_index = 0;
        self.arena.reset();
    }

    fn drop_blocks(&mut self, _blocks: &mut [Self::T]) {
        // Intentional no-op
        //
        // All allocations made by 'LinearBlockFactory' are from internal pools. There's no work
        // that needs to be done here, the pools clean themselves up while being dropped.
    }
}

pub struct DescriptorArenaHeap {
    pub(crate) _device: AnyArc<Device>,
    pub(crate) resource_block: DescriptorChunk,
    pub(crate) pool: ParameterBlockPool<HeapBlockFactory>,
}

declare_interfaces!(DescriptorArenaHeap, [IDescriptorArena]);

impl IGetPlatformInterface for DescriptorArenaHeap {
    unsafe fn __query_platform_interface(&self, _target: TypeId, _out: *mut ()) -> Option<()> {
        None
    }
}

impl IDescriptorArena for DescriptorArenaHeap {
    fn allocate_block(
        &self,
        layout: &dyn IParameterBlockLayout,
    ) -> Result<ParameterBlockHandle, DescriptorAllocateError> {
        let layout = unwrap::parameter_block_layout(layout);

        let mut blocks: [MaybeUninit<_>; 1] = [MaybeUninit::uninit(); 1];
        self.pool
            .allocate_blocks((&self.resource_block, layout), &mut blocks)?;

        unsafe {
            let block = blocks[0].assume_init();
            Ok(block)
        }
    }

    fn allocate_blocks(
        &self,
        layout: &dyn IParameterBlockLayout,
        num_blocks: usize,
    ) -> Result<Box<[ParameterBlockHandle]>, DescriptorAllocateError> {
        let layout = unwrap::parameter_block_layout(layout);

        let mut blocks = Box::new_uninit_slice(num_blocks);
        self.pool
            .allocate_blocks((&self.resource_block, layout), &mut blocks)?;

        let blocks = Box::leak(blocks);
        let blocks = NonNull::from(blocks);
        let blocks =
            NonNull::slice_from_raw_parts(blocks.cast::<ParameterBlockHandle>(), blocks.len());
        unsafe { Ok(Box::from_raw(blocks.as_ptr())) }
    }

    unsafe fn free(&self, blocks: &[ParameterBlockHandle]) {
        self.pool.free_blocks(blocks);
    }

    unsafe fn reset(&self) {
        unsafe {
            self.pool.reset_pool();
        }
    }
}

impl Drop for DescriptorArenaHeap {
    fn drop(&mut self) {
        // Safety:
        // It's not possible to use the DescriptorArena, and thus the Chunk, again as we're in
        // the drop implementation.
        //
        // We can't prevent user's further up the callstack from trying to use descriptors from
        // the pool (and arena) after calling this. This is reflected in all APIs that use them
        // being unsafe. We still leave preventing user-after-free to the caller.
        unsafe {
            self.resource_block
                .release_allocation_to_heap(self._device.descriptor_heaps.gpu_view_heap());
        }
    }
}

pub struct HeapBlockFactory {
    /// Allocation state used for allocating from the resource arena
    pub(crate) resource_pool: Box<OffsetAllocator>,
}

unsafe impl IBlockFactory for HeapBlockFactory {
    type Param<'a> = (&'a DescriptorChunk, &'a ParameterBlockLayout);
    type T = ParameterBlock;

    fn init_blocks<'a>(
        &mut self,
        p: Self::Param<'a>,
        blocks: &mut [MaybeUninit<Self::T>],
    ) -> Result<(), DescriptorAllocateError> {
        let (resource_arena, block_layout) = p;
        let num_resources = block_layout.compiled.resources.num_resources() as usize;
        let num_samplers = block_layout.compiled.samplers.num_samplers() as usize;

        for block in blocks {
            // Create the sampler array, only if the layout requires them.
            let samplers = if num_samplers == 0 {
                NonNull::slice_from_raw_parts(NonNull::dangling(), 0)
            } else {
                let layout = Layout::array::<Option<GPUDescriptorHandle>>(num_samplers).unwrap();
                let result = Global.allocate_zeroed(layout);
                let result = match result {
                    Ok(v) => v,
                    Err(_) => handle_alloc_error(layout),
                };
                NonNull::slice_from_raw_parts(result.cast(), num_samplers)
            };

            // Only allocate a resource block if we need some. We always free the resource block
            // and never recycle them like the sampler arrays.
            let (alloc, cpu, gpu) = if num_resources != 0 {
                let alloc = self.resource_pool.allocate(num_resources as u32);

                if alloc.is_fail() {
                    return Err(DescriptorAllocateError::OutOfMemory);
                }

                let (cpu, gpu) = resource_arena.get_handles_for_index(alloc.offset);
                (alloc, Some(cpu), Some(gpu))
            } else {
                (Default::default(), None, None)
            };

            let new = ParameterBlock {
                _layout: NonNull::from(block_layout),
                resource_allocation: alloc,
                resource_handle_cpu: cpu,
                resource_handle_gpu: gpu,
                samplers,
            };
            block.write(new);
        }
        Ok(())
    }

    fn reuse_blocks(
        &mut self,
        p: Self::Param<'_>,
        blocks: impl Iterator<Item = NonNull<Self::T>>,
    ) -> Result<(), DescriptorAllocateError> {
        let (resource_arena, block_layout) = p;
        let num_resources = block_layout.compiled.resources.num_resources() as usize;
        let num_samplers = block_layout.compiled.samplers.num_samplers() as usize;

        for mut block in blocks {
            unsafe {
                let block = block.as_mut();
                match (num_samplers, block.samplers.len()) {
                    // Doesn't need samplers, do nothing! We're done!
                    (0, _) => {}

                    // Needs samplers, but doesn't have any? Allocate a new array!
                    (needs, 0) => {
                        let layout = Layout::array::<Option<GPUDescriptorHandle>>(needs).unwrap();
                        let result = Global.allocate_zeroed(layout);
                        let result = match result {
                            Ok(v) => v,
                            Err(_) => handle_alloc_error(layout),
                        };
                        block.samplers = NonNull::slice_from_raw_parts(result.cast(), needs)
                    }

                    // Needs samplers, and has some already? Check if we have enough, otherwise
                    // make a new array.
                    (needs, has) => {
                        // Only need to allocate if there isn't enough space in the existing array
                        if needs > has {
                            // Free the old array
                            let layout = Layout::for_value(block.samplers.as_ref());
                            Global.deallocate(block.samplers.cast(), layout);

                            // And make a new one!
                            let layout =
                                Layout::array::<Option<GPUDescriptorHandle>>(needs).unwrap();
                            let result = Global.allocate_zeroed(layout);
                            let result = match result {
                                Ok(v) => v,
                                Err(_) => handle_alloc_error(layout),
                            };
                            block.samplers = NonNull::slice_from_raw_parts(result.cast(), needs)
                        }
                    }
                }

                // Only allocate a resource block if we need some. We always free the resource block
                // and never recycle them like the sampler arrays.
                if num_resources != 0 {
                    let allocation = self.resource_pool.allocate(num_resources as u32);

                    if allocation.is_fail() {
                        return Err(DescriptorAllocateError::OutOfMemory);
                    }

                    let (cpu, gpu) = resource_arena.get_handles_for_index(allocation.offset);
                    block.resource_allocation = allocation;
                    block.resource_handle_cpu = Some(cpu);
                    block.resource_handle_gpu = Some(gpu);
                }
            }
        }
        Ok(())
    }

    fn free_blocks(&mut self, blocks: impl Iterator<Item = NonNull<Self::T>>) {
        for mut block in blocks {
            unsafe {
                let block = block.as_mut();

                // Intentionally skip free-ing the sampler array as we are likely to be able to
                // re-use it in-place

                // Free the GPU descriptor heap allocation
                if !block.resource_allocation.is_fail() {
                    self.resource_pool.free(block.resource_allocation);
                    block.resource_allocation = Default::default();
                    block.resource_handle_cpu = None;
                    block.resource_handle_gpu = None;
                }
            }
        }
    }

    fn reset_blocks(&mut self, blocks: &mut [Self::T]) {
        for block in blocks {
            // Free the samplers using the global allocator
            unsafe {
                if !block.samplers.is_empty() {
                    let layout = Layout::for_value(block.samplers.as_ref());
                    Global.deallocate(block.samplers.cast(), layout);
                    block.samplers = NonNull::slice_from_raw_parts(NonNull::dangling(), 0);
                }
            }

            // Free the GPU descriptor heap allocation
            if !block.resource_allocation.is_fail() {
                self.resource_pool.free(block.resource_allocation);
                block.resource_allocation = Default::default();
                block.resource_handle_cpu = None;
                block.resource_handle_gpu = None;
            }
        }
    }

    fn drop_blocks(&mut self, blocks: &mut [Self::T]) {
        // We skip freeing the 'resource_allocation' in 'drop_blocks'. The pool is going to be
        // dropped too so there's no point, the memory will be cleaned up in bulk.
        for block in blocks {
            // Free the samplers using the global allocator
            unsafe {
                if !block.samplers.is_empty() {
                    let layout = Layout::for_value(block.samplers.as_ref());
                    Global.deallocate(block.samplers.cast(), layout);
                    block.samplers = NonNull::slice_from_raw_parts(NonNull::dangling(), 0);
                }
            }
        }
    }
}
