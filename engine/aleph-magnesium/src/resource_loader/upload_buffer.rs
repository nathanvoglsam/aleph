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

use smallbox::space::S8;
use smallbox::{SmallBox, smallbox};

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
/// allocated from [`rhi::CpuAccessMode::Write`] memory.
///
/// It is also the implemetation's responsibility to ensure that the device offset of the region is
/// aligned to at least 512 bytes. This is critical so users are able to ensure individual mip
/// levels can be aligned to 512 byte blocks within a single [`IUploadBuffer`].
pub unsafe trait IUploadBuffer: Send + Sync {
    /// The rhi [`rhi::BufferHandle`] that the object instance is borrowing from.
    fn handle(&self) -> &rhi::BufferHandle;

    /// Offset within the root buffer's allocation that this object instance's borrowed slice begins
    /// at.
    fn device_offset(&self) -> u64;

    /// Slice over the bytes in the buffer this object instance has borrowed. This will be GPU
    /// addressable for read access.
    fn bytes(&self) -> &[u8];

    /// [`IUploadBuffer::bytes`] but mutable...
    fn bytes_mut(&mut self) -> &mut [u8];
}

unsafe impl IUploadBuffer for UploadBuffer {
    fn handle(&self) -> &rhi::BufferHandle {
        &self.buffer
    }

    fn device_offset(&self) -> u64 {
        self.device_offset
    }

    fn bytes(&self) -> &[u8] {
        // Safety: It is unsafe to construct a UploadBuffer where this is unsound
        unsafe { self.data.0.as_ref() }
    }

    fn bytes_mut(&mut self) -> &mut [u8] {
        // Safety: It is unsafe to construct a UploadBuffer where this is unsound
        unsafe { self.data.0.as_mut() }
    }
}

pub struct UploadBuffer {
    /// A reference to the [`rhi::IBuffer`] that the owned buffer region was allocated from. This is
    /// only held to ensure the buffer is never dropped.
    buffer: rhi::BufferHandle,

    /// The offset from the start of the upload buffer within the associated buffer object.
    device_offset: u64,

    /// A slice referring to the actual buffer region this upload buffer wraps. Logically this is
    /// owned by the [`UploadBuffer`] and we have an exclusive borrow but we can't express
    /// this to rustc.
    ///
    /// How do we have an exclusive borrow? It's an interface requirement! It's unsafe to construct
    /// a [`UploadBuffer`] over a slice if the borrow isn't exclusive.
    data: DataSendSync,
}

impl UploadBuffer {
    /// Constructs a new [`UploadBuffer`] from the given [`IBuffer`] and slice pointer.
    ///
    /// # Safety
    ///
    /// It is the caller's responsibility to ensure that `buffer` must be allocated from
    /// [`CpuAccessMode::Write`] memory, `data` must be part of the buffer's memory mapping, and
    /// the `data` slice must be exclusively owned. The buffer must also allow the
    /// [`ResourceUsageFlags::COPY_SOURCE`] usage.
    pub unsafe fn new(buffer: rhi::BufferHandle, device_offset: u64, data: NonNull<[u8]>) -> Self {
        debug_assert_eq!(
            device_offset % 512,
            0,
            "device_offset must be 512 byte aligned!"
        );
        Self {
            buffer,
            device_offset,
            data: DataSendSync(data),
        }
    }

    /// Constructs a new instance from the given device and desc by allocating a whole new upload
    /// buffer with the requested number of bytes.
    pub fn new_owned(device: &dyn rhi::IDevice, size: u64) -> Result<Self, rhi::BufferCreateError> {
        let out = unsafe {
            let buffer = device.create_buffer(&rhi::BufferDesc {
                size,
                cpu_access: rhi::CpuAccessMode::Write,
                usage: rhi::ResourceUsageFlags::COPY_SOURCE,
                name: None,
            })?;

            let ptr = device.map_buffer(&buffer).unwrap();
            let data = NonNull::slice_from_raw_parts(ptr, size as usize);
            UploadBuffer::new(buffer, 0, data)
        };
        Ok(out)
    }

    pub fn into_smallbox(self) -> SmallBox<dyn IUploadBuffer, S8> {
        smallbox!(self)
    }
}

/// Internal Send+Sync wrapper
struct DataSendSync(NonNull<[u8]>);

// Because of 'data' we need to do these manually
unsafe impl Send for DataSendSync {}
unsafe impl Sync for DataSendSync {}
