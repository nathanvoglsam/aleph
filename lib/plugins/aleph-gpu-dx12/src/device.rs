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

use dx12::{AsWeakRef, CommandQueue, WeakRef};
use interfaces::any::declare_interfaces;
use interfaces::gpu::IDevice;

pub struct Device {
    pub(crate) device: dx12::Device,
    pub(crate) queues: Queues,
}

impl IDevice for Device {
    fn create_sampler(&self) {
        todo!()
    }
}

pub trait IDeviceExt: IDevice {
    fn get_raw_handle(&self) -> &dx12::Device;
    fn get_raw_general_queue(&self) -> Option<WeakRef<CommandQueue>>;
    fn get_raw_compute_queue(&self) -> Option<WeakRef<CommandQueue>>;
    fn get_raw_transfer_queue(&self) -> Option<WeakRef<CommandQueue>>;
}

impl IDeviceExt for Device {
    fn get_raw_handle(&self) -> &dx12::Device {
        &self.device
    }

    fn get_raw_general_queue(&self) -> Option<WeakRef<CommandQueue>> {
        self.queues.general.as_ref().map(|v| v.as_weak())
    }

    fn get_raw_compute_queue(&self) -> Option<WeakRef<CommandQueue>> {
        self.queues.compute.as_ref().map(|v| v.as_weak())
    }

    fn get_raw_transfer_queue(&self) -> Option<WeakRef<CommandQueue>> {
        self.queues.transfer.as_ref().map(|v| v.as_weak())
    }
}

declare_interfaces!(Device, [IDevice, IDeviceExt]);

pub(crate) struct Queues {
    pub general: Option<CommandQueue>,
    pub compute: Option<CommandQueue>,
    pub transfer: Option<CommandQueue>,
}
