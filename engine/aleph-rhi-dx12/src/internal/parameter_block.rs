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

use std::ptr::NonNull;

use aleph_alloc::offset_allocator;
use windows::utils::{CPUDescriptorHandle, GPUDescriptorHandle};

use crate::parameter_block_layout::ParameterBlockLayout;

/// This internal struct is a critical piece of the implementation of the parameter block API. The
/// RHI API specifies [`ParameterBlockHandle`] as an opaque handle to a 'descriptor set object'.
/// This  *is* that object, for the D3D12 implementation.
///
/// This tracks the necessary state to write descriptors and bind the set to a slot in the pipeline.
pub struct ParameterBlock {
    /// The descriptor set layout of this set
    pub _layout: NonNull<ParameterBlockLayout>,

    /// The allocation for the resource handles. Could be 'empty/null' if there's no allocation
    /// and the handles are either None or backed via a pool.
    pub resource_allocation: offset_allocator::Allocation,

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

impl ParameterBlock {
    #[track_caller]
    #[inline(always)]
    pub unsafe fn assume_r_handle(&self) -> (CPUDescriptorHandle, GPUDescriptorHandle) {
        unsafe {
            let cpu = self.resource_handle_cpu.unwrap_unchecked();
            let gpu = self.resource_handle_gpu.unwrap_unchecked();
            (cpu, gpu)
        }
    }
}

unsafe impl Send for ParameterBlock {}
unsafe impl Sync for ParameterBlock {}
