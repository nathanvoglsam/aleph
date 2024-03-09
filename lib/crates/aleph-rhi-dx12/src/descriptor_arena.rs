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

use std::alloc::{handle_alloc_error, Layout, LayoutError};
use std::any::TypeId;
use std::cell::Cell;
use std::mem::{transmute, MaybeUninit};
use std::ptr::NonNull;

use aleph_any::{declare_interfaces, AnyArc};
use aleph_rhi_api::*;
use aleph_rhi_impl_utils::offset_allocator::OffsetAllocator;
use allocator_api2::alloc::{Allocator, Global};
use blink_alloc::BlinkAlloc;
use windows::utils::{CPUDescriptorHandle, GPUDescriptorHandle};

use crate::descriptor_set_layout::DescriptorSetLayout;
use crate::device::Device;
use crate::internal::descriptor_chunk::DescriptorChunk;
use crate::internal::descriptor_set::DescriptorSet;
use crate::internal::descriptor_set_pool::DescriptorSetPool;
use crate::internal::unwrap;

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
    pub(crate) num_sets: Cell<u32>,

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
        layout: &DescriptorSetLayout,
        index: u32,
    ) -> (Option<CPUDescriptorHandle>, Option<GPUDescriptorHandle>) {
        if layout.resource_num != 0 {
            let (cpu, gpu) = self.resource_arena.get_handles_for_index(index);
            (Some(cpu), Some(gpu))
        } else {
            (None, None)
        }
    }
}

impl IDescriptorArena for DescriptorArenaLinear {
    fn allocate_set(
        &self,
        layout: &dyn IDescriptorSetLayout,
    ) -> Result<DescriptorSetHandle, DescriptorPoolAllocateError> {
        let layout = unwrap::descriptor_set_layout(layout);
        self.allocate_set_internal(layout)
    }

    fn allocate_sets(
        &self,
        layout: &dyn IDescriptorSetLayout,
        num_sets: usize,
    ) -> Result<Box<[DescriptorSetHandle]>, DescriptorPoolAllocateError> {
        let layout = unwrap::descriptor_set_layout(layout);
        let mut sets = Vec::with_capacity(num_sets);
        for _ in 0..num_sets {
            sets.push(self.allocate_set_internal(layout)?);
        }

        debug_assert_eq!(sets.len(), sets.capacity());
        debug_assert_eq!(sets.len(), num_sets);
        Ok(sets.into_boxed_slice())
    }

    unsafe fn free(&self, _sets: &[DescriptorSetHandle]) {
        unimplemented!()
    }

    unsafe fn reset(&self) {
        self.descriptor_bump_index.set(0);
        self.num_sets.set(0);
        self.set_pool.reset_unchecked();
    }
}

impl DescriptorArenaLinear {
    /// Internal version of [IDescriptorArena::allocate_set] that takes an unwrapped set layout
    /// so we don't repeatedly unwrap the same object in a loop when calling
    /// [IDescriptorArena::allocate_sets].
    fn allocate_set_internal(
        &self,
        layout: &DescriptorSetLayout,
    ) -> Result<DescriptorSetHandle, DescriptorPoolAllocateError> {
        if self.num_sets.get() == self.set_capacity {
            return Err(DescriptorPoolAllocateError::OutOfMemory);
        }

        if self.descriptor_bump_index.get() + layout.resource_num
            > self.resource_arena.num_descriptors
        {
            return Err(DescriptorPoolAllocateError::OutOfPoolMemory);
        }

        // Bump allocate the required number of descriptors from the set
        let set_index = self.descriptor_bump_index.get();
        self.descriptor_bump_index
            .set(self.descriptor_bump_index.get() + layout.resource_num);

        let (resource_handle_cpu, resource_handle_gpu) =
            self.get_optional_handles_for_index(layout, set_index);

        let handle = {
            let handle = Self::heap_allocate(
                &&self.set_pool,
                layout,
                layout.dynamic_constant_buffers.len(),
                layout.sampler_tables.len(),
                resource_handle_cpu,
                resource_handle_gpu,
            );
            handle
        };

        handle.ok_or(DescriptorPoolAllocateError::OutOfMemory)
    }

