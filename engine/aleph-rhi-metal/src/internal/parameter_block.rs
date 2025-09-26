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

use std::num::NonZero;
use std::ptr::NonNull;

use aleph_rhi_impl_utils::offset_allocator;
use objc2::runtime::ProtocolObject;
use objc2_metal::*;

use crate::parameter_block_layout::ParameterBlockLayout;

/// This internal struct is a critical piece of the implementation of the parameter block API. The
/// RHI API specifies [`ParameterBlockHandle`] as an opaque handle to a 'descriptor set object'.
/// This  *is* that object, for the Metal implementation.
///
/// This tracks the necessary state to write descriptors and bind the set to a slot in the pipeline.
pub struct ParameterBlock {
    /// The descriptor set layout of this block
    pub _layout: NonNull<ParameterBlockLayout>,

    /// The buffer object this parameter block's argument buffer is sub-allocated from. This is
    /// not retained because the lifetime of the parameter block itself is tied to the pool. It is
    /// already illegal to access this block in a context where accessing this pointer is invalid.
    pub backing_buffer: NonNull<ProtocolObject<dyn MTLBuffer>>,

    /// The allocation object that is associated with our argument buffer sub-allocation. This may
    /// be null/invalid if the pool/arena the block was allocated from does not use an allocation
    /// scheme that requires an offset allocator
    pub resource_allocation: offset_allocator::Allocation,

    /// CPU address for the backing allocation of the argument buffer we're using to back this
    /// parameter block. This is used for writing the descriptors into the buffer from the host
    /// before being bound.
    pub resource_handle_cpu: Option<NonNull<u64>>,

    /// Offset into 'backing_buffer' where the argument buffer sub-allocation is located.
    pub resource_handle_gpu: Option<NonZero<usize>>,

    /// Array of resource handles. Filled out by update calls and used for hazard tracking with
    /// useResources. This is the set of read only resources in the parameter block.
    pub reads: NonNull<[*mut ProtocolObject<dyn MTLResource>]>,

    /// Array of resource handles. Filled out by update calls and used for hazard tracking with
    /// useResources. This is the set of writable resources in the parameter block.
    pub writes: NonNull<[*mut ProtocolObject<dyn MTLResource>]>,
}

unsafe impl Send for ParameterBlock {}
unsafe impl Sync for ParameterBlock {}
