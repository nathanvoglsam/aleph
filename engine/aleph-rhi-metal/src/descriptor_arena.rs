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

use aleph_alloc::instrumentation::{IAllocationCategory, system};
use aleph_any::{AnyArc, declare_interfaces};
use aleph_rhi_api::*;
use aleph_rhi_impl_utils::offset_allocator::OffsetAllocator;
use aleph_rhi_impl_utils::parameter_block_pool::{IBlockFactory, ParameterBlockPool};
use aleph_rhi_impl_utils::{Rhi, RhiSystem};
use allocator_api2::alloc::Allocator;
use blink_alloc::BlinkAlloc;
use objc2::__framework_prelude::ProtocolObject;
use objc2_metal::MTLResource;

use crate::device::Device;
use crate::internal::memory_block::MemoryBlock;
use crate::internal::parameter_block::ParameterBlock;
use crate::internal::unwrap;
use crate::parameter_block_layout::ParameterBlockLayout;

pub struct DescriptorArenaLinear {
    pub(crate) _device: AnyArc<Device>,
    pub(crate) memory_block: MemoryBlock,
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

        let backing_buffer = NonNull::from(memory_block.buffer.as_ref());
        for block in blocks {
            // Get the sub-slice of the resource arena that we've allocated for this parameter
            // block.
            let offset = self.next_resource_index * 8;
            let cpu_addr = unsafe { memory_block.cpu_base.add(offset) };
            self.next_resource_index += num_arguments;

            let num_reads = block_layout.compiled.num_reads;
            let reads = alloc_resource_array(&self.arena, num_reads);

            let num_writes = block_layout.compiled.num_writes;
            let writes = alloc_resource_array(&self.arena, num_writes);

            let new = ParameterBlock {
                _layout: NonNull::from(block_layout),
                backing_buffer,
                resource_allocation: Default::default(), // Never used here
                cpu_addr: Some(cpu_addr.cast()),
                gpu_offset: offset,
                reads,
                writes,
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
            block.gpu_offset = 0;
            block.reads = NonNull::slice_from_raw_parts(NonNull::dangling(), 0);
            block.writes = NonNull::slice_from_raw_parts(NonNull::dangling(), 0);
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
    pub(crate) memory_block: MemoryBlock,
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

        let factory = HeapBlockFactory {
            resource_pool,
            heap: RhiSystem::default(),
        };
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

    /// Heap allocator we allocate the dynamically sized sub arrays inside each parameter block
    /// from.
    pub(crate) heap: RhiSystem,
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

        let backing_buffer = NonNull::from(memory_block.buffer.as_ref());
        for block in blocks {
            let num_reads = block_layout.compiled.num_reads;
            let reads = alloc_resource_array(&self.heap, num_reads);

            let num_writes = block_layout.compiled.num_writes;
            let writes = alloc_resource_array(&self.heap, num_writes);

            let alloc = self.resource_pool.allocate(num_arguments as u32);

            if alloc.is_fail() {
                return Err(DescriptorAllocateError::OutOfMemory);
            }

            // Get the sub-slice of the resource arena that we've allocated for this parameter
            // block.
            let offset = alloc.offset as usize * 8;
            let cpu_addr = unsafe { memory_block.cpu_base.add(offset) };

            let new = ParameterBlock {
                _layout: NonNull::from(block_layout),
                backing_buffer,
                resource_allocation: alloc,
                cpu_addr: Some(cpu_addr.cast()),
                gpu_offset: offset,
                reads,
                writes,
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

                let num_reads = block_layout.compiled.num_reads;
                reuse_resource_array(&self.heap, &mut block.reads, num_reads);

                let num_writes = block_layout.compiled.num_writes;
                reuse_resource_array(&self.heap, &mut block.writes, num_writes);

                let allocation = self.resource_pool.allocate(num_arguments as u32);

                if allocation.is_fail() {
                    return Err(DescriptorAllocateError::OutOfMemory);
                }

                // Get the sub-slice of the resource arena that we've allocated for this parameter
                // block.
                let offset = allocation.offset as usize * 8;
                let cpu = memory_block.cpu_base.add(offset);

                block.resource_allocation = allocation;
                block.cpu_addr = Some(cpu.cast());
                block.gpu_offset = offset;
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
                    block.gpu_offset = 0;
                }
            }
        }
    }

    fn reset_blocks(&mut self, blocks: &mut [Self::T]) {
        for block in blocks {
            // Free the read/write arrays
            unsafe {
                dealloc_resource_array(&self.heap, block.reads);
                dealloc_resource_array(&self.heap, block.writes);
            }

            // Free the GPU descriptor heap allocation
            if !block.resource_allocation.is_fail() {
                self.resource_pool.free(block.resource_allocation);
                block.resource_allocation = Default::default();
                block.cpu_addr = None;
                block.gpu_offset = 0;
            }
        }
    }

    fn drop_blocks(&mut self, blocks: &mut [Self::T]) {
        // We skip freeing the 'resource_allocation' in 'drop_blocks'. The pool is going to be
        // dropped too so there's no point, the memory will be cleaned up in bulk.
        for block in blocks {
            // Free the read/write arrays
            unsafe {
                dealloc_resource_array(&self.heap, block.reads);
                dealloc_resource_array(&self.heap, block.writes);
            }
        }
    }
}

fn alloc_resource_array(
    a: &impl Allocator,
    num: usize,
) -> NonNull<[*mut ProtocolObject<dyn MTLResource>]> {
    if num != 0 {
        let layout = Layout::array::<*mut ProtocolObject<dyn MTLResource>>(num).unwrap();
        let result = a.allocate_zeroed(layout);
        let arr = match result {
            Ok(v) => v,
            Err(_) => handle_alloc_error(layout),
        };
        NonNull::slice_from_raw_parts(arr.cast(), num)
    } else {
        NonNull::slice_from_raw_parts(NonNull::dangling(), 0)
    }
}

unsafe fn dealloc_resource_array(
    a: &impl Allocator,
    base: NonNull<[*mut ProtocolObject<dyn MTLResource>]>,
) {
    if !base.is_empty() {
        let layout = Layout::array::<*mut ProtocolObject<dyn MTLResource>>(base.len()).unwrap();
        unsafe { a.deallocate(base.cast(), layout) }
    }
}

unsafe fn reuse_resource_array(
    a: &impl Allocator,
    arr: &mut NonNull<[*mut ProtocolObject<dyn MTLResource>]>,
    wanted_num: usize,
) {
    let existing_num = arr.len();
    match (wanted_num, existing_num) {
        // Doesn't need resources, do nothing! We're done!
        (0, _) => {}

        // Needs resources, but doesn't have any? Allocate a new array!
        (needs, 0) => {
            *arr = alloc_resource_array(a, needs);
        }

        // Needs resources, and has some already? Check if we have enough, otherwise
        // make a new array.
        (needs, has) => {
            // Only need to allocate if there isn't enough space in the existing array
            if needs > has {
                unsafe {
                    dealloc_resource_array(a, *arr);
                }
                *arr = alloc_resource_array(a, needs);
            }
        }
    }
}
