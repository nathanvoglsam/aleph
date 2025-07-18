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

use aleph_any::AnyArc;
use aleph_object_system::unsafe_impl_iobject;
use aleph_rhi_api::*;
use aleph_rhi_impl_utils::owned_desc::OwnedBufferDesc;
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_foundation::NSRange;
use objc2_metal::*;

use crate::device::Device;

pub struct Buffer {
    pub(crate) _device: AnyArc<Device>,
    pub(crate) id: NonZeroU64,
    pub(crate) objects: BufferObjects,
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
    pub(crate) fn get_buffer_id(&self) -> std::num::NonZeroU64 {
        self.id
    }

    pub(crate) const fn desc(&self) -> &BufferDesc {
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

    pub(crate) fn flush_buffer_range(&self, offset: u64, len: u64) {
        // TODO: will we need this?
        if false {
            let range = NSRange::new(offset as usize, len as usize);
            self.objects.buffer.didModifyRange(range);
        }
    }

    pub(crate) fn invalidate_buffer_range(&self, offset: u64, len: u64) {
        todo!()
    }
}

pub struct BufferObjects {
    pub buffer: Retained<ProtocolObject<dyn MTLBuffer>>,
}
