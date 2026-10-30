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

use std::num::{NonZero, NonZeroU64};
use std::sync::Arc;

use aleph_alloc::BHashMap;
use aleph_gpu_allocator::{AllocationDesc, GpuAllocation, MemoryLocation};
use aleph_object_system::{ArcObject, Object, unsafe_impl_iobject};
use aleph_rhi_api::*;
use aleph_rhi_impl_utils::owned_desc::OwnedBufferDesc;
use aleph_rhi_impl_utils::{RhiRoot, abort_on_unwind};
use objc2::rc::{Retained, autoreleasepool};
use objc2::runtime::ProtocolObject;
use objc2_foundation::NSString;
use objc2_metal::*;
use parking_lot::Mutex;

use crate::device::Device;
use crate::internal::conv;
use crate::internal::image_view_pool::ImageViewAllocation;

pub struct Buffer {
    pub(crate) _device: Arc<Device>,
    pub(crate) id: NonZeroU64,
    pub(crate) objects: BufferObjects,
    pub(crate) allocation: Option<GpuAllocation>,
    pub(crate) gpu_addr: NonZero<u64>,
    pub(crate) views: Mutex<BHashMap<BufferViewDesc, (BufferView, ImageViewAllocation), RhiRoot>>,
    pub(crate) desc: OwnedBufferDesc,
}

unsafe_impl_iobject!(Buffer, "01980753-5c4f-7ae3-be3b-96cf500b987c");

impl Buffer {
    pub(crate) fn get(v: &BufferHandle) -> &Self {
        v.get()
            .downcast_ref::<Self>()
            .expect("Unknown Buffer implementation!")
    }

    fn remap_buffer_view_range(desc: &BufferDesc, view_desc: &BufferViewDesc) -> (u64, u64) {
        // NOTE: We do not protect against callers asking for a u64::MAX 'whole buffer' view where
        // the resulting view might exceed the maximum size of a texel buffer supported by the GPU.
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
            elements_after_offset
        } else {
            view_desc.num_elements
        };

        (first_element, num_elements as u64)
    }
}

impl Buffer {
    pub(crate) fn new(
        device: &Device,
        desc: &BufferDesc,
    ) -> Result<Arc<Object<Self>>, BufferCreateError> {
        let memory_location = match desc.cpu_access {
            CpuAccessMode::None => MemoryLocation::GpuLocal,
            CpuAccessMode::Read => MemoryLocation::GpuToCpu,
            CpuAccessMode::Write => MemoryLocation::CpuToGpu,
        };
        let alloc_info = AllocationDesc {
            location: memory_location,
            strategy: Default::default(),
            desc: desc.size as usize,
        };

        let (allocation, _, buffer) = unsafe {
            device
                .allocator
                .as_ref()
                .unwrap_unchecked()
                .allocate_buffer(device, &alloc_info)
                .ok_or(BufferCreateError::OutOfMemory)?
        };

        if let Some(name) = desc.name
            && device.context.options.debug
        {
            let mtl_name = NSString::from_str(name);
            buffer.setLabel(Some(&mtl_name));
        }

        let gpu_addr = buffer.gpuAddress();
        let gpu_addr = NonZero::new(gpu_addr).ok_or(BufferCreateError::Platform)?;

        let out = Buffer {
            _device: device.this.upgrade().unwrap(),
            id: device.object_counter.next_buffer(),
            objects: BufferObjects { buffer },
            allocation: Some(allocation),
            gpu_addr,
            views: Default::default(),
            desc: OwnedBufferDesc::new(desc.clone()),
        };
        let out = Object::new_arc(out);
        Ok(out)
    }

    pub(crate) fn create(
        device: &Device,
        desc: &BufferDesc,
    ) -> Result<BufferHandle, BufferCreateError> {
        let out = Self::new(device, desc)?;
        let out = ArcObject::from_object(out);
        unsafe { Ok(BufferHandle::new(out)) }
    }

    pub(crate) fn get_buffer_id(&self) -> NonZeroU64 {
        self.id
    }

    pub(crate) const fn desc(&self) -> &BufferDesc<'_> {
        self.desc.get()
    }

    pub(crate) fn map_buffer(&self) -> Result<std::ptr::NonNull<u8>, ResourceMapError> {
        let ptr = match self.desc().cpu_access {
            CpuAccessMode::None => return Err(ResourceMapError::MappedNullPointer),
            CpuAccessMode::Read | CpuAccessMode::Write => self.objects.buffer.contents(),
        };
        Ok(ptr.cast())
    }

    pub(crate) fn unmap_buffer(&self) -> Result<(), ResourceUnmapError> {
        // Intentional no-op. All buffers are always mapped
        Ok(())
    }

    pub(crate) fn get_buffer_view(&self, desc: &BufferViewDesc) -> Result<BufferView, ()> {
        let mut views = self.views.lock();
        if let Some((view, _)) = views.get(desc) {
            return Ok(*view);
        }

        let (first_element, num_elements) = Self::remap_buffer_view_range(self.desc(), desc);

        let bytes_per_element = desc.format.bytes_per_element() as u64;
        let offset_bytes = first_element.saturating_mul(bytes_per_element);
        let view_bytes = num_elements.saturating_mul(bytes_per_element);

        let buffer_size = self.desc().size;
        debug_assert!(offset_bytes < buffer_size);
        debug_assert!(view_bytes <= buffer_size.saturating_sub(offset_bytes));

        let pixel_format = conv::format_to_pixel_mtl(desc.format);
        if cfg!(debug_assertions) {
            let alignment = self
                ._device
                .device
                .minimumLinearTextureAlignmentForPixelFormat(pixel_format)
                as u64;
            debug_assert!(alignment == 0 || offset_bytes % alignment == 0);
        }

        let usage = if desc.writable {
            MTLTextureUsage::ShaderRead | MTLTextureUsage::ShaderWrite
        } else {
            MTLTextureUsage::ShaderRead
        };
        let (id, allocation) = unsafe {
            let descriptor = MTLTextureDescriptor::textureBufferDescriptorWithPixelFormat_width_resourceOptions_usage(
                pixel_format,
                num_elements as usize,
                self.objects.buffer.resourceOptions(),
                usage,
            );
            self._device.image_view_pools.alloc_buffer_view(
                self.id.get() as usize,
                &self.objects.buffer,
                &descriptor,
                offset_bytes as usize,
                view_bytes as usize,
            )
        };

        let view = unsafe { BufferView::from_raw_int(id.to_raw()).ok_or(())? };
        views.insert(desc.clone(), (view, allocation));
        Ok(view)
    }

    pub(crate) fn flush_buffer_range(&self, _offset: u64, _len: u64) {
        // Intentional no-op
    }

    pub(crate) fn invalidate_buffer_range(&self, _offset: u64, _len: u64) {
        // Intentional no-op
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        abort_on_unwind(|| unsafe {
            for (_, (_, allocation)) in self.views.get_mut().drain() {
                self._device
                    .image_view_pools
                    .free(self.id.get() as usize, allocation);
            }
            self._device
                .allocator
                .as_ref()
                .unwrap_unchecked()
                .free_allocation(self._device.as_ref(), self.allocation.take().unwrap());
        })
    }
}

pub struct BufferObjects {
    pub buffer: Retained<ProtocolObject<dyn MTLBuffer>>,
}
