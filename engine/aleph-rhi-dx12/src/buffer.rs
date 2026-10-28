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
use std::ops::Deref;
use std::ptr::NonNull;
use std::sync::Arc;

use aleph_alloc::BHashMap;
use aleph_gpu_allocator::GpuAllocation;
use aleph_object_system::unsafe_impl_iobject;
use aleph_rhi_api::*;
use aleph_rhi_impl_utils::RhiRoot;
use aleph_rhi_impl_utils::owned_desc::OwnedBufferDesc;
use parking_lot::Mutex;
use windows::Win32::Graphics::Direct3D12::*;
use windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT;
use windows::utils::{CPUDescriptorHandle, GPUDescriptorHandle};

use crate::device::Device;
use crate::internal::conv::texture_format_to_dxgi;

pub struct Buffer {
    pub(crate) _device: Arc<Device>,
    pub(crate) id: NonZeroU64,
    pub(crate) allocation: Option<GpuAllocation>,
    pub(crate) resource: ManuallyDrop<ID3D12Resource>,
    pub(crate) base_address: GPUDescriptorHandle,
    pub(crate) map_state: Mutex<MapState>,
    pub(crate) views: Mutex<BHashMap<BufferViewDesc, CPUDescriptorHandle, RhiRoot>>,
    pub(crate) desc: OwnedBufferDesc,
}

unsafe_impl_iobject!(Buffer, "01944e61-2e75-7ec2-951d-399527ca4856");

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            for (_desc, handle) in self.views.get_mut().drain() {
                self._device.descriptor_heaps.cpu_view_heap().free(handle);
            }

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

    fn make_uav_desc_for(
        format: DXGI_FORMAT,
        first_element: u64,
        num_elements: u32,
    ) -> D3D12_UNORDERED_ACCESS_VIEW_DESC {
        D3D12_UNORDERED_ACCESS_VIEW_DESC {
            Format: format,
            ViewDimension: D3D12_UAV_DIMENSION_BUFFER,
            Anonymous: D3D12_UNORDERED_ACCESS_VIEW_DESC_0 {
                Buffer: D3D12_BUFFER_UAV {
                    FirstElement: first_element,
                    NumElements: num_elements,
                    StructureByteStride: 0,
                    CounterOffsetInBytes: 0,
                    Flags: Default::default(),
                },
            },
        }
    }

    fn make_srv_desc_for(
        format: DXGI_FORMAT,
        first_element: u64,
        num_elements: u32,
    ) -> D3D12_SHADER_RESOURCE_VIEW_DESC {
        D3D12_SHADER_RESOURCE_VIEW_DESC {
            Format: format,
            ViewDimension: D3D12_SRV_DIMENSION_BUFFER,
            Shader4ComponentMapping: D3D12_DEFAULT_SHADER_4_COMPONENT_MAPPING,
            Anonymous: D3D12_SHADER_RESOURCE_VIEW_DESC_0 {
                Buffer: D3D12_BUFFER_SRV {
                    FirstElement: first_element,
                    NumElements: num_elements,
                    StructureByteStride: 0,
                    Flags: Default::default(),
                },
            },
        }
    }

    fn remap_buffer_view_range(desc: &BufferDesc, view_desc: &BufferViewDesc) -> (u64, u32) {
        // NOTE: We do not protect against callers asking for a u64::MAX 'whole buffer' view where
        // the resulting view might exceed the maximum size of a texel buffer supported by the GPU.
        //
        // We do at least clamp so it still fits in u32. It's not legal to ask for such a view, and
        // it's the caller's job to ensure they don't ask for such a view.
        //
        // D3D12 has a maximum size for texel buffers, and it's lower than u32::MAX. This means it's
        // okay to clamp to u32::MAX, since it's illegal for the caller to have asked for a view
        // that big in the first place.
        let first_element = view_desc.offset;
        let num_elements = if view_desc.num_elements == u64::MAX {
            // Calculate how many elements we could fit after the requested offset, rounding down
            // to the nearest whole element count.
            //
            // If the offset is beyond the end of the buffer, this will result in zero elements.
            let bytes_per_element = view_desc.format.bytes_per_element() as u64;
            let offset_bytes = first_element.saturating_mul(bytes_per_element);
            let bytes_after_offset = desc.size.saturating_sub(offset_bytes);
            let elements_after_offset = bytes_after_offset / bytes_per_element;
            u32::try_from(elements_after_offset).unwrap_or(u32::MAX)
        } else {
            u32::try_from(view_desc.num_elements).unwrap_or(u32::MAX)
        };

        (first_element, num_elements)
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

    pub(crate) fn get_view(&self, desc: &BufferViewDesc) -> Result<BufferView, ()> {
        let mut views = self.views.lock();

        let view = if let Some(view) = views.get(desc) {
            *view
        } else {
            let handle = self
                ._device
                .descriptor_heaps
                .cpu_view_heap()
                .allocate()
                .ok_or(())?;
            unsafe {
                let format = texture_format_to_dxgi(desc.format);
                let (first_element, num_elements) =
                    Self::remap_buffer_view_range(self.desc(), desc);

                if desc.writable {
                    let view = Self::make_uav_desc_for(format, first_element, num_elements);
                    self._device.device.CreateUnorderedAccessView(
                        self.resource.deref(),
                        None,
                        Some(&view),
                        handle.into(),
                    );
                } else {
                    let view = Self::make_srv_desc_for(format, first_element, num_elements);
                    self._device.device.CreateShaderResourceView(
                        self.resource.deref(),
                        Some(&view),
                        handle.into(),
                    );
                }
            }
            views.insert(desc.clone(), handle);
            handle
        };

        let view = NonNull::new(view.get_inner().get() as *mut ()).unwrap();
        unsafe { Ok(BufferView::from_raw(view)) }
    }
}

#[derive(Default)]
pub(crate) struct MapState {
    count: usize,
    ptr: Option<NonNull<u8>>,
}

unsafe impl Send for MapState {}
