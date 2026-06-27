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
use std::mem::MaybeUninit;
use std::ptr::NonNull;
use std::sync::Arc;

use aleph_alloc::instrumentation::{IAllocationCategory, system};
use aleph_alloc::offset_allocator::OffsetAllocator;
use aleph_rhi_api::*;
use aleph_rhi_impl_utils::parameter_block_pool::{IBlockFactory, ParameterBlockPool};
use aleph_rhi_impl_utils::{Rhi, RhiSystem};
use blink_alloc::BlinkAlloc;

use crate::device::Device;
use crate::internal::memory_block::MemoryBlock;
use crate::internal::parameter_block::ParameterBlock;
use crate::internal::unwrap;
use crate::parameter_block_layout::ParameterBlockLayout;

pub struct DescriptorArenaLinear {
    pub(crate) _device: Arc<Device>,
    pub(crate) memory_block: MemoryBlock,
    pub(crate) pool: ParameterBlockPool<LinearBlockFactory>,
}

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
            .allocate_blocks((&self.memory_block, layout), &mut blocks)?;

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
            .allocate_blocks((&self.memory_block, layout), &mut blocks)?;

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

impl DescriptorArenaLinear {
    pub(crate) fn create(
        device: &Device,
        desc: &DescriptorArenaDesc,
    ) -> Result<Box<dyn IDescriptorArena>, DescriptorPoolCreateError> {
        assert_eq!(desc.arena_type, DescriptorArenaType::Linear);

        let len = desc.num_blocks as usize * 16;
        let memory_block =
            MemoryBlock::new(device, len).ok_or(DescriptorPoolCreateError::Platform)?;

        let factory = LinearBlockFactory {
            next_resource_index: 0,
            arena: BlinkAlloc::new_in(system()),
        };
        let pool = ParameterBlockPool::new(factory, desc.num_blocks as usize);

        let pool: Box<dyn IDescriptorArena> = Box::new(DescriptorArenaLinear {
            _device: device.this.upgrade().unwrap(),
            memory_block,
            pool,
        });

        Ok(pool)
    }
}

pub struct LinearBlockFactory {
    /// Bump allocator offset into 'resource_arena'.
    pub next_resource_index: usize,

    /// A bump arena used to allocate the backing buffers for the useResources arrays.
    pub arena: BlinkAlloc<RhiSystem>,
}

unsafe impl IBlockFactory for LinearBlockFactory {
    type Param<'a> = (&'a MemoryBlock, &'a ParameterBlockLayout);
    type T = ParameterBlock;

