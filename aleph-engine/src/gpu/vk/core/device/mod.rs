//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use crate::gpu::vk::core::{
    GPUInfo, Instance, QueueFamily, QueueFamilyType, SwapChainSupport, VendorID,
};
use crate::gpu::vk::pipeline_cache::PipelineCache;
use erupt::extensions::khr_surface::{KhrSurfaceInstanceLoaderExt, SurfaceKHR};
use erupt::vk1_0::{
    DeviceCreateInfoBuilder, DeviceQueueCreateInfoBuilder, PhysicalDevice, PhysicalDeviceFeatures,
    PhysicalDeviceType, Queue, QueueFlags, Vk10DeviceLoaderExt, Vk10InstanceLoaderExt, TRUE,
};
use erupt::{DeviceLoader, InstanceLoader};
use parking_lot::Mutex;
use std::ffi::CStr;
use std::sync::Arc;

///
/// A builder wrapper for constructing a vulkan device
///
pub struct DeviceBuilder {}

impl DeviceBuilder {
    ///
    /// Gets a new device builder
    ///
    pub fn new() -> Self {
        Self {}
    }

    pub fn build(self, instance: &Arc<Instance>) -> Arc<Device> {
        log::trace!("Initializing Vulkan device");
        let instance_loader = instance.loader();
        let surface = instance.surface();

        let features = PhysicalDeviceFeatures::default();
        let physical_device = Self::select_device(&instance_loader, &features, surface)
            .expect("Failed to select a physical device");

        log::trace!("Checking swapchain support");
        let swapchain_support =
            Self::get_swapchain_support(&instance_loader, physical_device, surface);
        if swapchain_support.formats.is_empty() {
            panic!("No available swapchain formats");
        }
        if swapchain_support.present_modes.is_empty() {
            panic!("No available present modes");
        }

        log::trace!("Getting queue families");
        let queue_families = Self::get_queue_families(&instance_loader, physical_device, surface);

        log::trace!("Getting GPU info");
        let device_props =
            unsafe { instance_loader.get_physical_device_properties(physical_device, None) };

        //let extension_props = unsafe {
        //    instance_loader.enumerate_device_extension_properties(physical_device, None, None)
        //};

        let vendor_id = VendorID::from_raw(device_props.vendor_id);
        let device_name = device_props.device_name.as_ptr();
        let device_name = unsafe { CStr::from_ptr(device_name) };
        let device_name = device_name.to_str().unwrap().to_string();
        let api_version_major = erupt::version_major(device_props.api_version);
        let api_version_minor = erupt::version_minor(device_props.api_version);
        let api_version_patch = erupt::version_patch(device_props.api_version);
        let info = GPUInfo {
            vendor_id,
            device_name,
            api_version_major,
            api_version_minor,
            api_version_patch,
        };

        let enabled_features = PhysicalDeviceFeatures::default();

        let mut enabled_extensions = Vec::new();
        enabled_extensions.push(erupt::extensions::khr_swapchain::KHR_SWAPCHAIN_EXTENSION_NAME);

        // We're just going have a pre-allocated chunk of priorities bigger than we're ever going to
        // need to slice from to send to vulkan. Saves allocating when we don't need to
        static PRIORITIES: [f32; 128] = [1.0f32; 128];

        // Find a general queue. We guarantee a general queue will exist for use in the Aleph
        // renderer
        let mut general_queue = None;
        let mut general_family = None;
        for family in queue_families.iter() {
            if family.is_general() {
                let info = DeviceQueueCreateInfoBuilder::new()
                    .queue_family_index(family.index)
                    .queue_priorities(&PRIORITIES[0..1]);
                general_queue = Some(info);
                general_family = Some(family.clone());
                break;
            }
        }
        let general_queue = general_queue.expect("Failed to find a general queue family");
        let general_family = general_family.expect("Failed to find a general queue family");

        // Find an async compute queue if there is one
        let mut compute_queue = None;
        let mut compute_family = None;
        for family in queue_families.iter() {
            if family.is_async_compute() {
                let info = DeviceQueueCreateInfoBuilder::new()
                    .queue_family_index(family.index)
                    .queue_priorities(&PRIORITIES[0..1]);
                compute_queue = Some(info);
                compute_family = Some(family.clone());
                break;
            }
        }

        // Find a transfer queue if there is one
        let mut transfer_queue = None;
        let mut transfer_family = None;
        for family in queue_families.iter() {
            if family.is_transfer() {
                let info = DeviceQueueCreateInfoBuilder::new()
                    .queue_family_index(family.index)
                    .queue_priorities(&PRIORITIES[0..1]);
                transfer_queue = Some(info);
                transfer_family = Some(family.clone());
                break;
            }
        }

        // Build the list of queues to create
        let mut queue_create_infos = Vec::new();
        queue_create_infos.push(general_queue);

        // Create an async compute queue if we can
        if compute_queue.is_some() {
            queue_create_infos.push(compute_queue.unwrap());
        }

        // Create a transfer queue if we can
        if transfer_queue.is_some() {
            queue_create_infos.push(transfer_queue.unwrap());
        }

        log::trace!("Creating Vulkan device");
        let device_create_info = DeviceCreateInfoBuilder::new()
            .enabled_features(&enabled_features)
            .enabled_extension_names(&enabled_extensions)
            .queue_create_infos(&queue_create_infos);
        let device = unsafe {
            instance_loader
                .create_device(physical_device, &device_create_info, None, None)
                .expect("Failed to create vulkan device")
        };

        log::trace!("Loading device functions");
        let mut device_loader =
            DeviceLoader::new(&instance_loader, device).expect("Failed to create device loader");
        device_loader
            .load_vk1_0()
            .expect("Failed to load vulkan device functions");

        log::trace!("Loading general queue");
        let general_queue =
            unsafe { device_loader.get_device_queue(general_queue.queue_family_index, 0, None) };

        let compute_queue = compute_queue.map(|queue| {
            log::trace!("Loading async compute queue");
            unsafe { device_loader.get_device_queue(queue.queue_family_index, 0, None) }
        });

        let transfer_queue = transfer_queue.map(|queue| {
            log::trace!("Loading transfer queue");
            unsafe { device_loader.get_device_queue(queue.queue_family_index, 0, None) }
        });

        log::trace!("Loading functions for VK_KHR_swapchain");
        device_loader
            .load_khr_swapchain()
            .expect("Failed to load VK_KHR_swapchain functions");

        let device_loader = Arc::new(device_loader);

        let deferred_destruction = Mutex::new(Vec::new());

        let device = Device {
            info,
            physical_device,
            device_loader,
            general_queue,
            general_family,
            compute_queue,
            compute_family,
            transfer_queue,
            transfer_family,
            instance: instance.clone(),
            deferred_destruction,
        };

        PipelineCache::init(&device);

        Arc::new(device)
    }

