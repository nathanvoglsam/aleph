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
use std::sync::Arc;

use aleph_gpu_allocator::{AllocationDesc, GpuAllocation, MemoryLocation};
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_metal::MTLBuffer;

use crate::device::Device;

/// Wrapper over an owned 'MTLBuffer'. Simple utility for tracking an internal-use cpu-write style
/// memory buffer.
///
/// The primary client of this API are descriptor pools/arenas. They sub-allocate argument buffers
/// from a memory block.
pub struct MemoryBlock {
    pub device: Arc<Device>,
    pub _buffer: Retained<ProtocolObject<dyn MTLBuffer>>,
    pub allocation: Option<GpuAllocation>,
    pub len: usize,
    pub cpu_addr: NonNull<u8>,
    pub gpu_addr: NonZero<u64>,
}

unsafe impl Send for MemoryBlock {}

impl MemoryBlock {
    pub fn new(device: &Device, len: usize) -> Option<Self> {
        let alloc_info = AllocationDesc {
            location: MemoryLocation::CpuToGpu,
            strategy: Default::default(),
            desc: len,
        };
        let (allocation, _, buffer) = unsafe {
            device
                .allocator
                .as_ref()
                .unwrap_unchecked()
                .allocate_buffer(device, &alloc_info)?
        };

        let cpu_addr = buffer.contents().cast();
        let gpu_addr = buffer.gpuAddress();
        let gpu_addr = NonZero::new(gpu_addr)?;

        Some(Self {
            device: device.this.upgrade().unwrap(),
            _buffer: buffer,
            allocation: Some(allocation),
            len,
            cpu_addr,
            gpu_addr,
        })
    }
}

impl Drop for MemoryBlock {
    fn drop(&mut self) {
        unsafe {
            self.device
                .allocator
                .as_ref()
                .unwrap_unchecked()
                .free_allocation(&self.device, self.allocation.take().unwrap());
        }
    }
}
