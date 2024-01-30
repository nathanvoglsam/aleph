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

use aleph_any::{declare_interfaces, AnyArc, AnyWeak};
use aleph_rhi_api::*;

use crate::{NullContext, NullDevice, NullQueue};

pub struct NullAdapter {
    pub(crate) _this: AnyWeak<Self>,
    pub(crate) _context: AnyArc<NullContext>,
}

declare_interfaces!(NullAdapter, [IAdapter]);

crate::impl_platform_interface_passthrough!(NullAdapter);

impl IAdapter for NullAdapter {
    fn upgrade(&self) -> AnyArc<dyn IAdapter> {
        AnyArc::map::<dyn IAdapter, _>(self._this.upgrade().unwrap(), |v| v)
    }

    fn strong_count(&self) -> usize {
        self._this.strong_count()
    }

    fn weak_count(&self) -> usize {
        self._this.weak_count()
    }

    fn description(&self) -> AdapterDescription {
        AdapterDescription {
            name: "NullAdapter",
            vendor: AdapterVendor::Unknown,
        }
    }

    fn request_device(&self) -> Result<AnyArc<dyn IDevice>, RequestDeviceError> {
        fn make_queue(device_weak: &AnyWeak<NullDevice>) -> AnyArc<NullQueue> {
            AnyArc::new_cyclic(move |v| NullQueue {
                _this: v.clone(),
                _device: device_weak.clone(),
            })
        }

        let device = AnyArc::new_cyclic(move |v| {
            let general_queue = make_queue(v);
            let compute_queue = make_queue(v);
            let transfer_queue = make_queue(v);
            NullDevice {
                _this: v.clone(),
                _adapter: self._this.upgrade().unwrap(),
                _context: self._context._this.upgrade().unwrap(),
                general_queue: Some(general_queue),
                compute_queue: Some(compute_queue),
                transfer_queue: Some(transfer_queue),
            }
        });
        Ok(AnyArc::map::<dyn IDevice, _>(device, |v| v))
    }
}
