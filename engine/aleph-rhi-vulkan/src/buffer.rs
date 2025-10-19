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

use std::num::NonZeroU64;
use std::ptr::NonNull;

use aleph_any::AnyArc;
use aleph_gpu_allocator::GpuAllocation;
use aleph_object_system::unsafe_impl_iobject;
use aleph_rhi_api::*;
use aleph_rhi_impl_utils::owned_desc::OwnedBufferDesc;
use ash::vk;
use parking_lot::Mutex;

use crate::device::Device;
use crate::internal::allocation_callbacks::GLOBAL;

pub struct Buffer {
    pub(crate) _device: AnyArc<Device>,
    pub(crate) id: NonZeroU64,
    pub(crate) buffer: vk::Buffer,
    pub(crate) allocation: Option<GpuAllocation>,
    pub(crate) map_state: Mutex<MapState>,
    pub(crate) desc: OwnedBufferDesc,
}

unsafe_impl_iobject!(Buffer, "01944e48-b650-76a1-9637-5418f9becbeb");

impl Buffer {
    pub(crate) fn get(v: &BufferHandle) -> &Self {
        v.get()
            .downcast_ref::<Self>()
            .expect("Unknown Buffer implementation!")
    }

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
            self.desc().size
        } else {
            let size = size as u64;
            debug_assert!(
                size <= self.desc().size,
                "The requested view range is larger than the buffer"
            );
            size
        }
    }
}

impl Buffer {
    pub(crate) fn get_buffer_id(&self) -> NonZeroU64 {
        self.id
    }

    pub(crate) const fn desc(&self) -> &BufferDesc<'_> {
        self.desc.get()
    }

    pub(crate) fn map_buffer(&self, _device: &Device) -> Result<NonNull<u8>, ResourceMapError> {
        let mut lock = self.map_state.lock();

        if let Some(ptr) = lock.ptr {
            lock.count = lock.count.checked_add(1).unwrap();
            return Ok(ptr);
        }

        debug_assert_eq!(lock.count, 0);

        let ptr = lock.ptr.ok_or(ResourceMapError::MappedNullPointer)?;

        lock.count = lock.count.checked_add(1).unwrap();
        Ok(ptr)
    }

    pub(crate) fn unmap_buffer(&self, _device: &Device) -> Result<(), ResourceUnmapError> {
        let mut lock = self.map_state.lock();

        lock.count = lock
            .count
            .checked_sub(1)
            .ok_or(ResourceUnmapError::NotMapped)?;

        if lock.count > 0 {
            return Ok(());
        }

        Ok(())
    }

    pub(crate) fn flush_buffer_range(&self, device: &Device, offset: u64, len: u64) {
        // unsafe {
        //     device
        //         .allocator
        //         .flush_allocation(self.allocation, offset, len)
        //         .unwrap();
        // }
    }

    pub(crate) fn invalidate_buffer_range(&self, device: &Device, offset: u64, len: u64) {
        // unsafe {
        //     device
        //         .allocator
        //         .invalidate_allocation(self.allocation, offset, len)
        //         .unwrap();
        // }
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            self._device.device.destroy_buffer(self.buffer, GLOBAL);
            self._device
                .allocator
                .as_ref()
                .unwrap_unchecked()
                .free_allocation(self._device.as_ref(), self.allocation.take().unwrap());
        }
    }
}

#[derive(Default)]
pub(crate) struct MapState {
    pub(crate) count: usize,
    pub(crate) ptr: Option<NonNull<u8>>,
}

unsafe impl Send for MapState {}
