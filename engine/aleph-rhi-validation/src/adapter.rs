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

use aleph_rhi_api::*;

use crate::{ValidationContext, ValidationDevice, ValidationQueue};

pub struct ValidationAdapter {
    pub(crate) _this: Weak<Self>,
    pub(crate) _context: Arc<ValidationContext>,
    pub(crate) inner: Arc<dyn IAdapter>,
}

crate::impl_platform_interface_passthrough!(ValidationAdapter);

impl IAdapter for ValidationAdapter {
    fn upgrade(&self) -> Arc<dyn IAdapter> {
        self._this.upgrade().unwrap()
    }

    fn strong_count(&self) -> usize {
        self._this.strong_count()
    }

    fn weak_count(&self) -> usize {
        self._this.weak_count()
    }

    fn description(&self) -> AdapterDescription<'_> {
        self.inner.description()
    }

    fn request_device(&self) -> Result<Arc<dyn IDevice>, RequestDeviceError> {
        fn query_queue(
            inner: &dyn IDevice,
            device_weak: Weak<ValidationDevice>,
            queue_type: QueueType,
        ) -> Option<Arc<ValidationQueue>> {
            inner.get_queue(queue_type).map(|q| {
                // Query the inner queue for support for the debug interface. This controls whether
                // ValidationQueue can also expose IQueueDebug.
                Arc::new_cyclic(move |v| ValidationQueue {
                    _this: v.clone(),
                    _device: device_weak,
                    inner: q,
                    queue_type,
                })
            })
        }

        let inner = self.inner.request_device()?;

        let device = Arc::new_cyclic(move |v| {
            let general_queue = query_queue(inner.as_ref(), v.clone(), QueueType::General);
            let compute_queue = query_queue(inner.as_ref(), v.clone(), QueueType::Compute);
            let transfer_queue = query_queue(inner.as_ref(), v.clone(), QueueType::Transfer);
            ValidationDevice {
                _this: v.clone(),
                _adapter: self._this.upgrade().unwrap(),
                _context: self._context._this.upgrade().unwrap(),
                inner,
                pool_counter: Default::default(),
                general_queue,
                compute_queue,
                transfer_queue,
            }
        });
        Ok(device)
    }
}
