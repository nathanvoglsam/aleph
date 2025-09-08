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

use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_metal::{MTLBuffer, MTLDevice, MTLResourceOptions};

use crate::device::Device;

/// Wrapper over an owned 'MTLBuffer'. Simple utility for tracking an internal-use cpu-write style
/// memory buffer.
///
/// The primary client of this API are descriptor pools/arenas. They sub-allocate argument buffers
/// from a memory block.
pub struct MemoryBlock {
    /// The handle to the buffer we're wrapping over.
    pub buffer: Retained<ProtocolObject<dyn MTLBuffer>>,

    /// Size, in bytes, of the buffer object.
    pub len: usize,

    /// The CPU handle to the start of the memory block
    pub cpu_base: NonNull<u8>,

    /// The GPU handle to the start of the memory block
    pub gpu_base: NonZero<u64>,
}

unsafe impl Send for MemoryBlock {}

impl MemoryBlock {
    pub fn new(device: &Device, len: usize) -> Option<Self> {
        let options = MTLResourceOptions::HazardTrackingModeTracked
            | MTLResourceOptions::StorageModeShared
            | MTLResourceOptions::CPUCacheModeWriteCombined;

        let buffer = device.device.newBufferWithLength_options(len, options)?;

        let cpu_base = buffer.contents().cast();
        let gpu_base = NonZero::new(buffer.gpuAddress()).unwrap();

        Some(Self {
            buffer,
            len,
            cpu_base,
            gpu_base,
        })
    }
}
