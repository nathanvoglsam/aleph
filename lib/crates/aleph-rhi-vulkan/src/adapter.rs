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
use crate::device::Device;
use crate::internal::device_info::DeviceInfo;
use crate::queue::{Queue, QueueInfo};
use aleph_any::{declare_interfaces, AnyArc, AnyWeak};
use aleph_rhi_api::*;
use aleph_rhi_impl_utils::try_clone_value_into_slot;
use anyhow::anyhow;
use erupt::{vk, ExtendableFrom};
use std::any::TypeId;
use std::mem::ManuallyDrop;
use vulkan_alloc::vma;

pub struct Adapter {
    pub(crate) this: AnyWeak<Self>,
    pub(crate) context: AnyArc<Context>,
    pub(crate) name: String,
    pub(crate) vendor: AdapterVendor,
    pub(crate) physical_device: vk::PhysicalDevice,
    pub(crate) device_info: DeviceInfo,
}

declare_interfaces!(Adapter, [IAdapter]);

impl IGetPlatformInterface for Adapter {
    unsafe fn __query_platform_interface(&self, target: TypeId, out: *mut ()) -> Option<()> {
        try_clone_value_into_slot::<vk::PhysicalDevice>(&self.physical_device, out, target)
    }
}

impl Adapter {
    #[inline]
    fn get_queue_families(queue_families: &[vk::QueueFamilyProperties]) -> FoundQueueFamilies {
        let mut general = None;
        let mut compute = None;
        let mut transfer = None;

        for (index, family) in queue_families.iter().enumerate() {
            let create_info = vk::DeviceQueueCreateInfoBuilder::new()
                .queue_family_index(index as u32)
                .queue_priorities(&PRIORITIES[0..1]);

            if general.is_none() && Self::is_general_family(family) {
                general = Some(QueueFamily {
                    queue_info: QueueInfo::new(index as u32, family),
                    create_info,
                });
                continue;
            }

            if compute.is_none() && Self::is_async_compute_family(family) {
                compute = Some(QueueFamily {
                    queue_info: QueueInfo::new(index as u32, family),
                    create_info,
                });
                continue;
            }

            if transfer.is_none() && Self::is_dedicated_transfer_family(family) {
                transfer = Some(QueueFamily {
                    queue_info: QueueInfo::new(index as u32, family),
                    create_info,
                });
                continue;
            }
        }

        FoundQueueFamilies {
            general,
            compute,
            transfer,
        }
    }

    #[inline]
    fn is_general_family(family: &vk::QueueFamilyProperties) -> bool {
        /// The mask of queue requirements for a general queue
        const GENERAL_MASK: vk::QueueFlags = vk::QueueFlags::from_bits_truncate(
            vk::QueueFlags::GRAPHICS.bits()
                | vk::QueueFlags::COMPUTE.bits()
                | vk::QueueFlags::TRANSFER.bits(),
        );

        // For general
        family.queue_flags.contains(GENERAL_MASK)
    }

    #[inline]
    fn is_async_compute_family(family: &vk::QueueFamilyProperties) -> bool {
        /// The mask of queue requirements for a compute queue
        const COMPUTE_MASK: vk::QueueFlags = vk::QueueFlags::from_bits_truncate(
            vk::QueueFlags::COMPUTE.bits() | vk::QueueFlags::TRANSFER.bits(),
        );

        // For async compute we specifically want the non graphics queues so check for
        // compute+transfer and no graphics
        family.queue_flags.contains(COMPUTE_MASK)
            && !family.queue_flags.contains(vk::QueueFlags::GRAPHICS)
    }