    fn init_blocks<'a>(
        &mut self,
        p: Self::Param<'a>,
        blocks: &mut [MaybeUninit<Self::T>],
    ) -> Result<(), DescriptorAllocateError> {
        let (memory_block, block_layout) = p;
        let num_arguments = block_layout.compiled.num_arguments;

        // Check if the given block has enough space to serve the requested number of resources.
        let total_num = num_arguments * blocks.len();
        let end = self.next_resource_index + total_num;
        if end * 8 > memory_block.len {
            return Err(DescriptorAllocateError::OutOfMemory);
        }

        for block in blocks {
            // Get the sub-slice of the resource arena that we've allocated for this parameter
            // block.
            let offset = self.next_resource_index * 8;
            let cpu_addr = unsafe { memory_block.cpu_addr.add(offset) };
            let gpu_addr = memory_block.gpu_addr.saturating_add(offset as u64);
            self.next_resource_index += num_arguments;

            let new = ParameterBlock {
                resource_allocation: Default::default(), // Never used here
                cpu_addr: Some(cpu_addr.cast()),
                gpu_addr: Some(gpu_addr),
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
            block.cpu_addr = None;
            block.gpu_addr = None;
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
    pub(crate) _device: Arc<Device>,
    pub(crate) memory_block: MemoryBlock,
    pub(crate) pool: ParameterBlockPool<HeapBlockFactory>,
}

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
            .allocate_blocks((&self.memory_block, layout), &mut blocks)?;

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
            .allocate_blocks((&self.memory_block, layout), &mut blocks)?;

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

impl DescriptorArenaHeap {
    pub(crate) fn create(
        device: &Device,
        desc: &DescriptorArenaDesc,
    ) -> Result<Box<dyn IDescriptorArena>, DescriptorPoolCreateError> {
        assert_eq!(desc.arena_type, DescriptorArenaType::Heap);

        let len = desc.num_blocks * 16 * 8;
        let memory_block =
            MemoryBlock::new(device, len as usize).ok_or(DescriptorPoolCreateError::Platform)?;

        let resource_pool = Rhi::with(|| {
            let v = OffsetAllocator::new(len, desc.num_blocks * 2);
            Box::new(v)
        });

        let factory = HeapBlockFactory { resource_pool };
        let pool = ParameterBlockPool::new(factory, desc.num_blocks as usize);

        let pool: Box<dyn IDescriptorArena> = Box::new(DescriptorArenaHeap {
            _device: device.this.upgrade().unwrap(),
            memory_block,
            pool,
        });

        Ok(pool)
    }
}

pub struct HeapBlockFactory {
    /// Allocation state used for allocating from the resource arena
    pub(crate) resource_pool: Box<OffsetAllocator>,
}

unsafe impl IBlockFactory for HeapBlockFactory {
    type Param<'a> = (&'a MemoryBlock, &'a ParameterBlockLayout);
    type T = ParameterBlock;

    fn init_blocks<'a>(
        &mut self,
        p: Self::Param<'a>,
        blocks: &mut [MaybeUninit<Self::T>],
    ) -> Result<(), DescriptorAllocateError> {
        let (memory_block, block_layout) = p;
        let num_arguments = block_layout.compiled.num_arguments;

        for block in blocks {
            let alloc = self.resource_pool.allocate(num_arguments as u32);

            if alloc.is_fail() {
                return Err(DescriptorAllocateError::OutOfMemory);
            }

            // Get the sub-slice of the resource arena that we've allocated for this parameter
            // block.
            let offset = alloc.offset as usize * 8;
            let cpu_addr = unsafe { memory_block.cpu_addr.add(offset) };
            let gpu_addr = memory_block.gpu_addr.saturating_add(offset as u64);

            let new = ParameterBlock {
                resource_allocation: alloc,
                cpu_addr: Some(cpu_addr.cast()),
                gpu_addr: Some(gpu_addr),
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
        let (memory_block, block_layout) = p;
        let num_arguments = block_layout.compiled.num_arguments;

        for mut block in blocks {
            unsafe {
                let block = block.as_mut();

                let allocation = self.resource_pool.allocate(num_arguments as u32);

                if allocation.is_fail() {
                    return Err(DescriptorAllocateError::OutOfMemory);
                }

                // Get the sub-slice of the resource arena that we've allocated for this parameter
                // block.
                let offset = allocation.offset as usize * 8;
                let cpu = memory_block.cpu_addr.add(offset);
                let gpu = memory_block.gpu_addr.saturating_add(offset as u64);

                block.resource_allocation = allocation;
                block.cpu_addr = Some(cpu.cast());
                block.gpu_addr = Some(gpu);
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
                    block.cpu_addr = None;
                    block.gpu_addr = None;
                }
            }
        }
    }

    fn reset_blocks(&mut self, blocks: &mut [Self::T]) {
        for block in blocks {
            // Free the GPU descriptor heap allocation
            if !block.resource_allocation.is_fail() {
                self.resource_pool.free(block.resource_allocation);
                block.resource_allocation = Default::default();
                block.cpu_addr = None;
                block.gpu_addr = None;
            }
        }
    }

    fn drop_blocks(&mut self, _blocks: &mut [Self::T]) {
        // no-op
    }
}