    pub fn heap_allocate(
        allocator: &impl Allocator,
        layout: &DescriptorSetLayout,
        num_dynamic_cbs: usize,
        num_samplers: usize,
        resource_handle_cpu: Option<CPUDescriptorHandle>,
        resource_handle_gpu: Option<GPUDescriptorHandle>,
    ) -> Option<DescriptorSetHandle> {
        // Make sure we can just allocate some u64 off the end of the set with no alignment issues
        assert_eq!(
            std::mem::align_of::<DescriptorSet>(),
            std::mem::align_of::<u64>()
        );

        // The size is equal to one constant buffer object + num_dynamic_cbs u64s + num_samplers
        // u64s.
        let mem_layout =
            Self::descriptor_set_allocation_layout(num_dynamic_cbs, num_samplers).unwrap();
        let mut set = match allocator.allocate_zeroed(mem_layout) {
            Ok(v) => v.cast::<MaybeUninit<DescriptorSet>>(),
            Err(_) => return None,
        };

        let (dynamic_constant_buffers, samplers) =
            Self::get_allocated_arrays(set, num_dynamic_cbs, num_samplers);

        unsafe {
            let set_uninit = set.as_mut();
            set_uninit.write(DescriptorSet {
                _layout: NonNull::from(layout),
                dynamic_constant_buffers,
                resource_allocation: Default::default(),
                resource_handle_cpu,
                resource_handle_gpu,
                samplers,
            });
        }

        unsafe { Some(DescriptorSetHandle::from_raw(set.cast())) }
    }

    pub const fn descriptor_set_allocation_layout(
        num_dynamic_cbs: usize,
        num_samplers: usize,
    ) -> Result<Layout, LayoutError> {
        let size = std::mem::size_of::<DescriptorSet>();
        let size = size + num_dynamic_cbs * std::mem::size_of::<u64>();
        let size = size + num_samplers * std::mem::size_of::<u64>();
        let align = std::mem::align_of::<DescriptorSet>();

        std::alloc::Layout::from_size_align(size, align)
    }

