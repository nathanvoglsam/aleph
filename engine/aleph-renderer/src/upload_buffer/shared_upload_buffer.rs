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

use aleph_rhi_api::*;

/// The interface expected of some object that abstracts over the source of a byte buffer that can
/// be used with copy commands on the GPU. Typically any buffer memory yielded by this interface
/// will be uploaded from an upload heap in the GPU API.
///
/// # Performance Warning
///
/// It is likely for the underlying upload memory to be mapped as write-combined or uncached. This
/// will make reads from the upload memory very expensive as well as make random writes expensive.
///
/// It is highly recommended to only write to this memory once, sequentially, and _never_ read it.
///
/// # Safety
///
/// It is the implementation's responsibility to ensure that the buffer yielded by this interface
/// is valid to use as an upload source in the platform's GPU API. Typically this means it must be
/// allocated from [`CpuAccessMode::Write`] memory.
///
/// It is also the implemetation's responsibility to ensure that the device offset of the region is
/// aligned to at least 512 bytes. This is critical so users are able to ensure individual mip
/// levels can be aligned to 512 byte blocks within a single [`IUploadBuffer`].
pub unsafe trait IUploadBuffer: Send + Sync {
    fn buffer(&self) -> &BufferHandle;
    fn device_offset(&self) -> u64;
    fn bytes(&self) -> &[u8];
    fn bytes_mut(&mut self) -> &mut [u8];
}

pub struct SharedUploadBuffer {
    /// A reference to the 'IBuffer' that the owned buffer region was allocated from. This is only
    /// held to ensure the buffer is never dropped.
    buffer: BufferHandle,

    /// The offset from the start of the upload buffer within the associated buffer object.
    device_offset: u64,

    /// A slice referring to the actual buffer region this upload buffer wraps. Logically this is
    /// owned by the [`SharedUploadBuffer`] and we have an exclusive borrow but we can't express
    /// this to rustc.
    ///
    /// How do we have an exclusive borrow? It's an interface requirement! It's unsafe to construct
    /// a [`SharedUploadBuffer`] over a slice if the borrow isn't exclusive.
    data: NonNull<[u8]>,
}

impl SharedUploadBuffer {
    /// Constructs a new [`SharedUploadBuffer`] from the given [`IBuffer`] and slice pointer.
    ///
    /// # Safety
    ///
    /// It is the caller's responsibility to ensure that `buffer` must be allocated from
    /// [`CpuAccessMode::Write`] memory, `data` must be part of the buffer's memory mapping, and
    /// the `data` slice must be exclusively owned. The buffer must also allow the
    /// [`ResourceUsageFlags::COPY_SOURCE`] usage.
    pub unsafe fn new(buffer: BufferHandle, device_offset: u64, data: NonNull<[u8]>) -> Self {
        debug_assert_eq!(
            device_offset % 512,
            0,
            "device_offset must be 512 byte aligned!"
        );
        Self {
            buffer,
            device_offset,
            data,
        }
    }
}

// Because of 'data' we need to do these manually
unsafe impl Send for SharedUploadBuffer {}
unsafe impl Sync for SharedUploadBuffer {}

unsafe impl IUploadBuffer for SharedUploadBuffer {
    fn buffer(&self) -> &BufferHandle {
        &self.buffer
    }

    fn device_offset(&self) -> u64 {
        self.device_offset
    }

    fn bytes(&self) -> &[u8] {
        // Safety: It is unsafe to construct a [`SharedUploadBuffer`] where this is unsound
        unsafe { self.data.as_ref() }
    }

    fn bytes_mut(&mut self) -> &mut [u8] {
        // Safety: It is unsafe to construct a [`SharedUploadBuffer`] where this is unsound
        unsafe { self.data.as_mut() }
    }
}
