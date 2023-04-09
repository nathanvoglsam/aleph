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

use aleph_vulkan_alloc_sys::raw;

use core::mem;
use core::ptr;

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
    #[inline]
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
    /// Fill out the function pointers from erupt's function pointer tables
    ///
    #[inline]
    pub fn erupt_tables(
        mut self,
        instance_loader: &erupt::InstanceLoader,
        device_loader: &erupt::DeviceLoader,
    ) -> Self {
        // Get the function lists
        let inst = instance_loader;
        let dev = device_loader;

        let func = inst.get_physical_device_properties.unwrap();
        let func: raw::PFN_vkGetPhysicalDeviceProperties = unsafe { mem::transmute(func) };
        self = self.get_physical_device_properties(func);

        // === //

        let func = inst.get_physical_device_memory_properties.unwrap();
        let func: raw::PFN_vkGetPhysicalDeviceMemoryProperties = unsafe { mem::transmute(func) };
        self = self.get_physical_device_memory_properties(func);

        // === //

        let func = dev.allocate_memory.unwrap();
        let func: raw::PFN_vkAllocateMemory = unsafe { mem::transmute(func) };
        self = self.allocate_memory(func);

        // === //

        let func = dev.free_memory.unwrap();
        let func: raw::PFN_vkFreeMemory = unsafe { mem::transmute(func) };
        self = self.free_memory(func);

        // === //

        let func = dev.map_memory.unwrap();
        let func: raw::PFN_vkMapMemory = unsafe { mem::transmute(func) };
        self = self.map_memory(func);

        // === //

        let func = dev.unmap_memory.unwrap();
        let func: raw::PFN_vkUnmapMemory = unsafe { mem::transmute(func) };
        self = self.unmap_memory(func);

        // === //

        let func = dev.flush_mapped_memory_ranges.unwrap();
        let func: raw::PFN_vkFlushMappedMemoryRanges = unsafe { mem::transmute(func) };
        self = self.flush_mapped_memory_ranges(func);

        // === //

        let func = dev.invalidate_mapped_memory_ranges.unwrap();
        let func: raw::PFN_vkInvalidateMappedMemoryRanges = unsafe { mem::transmute(func) };
        self = self.invalidate_mapped_memory_ranges(func);

        // === //

        let func = dev.bind_buffer_memory.unwrap();
        let func: raw::PFN_vkBindBufferMemory = unsafe { mem::transmute(func) };
        self = self.bind_buffer_memory(func);

        // === //

        let func = dev.bind_image_memory.unwrap();
        let func: raw::PFN_vkBindImageMemory = unsafe { mem::transmute(func) };
        self = self.bind_image_memory(func);

        // === //

        let func = dev.get_buffer_memory_requirements.unwrap();
        let func: raw::PFN_vkGetBufferMemoryRequirements = unsafe { mem::transmute(func) };
        self = self.get_buffer_memory_requirements(func);

        // === //

        let func = dev.get_image_memory_requirements.unwrap();
        let func: raw::PFN_vkGetImageMemoryRequirements = unsafe { mem::transmute(func) };
        self = self.get_image_memory_requirements(func);

        // === //

        let func = dev.create_buffer.unwrap();
        let func: raw::PFN_vkCreateBuffer = unsafe { mem::transmute(func) };
        self = self.create_buffer(func);

        // === //

        let func = dev.destroy_buffer.unwrap();
        let func: raw::PFN_vkDestroyBuffer = unsafe { mem::transmute(func) };
        self = self.destroy_buffer(func);

        // === //

        let func = dev.create_image.unwrap();
        let func: raw::PFN_vkCreateImage = unsafe { mem::transmute(func) };
        self = self.create_image(func);

        // === //

        let func = dev.destroy_image.unwrap();
        let func: raw::PFN_vkDestroyImage = unsafe { mem::transmute(func) };
        self = self.destroy_image(func);

        // === //

        let func = dev.cmd_copy_buffer.unwrap();
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
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn allocate_memory(mut self, f: raw::PFN_vkAllocateMemory) -> Self {
        self.functions.vkAllocateMemory = f;
        self
    }

    ///
    ///
    ///
    #[inline]
    pub fn free_memory(mut self, f: raw::PFN_vkFreeMemory) -> Self {
        self.functions.vkFreeMemory = f;
        self
    }

    ///
    ///
    ///
    #[inline]
    pub fn map_memory(mut self, f: raw::PFN_vkMapMemory) -> Self {
        self.functions.vkMapMemory = f;
        self
    }

    ///
    ///
    ///
    #[inline]
    pub fn unmap_memory(mut self, f: raw::PFN_vkUnmapMemory) -> Self {
        self.functions.vkUnmapMemory = f;
        self
    }

    ///
    ///
    ///
    #[inline]
    pub fn flush_mapped_memory_ranges(mut self, f: raw::PFN_vkFlushMappedMemoryRanges) -> Self {
        self.functions.vkFlushMappedMemoryRanges = f;
        self
    }

    ///
    ///
    ///
    #[inline]
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
    #[inline]
    pub fn bind_buffer_memory(mut self, f: raw::PFN_vkBindBufferMemory) -> Self {
        self.functions.vkBindBufferMemory = f;
        self
    }

    ///
    ///
    ///
    #[inline]
    pub fn bind_image_memory(mut self, f: raw::PFN_vkBindImageMemory) -> Self {
        self.functions.vkBindImageMemory = f;
        self
    }

    ///
    ///
    ///
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn create_buffer(mut self, f: raw::PFN_vkCreateBuffer) -> Self {
        self.functions.vkCreateBuffer = f;
        self
    }

    ///
    ///
    ///
    #[inline]
    pub fn destroy_buffer(mut self, f: raw::PFN_vkDestroyBuffer) -> Self {
        self.functions.vkDestroyBuffer = f;
        self
    }

    ///
    ///
    ///
    #[inline]
    pub fn create_image(mut self, f: raw::PFN_vkCreateImage) -> Self {
        self.functions.vkCreateImage = f;
        self
    }

    ///
    ///
    ///
    #[inline]
    pub fn destroy_image(mut self, f: raw::PFN_vkDestroyImage) -> Self {
        self.functions.vkDestroyImage = f;
        self
    }

    ///
    ///
    ///
    #[inline]
    pub fn cmd_copy_buffer(mut self, f: raw::PFN_vkCmdCopyBuffer) -> Self {
        self.functions.vkCmdCopyBuffer = f;
        self
    }

    ///
    ///
    ///
    #[inline]
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
    #[inline]
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
    pub const fn build(self) -> raw::VmaVulkanFunctions {
        self.functions
    }
}

impl Default for VulkanFunctionsBuilder {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