    ///
    /// Chose the best available physical device
    ///
    fn select_device(
        instance: &InstanceLoader,
        features: &PhysicalDeviceFeatures,
        surface: SurfaceKHR,
    ) -> Option<PhysicalDevice> {
        let devices = unsafe {
            instance
                .enumerate_physical_devices(None)
                .expect("Failed to enumerate vulkan devices")
        };
        let mut scores: Vec<(PhysicalDevice, i32)> = Vec::new();

        'device_loop: for device in devices.iter() {
            let score = (*device, 0i32);

            scores.push(score);

            let score = scores.last_mut().unwrap();

            let (props, feats, extns) = unsafe {
                let props = instance.get_physical_device_properties(*device, None);
                let feats = instance.get_physical_device_features(*device, None);
                let extns = instance
                    .enumerate_device_extension_properties(*device, None, None)
                    .expect("Failed to list extension properties");
                (props, feats, extns)
            };

            //let _name = unsafe { CStr::from_ptr(props.device_name.as_ptr()).to_str().unwrap() };

            // Prioritize NVIDIA or AMD as they'll almost always be the fastest GPU available
            // TODO: Update this when Intel Xe is less of a mystery
            let vendor = VendorID::from_raw(props.vendor_id);
            if vendor == VendorID::AMD || vendor == VendorID::NVIDIA {
                score.1 += 10000;
            }

            // Get the name of the VK_KHR_surface extension as a rusty &str
            let khr_surface_name = erupt::extensions::khr_surface::KHR_SURFACE_EXTENSION_NAME;
            let khr_surface_name = unsafe { CStr::from_ptr(khr_surface_name) };
            let khr_surface_name = khr_surface_name.to_str().unwrap();
            for e in extns.iter() {
                // Get the name of the extension we're currently checking as a &str
                // May panic but shouldn't ever as the stringss will just be ASCII
                let current_name = e.extension_name.as_ptr();
                let current_name = unsafe { CStr::from_ptr(current_name) };
                let current_name = current_name.to_str().unwrap();

                if current_name == khr_surface_name {
                    score.1 = -1_000_000;
                    continue 'device_loop;
                }
            }

            let swap_support = Self::get_swapchain_support(instance, *device, surface);

            if swap_support.present_modes.is_empty() {
                score.1 = -1_000_000;
                continue;
            }

            if swap_support.formats.is_empty() {
                score.1 = -1_000_000;
                continue;
            }

            // This should never happen but always good to be safe
            if props.api_version < erupt::make_version(1, 0, 0) {
                score.1 = -100_000;
                continue;
            }

            if props.device_type == PhysicalDeviceType::DISCRETE_GPU {
                score.1 += 10000;
            } else if props.device_type == PhysicalDeviceType::INTEGRATED_GPU {
                score.1 += 1000;
            }

            // Tesselation shaders are very powerful so we would like to weight it pretty high
            if features.tessellation_shader == TRUE && feats.tessellation_shader == TRUE {
                score.1 += 3000
            }

            // Geometry shaders are pretty crap but may as well check for them
            if features.geometry_shader == TRUE && feats.geometry_shader == TRUE {
                score.1 += 3000
            }
        }

