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

use aleph_gpu_allocator::{AllocationDesc, GpuAllocation, MemoryLocation};
use aleph_object_system::{ArcObject, Object, unsafe_impl_iobject};
use aleph_rhi_api::*;
use aleph_rhi_impl_utils::owned_desc::OwnedBufferDesc;
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_foundation::NSString;
use objc2_metal::*;

use crate::device::Device;

pub struct Buffer {
    pub(crate) _device: Arc<Device>,
    pub(crate) id: NonZeroU64,
    pub(crate) objects: BufferObjects,
    pub(crate) allocation: Option<GpuAllocation>,
    pub(crate) gpu_addr: NonZero<u64>,
    pub(crate) desc: OwnedBufferDesc,
}

unsafe_impl_iobject!(Buffer, "01980753-5c4f-7ae3-be3b-96cf500b987c");

impl Buffer {
    pub(crate) fn get(v: &BufferHandle) -> &Self {
        v.get()
            .downcast_ref::<Self>()
            .expect("Unknown Buffer implementation!")
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
            && device.context.debug
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

    pub(crate) fn flush_buffer_range(&self, _offset: u64, _len: u64) {
        // Intentional no-op
    }

    pub(crate) fn invalidate_buffer_range(&self, _offset: u64, _len: u64) {
        // Intentional no-op
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            self._device
                .allocator
                .as_ref()
                .unwrap_unchecked()
                .free_allocation(self._device.as_ref(), self.allocation.take().unwrap());
        }
    }
}

pub struct BufferObjects {
    pub buffer: Retained<ProtocolObject<dyn MTLBuffer>>,
}
