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
use std::any::TypeId;
use std::mem::MaybeUninit;
use std::num::NonZero;
use std::ptr::NonNull;

use aleph_any::{AnyArc, declare_interfaces};
use aleph_rhi_api::*;
use aleph_rhi_impl_utils::RhiSystem;
use aleph_rhi_impl_utils::parameter_block_pool::{IBlockFactory, ParameterBlockPool};
use allocator_api2::alloc::Allocator;
use blink_alloc::BlinkAlloc;
use objc2::__framework_prelude::ProtocolObject;
use objc2_metal::MTLResource;

use crate::device::Device;
use crate::internal::memory_block::MemoryBlock;
use crate::internal::parameter_block::ParameterBlock;
use crate::internal::unwrap;
use crate::parameter_block_layout::ParameterBlockLayout;

pub struct DescriptorPool {
    pub(crate) _device: AnyArc<Device>,
    pub(crate) _layout: AnyArc<ParameterBlockLayout>,
    pub(crate) block: MemoryBlock,
    pub(crate) pool: ParameterBlockPool<LinearBlockFactory>,
}

declare_interfaces!(DescriptorPool, [IDescriptorPool]);

impl IGetPlatformInterface for DescriptorPool {
    unsafe fn __query_platform_interface(&self, _target: TypeId, _out: *mut ()) -> Option<()> {
        None
    }
}

impl IDescriptorPool for DescriptorPool {
    fn allocate_block(&mut self) -> Result<ParameterBlockHandle, DescriptorAllocateError> {
        let mut blocks: [MaybeUninit<_>; 1] = [MaybeUninit::uninit(); 1];
        self.pool
            .allocate_blocks((&self.block, &self._layout), &mut blocks)?;

        unsafe {
            let block = blocks[0].assume_init();
            Ok(block)
        }
    }

    fn allocate_blocks(
        &mut self,
        num_blocks: usize,
    ) -> Result<Box<[ParameterBlockHandle]>, DescriptorAllocateError> {
        let mut blocks = Box::new_uninit_slice(num_blocks);
        self.pool
            .allocate_blocks((&self.block, &self._layout), &mut blocks)?;

        let blocks = Box::leak(blocks);
        let blocks = NonNull::from(blocks);
        let blocks =
            NonNull::slice_from_raw_parts(blocks.cast::<ParameterBlockHandle>(), blocks.len());
        unsafe { Ok(Box::from_raw(blocks.as_ptr())) }
    }

    unsafe fn free(&mut self, blocks: &[ParameterBlockHandle]) {
        self.pool.free_blocks(blocks)
    }

    unsafe fn reset(&mut self) {
        unsafe {
            self.pool.reset_pool();
        }
    }
}

impl DescriptorPool {
    pub(crate) fn create(
        device: &Device,
        desc: &DescriptorPoolDesc,
    ) -> Result<Box<dyn IDescriptorPool>, DescriptorPoolCreateError> {
        let layout = unwrap::parameter_block_layout(desc.layout);

        let len = layout.compiled.num_arguments * desc.num_blocks as usize;
        let block = MemoryBlock::new(device, len).ok_or(DescriptorPoolCreateError::Platform)?;

        let factory = LinearBlockFactory {
            next_resource_index: 0,
            arena: BlinkAlloc::new_in(RhiSystem::default()),
        };
        let pool = ParameterBlockPool::new(factory, desc.num_blocks as usize);

        let pool: Box<dyn IDescriptorPool> = Box::new(DescriptorPool {
            _device: device.this.upgrade().unwrap(),
            _layout: layout.this.upgrade().unwrap(),
            block,
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
        let num_arguments = 0; // TODO: total including samplers

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
            let cpu = unsafe { memory_block.cpu_base.add(offset) };
            let gpu = memory_block.gpu_base.saturating_add(offset as u64);
            self.next_resource_index += num_arguments;

            let reads = if block_layout.compiled.num_reads != 0 {
                let layout = Layout::array::<NonNull<ProtocolObject<dyn MTLResource>>>(
                    block_layout.compiled.num_reads,
                )
                .unwrap();
                let arr = self
                    .arena
                    .allocate_zeroed(layout)
                    .map_err(|_| DescriptorAllocateError::OutOfMemory)?;
                Some(arr.cast())
            } else {
                None
            };

            let writes = if block_layout.compiled.num_writes != 0 {
                let layout = Layout::array::<NonNull<ProtocolObject<dyn MTLResource>>>(
                    block_layout.compiled.num_writes,
                )
                .unwrap();
                let arr = self
                    .arena
                    .allocate_zeroed(layout)
                    .map_err(|_| DescriptorAllocateError::OutOfMemory)?;
                Some(arr.cast())
            } else {
                None
            };

            let new = ParameterBlock {
                _layout: NonNull::from(block_layout),
                backing_buffer,
                resource_allocation: Default::default(), // Never used here
                resource_handle_cpu: Some(cpu.cast()),
                resource_handle_gpu: NonZero::new(gpu.get() as usize),
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
            block.resource_handle_cpu = None;
            block.resource_handle_gpu = None;
            block.reads = None;
            block.writes = None;
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