        let mut final_device = (PhysicalDevice::null(), -100_000_000i32);

        for score in scores.iter() {
            if score.0 != PhysicalDevice::null() && score.1 > final_device.1 {
                final_device = *score;
            }
        }

        if final_device.0 == PhysicalDevice::null() && final_device.1 <= 0 {
            return None;
        }

        Some(final_device.0)
    }

    ///
    /// Returns whether the device supports the surface in use by ctx and additional information
    /// about the devices swapchain abilities
    ///
    /// Returns Some(SwapChainSupport) if can present to surface and None if the device can not present
    /// to the surface
    ///
    fn get_swapchain_support(
        instance: &InstanceLoader,
        physical_device: PhysicalDevice,
        surface: SurfaceKHR,
    ) -> SwapChainSupport {
        let capabilities = unsafe {
            instance
                .get_physical_device_surface_capabilities_khr(physical_device, surface, None)
                .expect("Failed to retrieve surface capabilities")
        };
        let formats = unsafe {
            instance
                .get_physical_device_surface_formats_khr(physical_device, surface, None)
                .expect("Failed to retrieve supported surface formats")
        };
        let present_modes = unsafe {
            instance
                .get_physical_device_surface_present_modes_khr(physical_device, surface, None)
                .expect("Failed to retrieve support present modes")
        };

        SwapChainSupport {
            capabilities,
            formats,
            present_modes,
        }
    }

    ///
    /// Internal function for setting up the list of queue indices
    ///
    fn get_queue_families(
        instance: &InstanceLoader,
        physical_device: PhysicalDevice,
        surface: SurfaceKHR,
    ) -> Vec<QueueFamily> {
        unsafe {
            instance
                .get_physical_device_queue_family_properties(physical_device, None)
                .drain(..)
                .enumerate()
                .map(|(queue_family_index, family)| {
                    let mut index = QueueFamily {
                        index: queue_family_index as u32,
                        count: family.queue_count,
                        family_type: QueueFamilyType::default(),
                    };

                    if instance
                        .get_physical_device_surface_support_khr(
                            physical_device,
                            queue_family_index as u32,
                            surface,
                            None,
                        )
                        .expect("Failed to check for surface support")
                    {
                        index.family_type.set(QueueFamilyType::PRESENT);
                    }

                    if family.queue_flags.intersects(QueueFlags::GRAPHICS) {
                        index.family_type.set(QueueFamilyType::GRAPHICS);
                    }

                    if family.queue_flags.intersects(QueueFlags::COMPUTE) {
                        index.family_type.set(QueueFamilyType::COMPUTE);
                    }

                    if family.queue_flags.intersects(QueueFlags::TRANSFER) {
                        index.family_type.set(QueueFamilyType::TRANSFER);
                    }

                    if family.queue_flags.intersects(QueueFlags::SPARSE_BINDING) {
                        index.family_type.set(QueueFamilyType::SPARSE_BINDING);
                    }

                    index
                })
                .collect()
        }
    }
}

