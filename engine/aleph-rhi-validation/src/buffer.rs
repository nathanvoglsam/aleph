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

use std::sync::{Arc, Weak};

use aleph_alloc::{BBox, BHashMap};
use aleph_object_system::{Object, unsafe_impl_iobject};
use aleph_rhi_api::*;
use aleph_rhi_impl_utils::RhiRoot;
use parking_lot::Mutex;

use crate::ValidationDevice;

pub struct ValidationBuffer {
    pub(crate) _this: Weak<Object<ValidationBuffer>>,
    pub(crate) _device: Arc<ValidationDevice>,
    pub(crate) size: u64,
    pub(crate) usage: ResourceUsageFlags,
    pub(crate) name: Option<BBox<str, RhiRoot>>,
    pub(crate) inner: BufferHandle,
    pub(crate) views: Mutex<BHashMap<BufferViewDesc, Box<ValidationBufferView>, RhiRoot>>,
}

unsafe_impl_iobject!(ValidationBuffer, "01944e4f-dacb-7122-bdec-ae9da89e1a0e");

impl ValidationBuffer {
    pub(crate) fn get(v: &BufferHandle) -> &Self {
        v.get()
            .downcast_ref::<Self>()
            .expect("Unknown Buffer implementation!")
    }

    pub(crate) fn get_view(
        &self,
        device: &ValidationDevice,
        desc: &BufferViewDesc,
    ) -> Result<BufferView, ()> {
        let mut views = self.views.lock();
        let view = if let Some(v) = views.get(desc) {
            let v = Box::as_ptr(v);
            unsafe { std::mem::transmute::<_, BufferView>(v) }
        } else {
            let buffer_view = Box::new(ValidationBufferView {
                _buffer: self._this.clone(),
                buffer_view: device.inner.get_buffer_view(&self.inner, desc)?,
                desc: desc.clone(),
            });

            let v = Box::as_ptr(&buffer_view);
            let v = unsafe { std::mem::transmute::<_, BufferView>(v) };
            views.insert(desc.clone(), buffer_view);

            v
        };
        Ok(view)
    }

    pub fn validate_range(&self, offset: u64, len: u64) {
        let size = self.size;
        assert!(
            offset < size,
            "Invalidation range (offset: {offset}, len: {len}) outside buffer size ({size})",
        );
        assert!(
            offset + len < size,
            "Invalidation range (offset: {offset}, len: {len}) outside buffer size ({size})",
        );
    }
}

pub struct ValidationBufferView {
    pub _buffer: Weak<Object<ValidationBuffer>>,
    pub buffer_view: BufferView,
    pub desc: BufferViewDesc,
}

impl ValidationBufferView {
    pub unsafe fn get(v: &BufferView) -> &ValidationBufferView {
        unsafe {
            std::mem::transmute::<_, *const ValidationBufferView>(*v)
                .as_ref()
                .unwrap()
        }
    }
}
