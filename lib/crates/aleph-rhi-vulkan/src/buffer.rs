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

use std::any::TypeId;
use std::ptr::NonNull;

use aleph_any::{declare_interfaces, AnyArc, AnyWeak};
use aleph_rhi_api::*;
use aleph_rhi_impl_utils::try_clone_value_into_slot;
use ash::vk;
use vulkan_alloc::vma;

use crate::device::Device;

pub struct Buffer {
    pub(crate) _this: AnyWeak<Self>,
    pub(crate) _device: AnyArc<Device>,
    pub(crate) buffer: vk::Buffer,
    pub(crate) allocation: vma::Allocation,
    pub(crate) desc: BufferDesc<'static>,
    pub(crate) name: Option<String>,
}

declare_interfaces!(Buffer, [IBuffer]);

impl IGetPlatformInterface for Buffer {
    unsafe fn __query_platform_interface(&self, target: TypeId, out: *mut ()) -> Option<()> {
        try_clone_value_into_slot(&self.buffer, out, target)
    }
}

impl Buffer {
    /// When creating buffer views the API user can request the full buffer's range be bound without
    /// knowing the size by specifying the size as [u32::MAX]. This function is a utility wrapper
    /// for clamping the given size to the buffer's size when a full buffer binding is requested.
    ///
    /// This will only clamp when u32::MAX is given, otherwise the value will pass through
    /// unchanged. If the size is larger than the buffer then we'll likely UB in the underlying
    /// rendering API. We do have a debug assert here though for this case, but it will only run on
    /// debug builds.
    pub(crate) fn clamp_max_size_for_view(&self, size: u32) -> u64 {
        if size == u32::MAX {
            self.desc.size
        } else {
            let size = size as u64;
            debug_assert!(
                size <= self.desc.size,
                "The requested view range is larger than the buffer"
            );
            size
        }
    }
}

impl IBuffer for Buffer {
    fn upgrade(&self) -> AnyArc<dyn IBuffer> {
        AnyArc::map::<dyn IBuffer, _>(self._this.upgrade().unwrap(), |v| v)
    }

    fn strong_count(&self) -> usize {
        self._this.strong_count()
    }

    fn weak_count(&self) -> usize {
        self._this.weak_count()
    }

    fn desc(&self) -> BufferDesc {
        let mut desc = self.desc.clone();
        desc.name = self.name.as_deref();
        desc
    }

    fn desc_ref(&self) -> &BufferDesc {
        &self.desc
    }

    fn map(&self) -> Result<NonNull<u8>, ResourceMapError> {
        unsafe {
            let ptr = self
                ._device
                .allocator
                .map_memory(self.allocation)
                .map_err(|v| log::error!("Platform Error: {:#?}", v))?;
            ptr.ok_or(ResourceMapError::MappedNullPointer)
                .map(|v| v.cast::<u8>())
        }
    }

    fn unmap(&self) {
        unsafe {
            self._device.allocator.unmap_memory(self.allocation);
        }
    }

    fn flush_range(&self, offset: u64, len: u64) {
        unsafe {
            self._device
                .allocator
                .flush_allocation(self.allocation, offset, len)
                .unwrap();
        }
    }

    fn invalidate_range(&self, offset: u64, len: u64) {
        unsafe {
            self._device
                .allocator
                .invalidate_allocation(self.allocation, offset, len)
                .unwrap();
        }
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            self._device
                .allocator
                .destroy_buffer(self.buffer, self.allocation);
        }
    }
}
