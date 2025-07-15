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
use aleph_object_system::unsafe_impl_iobject;
use aleph_rhi_api::*;
use aleph_rhi_impl_utils::owned_desc::OwnedBufferDesc;
use parking_lot::Mutex;

use crate::device::Device;

pub struct Buffer {
    pub(crate) _device: AnyArc<Device>,
    pub(crate) id: NonZeroU64,
    pub(crate) map_state: Mutex<MapState>,
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

    pub(crate) fn map_buffer(
        &self,
        device: &Device,
    ) -> Result<std::ptr::NonNull<u8>, ResourceMapError> {
        todo!()
    }

    pub(crate) fn unmap_buffer(&self, device: &Device) -> Result<(), ResourceUnmapError> {
        todo!()
    }

    pub(crate) fn flush_buffer_range(&self, device: &Device, offset: u64, len: u64) {
        todo!()
    }

    pub(crate) fn invalidate_buffer_range(&self, device: &Device, offset: u64, len: u64) {
        todo!()
    }
}

#[derive(Default)]
pub(crate) struct MapState {
    pub(crate) count: usize,
    pub(crate) ptr: Option<NonNull<u8>>,
}

unsafe impl Send for MapState {}
