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
use std::alloc::LayoutError;
use std::mem::MaybeUninit;
use std::ptr::NonNull;

use aleph_rhi_api::*;
use allocator_api2::alloc::Allocator;
use windows::utils::{CPUDescriptorHandle, GPUDescriptorHandle};

use crate::descriptor_set_layout::DescriptorSetLayout;

/// This internal struct is a critical piece of the implementation of the descriptor sets API. The
/// RHI API specifies [DescriptorSetHandle] as an opaque handle to a 'descriptor set object'. This
/// *is* that object, for the D3D12 implementation.
///
/// This tracks the necessary state to write descriptors and bind the set to a slot in the pipeline.
pub struct DescriptorSet {
    /// The descriptor set layout of this set
    pub _layout: NonNull<DescriptorSetLayout>,

    pub dynamic_constant_buffers: NonNull<[u64]>,

    /// The CPU virtual address of the beginning of the set's memory in the resource heap. This can
    /// be null when no resources are present in the set layout (it contains only samplers).
    pub resource_handle_cpu: Option<CPUDescriptorHandle>,

    /// The GPU virtual address of the beginning of the set's memory in the resource heap. This can
    /// be null when no resources are present in the set layout (it contains only samplers).
    pub resource_handle_gpu: Option<GPUDescriptorHandle>,

    /// A list of all the distinct samplers in the order they are expected to be arranged as
    /// distinct descriptor tables.
    pub samplers: NonNull<[Option<GPUDescriptorHandle>]>,
}

impl DescriptorSet {
    #[track_caller]
    #[inline(always)]
    pub unsafe fn assume_r_handle(&self) -> (CPUDescriptorHandle, GPUDescriptorHandle) {
        let cpu = self.resource_handle_cpu.unwrap_unchecked();
        let gpu = self.resource_handle_gpu.unwrap_unchecked();
        (cpu, gpu)
    }

    /// Grabs the pointer inside a [DescriptorSetHandle] as a non-null [DescriptorSet] ptr
    ///
    /// # Safety
    ///
    /// Lets be real. You're going to be making a reference out of this pointer...
    ///
    /// This has all the soundness requirements for creating a reference from a raw pointer. At the
    /// very least a [DescriptorSetHandle] is guaranteed to be non-null, but many things are not
    /// known at this call-site without the caller tracking these things themselves.
    ///
    /// - It is the caller's responsibility to ensure that no mutable reference can exist at the
    ///   same time as the reference this function creates. If it can't be proven statically then
    ///   locks must be used. This in general means:
    ///     - DescriptorSet objects themselves are immutable, so access to the set object itself
    ///       requires no synchronization. But the descriptor memory that the sets point to
    ///       *is not* immutable. The caller must synchronize access to the descriptor memory.
    /// - It is the caller's responsibility to ensure the handle still points to a live set object.
    ///   This means:
    ///     - The caller must ensure that they do not use any sets after the pool they were
    ///       allocated from are destroyed.
    /// - It is the caller's responsibility to ensure the handle points to a value of the correct
    ///   type. This is more subtle, but:
    ///     - Handles allocated from a different device will point to a different type. They will
    ///       also point to descriptor memory on another device. Thus it becomes a rust soundness
    ///       issue *as well as* an API soundness issue as using a set from one device on another
    ///       device is invalid as the set allocation is local to the device it was created from.
    ///     - The caller must ensure that they only use set handles with the device they were
    ///       created from. If the two devices use different implementations then the handles
    ///       *will* be interpreted as the incorrect type.
    ///
    pub fn ptr_from_handle(handle: DescriptorSetHandle) -> NonNull<DescriptorSet> {
        let inner: NonNull<()> = handle.into();
        inner.cast()
    }

    pub fn heap_allocate(
        allocator: &impl Allocator,
        layout: &DescriptorSetLayout,
        num_dynamic_cbs: usize,
        num_samplers: usize,
        resource_handle_cpu: Option<CPUDescriptorHandle>,
        resource_handle_gpu: Option<GPUDescriptorHandle>,
    ) -> DescriptorSetHandle {
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
            Err(_) => allocator_api2::alloc::handle_alloc_error(mem_layout),
        };

        let (dynamic_constant_buffers, samplers) =
            Self::get_allocated_arrays(set, num_dynamic_cbs, num_samplers);

        unsafe {
            let set_uninit = set.as_mut();
            set_uninit.write(DescriptorSet {
                _layout: NonNull::from(layout),
                dynamic_constant_buffers,
                resource_handle_cpu,
                resource_handle_gpu,
                samplers,
            });
        }

        unsafe { DescriptorSetHandle::from_raw(set.cast()) }
    }

    pub unsafe fn free_heap_allocated(allocator: &impl Allocator, handle: DescriptorSetHandle) {
        let set = Self::ptr_from_handle(handle);
        let layout = {
            let set_ref = set.as_ref();
            let num_dynamic_cbs = set_ref.dynamic_constant_buffers.len();
            let num_samplers = set_ref.samplers.len();
            Self::descriptor_set_allocation_layout(num_dynamic_cbs, num_samplers).unwrap()
        };

        // Drop the set object
        std::ptr::drop_in_place(set.as_ptr());

        // Free the set's allocation
        allocator.deallocate(set.cast(), layout);
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

unsafe impl Send for DescriptorSet {}
unsafe impl Sync for DescriptorSet {}
