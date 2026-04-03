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
use std::ffi::CStr;
use std::mem::ManuallyDrop;
use std::sync::{Arc, Weak};

use aleph_alloc::BVec;
use aleph_alloc::instrumentation::IAllocationCategory;
use aleph_gpu_allocator::GpuAllocator;
use aleph_rhi_api::*;
use aleph_rhi_impl_utils::object_counter::ObjectCounter;
use aleph_rhi_impl_utils::{Rhi, RhiSystem, try_clone_value_into_slot};
use ash::vk;

use crate::context::Context;
use crate::device::{CommandListPool, Device};
use crate::internal::allocation_callbacks::GLOBAL;
use crate::internal::device_info::DeviceInfo;
use crate::internal::semaphore_pool::SemaphorePool;
use crate::queue::{Queue, QueueInfo};

pub struct Adapter {
    pub(crate) _this: Weak<Self>,
    pub(crate) context: Arc<Context>,
    pub(crate) name: String,
    pub(crate) vendor: AdapterVendor,
    pub(crate) physical_device: vk::PhysicalDevice,
    pub(crate) device_info: DeviceInfo,
}

impl IGetPlatformInterface for Adapter {
    unsafe fn __query_platform_interface(&self, target: TypeId, out: *mut ()) -> Option<()> {
        unsafe {
            try_clone_value_into_slot::<vk::PhysicalDevice>(&self.physical_device, out, target)
        }
    }
}

impl Adapter {
    fn select_queue_families(
        queue_families: &[vk::QueueFamilyProperties],
    ) -> (
        [Option<QueueInfo>; 3],
        BVec<vk::DeviceQueueCreateInfo<'_>, RhiSystem>,
    ) {
        // Produce a parallel list from 'queue_families'. Stores how many queues we've allocated
        // from each family.
        let mut family_counts = BVec::new_in(RhiSystem::default());
        family_counts.extend((0..queue_families.len()).map(|_| 0u32));

        // Select a queue family for each RHI queue.
        //
        // The set of queues available from Vulkan will vary from device to device, and even on the
        // same device with different drivers. Some devices will have clear and obvious queues for
        // each use case. Some will have 16 general queues.
        //
        // To that end each function will try to find the most specific queue possible before
        // attempting to fall back to less and less specific queues. We could end up creating 3
        // general queues if we have to. The underlying queue family is not exposed in the RHI.
        let queues = [
            Adapter::select_general_queue_family(&queue_families, &mut family_counts),
            Adapter::select_compute_queue_family(&queue_families, &mut family_counts),
            Adapter::select_transfer_queue_family(&queue_families, &mut family_counts),
        ];

        // Generate the 'vk::DeviceQueueCreateInfo' we pass to Vulkan.
        let mut queue_create_infos = BVec::with_capacity_in(3, RhiSystem::default());
        for (index, &count) in family_counts.iter().enumerate() {
            // Simply add an entry for any family that isn't empty. Each family must appear only
            // once in the list.
            if count > 0 {
                queue_create_infos.push(
                    vk::DeviceQueueCreateInfo::default()
                        .queue_family_index(u32::try_from(index).unwrap())
                        .queue_priorities(&PRIORITIES[0..count as usize]),
                );
            }
        }
        (queues, queue_create_infos)
    }

    fn find_queue_family(
        queue_families: &[vk::QueueFamilyProperties],
        family_counts: &mut [u32],
        filter: impl Fn(&vk::QueueFamilyProperties) -> bool,
    ) -> Option<QueueInfo> {
        for ((family_index, family), queue_count) in
            queue_families.iter().enumerate().zip(family_counts)
        {
            let queue_index = *queue_count;
            if filter(family) {
                if family.queue_count > queue_index {
                    // Consume a queue from the selected family by incrementing 'queue_count'.
                    *queue_count = queue_count.checked_add(1).unwrap();

                    log::info!(
                        "Selected additional queue from family '{}' ({:?}).",
                        family_index,
                        family.queue_flags
                    );

                    return Some(QueueInfo::new(family_index as u32, queue_index, family));
                } else {
                    log::warn!(
                        "Queue family '{}' is exhausted. Failed to make another queue.",
                        family_index
                    );
                }
            }
        }
        None
    }

    fn select_general_queue_family(
        queue_families: &[vk::QueueFamilyProperties],
        family_counts: &mut [u32],
    ) -> Option<QueueInfo> {
        log::info!("Searching for 'QueueType::General' queue family.");

        // Vulkan guarantees at least one queue with these properties. graphics + compute +
        // transfer.
        if let Some(v) =
            Self::find_queue_family(queue_families, family_counts, Self::is_general_family)
        {
            return Some(v);
        }

        log::error!("Failed to find a queue for 'QueueType::General'.");

        None
    }

