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

use crate::adapter::Adapter;
use crate::context::Context;
use crate::internal::queue_present_support::QueuePresentSupportFlags;
use crate::internal::queues::Queues;
use crate::surface::Surface;
use erupt::vk;
use interfaces::gpu::IDevice;
use interfaces::ref_ptr::{ref_ptr_object, RefPtr};
use std::collections::HashMap;

ref_ptr_object! {
    pub struct Device: IDevice, IDeviceExt {
        pub(crate) device_loader: erupt::DeviceLoader,
        pub(crate) queues: Queues,
        pub(crate) adapter: RefPtr<Adapter>,
        pub(crate) context: RefPtr<Context>,
        pub(crate) surface_support: HashMap<usize, QueuePresentSupportFlags>,
    }
}

impl IDevice for Device {
    fn create_sampler(&self) {
        todo!()
    }
}

pub trait IDeviceExt: IDevice {
    fn get_raw_handle(&self) -> &erupt::DeviceLoader;
    fn get_raw_general_queue(&self) -> Option<vk::Queue>;
    fn get_raw_compute_queue(&self) -> Option<vk::Queue>;
    fn get_raw_transfer_queue(&self) -> Option<vk::Queue>;
}

impl IDeviceExt for Device {
    fn get_raw_handle(&self) -> &erupt::DeviceLoader {
        &self.device_loader
    }

    fn get_raw_general_queue(&self) -> Option<vk::Queue> {
        self.queues.general.as_ref().map(|v| v.queue)
    }

    fn get_raw_compute_queue(&self) -> Option<vk::Queue> {
        self.queues.compute.as_ref().map(|v| v.queue)
    }

    fn get_raw_transfer_queue(&self) -> Option<vk::Queue> {
        self.queues.transfer.as_ref().map(|v| v.queue)
    }
}
