//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use vma_sys::raw;

use core::mem;
use core::ptr;
use erupt::{DeviceLoader, InstanceLoader};

///
/// Builder wrapper for the VmaVulkanFunctions struct provided by vma-sys
///
pub struct VulkanFunctionsBuilder {
    functions: raw::VmaVulkanFunctions,
}

impl VulkanFunctionsBuilder {
    ///
    /// Construct a new VulkanFunctionsBuilder
    ///
    pub fn new() -> Self {
        let functions = raw::VmaVulkanFunctions {
            vkGetPhysicalDeviceProperties: None,
            vkGetPhysicalDeviceMemoryProperties: None,
            vkAllocateMemory: None,
            vkFreeMemory: None,
            vkMapMemory: None,
            vkUnmapMemory: None,
            vkFlushMappedMemoryRanges: None,
            vkInvalidateMappedMemoryRanges: None,
            vkBindBufferMemory: None,
            vkBindImageMemory: None,
            vkGetBufferMemoryRequirements: None,
            vkGetImageMemoryRequirements: None,
            vkCreateBuffer: None,
            vkDestroyBuffer: None,
            vkCreateImage: None,
            vkDestroyImage: None,
            vkCmdCopyBuffer: None,
            vkGetBufferMemoryRequirements2KHR: None,
            vkGetImageMemoryRequirements2KHR: None,
        };

        VulkanFunctionsBuilder { functions }
    }

    ///
    /// Fill out the function pointers from ash's function pointer tables
    ///
    pub fn erupt_tables(
        mut self,
        instance_loader: &InstanceLoader,
        device_loader: &DeviceLoader,
    ) -> Self {
        // Get the function lists
        let inst = instance_loader.vk1_0.as_ref().unwrap();
        let dev = device_loader.vk1_0.as_ref().unwrap();

        let func = inst.get_physical_device_properties;
        let func: raw::PFN_vkGetPhysicalDeviceProperties = unsafe { mem::transmute(func) };
        self = self.get_physical_device_properties(func);

        // === //

        let func = inst.get_physical_device_memory_properties;
        let func: raw::PFN_vkGetPhysicalDeviceMemoryProperties = unsafe { mem::transmute(func) };
        self = self.get_physical_device_memory_properties(func);

        // === //

        let func = dev.allocate_memory;
        let func: raw::PFN_vkAllocateMemory = unsafe { mem::transmute(func) };
        self = self.allocate_memory(func);

        // === //

        let func = dev.free_memory;
        let func: raw::PFN_vkFreeMemory = unsafe { mem::transmute(func) };
        self = self.free_memory(func);

        // === //

        let func = dev.map_memory;
        let func: raw::PFN_vkMapMemory = unsafe { mem::transmute(func) };
        self = self.map_memory(func);

        // === //

        let func = dev.unmap_memory;
        let func: raw::PFN_vkUnmapMemory = unsafe { mem::transmute(func) };
        self = self.unmap_memory(func);

        // === //

        let func = dev.flush_mapped_memory_ranges;
        let func: raw::PFN_vkFlushMappedMemoryRanges = unsafe { mem::transmute(func) };
        self = self.flush_mapped_memory_ranges(func);

        // === //

        let func = dev.invalidate_mapped_memory_ranges;
        let func: raw::PFN_vkInvalidateMappedMemoryRanges = unsafe { mem::transmute(func) };
        self = self.invalidate_mapped_memory_ranges(func);

        // === //

        let func = dev.bind_buffer_memory;
        let func: raw::PFN_vkBindBufferMemory = unsafe { mem::transmute(func) };
        self = self.bind_buffer_memory(func);

        // === //

        let func = dev.bind_image_memory;
        let func: raw::PFN_vkBindImageMemory = unsafe { mem::transmute(func) };
        self = self.bind_image_memory(func);

        // === //

        let func = dev.get_buffer_memory_requirements;
        let func: raw::PFN_vkGetBufferMemoryRequirements = unsafe { mem::transmute(func) };
        self = self.get_buffer_memory_requirements(func);

        // === //

        let func = dev.get_image_memory_requirements;
        let func: raw::PFN_vkGetImageMemoryRequirements = unsafe { mem::transmute(func) };
        self = self.get_image_memory_requirements(func);

        // === //

        let func = dev.create_buffer;
        let func: raw::PFN_vkCreateBuffer = unsafe { mem::transmute(func) };
        self = self.create_buffer(func);

        // === //

        let func = dev.destroy_buffer;
        let func: raw::PFN_vkDestroyBuffer = unsafe { mem::transmute(func) };
        self = self.destroy_buffer(func);

        // === //

        let func = dev.create_image;
        let func: raw::PFN_vkCreateImage = unsafe { mem::transmute(func) };
        self = self.create_image(func);

        // === //

        let func = dev.destroy_image;
        let func: raw::PFN_vkDestroyImage = unsafe { mem::transmute(func) };
        self = self.destroy_image(func);

        // === //

        let func = dev.cmd_copy_buffer;
        let func: raw::PFN_vkCmdCopyBuffer = unsafe { mem::transmute(func) };
        self = self.cmd_copy_buffer(func);

        // === //

        let func: *mut u32 = ptr::null_mut();
        let func: raw::PFN_vkGetBufferMemoryRequirements2KHR = unsafe { mem::transmute(func) };
        self = self.get_buffer_memory_requirements2_khr(func);

        // === //

        let func: *mut u32 = ptr::null_mut();
        let func: raw::PFN_vkGetImageMemoryRequirements2KHR = unsafe { mem::transmute(func) };
        self = self.get_image_memory_requirements2_khr(func);

        // === //

        self
    }

