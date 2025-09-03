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

use std::alloc::{Layout, LayoutError, handle_alloc_error};
use std::any::TypeId;
use std::cell::Cell;
use std::mem::{MaybeUninit, transmute};
use std::ptr::NonNull;

use aleph_any::{AnyArc, declare_interfaces};
use aleph_rhi_api::*;
use aleph_rhi_impl_utils::offset_allocator::OffsetAllocator;
use allocator_api2::alloc::{Allocator, Global};
use blink_alloc::BlinkAlloc;
use windows::utils::{CPUDescriptorHandle, GPUDescriptorHandle};

use crate::device::Device;
use crate::internal::descriptor_chunk::DescriptorChunk;
use crate::internal::parameter_block::ParameterBlock;
use crate::internal::parameter_block_pool::ParameterBlockPool;
use crate::internal::unwrap;
use crate::parameter_block_layout::ParameterBlockLayout;

pub struct DescriptorArenaLinear {
    pub(crate) _device: AnyArc<Device>,

    /// The base address of the arena this pool allocates resource descriptors from
    pub(crate) resource_arena: DescriptorChunk,

    /// Bump allocator that descriptor set objects are allocated from
    pub(crate) set_pool: BlinkAlloc,

    /// The bump state for the descriptor pool. Used to bump allocate descriptor blocks from the
    /// resource arena.
    pub(crate) descriptor_bump_index: Cell<u32>,

    /// The number of descriptor set objects currently allocated from the arena.
    pub(crate) num_blocks: Cell<u32>,

    /// The maximum number of descriptor sets that can be allocated from the pool
    pub(crate) set_capacity: u32,
}

declare_interfaces!(DescriptorArenaLinear, [IDescriptorArena]);

impl IGetPlatformInterface for DescriptorArenaLinear {
    unsafe fn __query_platform_interface(&self, _target: TypeId, _out: *mut ()) -> Option<()> {
        None
    }
}

impl DescriptorArenaLinear {
    fn get_optional_handles_for_index(
        &self,
        layout: &ParameterBlockLayout,
        index: u32,
    ) -> (Option<CPUDescriptorHandle>, Option<GPUDescriptorHandle>) {
        if layout.compiled.resources.num_resources() != 0 {
            let (cpu, gpu) = self.resource_arena.get_handles_for_index(index);
            (Some(cpu), Some(gpu))
        } else {
            (None, None)
        }
    }
}

impl IDescriptorArena for DescriptorArenaLinear {
    fn allocate_block(
        &self,
        layout: &dyn IParameterBlockLayout,
    ) -> Result<ParameterBlockHandle, DescriptorAllocateError> {
        let layout = unwrap::parameter_block_layout(layout);
        self.allocate_set_internal(layout)
    }

    fn allocate_blocks(
        &self,
        layout: &dyn IParameterBlockLayout,
        num_blocks: usize,
    ) -> Result<Box<[ParameterBlockHandle]>, DescriptorAllocateError> {
        let layout = unwrap::parameter_block_layout(layout);
        let mut sets = Vec::with_capacity(num_blocks);
        for _ in 0..num_blocks {
            sets.push(self.allocate_set_internal(layout)?);
        }

        debug_assert_eq!(sets.len(), sets.capacity());
        debug_assert_eq!(sets.len(), num_blocks);
        Ok(sets.into_boxed_slice())
    }

    unsafe fn free(&self, _blocks: &[ParameterBlockHandle]) {
        unimplemented!()
    }

    unsafe fn reset(&self) {
        unsafe {
            self.descriptor_bump_index.set(0);
            self.num_blocks.set(0);
            self.set_pool.reset_unchecked();
        }
    }
}

impl DescriptorArenaLinear {
    /// Internal version of [IDescriptorArena::allocate_set] that takes an unwrapped set layout
    /// so we don't repeatedly unwrap the same object in a loop when calling
    /// [IDescriptorArena::allocate_sets].
    fn allocate_set_internal(
        &self,
        layout: &ParameterBlockLayout,
    ) -> Result<ParameterBlockHandle, DescriptorAllocateError> {
        if self.num_blocks.get() == self.set_capacity {
            return Err(DescriptorAllocateError::OutOfMemory);
        }

        let num_resources = layout.compiled.resources.num_resources();
        let num_samplers = layout.compiled.samplers.num_samplers();
        let bumped_index = self.descriptor_bump_index.get() + num_resources;
        if bumped_index > self.resource_arena.num_descriptors {
            return Err(DescriptorAllocateError::OutOfPoolMemory);
        }

        // Bump allocate the required number of descriptors from the set
        let set_index = self.descriptor_bump_index.get();
        self.descriptor_bump_index.set(bumped_index);

        let (resource_handle_cpu, resource_handle_gpu) =
            self.get_optional_handles_for_index(layout, set_index);

        let handle = {
            Self::heap_allocate(
                &&self.set_pool,
                layout,
                num_samplers as usize,
                resource_handle_cpu,
                resource_handle_gpu,
            )
        };

        handle.ok_or(DescriptorAllocateError::OutOfMemory)
    }

