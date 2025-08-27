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

use aleph_any::{AnyArc, AnyWeak, declare_interfaces};
use aleph_rhi_api::*;
use aleph_rhi_impl_utils::object_counter::ObjectCounter;
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_metal::*;

use crate::context::Context;
use crate::device::Device;
use crate::queue::Queue;
use crate::surface::Surface;

pub struct Adapter {
    pub(crate) this: AnyWeak<Self>,
    pub(crate) context: AnyArc<Context>,
    pub(crate) surface: Option<AnyArc<Surface>>,
    pub(crate) name: String,
    pub(crate) vendor: AdapterVendor,
    pub(crate) objects: AdapterObjects,
}

declare_interfaces!(Adapter, [IAdapter]);

impl IGetPlatformInterface for Adapter {
    unsafe fn __query_platform_interface(&self, _target: TypeId, _out: *mut ()) -> Option<()> {
        todo!()
    }
}

impl IAdapter for Adapter {
    fn upgrade(&self) -> AnyArc<dyn IAdapter> {
        AnyArc::map::<dyn IAdapter, _>(self.this.upgrade().unwrap(), |v| v)
    }

    fn strong_count(&self) -> usize {
        self.this.strong_count()
    }

    fn weak_count(&self) -> usize {
        self.this.weak_count()
    }

    fn description(&self) -> AdapterDescription<'_> {
        AdapterDescription {
            name: &self.name,
            vendor: self.vendor,
        }
    }

    fn request_device(&self) -> Result<AnyArc<dyn IDevice>, RequestDeviceError> {
        if let Some(surface) = self.surface.as_ref() {
            unsafe {
                surface.objects.layer.setDevice(Some(&self.objects.device));
            }

            let preferred = unsafe { surface.objects.layer.preferredDevice() };
            if let Some(preferred) = preferred {
                if preferred != self.objects.device {
                    log::warn!("Selected Device is not Preferred by CAMetalLayer");
                }
            }
        }

        let device = AnyArc::new_cyclic(move |v| {
            let mut device = Device {
                this: v.clone(),
                _adapter: self.this.upgrade().unwrap(),
                context: self.context.clone(),
                device: self.objects.device.clone(),
                listener: MTLSharedEventListener::new(),
                general_queue: None,
                compute_queue: None,
                transfer_queue: None,
                // command_list_pool: CommandListPool::new(),
                object_counter: ObjectCounter::new(),
            };

            Self::build_queue_objects(&mut device);

            device
        });

        Ok(AnyArc::map::<dyn IDevice, _>(device, |v| v))
    }
}

impl Adapter {
    fn build_queue_objects(device: &mut Device) {
        let queue = Queue::new(device, QueueType::General);
        device.general_queue = queue;

        let queue = Queue::new(device, QueueType::Compute);
        device.compute_queue = queue;

        let queue = Queue::new(device, QueueType::Transfer);
        device.transfer_queue = queue;

        if device.general_queue.is_none() {
            panic!("No general queue was created! Something is very wrong!");
        }

        if device.compute_queue.is_none() {
            log::warn!("No compute queue was created!");
        }

        if device.transfer_queue.is_none() {
            log::warn!("No transfer queue was created!");
        }
    }
}

/// Wrapper struct to limit the scope of our 'unsafe impl Send+Sync'
pub struct AdapterObjects {
    pub device: Retained<ProtocolObject<dyn MTLDevice>>,
}

// Safety: Needed because of 'MTLDevice'
unsafe impl Send for AdapterObjects {}
unsafe impl Sync for AdapterObjects {}
