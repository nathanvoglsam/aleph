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
use crate::device::{Device, Queues};
use crate::internal::descriptor_allocator_cpu::DescriptorAllocatorCPU;
use crate::internal::descriptor_heap_info::DescriptorHeapInfo;
use crate::internal::in_flight_command_list::ReturnToPool;
use crate::internal::queue::Queue;
use dx12::{dxgi, MessageSeverity};
use interfaces::any::{declare_interfaces, AnyArc, AnyWeak};
use interfaces::anyhow::anyhow;
use interfaces::gpu::{AdapterDescription, IAdapter, IDevice, RequestDeviceError};

pub struct Adapter {
    pub(crate) this: AnyWeak<Self>,
    pub(crate) context: AnyArc<Context>,
    pub(crate) name: String,
    pub(crate) adapter: dxgi::Adapter,
}

declare_interfaces!(Adapter, [IAdapter, IAdapterExt]);

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
    fn upgrade(&self) -> AnyArc<dyn IAdapter> {
        AnyArc::map::<dyn IAdapter, _>(self.this.upgrade().unwrap(), |v| v)
    }

    fn strong_count(&self) -> usize {
        self.this.strong_count()
    }

    fn weak_count(&self) -> usize {
        self.this.weak_count()
    }

    fn description(&self) -> AdapterDescription {
        AdapterDescription { name: &self.name }
    }

    fn request_device(&self) -> Result<AnyArc<dyn IDevice>, RequestDeviceError> {
        // Create the actual d3d12 device
        let device = dx12::Device::new(&self.adapter, dx12::FeatureLevel::Level_11_0)
            .map_err(|e| anyhow!(e))?;

        // Load our 3 queues
        let queues = Queues {
            general: Adapter::create_queue(&device, dx12::CommandListType::Direct),
            compute: Adapter::create_queue(&device, dx12::CommandListType::Compute),
            transfer: Adapter::create_queue(&device, dx12::CommandListType::Copy),
        };

        let debug_message_cookie = if self.context.debug.is_some() {
            // SAFETY: Should be safe but I don't have a proof
            unsafe {
                device
                    .register_message_callback(move |category, severity, id, description| {
                        let level = match severity {
                            MessageSeverity::Corruption => aleph_log::Level::Error,
                            MessageSeverity::Error => aleph_log::Level::Error,
                            MessageSeverity::Warning => aleph_log::Level::Warn,
                            MessageSeverity::Info => aleph_log::Level::Info,
                            MessageSeverity::Message => aleph_log::Level::Info,
                        };

                        aleph_log::log!(level, "[{:?}] [{:?}] {:?}", category, id, description);
                    })
                    .ok()
            }
        } else {
            None
        };

        // Bundle and return the device
        let device = AnyArc::new_cyclic(move |v| Device {
            this: v.clone(),
            _adapter: self.this.upgrade().unwrap(),
            debug_message_cookie,
            descriptor_heap_info: DescriptorHeapInfo::new(&device),
            rtv_heap: DescriptorAllocatorCPU::new(
                device.clone(),
                dx12::DescriptorHeapType::RenderTargetView,
            ),
            dsv_heap: DescriptorAllocatorCPU::new(
                device.clone(),
                dx12::DescriptorHeapType::DepthStencilView,
            ),
            _sampler_heap: DescriptorAllocatorCPU::new(
                device.clone(),
                dx12::DescriptorHeapType::Sampler,
            ),
            device,
            queues,
        });
        Ok(AnyArc::map::<dyn IDevice, _>(device, |v| v))
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
