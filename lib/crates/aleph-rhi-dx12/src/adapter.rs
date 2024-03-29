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
use std::ops::Deref;

use aleph_any::{declare_interfaces, AnyArc, AnyWeak};
use aleph_rhi_api::*;
use aleph_rhi_impl_utils::try_clone_value_into_slot;
use parking_lot::Mutex;
use windows::core::CanInto;
use windows::Win32::Graphics::Direct3D::*;
use windows::Win32::Graphics::Direct3D12::*;
use windows::Win32::Graphics::Dxgi::*;

use crate::context::Context;
use crate::device::Device;
use crate::internal::conv::queue_type_to_dx12;
use crate::internal::create_device::create_device;
use crate::internal::descriptor_heap_info::DescriptorHeapInfo;
use crate::internal::descriptor_heaps::DescriptorHeaps;
use crate::internal::register_message_callback::{
    category_name, device_register_message_callback, message_id_name,
};
use crate::queue::Queue;

pub struct Adapter {
    pub(crate) this: AnyWeak<Self>,
    pub(crate) context: AnyArc<Context>,
    pub(crate) name: String,
    pub(crate) vendor: AdapterVendor,
    pub(crate) adapter: Mutex<IDXGIAdapter1>,
}

/// # Safety
///
/// This is safe as `IDXGIAdapter1` can be sent across threads, making it 'send'. All access is
/// through a mutex, making it 'sync'.
unsafe impl Send for Adapter {}
unsafe impl Sync for Adapter {}

declare_interfaces!(Adapter, [IAdapter]);

impl IGetPlatformInterface for Adapter {
    unsafe fn __query_platform_interface(&self, target: TypeId, out: *mut ()) -> Option<()> {
        try_clone_value_into_slot(self.adapter.lock().deref(), out, target)
    }
}

impl Adapter {
    fn create_queue(
        device: &impl CanInto<ID3D12Device>,
        queue_type: QueueType,
        debug: bool,
    ) -> Option<AnyArc<Queue>> {
        let device = device.can_into();
        unsafe {
            let desc = D3D12_COMMAND_QUEUE_DESC {
                Type: queue_type_to_dx12(queue_type),
                Priority: 0,
                Flags: Default::default(),
                NodeMask: 0,
            };
            device
                .CreateCommandQueue(&desc)
                .ok()
                .map(|v| Queue::new(device, queue_type, debug, v))
        }
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
        AdapterDescription {
            name: &self.name,
            vendor: self.vendor,
        }
    }

    fn request_device(&self) -> Result<AnyArc<dyn IDevice>, RequestDeviceError> {
        let adapter = self.adapter.lock();

        // Create the actual d3d12 device
        let device = create_device(adapter.deref(), D3D_FEATURE_LEVEL_11_0)
            .map_err(|e| log::error!("Platform Error: {:#?}", e))?;

        let debug_queue = self.context.debug.is_some() && pix::is_library_available();

        // Load our 3 queues
        let general_queue = Adapter::create_queue(&device, QueueType::General, debug_queue);
        let compute_queue = Adapter::create_queue(&device, QueueType::Compute, debug_queue);
        let transfer_queue = Adapter::create_queue(&device, QueueType::Transfer, debug_queue);

        let debug_message_cookie = if self.context.debug.is_some() {
            // SAFETY: Should be safe but I don't have a proof
            unsafe {
                device_register_message_callback(
                    &device,
                    move |category, severity, id, description| {
                        let category = category_name(&category).unwrap_or("Unknown Category");
                        let level = match severity {
                            D3D12_MESSAGE_SEVERITY_CORRUPTION => log::Level::Error,
                            D3D12_MESSAGE_SEVERITY_ERROR => log::Level::Error,
                            D3D12_MESSAGE_SEVERITY_WARNING => log::Level::Warn,
                            D3D12_MESSAGE_SEVERITY_INFO => log::Level::Info,
                            D3D12_MESSAGE_SEVERITY_MESSAGE => log::Level::Info,
                            _ => log::Level::Info,
                        };
                        let id = message_id_name(&id).unwrap_or("Unknown ID");

                        log::log!(level, "[{:?}] [{:?}] {:?}", category, id, description);

                        // Break on debugger, if one is attached (assuming the platform supports the behavior)
                        debug_break();
                    },
                )
                .ok()
            }
        } else {
            None
        };

        let descriptor_heaps =
            DescriptorHeaps::new(&device).map_err(|e| log::error!("Platform Error: {:#?}", e))?;

        // Bundle and return the device
        let device = AnyArc::new_cyclic(move |v| Device {
            this: v.clone(),
            _context: self.context.clone(),
            _adapter: self.this.upgrade().unwrap(),
            debug_message_cookie,
            descriptor_heap_info: DescriptorHeapInfo::new(&device),
            descriptor_heaps,
            device,
            general_queue,
            compute_queue,
            transfer_queue,
        });
        Ok(AnyArc::map::<dyn IDevice, _>(device, |v| v))
    }
}

#[cfg(target_os = "windows")]
#[inline(always)]
fn debug_break() {
    unsafe {
        use aleph_windows::Win32::System::Diagnostics::Debug::{DebugBreak, IsDebuggerPresent};

        let debugger_present: bool = IsDebuggerPresent().as_bool();
        if debugger_present {
            DebugBreak();
        }
    }
}

#[cfg(not(target_os = "windows"))]
#[inline(always)]
fn debug_break() {}