    pub fn heap_allocate(
        allocator: &impl Allocator,
        layout: &ParameterBlockLayout,
        num_samplers: usize,
        resource_handle_cpu: Option<CPUDescriptorHandle>,
        resource_handle_gpu: Option<GPUDescriptorHandle>,
    ) -> Option<ParameterBlockHandle> {
        // Make sure we can just allocate some u64 off the end of the set with no alignment issues
        assert_eq!(align_of::<ParameterBlock>(), align_of::<u64>());

        // The size is equal to one constant buffer object + num_dynamic_cbs u64s + num_samplers
        // u64s.
        let mem_layout = Self::descriptor_set_allocation_layout(num_samplers).unwrap();
        let mut set = match allocator.allocate_zeroed(mem_layout) {
            Ok(v) => v.cast::<MaybeUninit<ParameterBlock>>(),
            Err(_) => return None,
        };

        let samplers = Self::get_allocated_arrays(set, num_samplers);

        unsafe {
            let set_uninit = set.as_mut();
            set_uninit.write(ParameterBlock {
                _layout: NonNull::from(layout),
                resource_allocation: Default::default(),
                resource_handle_cpu,
                resource_handle_gpu,
                samplers,
            });
        }

        unsafe { Some(ParameterBlockHandle::from_raw(set.cast())) }
    }

    pub const fn descriptor_set_allocation_layout(
        num_samplers: usize,
    ) -> Result<Layout, LayoutError> {
        let size = size_of::<ParameterBlock>();
        let size = size + num_samplers * size_of::<Option<GPUDescriptorHandle>>();
        let align = align_of::<ParameterBlock>();

        Layout::from_size_align(size, align)
    }