    fn select_compute_queue_family(
        queue_families: &[vk::QueueFamilyProperties],
        family_counts: &mut [u32],
    ) -> Option<QueueInfo> {
        log::info!("Searching for 'QueueType::Compute' queue family.");

        // First we look for a dedicated async compute queue by finding compute + transfer +
        // !graphics.
        if let Some(v) =
            Self::find_queue_family(queue_families, family_counts, Self::is_async_compute_family)
        {
            return Some(v);
        }

        log::warn!(
            "Failed to find a dedicated async-compute queue for 'QueueType::Compute'. Trying fallback."
        );

        // If there's no specific async compute queue we try make a second general queue and use
        // that for the RHI's "compute" queue.
        if let Some(v) =
            Self::find_queue_family(queue_families, family_counts, Self::is_general_family)
        {
            return Some(v);
        }

        log::warn!("Failed to find a fallback general queue for 'QueueType::Compute'.");

        None
    }

    fn select_transfer_queue_family(
        queue_families: &[vk::QueueFamilyProperties],
        family_counts: &mut [u32],
    ) -> Option<QueueInfo> {
        log::info!("Searching for 'QueueType::Transfer' queue family.");

        // Some drivers have dedicated queues with only the transfer capability is available. We
        // want that one as often they're connected to DMA engines.
        if let Some(v) =
            Self::find_queue_family(queue_families, family_counts, Self::is_transfer_family)
        {
            return Some(v);
        }

        log::warn!(
            "Failed to find a dedicated transfer queue for 'QueueType::Transfer'. Trying fallback."
        );

        // If there's no transfer only queue we try making a second async compute queue instead.
        // This will always
        if let Some(v) =
            Self::find_queue_family(queue_families, family_counts, Self::is_async_compute_family)
        {
            return Some(v);
        }

        log::warn!(
            "Failed to find a fallback async compute queue for 'QueueType::Transfer'. Trying fallback."
        );

        if let Some(v) =
            Self::find_queue_family(queue_families, family_counts, Self::is_general_family)
        {
            return Some(v);
        }

        log::warn!("Failed to find a fallback general queue for 'QueueType::Transfer'.");

        None
    }

    const fn is_general_family(family: &vk::QueueFamilyProperties) -> bool {
        family.queue_flags.contains(vk::QueueFlags::GRAPHICS)
            && family.queue_flags.contains(vk::QueueFlags::COMPUTE)
            && family.queue_flags.contains(vk::QueueFlags::TRANSFER)
    }

    const fn is_async_compute_family(family: &vk::QueueFamilyProperties) -> bool {
        // For async compute we specifically want the non graphics queues so check for
        // compute+transfer and no graphics
        family.queue_flags.contains(vk::QueueFlags::COMPUTE)
            && family.queue_flags.contains(vk::QueueFlags::TRANSFER)
            && !family.queue_flags.contains(vk::QueueFlags::GRAPHICS)
    }

    const fn is_transfer_family(family: &vk::QueueFamilyProperties) -> bool {
        // For transfer we specifically want a queue that only supports transfer operations so check
        // for transfer and nothing else
        family.queue_flags.contains(vk::QueueFlags::TRANSFER)
            && !family.queue_flags.contains(vk::QueueFlags::GRAPHICS)
            && !family.queue_flags.contains(vk::QueueFlags::COMPUTE)
    }

    unsafe fn build_queue_objects(queues: &[Option<QueueInfo>; 3], device: &mut Device) {
        unsafe {
            let device_loader = &device.device;

            if let Some(info) = queues[0].as_ref() {
                let family = info.family_index;
                let index = info.queue_index;

                log::info!("Found queue family '{family}[{index}]' for 'QueueType::General'",);

                let handle = device_loader.get_device_queue(family, index);
                let queue = Queue::new(handle, device, QueueType::General, info.clone());
                device.general_queue = Some(queue);
            } else {
                log::error!("No queue family found for 'QueueType::General'");
            }

            if let Some(info) = queues[1].as_ref() {
                let family = info.family_index;
                let index = info.queue_index;
                log::info!("Found queue family '{family}[{index}]' for 'QueueType::Compute'",);

                let handle = device_loader.get_device_queue(family, index);
                let queue = Queue::new(handle, device, QueueType::Compute, info.clone());
                device.compute_queue = Some(queue);
            } else {
                log::warn!("No queue family found for 'QueueType::Compute'");
            }

            if let Some(info) = queues[2].as_ref() {
                let family = info.family_index;
                let index = info.queue_index;

                log::info!("Found queue family '{family}[{index}]' for 'QueueType::Transfer'",);

                let handle = device_loader.get_device_queue(family, index);
                let queue = Queue::new(handle, device, QueueType::Transfer, info.clone());
                device.transfer_queue = Some(queue);
            } else {
                log::warn!("No queue family found for 'QueueType::Transfer'");
            }
        }
    }