    #[inline]
    fn is_dedicated_transfer_family(family: &vk::QueueFamilyProperties) -> bool {
        /// The mask of queue requirements for a transfer queue
        const TRANSFER_MASK: vk::QueueFlags = vk::QueueFlags::TRANSFER;

        // For transfer we specifically want a queue that only supports transfer operations so check
        // for transfer and nothing else
        family.queue_flags.contains(TRANSFER_MASK)
            && !family.queue_flags.contains(vk::QueueFlags::GRAPHICS)
            && !family.queue_flags.contains(vk::QueueFlags::COMPUTE)
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
        let mut enabled_extensions = Vec::with_capacity(4);
        enabled_extensions.push(vk::KHR_SWAPCHAIN_EXTENSION_NAME);
        enabled_extensions.push(vk::KHR_DYNAMIC_RENDERING_EXTENSION_NAME);

        if self
            .device_info
            .supports_extension_ptr(vk::KHR_SYNCHRONIZATION_2_EXTENSION_NAME)
        {
            enabled_extensions.push(vk::KHR_SYNCHRONIZATION_2_EXTENSION_NAME);
        }

        if self
            .device_info
            .supports_extension_ptr(vk::KHR_PORTABILITY_SUBSET_EXTENSION_NAME)
        {
            enabled_extensions.push(vk::KHR_PORTABILITY_SUBSET_EXTENSION_NAME);
        }

        // Find our general, async compute and transfer queue families
        let queue_families = unsafe {
            self.context
                .instance_loader
                .get_physical_device_queue_family_properties(self.physical_device, None)
        };
        let found_families = Adapter::get_queue_families(&queue_families);
        let queue_create_infos = found_families.build_create_info_list();

        let enabled_10_features = self.device_info.features_10.clone();
        let mut enabled_11_features = self.device_info.features_11.clone();
        let mut enabled_12_features = self.device_info.features_12.clone();
        let mut dynamic_rendering_features = self.device_info.dynamic_rendering_features.clone();
        let mut portability_features = self.device_info.portability_features.clone();
        let mut synchronization_2_features = self.device_info.synchronization_2_features.clone();

        let device_create_info = vk::DeviceCreateInfoBuilder::new()
            .extend_from(&mut enabled_11_features)
            .extend_from(&mut enabled_12_features)
            .extend_from(&mut dynamic_rendering_features)
            .enabled_features(&enabled_10_features)
            .enabled_extension_names(&enabled_extensions)
            .queue_create_infos(&queue_create_infos);

        let device_create_info = if self
            .device_info
            .supports_extension_ptr(vk::KHR_PORTABILITY_SUBSET_EXTENSION_NAME)
        {
            device_create_info.extend_from(&mut portability_features)
        } else {
            device_create_info
        };
        let device_create_info = if self
            .device_info
            .supports_extension_ptr(vk::KHR_SYNCHRONIZATION_2_EXTENSION_NAME)
        {
            device_create_info.extend_from(&mut synchronization_2_features)
        } else {
            device_create_info
        };

        let device_loader = unsafe {
            erupt::DeviceLoaderBuilder::new()
                .build(
                    &self.context.instance_loader,
                    self.physical_device,
                    &device_create_info,
                )
                .map_err(|e| anyhow!(e))?
        };

        let allocator = vma::Allocator::builder()
            .vulkan_api_version(vk::API_VERSION_1_2)
            .build(
                &self.context.instance_loader,
                &device_loader,
                self.physical_device,
            )
            .map_err(|v| anyhow!(v))?;

        let device = AnyArc::new_cyclic(move |v| {
            let mut device = Device {
                this: v.clone(),
                adapter: self.this.upgrade().unwrap(),
                context: self.context.clone(),
                device_loader: ManuallyDrop::new(device_loader),
                allocator: ManuallyDrop::new(allocator),
                general_queue: None,
                compute_queue: None,
                transfer_queue: None,
            };

            unsafe { found_families.build_queue_objects(&mut device) };

            device
        });

        Ok(AnyArc::map::<dyn IDevice, _>(device, |v| v))
    }
}

// We're just going have a pre-allocated chunk of priorities bigger than we're ever going to
// need to slice from to send to vulkan. Saves allocating when we don't need to
static PRIORITIES: [f32; 128] = [1.0f32; 128];

struct FoundQueueFamilies<'a> {
    general: Option<QueueFamily<'a>>,
    compute: Option<QueueFamily<'a>>,
    transfer: Option<QueueFamily<'a>>,
}

impl<'a> FoundQueueFamilies<'a> {
    fn build_create_info_list(&self) -> Vec<vk::DeviceQueueCreateInfoBuilder> {
        // List to flatten the set of queue create infos into so we can pass it into vkCreateDevice
        let mut queue_create_infos = Vec::with_capacity(4);

        if let Some(info) = self.general.as_ref() {
            queue_create_infos.push(info.create_info);
        }
        if let Some(info) = self.compute.as_ref() {
            queue_create_infos.push(info.create_info);
        }
        if let Some(info) = self.transfer.as_ref() {
            queue_create_infos.push(info.create_info);
        }

        queue_create_infos
    }

    unsafe fn build_queue_objects(&self, device: &mut Device) {
        let device_loader = &device.device_loader;

        if let Some(info) = self.general.as_ref() {
            let handle = device_loader.get_device_queue(info.queue_info.family_index, 0);
            let queue = Queue::new(handle, device, QueueType::General, info.queue_info.clone());
            device.general_queue = Some(queue);
        }
        if let Some(info) = self.compute.as_ref() {
            let handle = device_loader.get_device_queue(info.queue_info.family_index, 0);
            let queue = Queue::new(handle, device, QueueType::Compute, info.queue_info.clone());
            device.compute_queue = Some(queue);
        }
        if let Some(info) = self.transfer.as_ref() {
            let handle = device_loader.get_device_queue(info.queue_info.family_index, 0);
            let queue = Queue::new(handle, device, QueueType::Transfer, info.queue_info.clone());
            device.transfer_queue = Some(queue);
        }
    }
}

struct QueueFamily<'a> {
    queue_info: QueueInfo,
    create_info: vk::DeviceQueueCreateInfoBuilder<'a>,
}