    ///
    ///
    ///
    pub fn get_physical_device_properties(
        mut self,
        f: raw::PFN_vkGetPhysicalDeviceProperties,
    ) -> Self {
        self.functions.vkGetPhysicalDeviceProperties = f;
        self
    }

    ///
    ///
    ///
    pub fn get_physical_device_memory_properties(
        mut self,
        f: raw::PFN_vkGetPhysicalDeviceMemoryProperties,
    ) -> Self {
        self.functions.vkGetPhysicalDeviceMemoryProperties = f;
        self
    }

    ///
    ///
    ///
    pub fn allocate_memory(mut self, f: raw::PFN_vkAllocateMemory) -> Self {
        self.functions.vkAllocateMemory = f;
        self
    }

    ///
    ///
    ///
    pub fn free_memory(mut self, f: raw::PFN_vkFreeMemory) -> Self {
        self.functions.vkFreeMemory = f;
        self
    }

    ///
    ///
    ///
    pub fn map_memory(mut self, f: raw::PFN_vkMapMemory) -> Self {
        self.functions.vkMapMemory = f;
        self
    }

    ///
    ///
    ///
    pub fn unmap_memory(mut self, f: raw::PFN_vkUnmapMemory) -> Self {
        self.functions.vkUnmapMemory = f;
        self
    }

    ///
    ///
    ///
    pub fn flush_mapped_memory_ranges(mut self, f: raw::PFN_vkFlushMappedMemoryRanges) -> Self {
        self.functions.vkFlushMappedMemoryRanges = f;
        self
    }

    ///
    ///
    ///
    pub fn invalidate_mapped_memory_ranges(
        mut self,
        f: raw::PFN_vkInvalidateMappedMemoryRanges,
    ) -> Self {
        self.functions.vkInvalidateMappedMemoryRanges = f;
        self
    }

    ///
    ///
    ///
    pub fn bind_buffer_memory(mut self, f: raw::PFN_vkBindBufferMemory) -> Self {
        self.functions.vkBindBufferMemory = f;
        self
    }

    ///
    ///
    ///
    pub fn bind_image_memory(mut self, f: raw::PFN_vkBindImageMemory) -> Self {
        self.functions.vkBindImageMemory = f;
        self
    }

    ///
    ///
    ///
    pub fn get_buffer_memory_requirements(
        mut self,
        f: raw::PFN_vkGetBufferMemoryRequirements,
    ) -> Self {
        self.functions.vkGetBufferMemoryRequirements = f;
        self
    }

    ///
    ///
    ///
    pub fn get_image_memory_requirements(
        mut self,
        f: raw::PFN_vkGetImageMemoryRequirements,
    ) -> Self {
        self.functions.vkGetImageMemoryRequirements = f;
        self
    }

    ///
    ///
    ///
    pub fn create_buffer(mut self, f: raw::PFN_vkCreateBuffer) -> Self {
        self.functions.vkCreateBuffer = f;
        self
    }

    ///
    ///
    ///
    pub fn destroy_buffer(mut self, f: raw::PFN_vkDestroyBuffer) -> Self {
        self.functions.vkDestroyBuffer = f;
        self
    }

    ///
    ///
    ///
    pub fn create_image(mut self, f: raw::PFN_vkCreateImage) -> Self {
        self.functions.vkCreateImage = f;
        self
    }

    ///
    ///
    ///
    pub fn destroy_image(mut self, f: raw::PFN_vkDestroyImage) -> Self {
        self.functions.vkDestroyImage = f;
        self
    }

    ///
    ///
    ///
    pub fn cmd_copy_buffer(mut self, f: raw::PFN_vkCmdCopyBuffer) -> Self {
        self.functions.vkCmdCopyBuffer = f;
        self
    }

    ///
    ///
    ///
    pub fn get_buffer_memory_requirements2_khr(
        mut self,
        f: raw::PFN_vkGetBufferMemoryRequirements2KHR,
    ) -> Self {
        self.functions.vkGetBufferMemoryRequirements2KHR = f;
        self
    }

    ///
    ///
    ///
    pub fn get_image_memory_requirements2_khr(
        mut self,
        f: raw::PFN_vkGetImageMemoryRequirements2KHR,
    ) -> Self {
        self.functions.vkGetImageMemoryRequirements2KHR = f;
        self
    }

    ///
    /// Return the VmaVulkanFunctions struct if it is valid, otherwise return None
    ///
    pub fn build(self) -> raw::VmaVulkanFunctions {
        self.functions
    }
}

impl Default for VulkanFunctionsBuilder {
    fn default() -> Self {
        Self::new()
    }
}