///
///
///
pub struct Device {
    info: GPUInfo,
    physical_device: PhysicalDevice,
    device_loader: Arc<DeviceLoader>,
    general_queue: Queue,
    general_family: QueueFamily,
    compute_queue: Option<Queue>,
    compute_family: Option<QueueFamily>,
    transfer_queue: Option<Queue>,
    transfer_family: Option<QueueFamily>,
    instance: Arc<Instance>,
    deferred_destruction: Mutex<Vec<Box<dyn Fn(&Device) + 'static>>>,
}

impl Device {
    ///
    /// Get a builder for constructing a device. Just a wrapper for `DeviceBuilder::new`
    ///
    pub fn builder() -> DeviceBuilder {
        DeviceBuilder::new()
    }

    ///
    /// Get a reference to the instance loader
    ///
    pub fn loader(&self) -> &Arc<DeviceLoader> {
        &self.device_loader
    }

    ///
    /// Get the surface this device is working with
    ///
    /// Just calls `Instance::surface` on the instance ref held by the device
    ///
    pub fn surface(&self) -> SurfaceKHR {
        self.instance.surface()
    }

    ///
    /// Gets the general queue we're using
    ///
    pub fn general_queue(&self) -> Queue {
        self.general_queue
    }

    ///
    /// Gets the `QueueFamily` of the general queue
    ///
    pub fn general_family(&self) -> &QueueFamily {
        &self.general_family
    }

    ///
    /// Gets the compute queue we're using
    ///
    /// This will not always exist so don't assume it will (Intel iGPUs only have a single queue)
    ///
    pub fn compute_queue(&self) -> Option<Queue> {
        self.compute_queue
    }

    ///
    /// Gets the `QueueFamily` of the compute queue, if there is one
    ///
    pub fn compute_family(&self) -> Option<&QueueFamily> {
        self.compute_family.as_ref()
    }

    ///
    /// Gets the transfer queue we're using
    ///
    /// This will not always exist so don't assume it will (Intel iGPUs only have a single queue)
    ///
    pub fn transfer_queue(&self) -> Option<Queue> {
        self.transfer_queue
    }

    ///
    /// Gets the `QueueFamily` of the transfer queue, if there is one
    ///
    pub fn transfer_family(&self) -> Option<&QueueFamily> {
        self.transfer_family.as_ref()
    }

    ///
    /// Returns the information about the GPU we collected while constructing the device
    ///
    pub fn info(&self) -> &GPUInfo {
        &self.info
    }

    ///
    /// Returns the supported presentation modes and surface formats for this vulkan device
    ///
    pub fn swapchain_support(&self) -> SwapChainSupport {
        DeviceBuilder::get_swapchain_support(
            self.instance.loader(),
            self.physical_device,
            self.instance.surface(),
        )
    }

    ///
    /// Returns the physical device handle
    ///
    pub fn physical_device(&self) -> PhysicalDevice {
        self.physical_device
    }

    ///
    /// Get the instance that this device is associated with
    ///
    pub fn instance(&self) -> &Arc<Instance> {
        &self.instance
    }

    ///
    /// Appends a function to the deferred destruction list. This can be used to defer destruction
    /// of some items until the `Device` it self is being dropped. This can be used to guarantee
    /// that objects live as long as the device as an invariant when dealing with unsafe code.
    ///
    /// This should be used to uphold invariants for unsafe code and not as a general purpose tool
    /// for object destruction.
    ///
    pub fn defer(&self, func: impl Fn(&Device) + 'static) {
        self.deferred_destruction.lock().push(Box::new(func));
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        PipelineCache::destroy(self);
        unsafe {
            self.deferred_destruction.lock().drain(..).for_each(|v| {
                v(self);
            });
            log::trace!("Destroying Vulkan device");
            self.device_loader.destroy_device(None);
        }
    }
}
