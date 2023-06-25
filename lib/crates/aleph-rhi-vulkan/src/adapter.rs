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
use crate::internal::profile::CreateProfile;
use crate::queue::{Queue, QueueInfo};
use aleph_any::{declare_interfaces, AnyArc, AnyWeak};
use aleph_rhi_api::*;
use aleph_rhi_impl_utils::try_clone_value_into_slot;
use ash::vk;
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
            let create_info = vk::DeviceQueueCreateInfo::builder()
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
    const fn is_general_family(family: &vk::QueueFamilyProperties) -> bool {
        /// The mask of queue requirements for a general queue
        const GENERAL_MASK: vk::QueueFlags = vk::QueueFlags::from_raw(
            vk::QueueFlags::GRAPHICS.as_raw()
                | vk::QueueFlags::COMPUTE.as_raw()
                | vk::QueueFlags::TRANSFER.as_raw(),
        );

        // For general
        family.queue_flags.contains(GENERAL_MASK)
    }

    #[inline]
    const fn is_async_compute_family(family: &vk::QueueFamilyProperties) -> bool {
        /// The mask of queue requirements for a compute queue
        const COMPUTE_MASK: vk::QueueFlags = vk::QueueFlags::from_raw(
            vk::QueueFlags::COMPUTE.as_raw() | vk::QueueFlags::TRANSFER.as_raw(),
        );

        // For async compute we specifically want the non graphics queues so check for
        // compute+transfer and no graphics
        family.queue_flags.contains(COMPUTE_MASK)
            && !family.queue_flags.contains(vk::QueueFlags::GRAPHICS)
    }

    #[inline]
    const fn is_dedicated_transfer_family(family: &vk::QueueFamilyProperties) -> bool {
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
        enabled_extensions.push(vk::KhrDynamicRenderingFn::name().as_ptr());

        if self
            .device_info
            .supports_extension_cstr(vk::KhrSwapchainFn::name())
        {
            enabled_extensions.push(vk::KhrSwapchainFn::name().as_ptr());
        }

        if self
            .device_info
            .supports_extension_ptr(vk::KhrSynchronization2Fn::name().as_ptr())
        {
            enabled_extensions.push(vk::KhrSynchronization2Fn::name().as_ptr());
        }

        if self
            .device_info
            .supports_extension_ptr(vk::KhrPortabilitySubsetFn::name().as_ptr())
        {
            enabled_extensions.push(vk::KhrPortabilitySubsetFn::name().as_ptr());
        }

        // Find our general, async compute and transfer queue families
        let queue_families = unsafe {
            self.context
                .instance
                .get_physical_device_queue_family_properties(self.physical_device)
        };
        let found_families = Adapter::get_queue_families(&queue_families);
        let queue_create_infos = found_families.build_create_info_list();

        let enabled_10_features = vk::PhysicalDeviceFeatures::minimum();
        let mut enabled_11_features = vk::PhysicalDeviceVulkan11Features::minimum();
        let mut enabled_12_features = vk::PhysicalDeviceVulkan12Features::minimum();
        let mut dynamic_rendering_features = vk::PhysicalDeviceDynamicRenderingFeatures::minimum();
        let mut portability_features = self.device_info.portability_features.clone();
        let mut synchronization_2_features = self.device_info.synchronization_2_features.clone();

        let device_create_info = vk::DeviceCreateInfo::builder()
            .push_next(&mut enabled_11_features)
            .push_next(&mut enabled_12_features)
            .push_next(&mut dynamic_rendering_features)
            .enabled_features(&enabled_10_features)
            .enabled_extension_names(&enabled_extensions)
            .queue_create_infos(&queue_create_infos);

        let device_create_info = if self
            .device_info
            .supports_extension_cstr(vk::KhrPortabilitySubsetFn::name())
        {
            device_create_info.push_next(&mut portability_features)
        } else {
            device_create_info
        };
        let device_create_info = if self
            .device_info
            .supports_extension_cstr(vk::KhrSynchronization2Fn::name())
        {
            device_create_info.push_next(&mut synchronization_2_features)
        } else {
            device_create_info
        };

        let device = unsafe {
            self.context
                .instance
                .create_device(self.physical_device, &device_create_info, None)
                .map_err(|e| log::error!("Platform Error: {:#?}", e))?
        };

        let dynamic_rendering =
            ash::extensions::khr::DynamicRendering::new(&self.context.instance, &device);

        let swapchain = if self
            .device_info
            .supports_extension_cstr(vk::KhrSwapchainFn::name())
        {
            Some(ash::extensions::khr::Swapchain::new(
                &self.context.instance,
                &device,
            ))
        } else {
            None
        };

        let synchronization_2 = if self
            .device_info
            .supports_extension_ptr(vk::KhrSynchronization2Fn::name().as_ptr())
        {
            Some(ash::extensions::khr::Synchronization2::new(
                &self.context.instance,
                &device,
            ))
        } else {
            None
        };

        let allocator = vma::Allocator::builder()
            .vulkan_api_version(vk::API_VERSION_1_2)
            .build(&self.context.instance, &device, self.physical_device)
            .map_err(|v| log::error!("Platform Error: {:#?}", v))?;

        let device = AnyArc::new_cyclic(move |v| {
            let mut device = Device {
                this: v.clone(),
                adapter: self.this.upgrade().unwrap(),
                context: self.context.clone(),
                device: ManuallyDrop::new(device),
                dynamic_rendering,
                swapchain,
                synchronization_2,
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
    fn build_create_info_list(&self) -> Vec<vk::DeviceQueueCreateInfo> {
        // List to flatten the set of queue create infos into so we can pass it into vkCreateDevice
        let mut queue_create_infos: Vec<vk::DeviceQueueCreateInfo> = Vec::with_capacity(4);

        if let Some(info) = self.general.as_ref() {
            queue_create_infos.push(info.create_info.clone());
        }
        if let Some(info) = self.compute.as_ref() {
            queue_create_infos.push(info.create_info.clone());
        }
        if let Some(info) = self.transfer.as_ref() {
            queue_create_infos.push(info.create_info.clone());
        }

        queue_create_infos
    }

    unsafe fn build_queue_objects(&self, device: &mut Device) {
        let device_loader = &device.device;

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