    fn inner_request_device(&self) -> Result<Arc<dyn IDevice>, RequestDeviceError> {
        let DeviceInfo {
            extensions: minimum_extensions,
            ..
        } = DeviceInfo::minimum();
        let mut enabled_extensions: Vec<_> = minimum_extensions
            .iter()
            .map(|v| v.extension_name.as_ptr())
            .collect();

        let is_supported = |v: &CStr| self.device_info.supports_extension_cstr(v);
        let mut enable_if_supported = |wanted: &CStr| {
            if is_supported(wanted) {
                enabled_extensions.push(wanted.as_ptr());
                true
            } else {
                false
            }
        };

        enable_if_supported(ash::khr::swapchain::NAME);
        enable_if_supported(ash::khr::portability_subset::NAME);

        // Find our general, async compute and transfer queue families
        let queue_families = unsafe {
            self.context
                .instance
                .get_physical_device_queue_family_properties(self.physical_device)
        };
        let (queues, queue_create_infos) = Self::select_queue_families(&queue_families);

        let DeviceInfo {
            features_10,
            mut features_11,
            mut features_12,
            mut features_13,
            ..
        } = DeviceInfo::minimum();
        let mut device_create_info = vk::DeviceCreateInfo::default()
            .push_next(&mut features_11)
            .push_next(&mut features_12)
            .push_next(&mut features_13)
            .enabled_features(&features_10)
            .enabled_extension_names(&enabled_extensions)
            .queue_create_infos(&queue_create_infos);

        let DeviceInfo {
            mut portability_features,
            ..
        } = self.device_info.clone();
        if is_supported(ash::khr::portability_subset::NAME) {
            device_create_info = device_create_info.push_next(&mut portability_features)
        }

        let device = unsafe {
            self.context
                .instance
                .create_device(self.physical_device, &device_create_info, GLOBAL)
                .inspect_err(|e| log::error!("Platform Error: {:#?}", e))
                .map_err(|_| RequestDeviceError::Platform)?
        };

        let push_descriptor =
            ash::khr::push_descriptor::Device::new(&self.context.instance, &device);

        let swapchain = if is_supported(ash::khr::swapchain::NAME) {
            Some(ash::khr::swapchain::Device::new(
                &self.context.instance,
                &device,
            ))
        } else {
            None
        };

        let debug_loader = if self.context.debug_loader.is_some() {
            Some(ash::ext::debug_utils::Device::new(
                &self.context.instance,
                &device,
            ))
        } else {
            None
        };

        Ok(Arc::new_cyclic(move |v| {
            let queues = queues;
            let mut device = Device {
                _this: v.clone(),
                adapter: self._this.upgrade().unwrap(),
                context: self.context.clone(),
                device: ManuallyDrop::new(device),
                push_descriptor,
                swapchain,
                debug_loader,
                allocator: None,
                general_queue: None,
                compute_queue: None,
                transfer_queue: None,
                command_list_pool: CommandListPool::new(),
                object_counter: ObjectCounter::new(),
                swap_semaphore_pool: SemaphorePool::new(),
            };

            let allocator = ManuallyDrop::new(GpuAllocator::new(&device));
            device.allocator = Some(allocator);

            unsafe { Self::build_queue_objects(&queues, &mut device) };

            device
        }))
    }
}

impl IAdapter for Adapter {
    fn upgrade(&self) -> Arc<dyn IAdapter> {
        self._this.upgrade().unwrap()
    }

    fn strong_count(&self) -> usize {
        self._this.strong_count()
    }

    fn weak_count(&self) -> usize {
        self._this.weak_count()
    }

    fn description(&self) -> AdapterDescription<'_> {
        AdapterDescription {
            name: &self.name,
            vendor: self.vendor,
        }
    }

    fn request_device(&self) -> Result<Arc<dyn IDevice>, RequestDeviceError> {
        Rhi::with(|| self.inner_request_device())
    }
}

// We're just going have a pre-allocated chunk of priorities bigger than we're ever going to
// need to slice from to send to vulkan. Saves allocating when we don't need to
static PRIORITIES: [f32; 128] = [1.0f32; 128];
