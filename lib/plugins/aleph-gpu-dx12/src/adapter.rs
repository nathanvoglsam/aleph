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

use crate::context::Context;
use crate::descriptor_allocator_cpu::DescriptorAllocatorCPU;
use crate::device::{Device, Queues};
use crate::internal::in_flight_command_list::ReturnToPool;
use crate::internal::queue::Queue;
use dx12::dxgi;
use interfaces::anyhow::anyhow;
use interfaces::gpu::{AdapterDescription, IAdapter, IDevice, RequestDeviceError};
use interfaces::ref_ptr::{ref_ptr_init, ref_ptr_object, RefPtr, RefPtrObject};

ref_ptr_object! {
    pub struct Adapter: IAdapter, IAdapterExt {
        pub(crate) name: String,
        pub(crate) adapter: dxgi::Adapter,
        pub(crate) context: RefPtr<Context>,
    }
}

impl Adapter {
    fn create_queue<T: ReturnToPool>(
        device: &dx12::Device,
        queue_type: dx12::CommandListType,
    ) -> Option<Queue<T>> {
        let desc = dx12::CommandQueueDesc::builder()
            .queue_type(queue_type)
            .priority(0)
            .build();
        device
            .create_command_queue(&desc)
            .ok()
            .map(|v| Queue::<T>::new(device, v))
    }
}

impl IAdapter for Adapter {
    fn description(&self) -> AdapterDescription {
        AdapterDescription { name: &self.name }
    }

    fn request_device(&self) -> Result<RefPtr<dyn IDevice>, RequestDeviceError> {
        // Create the actual d3d12 device
        let device = dx12::Device::new(&self.adapter, dx12::FeatureLevel::Level_11_0)
            .map_err(|e| anyhow!(e))?;

        // Load our 3 queues
        let queues = Queues {
            general: Adapter::create_queue(&device, dx12::CommandListType::Direct),
            compute: Adapter::create_queue(&device, dx12::CommandListType::Compute),
            transfer: Adapter::create_queue(&device, dx12::CommandListType::Copy),
        };

        // Bundle and return the device
        let device = ref_ptr_init! {
            Device {
                rtv_heap: DescriptorAllocatorCPU::new(device.clone(), dx12::DescriptorHeapType::RenderTargetView),
                dsv_heap: DescriptorAllocatorCPU::new(device.clone(), dx12::DescriptorHeapType::DepthStencilView),
                device: device,
                queues: queues,
                adapter: self.as_ref_ptr(),
            }
        };
        let device: RefPtr<Device> = RefPtr::new(device);
        Ok(device.query_interface().unwrap())
    }
}

pub trait IAdapterExt: IAdapter {
    fn get_raw_handle(&self) -> dxgi::Adapter;
}

impl IAdapterExt for Adapter {
    fn get_raw_handle(&self) -> dxgi::Adapter {
        self.adapter.clone()
    }
}
