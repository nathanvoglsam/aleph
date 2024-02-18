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

use std::any::TypeId;
use std::mem::transmute;
use std::ptr;
use std::ptr::NonNull;

use aleph_any::{AnyArc, AnyWeak, IAny, TraitObject};
use aleph_rhi_api::*;

use crate::NullDevice;

pub struct NullQueue {
    pub(crate) _this: AnyWeak<Self>,
    pub(crate) _device: AnyWeak<NullDevice>,
}

// Unwrapped declare_interfaces as we need to inject a custom condition for returning IQueueDebug
impl IAny for NullQueue {
    #[allow(bare_trait_objects)]
    fn __query_interface(&self, target: TypeId) -> Option<TraitObject> {
        unsafe {
            if target == TypeId::of::<dyn IQueue>() {
                return Some(transmute(self as &dyn IQueue));
            }
            if target == TypeId::of::<dyn IQueueDebug>() {
                return Some(transmute(self as &dyn IQueueDebug));
            }
            if target == TypeId::of::<dyn IAny>() {
                return Some(transmute(self as &dyn IAny));
            }
        }
        unsafe {
            if target == TypeId::of::<NullQueue>() {
                Some(TraitObject {
                    data: NonNull::new_unchecked(self as *const _ as *mut ()),
                    vtable: ptr::null_mut(),
                    phantom: Default::default(),
                })
            } else {
                None
            }
        }
    }
}

crate::impl_platform_interface_passthrough!(NullQueue);

impl IQueue for NullQueue {
    fn upgrade(&self) -> AnyArc<dyn IQueue> {
        AnyArc::map::<dyn IQueue, _>(self._this.upgrade().unwrap(), |v| v)
    }

    fn strong_count(&self) -> usize {
        self._this.strong_count()
    }

    fn weak_count(&self) -> usize {
        self._this.weak_count()
    }

    fn queue_properties(&self) -> QueueProperties {
        QueueProperties {
            min_image_transfer_granularity: Default::default(),
        }
    }

    fn garbage_collect(&self) {}

    fn wait_idle(&self) {}

    unsafe fn submit(&self, _desc: &QueueSubmitDesc) -> Result<(), QueueSubmitError> {
        Ok(())
    }

    unsafe fn present(&self, _desc: &QueuePresentDesc) -> Result<(), QueuePresentError> {
        Ok(())
    }
}

impl IQueueDebug for NullQueue {
    fn set_marker(&self, _color: Color, _message: &str) {}

    fn begin_event(&self, _color: Color, _message: &str) {}

    fn end_event(&self) {}
}
