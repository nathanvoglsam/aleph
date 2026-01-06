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

use aleph_any::AnyArc;
use aleph_object_system::{Object, unsafe_impl_iobject};
use aleph_rhi_api::*;
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_metal::*;

use crate::device::Device;

pub struct Fence {
    pub(crate) _device: AnyArc<Device>,
    pub(crate) objects: FenceObjects,
}

unsafe_impl_iobject!(Fence, "01980753-5c4f-7ae3-be3b-96ea6487c813");

impl Fence {
    pub(crate) fn create(device: &Device, value: u64) -> Result<FenceHandle, FenceCreateError> {
        let event = match device.device.newSharedEvent() {
            Some(v) => v,
            None => return Err(FenceCreateError::Platform),
        };

        event.setSignaledValue(value);

        let fence = Fence {
            _device: device.this.upgrade().unwrap(),
            objects: FenceObjects { event },
        };
        let fence = Object::new_arc_opaque(fence);
        unsafe { Ok(FenceHandle::new(fence)) }
    }

    pub(crate) fn get(v: &FenceHandle) -> &Self {
        v.get()
            .downcast_ref::<Self>()
            .expect("Unknown Fence implementation!")
    }
}

pub struct FenceObjects {
    pub event: Retained<ProtocolObject<dyn MTLSharedEvent>>,
}