    fn get_allocated_arrays(
        set: NonNull<MaybeUninit<DescriptorSet>>,
        num_dynamic_cbs: usize,
        num_samplers: usize,
    ) -> (NonNull<[u64]>, NonNull<[Option<GPUDescriptorHandle>]>) {
        unsafe {
            let dynamic_constant_buffers = set.as_ptr().add(1).cast::<u64>();
            let samplers = dynamic_constant_buffers
                .add(num_dynamic_cbs)
                .cast::<Option<GPUDescriptorHandle>>();

            // We use alloc-zeroed so we know that these arrays will be zero initialized so we don't
            // need to do anything here

            let dynamic_constant_buffers = if num_dynamic_cbs != 0 {
                std::slice::from_raw_parts_mut(dynamic_constant_buffers, num_dynamic_cbs)
            } else {
                &mut []
            };
            let samplers = if num_samplers != 0 {
                std::slice::from_raw_parts_mut(samplers, num_samplers)
            } else {
                &mut []
            };

            (
                NonNull::from(dynamic_constant_buffers),
                NonNull::from(samplers),
            )
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
    pub(crate) set_pool: DescriptorSetPool,

    /// A list of all the handles that are currently live. Used so we can fully clean up after the
    /// arena when it's being dropped.
    pub(crate) live_handles: Cell<Vec<DescriptorSetHandle>>,
}

declare_interfaces!(DescriptorArenaHeap, [IDescriptorArena]);

impl IGetPlatformInterface for DescriptorArenaHeap {
    unsafe fn __query_platform_interface(&self, _target: TypeId, _out: *mut ()) -> Option<()> {
        None
    }
}

impl IDescriptorArena for DescriptorArenaHeap {
    fn allocate_set(
        &self,
        layout: &dyn IDescriptorSetLayout,
    ) -> Result<DescriptorSetHandle, DescriptorPoolAllocateError> {
        let set_layout = unwrap::descriptor_set_layout(layout);

        let mut set = MaybeUninit::uninit();
        self.set_pool
            .allocate_sets(std::slice::from_mut(&mut set))
            .ok_or(DescriptorPoolAllocateError::OutOfPoolMemory)?;

        // Safety: allocate_sets is requried to intialize this so this is safe
        let set = unsafe { set.assume_init() };

        unsafe {
            let mut resource_pool = self.resource_pool.take().unwrap();
            let mut live_handles = self.live_handles.take();
            let out = match self.allocate_set(&mut resource_pool, set_layout, set) {
                Some(_) => {
                    live_handles.push(set);
                    Ok(set)
                }
                None => {
                    // Return the set back to the pool
                    self.set_pool.free_sets(&[set]);
                    Err(DescriptorPoolAllocateError::OutOfMemory)
                }
            };
            self.resource_pool.set(Some(resource_pool));
            self.live_handles.set(live_handles);

            out
        }
    }

    fn allocate_sets(
        &self,
        layout: &dyn IDescriptorSetLayout,
        num_sets: usize,
    ) -> Result<Box<[DescriptorSetHandle]>, DescriptorPoolAllocateError> {
        let set_layout = unwrap::descriptor_set_layout(layout);

        let mut sets = vec![MaybeUninit::uninit(); num_sets];
        self.set_pool
            .allocate_sets(&mut sets)
            .ok_or(DescriptorPoolAllocateError::OutOfPoolMemory)?;

        debug_assert_eq!(sets.len(), sets.capacity());
        debug_assert_eq!(sets.len(), num_sets);
        let sets = sets.into_boxed_slice();
        let sets: Box<[DescriptorSetHandle]> = unsafe { transmute(sets) };

        unsafe {
            let mut resource_pool = self.resource_pool.take().unwrap();
            let mut live_handles = self.live_handles.take();

            let mut num_allocated = 0usize;
            for &handle in sets.iter() {
                match self.allocate_set(&mut resource_pool, set_layout, handle) {
                    Some(_) => {
                        live_handles.push(handle);
                        num_allocated += 1;
                    }
                    None => {
                        // Return the set back to the pool
                        self.set_pool.free_sets(&[handle]);
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

    unsafe fn free(&self, sets: &[DescriptorSetHandle]) {
        let mut live_handles = self.live_handles.take();
        let mut resource_pool = self.resource_pool.take().unwrap();

        for &handle in sets {
            Self::deallocate_set(&mut resource_pool, handle);

            let index = live_handles
                .iter()
                .enumerate()
                .find_map(|(i, &v)| if v == handle { Some(i) } else { None })
                .unwrap();
            live_handles.swap_remove(index);
        }

        self.resource_pool.set(Some(resource_pool));
        self.live_handles.set(live_handles);

        self.set_pool.free_sets(sets)
    }

    unsafe fn reset(&self) {
        self.set_pool.reset_pool();

        let mut resource_pool = self.resource_pool.take().unwrap();
        resource_pool.reset();
        self.resource_pool.set(Some(resource_pool));
    }
}

impl DescriptorArenaHeap {
    unsafe fn allocate_set(
        &self,
        resource_pool: &mut OffsetAllocator,
        set_layout: &DescriptorSetLayout,
        handle: DescriptorSetHandle,
    ) -> Option<()> {
        let global_alloc = Global;

        let num_dynamic_cbs = set_layout.dynamic_constant_buffers.len();
        let num_samplers = set_layout.sampler_tables.len();
        let num_resources = set_layout.resource_num;

        let v = DescriptorSet::ptr_from_handle(handle).as_mut();
        v._layout = NonNull::from(set_layout);

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
        if num_dynamic_cbs != 0 {
            let layout = Layout::array::<u64>(num_dynamic_cbs).unwrap();
            let result = global_alloc.allocate_zeroed(layout);
            let result = match result {
                Ok(v) => v,
                Err(_) => handle_alloc_error(layout),
            };
            let dynamic_cbs = std::slice::from_raw_parts(result.cast().as_ptr(), num_dynamic_cbs);
            v.dynamic_constant_buffers = NonNull::from(dynamic_cbs);
        } else {
            v.dynamic_constant_buffers = NonNull::from(&[])
        }

        if num_samplers != 0 {
            let layout = Layout::array::<u64>(num_samplers).unwrap();
            let result = global_alloc.allocate_zeroed(layout);
            let result = match result {
                Ok(v) => v,
                Err(_) => handle_alloc_error(layout),
            };
            let samplers = std::slice::from_raw_parts(result.cast().as_ptr(), num_dynamic_cbs);
            v.samplers = NonNull::from(samplers);
        } else {
            v.samplers = NonNull::from(&[])
        }

        Some(())
    }

    unsafe fn deallocate_set(resource_pool: &mut OffsetAllocator, handle: DescriptorSetHandle) {
        let global_alloc = Global;

        let set = DescriptorSet::ptr_from_handle(handle).as_mut();

        let dynamic_cbs = set.dynamic_constant_buffers.as_ref();
        if dynamic_cbs.len() != 0 {
            let layout = Layout::for_value(dynamic_cbs);
            global_alloc.deallocate(set.dynamic_constant_buffers.cast(), layout);
        }

        let samplers = set.samplers.as_ref();
        if samplers.len() != 0 {
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
