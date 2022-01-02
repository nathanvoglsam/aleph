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

use crate::device::Device;
use dx12::dxgi;
use interfaces::any::{declare_interfaces, AnyArc};
use interfaces::gpu::{AdapterDescription, IGpuAdapter, IGpuDevice, RequestDeviceError};

pub struct Adapter {
    pub(crate) name: String,
    pub(crate) adapter: dxgi::Adapter,
}

impl IGpuAdapter for Adapter {
    fn description(&mut self) -> AdapterDescription {
        AdapterDescription { name: &self.name }
    }

    fn request_device(&mut self) -> Result<AnyArc<dyn IGpuDevice>, RequestDeviceError> {
        // Create the actual d3d12 device
        let device =
            dx12::Device::new(&self.adapter, dx12::FeatureLevel::Level_11_0).map_err(|e| {
                let e = Box::new(e);
                RequestDeviceError::Platform(e)
            })?;

        // Create a single direct queue
        let desc = dx12::CommandQueueDesc::builder()
            .queue_type(dx12::CommandListType::Direct)
            .priority(0)
            .build();
        let queue = device.create_command_queue(&desc).map_err(|e| {
            let e = Box::new(e);
            RequestDeviceError::Platform(e)
        })?;

        // Bundle and return the device
        let device = Device { device, queue };
        let device = AnyArc::new(device);
        Ok(device.query_interface().unwrap())
    }
}

pub trait IGpuAdapterExt: IGpuAdapter {
    fn get_raw_handle(&self) -> &dxgi::Adapter;
}

impl IGpuAdapterExt for Adapter {
    fn get_raw_handle(&self) -> &dxgi::Adapter {
        &self.adapter
    }
}

declare_interfaces!(Adapter, [IGpuAdapter, IGpuAdapterExt]);
