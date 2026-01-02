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

use std::mem::ManuallyDrop;
use std::num::NonZeroU64;
use std::ptr::NonNull;

use aleph_any::AnyArc;
use aleph_gpu_allocator::GpuAllocation;
use aleph_object_system::unsafe_impl_iobject;
use aleph_rhi_api::*;
use aleph_rhi_impl_utils::owned_desc::OwnedBufferDesc;
use parking_lot::Mutex;
use windows::Win32::Graphics::Direct3D12::*;
use windows::utils::GPUDescriptorHandle;

use crate::device::Device;

pub struct Buffer {
    pub(crate) _device: AnyArc<Device>,
    pub(crate) id: NonZeroU64,
    pub(crate) allocation: Option<GpuAllocation>,
    pub(crate) resource: ManuallyDrop<ID3D12Resource>,
    pub(crate) base_address: GPUDescriptorHandle,
    pub(crate) map_state: Mutex<MapState>,
    pub(crate) desc: OwnedBufferDesc,
}

unsafe_impl_iobject!(Buffer, "01944e61-2e75-7ec2-951d-399527ca4856");

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            // Unmap the buffer if it's still mapped
            if self.map_state.get_mut().count > 0 {
                self.resource.Unmap(0, None);
            }

            ManuallyDrop::drop(&mut self.resource);

            self._device
                .allocator
                .as_ref()
                .unwrap_unchecked()
                .free_allocation(self._device.as_ref(), self.allocation.take().unwrap());
        }
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
    pub(crate) fn clamp_max_size_for_view(&self, size: u32) -> u32 {
        if size == u32::MAX {
            self.desc()
                .size
                .try_into()
                .expect("The buffer is too large to create a full range view")
        } else {
            debug_assert!(
                (size as u64) <= self.desc().size,
                "The requested view range is larger than the buffer"
            );
            size
        }
    }
}

impl Buffer {
    pub(crate) fn get(v: &BufferHandle) -> &Self {
        v.get()
            .downcast_ref::<Self>()
            .expect("Unknown Buffer implementation!")
    }

    pub(crate) fn get_id(&self) -> NonZeroU64 {
        self.id
    }

    pub(crate) const fn desc(&self) -> &BufferDesc<'_> {
        self.desc.get()
    }

    pub(crate) fn map(&self) -> Result<NonNull<u8>, ResourceMapError> {
        let mut lock = self.map_state.lock();

        if let Some(ptr) = lock.ptr {
            lock.count = lock.count.checked_add(1).unwrap();
            return Ok(ptr);
        }

        debug_assert_eq!(lock.count, 0);

        // TODO: should we expose 'read_range'?
        unsafe {
            let mut ptr = std::ptr::null_mut();
            self.resource
                .Map(0, None, Some(&mut ptr))
                .inspect_err(|v| log::error!("Platform Error: {:#?}", v))
                .map_err(|_| ResourceMapError::Platform)?;
            let ptr = NonNull::new(ptr as *mut u8).ok_or(ResourceMapError::MappedNullPointer)?;

            lock.count = lock.count.checked_add(1).unwrap();
            lock.ptr = Some(ptr);
            Ok(ptr)
        }
    }

    pub(crate) fn unmap(&self) -> Result<(), ResourceUnmapError> {
        let mut lock = self.map_state.lock();

        lock.count = lock
            .count
            .checked_sub(1)
            .ok_or(ResourceUnmapError::NotMapped)?;

        if lock.count > 0 {
            return Ok(());
        }

        // TODO: should we expose 'written_range'
        unsafe {
            self.resource.Unmap(0, None);
            lock.ptr = None;
        }

        Ok(())
    }
}

#[derive(Default)]
pub(crate) struct MapState {
    count: usize,
    ptr: Option<NonNull<u8>>,
}

unsafe impl Send for MapState {}