    fn get_allocated_arrays(
        set: NonNull<MaybeUninit<ParameterBlock>>,
        num_samplers: usize,
    ) -> NonNull<[Option<GPUDescriptorHandle>]> {
        unsafe {
            let samplers = set.as_ptr().add(1).cast::<Option<GPUDescriptorHandle>>();

            // We use alloc-zeroed so we know that these arrays will be zero initialized so we don't
            // need to do anything here
            let samplers = if num_samplers != 0 {
                std::slice::from_raw_parts_mut(samplers, num_samplers)
            } else {
                &mut []
            };

            NonNull::from(samplers)
        }
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

pub struct DescriptorArenaHeap {
    pub(crate) _device: AnyArc<Device>,

    /// The base address of the arena this pool allocates resource descriptors from
    pub(crate) resource_block: DescriptorChunk,

    /// Allocation state used for allocating from the resource arena
    pub(crate) resource_pool: Cell<Option<Box<OffsetAllocator>>>,

    /// Object pool allocator that descriptor set objects are allocated from.
    pub(crate) set_pool: ParameterBlockPool,

    /// A list of all the handles that are currently live. Used so we can fully clean up after the
    /// arena when it's being dropped.
    pub(crate) live_handles: Cell<Vec<ParameterBlockHandle>>,
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
        let block_layout = unwrap::parameter_block_layout(layout);

        let mut set = MaybeUninit::uninit();
        self.set_pool
            .allocate_blocks(std::slice::from_mut(&mut set))
            .ok_or(DescriptorAllocateError::OutOfPoolMemory)?;

        // Safety: allocate_sets is requried to intialize this so this is safe
        let set = unsafe { set.assume_init() };

        unsafe {
            let mut resource_pool = self.resource_pool.take().unwrap();
            let mut live_handles = self.live_handles.take();
            let out = match self.inner_allocate_block(&mut resource_pool, block_layout, set) {
                Some(_) => {
                    live_handles.push(set);
                    Ok(set)
                }
                None => {
                    // Return the set back to the pool
                    self.set_pool.free_blocks(&[set]);
                    Err(DescriptorAllocateError::OutOfMemory)
                }
            };
            self.resource_pool.set(Some(resource_pool));
            self.live_handles.set(live_handles);

            out
        }
    }

    fn allocate_blocks(
        &self,
        layout: &dyn IParameterBlockLayout,
        num_blocks: usize,
    ) -> Result<Box<[ParameterBlockHandle]>, DescriptorAllocateError> {
        let block_layout = unwrap::parameter_block_layout(layout);

        let mut sets = vec![MaybeUninit::uninit(); num_blocks];
        self.set_pool
            .allocate_blocks(&mut sets)
            .ok_or(DescriptorAllocateError::OutOfPoolMemory)?;

        debug_assert_eq!(sets.len(), sets.capacity());
        debug_assert_eq!(sets.len(), num_blocks);
        let sets = sets.into_boxed_slice();
        let sets: Box<[ParameterBlockHandle]> = unsafe { transmute(sets) };

        unsafe {
            let mut resource_pool = self.resource_pool.take().unwrap();
            let mut live_handles = self.live_handles.take();

            let mut num_allocated = 0usize;
            for &handle in sets.iter() {
                match self.inner_allocate_block(&mut resource_pool, block_layout, handle) {
                    Some(_) => {
                        live_handles.push(handle);
                        num_allocated += 1;
                    }
                    None => {
                        // Return the set back to the pool
                        self.set_pool.free_blocks(&[handle]);
                        break;
                    }
                };
            }

            self.resource_pool.set(Some(resource_pool));
            self.live_handles.set(live_handles);

            // If we failed to allocate enough sets (i.e we got OOM in the allocation loop) we need
            // to go and free the ones we did successfully allocate. Thankfully we can just call
            // 'free'.
            if num_allocated != sets.len() {
                self.free(&sets[0..num_allocated])
            }
        }

        Ok(sets)
    }

    unsafe fn free(&self, sets: &[ParameterBlockHandle]) {
        let mut live_handles = self.live_handles.take();
        let mut resource_pool = self.resource_pool.take().unwrap();

        for &handle in sets {
            unsafe { Self::deallocate_set(&mut resource_pool, handle) };

            let index = live_handles
                .iter()
                .enumerate()
                .find_map(|(i, &v)| if v == handle { Some(i) } else { None })
                .unwrap();
            live_handles.swap_remove(index);
        }

        self.resource_pool.set(Some(resource_pool));
        self.live_handles.set(live_handles);

        self.set_pool.free_blocks(sets)
    }

    unsafe fn reset(&self) {
        unsafe {
            self.set_pool.reset_pool();
        }

        let mut resource_pool = self.resource_pool.take().unwrap();
        resource_pool.reset();
        self.resource_pool.set(Some(resource_pool));
    }
}

impl DescriptorArenaHeap {
    unsafe fn inner_allocate_block(
        &self,
        resource_pool: &mut OffsetAllocator,
        block_layout: &ParameterBlockLayout,
        handle: ParameterBlockHandle,
    ) -> Option<()> {
        let global_alloc = Global;

        let num_resources = block_layout.compiled.resources.num_resources();
        let num_samplers = block_layout.compiled.samplers.num_samplers() as usize;

        let v = unsafe { ParameterBlock::ptr_from_handle(handle).as_mut() };
        v._layout = NonNull::from(block_layout);

        if num_resources != 0 {
            let allocation = resource_pool.allocate(num_resources);

            if allocation.is_fail() {
                return None;
            }

            let (cpu, gpu) = self.resource_block.get_handles_for_index(allocation.offset);
            v.resource_handle_cpu = Some(cpu);
            v.resource_handle_gpu = Some(gpu);
        } else {
            v.resource_handle_cpu = None;
            v.resource_handle_gpu = None;
        }

        // OOM here is a panic as if we OOM on the global alloc we're probably hosed.
        if num_samplers != 0 {
            let layout = Layout::array::<Option<GPUDescriptorHandle>>(num_samplers).unwrap();
            let result = global_alloc.allocate_zeroed(layout);
            let result = match result {
                Ok(v) => v,
                Err(_) => handle_alloc_error(layout),
            };
            let samplers =
                unsafe { std::slice::from_raw_parts(result.cast().as_ptr(), num_samplers) };
            v.samplers = NonNull::from(samplers);
        } else {
            v.samplers = NonNull::from(&[])
        }

        Some(())
    }

    unsafe fn deallocate_set(resource_pool: &mut OffsetAllocator, handle: ParameterBlockHandle) {
        unsafe {
            let global_alloc = Global;

            let set = ParameterBlock::ptr_from_handle(handle).as_mut();

            let samplers = set.samplers.as_ref();
            if !samplers.is_empty() {
                let layout = Layout::for_value(samplers);
                global_alloc.deallocate(set.samplers.cast(), layout);
            }

            if !set.resource_allocation.is_fail() {
                resource_pool.free(set.resource_allocation);
                set.resource_allocation = Default::default();
            }

            set.resource_handle_cpu = None;
            set.resource_handle_gpu = None;
        }
    }
}

impl Drop for DescriptorArenaHeap {
    fn drop(&mut self) {
        let live_handles = self.live_handles.take();
        let mut resource_pool = self.resource_pool.take().unwrap();
        for handle in live_handles {
            unsafe {
                Self::deallocate_set(&mut resource_pool, handle);
            }
        }

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
