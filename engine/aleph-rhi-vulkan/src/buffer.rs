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

use std::hint::unreachable_unchecked;
use std::num::NonZeroU64;
use std::ptr::NonNull;
use std::sync::Arc;

use aleph_alloc::BHashMap;
use aleph_gpu_allocator::GpuAllocation;
use aleph_object_system::unsafe_impl_iobject;
use aleph_rhi_api::*;
use aleph_rhi_impl_utils::owned_desc::OwnedBufferDesc;
use aleph_rhi_impl_utils::{RhiRoot, abort_on_unwind};
use ash::vk;
use parking_lot::Mutex;

use crate::device::Device;
use crate::internal::allocation_callbacks::GLOBAL;
use crate::internal::conv;

pub struct Buffer {
    pub(crate) _device: Arc<Device>,
    pub(crate) id: NonZeroU64,
    pub(crate) buffer: vk::Buffer,
    pub(crate) allocation: Option<GpuAllocation>,
    pub(crate) map_state: Mutex<MapState>,
    pub(crate) memory: vk::DeviceMemory,
    pub(crate) block_size: u64,
    pub(crate) views: Mutex<BHashMap<BufferViewDesc, vk::BufferView, RhiRoot>>,
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

    fn create_view_for_usage(
        &self,
        device: &Device,
        desc: &BufferViewDesc,
    ) -> Result<vk::BufferView, ()> {
        let format = conv::texture_format_to_vk(desc.format);

        // Caller's responsibility to ensure the offset and range are valid for the buffer.
        let offset = desc
            .offset
            .saturating_mul(desc.format.bytes_per_element() as u64);
        let range = if desc.num_elements == u64::MAX {
            vk::WHOLE_SIZE
        } else {
            desc.num_elements
                .saturating_mul(desc.format.bytes_per_element() as u64)
        };

        let mut create_info = vk::BufferViewCreateInfo {
            flags: vk::BufferViewCreateFlags::empty(),
            buffer: self.buffer,
            format: format,
            offset: offset,
            range: range,
            ..Default::default()
        };

        // If the device supports maintenance5, we attach a reduced usage scope as an optimization
        // hint to the driver about the intended usage of the buffer view.
        //
        // The 'mutable' flag decides whether the view can be used as a storage texel buffer.
        let mut usage_info = vk::BufferUsageFlags2CreateInfoKHR {
            ..Default::default()
        };
        let device_info = &device.adapter.device_info;
        if device_info.maintenance5_features.maintenance5 != vk::FALSE {
            usage_info.usage = if desc.writable {
                vk::BufferUsageFlags2KHR::UNIFORM_TEXEL_BUFFER
                    | vk::BufferUsageFlags2KHR::STORAGE_TEXEL_BUFFER
            } else {
                vk::BufferUsageFlags2KHR::UNIFORM_TEXEL_BUFFER
            };
            create_info = create_info.push_next(&mut usage_info);
        }

        let view = unsafe {
            device
                .device
                .create_buffer_view(&create_info, GLOBAL)
                .map_err(|_| ())? // TODO: error handling
        };

        Ok(view)
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

    pub(crate) fn get_buffer_view(
        &self,
        device: &Device,
        desc: &BufferViewDesc,
    ) -> Result<BufferView, ()> {
        let mut views = self.views.lock();

        let view = if let Some(view) = views.get(desc) {
            *view
        } else {
            let view = self.create_view_for_usage(device, desc)?;
            views.insert(desc.clone(), view);
            view
        };

        unsafe { Ok(std::mem::transmute::<_, BufferView>(view)) }
    }

    pub(crate) fn flush_buffer_range(&self, device: &Device, offset: u64, len: u64) {
        unsafe {
            let limits = &device.adapter.device_info.properties_10.limits;
            let (true_offset, true_len) =
                self.get_range_for_flush_or_invalidate(offset, len, limits.non_coherent_atom_size);
            let ranges = [vk::MappedMemoryRange {
                memory: self.memory,
                offset: true_offset,
                size: true_len,
                ..Default::default()
            }];
            device.device.flush_mapped_memory_ranges(&ranges).unwrap();
        }
    }

    pub(crate) fn invalidate_buffer_range(&self, device: &Device, offset: u64, len: u64) {
        unsafe {
            let limits = &device.adapter.device_info.properties_10.limits;
            let (true_offset, true_len) =
                self.get_range_for_flush_or_invalidate(offset, len, limits.non_coherent_atom_size);
            let ranges = [vk::MappedMemoryRange {
                memory: self.memory,
                offset: true_offset,
                size: true_len,
                ..Default::default()
            }];
            device
                .device
                .invalidate_mapped_memory_ranges(&ranges)
                .unwrap();
        }
    }
}

impl Buffer {
    /// Given an offset and length within the buffer, returns a new offset + len pair this is valid
    /// for use in a [`vk::MappedMemoryRange`].
    ///
    /// This will expand the range to satisfy the alignment requirements of the non-coherent atom
    /// size, and clip the resulting range to the boundaries of the buffer or the memory block if
    /// the buffer reaches to the end of the block.
    ///
    /// This may cause the returned range to be larger than the originally requested range, and the
    /// range may be larger than the buffer itself. The expected use for flush/invalidate ranges
    /// means that we may _over_ synchronize, but this is acceptable for the intended use case.
    ///
    /// # Safety
    ///
    /// The caller must ensure non_coherent_atom_size is a power of two.
    unsafe fn get_range_for_flush_or_invalidate(
        &self,
        offset: u64,
        len: u64,
        non_coherent_atom_size: u64,
    ) -> (u64, u64) {
        unsafe {
            if !non_coherent_atom_size.is_power_of_two() {
                // Vulkan promises to never give as a NPOT value for nonCoherentAtomSize.
                // This is optimization should give the compiler enough info to transform the
                // div/mul operations to bitshifts.
                unreachable_unchecked()
            }
        }

        // Get the offset within the memory block this buffer belongs to.
        let memory_offset = self
            .allocation
            .as_ref()
            .map(|v| v.block_offset() as u64)
            .unwrap_or(0);

        // This is the end of the memory region this buffer occupies within the memory block.
        let memory_end = memory_offset + self.desc().size;

        // The true offset we give to Vulkan is within the memory block, not the buffer. Then we
        // align down to the nearest multiple of non_coherent_atom_size. This may dilate the range
        // beyond the buffer's actual range.
        let true_start = memory_offset + offset;
        let true_offset = true_start / non_coherent_atom_size * non_coherent_atom_size;

        // Clip the requested range to the buffer, align its end up to the atom size, then clip
        // the result to the end of the memory block.
        let true_end = true_start.saturating_add(len).min(memory_end);
        let true_end = true_end
            .div_ceil(non_coherent_atom_size)
            .saturating_mul(non_coherent_atom_size)
            .min(self.block_size);
        let true_len = true_end - true_offset;

        (true_offset, true_len)
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        abort_on_unwind(|| unsafe {
            for (_desc, view) in self.views.get_mut().drain() {
                self._device.device.destroy_buffer_view(view, GLOBAL);
            }

            self._device.device.destroy_buffer(self.buffer, GLOBAL);
            self._device
                .allocator
                .as_ref()
                .unwrap_unchecked()
                .free_allocation(self._device.as_ref(), self.allocation.take().unwrap());
        })
    }
}

#[derive(Default)]
pub(crate) struct MapState {
    pub(crate) count: usize,
    pub(crate) ptr: Option<NonNull<u8>>,
}

unsafe impl Send for MapState {}
