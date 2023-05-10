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

use crate::raw;
use ash::vk;

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
    /// Fill out the function pointers from ash's function pointer tables
    ///
    #[inline]
    pub fn ash_vk_1_0(
        mut self,
        instance_loader: &ash::Instance,
        device_loader: &ash::Device,
    ) -> Self {
        // Get the function lists
        let inst = instance_loader;
        let dev = device_loader;

        let func = inst.fp_v1_0().get_physical_device_properties;
        self = self.get_physical_device_properties(func);

        // === //

        let func = inst.fp_v1_0().get_physical_device_memory_properties;
        self = self.get_physical_device_memory_properties(func);

        // === //

        let func = dev.fp_v1_0().allocate_memory;
        self = self.allocate_memory(func);

        // === //

        let func = dev.fp_v1_0().free_memory;
        self = self.free_memory(func);

        // === //

        let func = dev.fp_v1_0().map_memory;
        self = self.map_memory(func);

        // === //

        let func = dev.fp_v1_0().unmap_memory;
        self = self.unmap_memory(func);

        // === //

        let func = dev.fp_v1_0().flush_mapped_memory_ranges;
        self = self.flush_mapped_memory_ranges(func);

        // === //

        let func = dev.fp_v1_0().invalidate_mapped_memory_ranges;
        self = self.invalidate_mapped_memory_ranges(func);

        // === //

        let func = dev.fp_v1_0().bind_buffer_memory;
        self = self.bind_buffer_memory(func);

        // === //

        let func = dev.fp_v1_0().bind_image_memory;
        self = self.bind_image_memory(func);

        // === //

        let func = dev.fp_v1_0().get_buffer_memory_requirements;
        self = self.get_buffer_memory_requirements(func);

        // === //

        let func = dev.fp_v1_0().get_image_memory_requirements;
        self = self.get_image_memory_requirements(func);

        // === //

        let func = dev.fp_v1_0().create_buffer;
        self = self.create_buffer(func);

        // === //

        let func = dev.fp_v1_0().destroy_buffer;
        self = self.destroy_buffer(func);

        // === //

        let func = dev.fp_v1_0().create_image;
        self = self.create_image(func);

        // === //

        let func = dev.fp_v1_0().destroy_image;
        self = self.destroy_image(func);

        // === //

        let func = dev.fp_v1_0().cmd_copy_buffer;
        self = self.cmd_copy_buffer(func);

        // === //

        self
    }

    ///
    /// Fill out the function pointers from ash's function pointer tables
    ///
    #[inline]
    pub fn ash_vk_1_1(
        mut self,
        instance_loader: &ash::Instance,
        device_loader: &ash::Device,
    ) -> Self {
        // Get the function lists
        let inst = instance_loader;
        let dev = device_loader;

        let func = inst.fp_v1_0().get_physical_device_properties;
        self = self.get_physical_device_properties(func);

        // === //

        let func = inst.fp_v1_0().get_physical_device_memory_properties;
        self = self.get_physical_device_memory_properties(func);

        // === //

        let func = dev.fp_v1_0().allocate_memory;
        self = self.allocate_memory(func);

        // === //

        let func = dev.fp_v1_0().free_memory;
        self = self.free_memory(func);

        // === //

        let func = dev.fp_v1_0().map_memory;
        self = self.map_memory(func);

        // === //

        let func = dev.fp_v1_0().unmap_memory;
        self = self.unmap_memory(func);

        // === //

        let func = dev.fp_v1_0().flush_mapped_memory_ranges;
        self = self.flush_mapped_memory_ranges(func);

        // === //

        let func = dev.fp_v1_0().invalidate_mapped_memory_ranges;
        self = self.invalidate_mapped_memory_ranges(func);

        // === //

        let func = dev.fp_v1_0().bind_buffer_memory;
        self = self.bind_buffer_memory(func);

        // === //

        let func = dev.fp_v1_0().bind_image_memory;
        self = self.bind_image_memory(func);

        // === //

        let func = dev.fp_v1_0().get_buffer_memory_requirements;
        self = self.get_buffer_memory_requirements(func);

        // === //

        let func = dev.fp_v1_0().get_image_memory_requirements;
        self = self.get_image_memory_requirements(func);

        // === //

        let func = dev.fp_v1_0().create_buffer;
        self = self.create_buffer(func);

        // === //

        let func = dev.fp_v1_0().destroy_buffer;
        self = self.destroy_buffer(func);

        // === //

        let func = dev.fp_v1_0().create_image;
        self = self.create_image(func);

        // === //

        let func = dev.fp_v1_0().destroy_image;
        self = self.destroy_image(func);

        // === //

        let func = dev.fp_v1_0().cmd_copy_buffer;
        self = self.cmd_copy_buffer(func);

        // === //

        let func = dev.fp_v1_1().get_buffer_memory_requirements2;
        self = self.get_buffer_memory_requirements2_khr(func);

        // === //

        let func = dev.fp_v1_1().get_image_memory_requirements2;
        self = self.get_image_memory_requirements2_khr(func);

        self
    }

    // pub fn ash_get_memory_requirements_2(
    //     mut self,
    //     loader: &ash::extensions::khr::GetMemoryRequirements2,
    // ) -> Self {
    //     let func = loader.fp().get_buffer_memory_requirements2_khr;
    //     self = self.get_buffer_memory_requirements2_khr(func);
    //
    //     let func = loader.fp().get_image_memory_requirements2_khr;
    //     self = self.get_image_memory_requirements2_khr(func);
    //
    //     self
    // }

    ///
    ///
    ///
    #[inline]
    pub fn get_physical_device_properties(
        mut self,
        f: vk::PFN_vkGetPhysicalDeviceProperties,
    ) -> Self {
        self.functions.vkGetPhysicalDeviceProperties = Some(f);
        self
    }

    ///
    ///
    ///
    #[inline]
    pub fn get_physical_device_memory_properties(
        mut self,
        f: vk::PFN_vkGetPhysicalDeviceMemoryProperties,
    ) -> Self {
        self.functions.vkGetPhysicalDeviceMemoryProperties = Some(f);
        self
    }

    ///
    ///
    ///
    #[inline]
    pub fn allocate_memory(mut self, f: vk::PFN_vkAllocateMemory) -> Self {
        self.functions.vkAllocateMemory = Some(f);
        self
    }

    ///
    ///
    ///
    #[inline]
    pub fn free_memory(mut self, f: vk::PFN_vkFreeMemory) -> Self {
        self.functions.vkFreeMemory = Some(f);
        self
    }

    ///
    ///
    ///
    #[inline]
    pub fn map_memory(mut self, f: vk::PFN_vkMapMemory) -> Self {
        self.functions.vkMapMemory = Some(f);
        self
    }

    ///
    ///
    ///
    #[inline]
    pub fn unmap_memory(mut self, f: vk::PFN_vkUnmapMemory) -> Self {
        self.functions.vkUnmapMemory = Some(f);
        self
    }

    ///
    ///
    ///
    #[inline]
    pub fn flush_mapped_memory_ranges(mut self, f: vk::PFN_vkFlushMappedMemoryRanges) -> Self {
        self.functions.vkFlushMappedMemoryRanges = Some(f);
        self
    }

    ///
    ///
    ///
    #[inline]
    pub fn invalidate_mapped_memory_ranges(
        mut self,
        f: vk::PFN_vkInvalidateMappedMemoryRanges,
    ) -> Self {
        self.functions.vkInvalidateMappedMemoryRanges = Some(f);
        self
    }

    ///
    ///
    ///
    #[inline]
    pub fn bind_buffer_memory(mut self, f: vk::PFN_vkBindBufferMemory) -> Self {
        self.functions.vkBindBufferMemory = Some(f);
        self
    }

    ///
    ///
    ///
    #[inline]
    pub fn bind_image_memory(mut self, f: vk::PFN_vkBindImageMemory) -> Self {
        self.functions.vkBindImageMemory = Some(f);
        self
    }

    ///
    ///
    ///
    #[inline]
    pub fn get_buffer_memory_requirements(
        mut self,
        f: vk::PFN_vkGetBufferMemoryRequirements,
    ) -> Self {
        self.functions.vkGetBufferMemoryRequirements = Some(f);
        self
    }

    ///
    ///
    ///
    #[inline]
    pub fn get_image_memory_requirements(
        mut self,
        f: vk::PFN_vkGetImageMemoryRequirements,
    ) -> Self {
        self.functions.vkGetImageMemoryRequirements = Some(f);
        self
    }

    ///
    ///
    ///
    #[inline]
    pub fn create_buffer(mut self, f: vk::PFN_vkCreateBuffer) -> Self {
        self.functions.vkCreateBuffer = Some(f);
        self
    }

    ///
    ///
    ///
    #[inline]
    pub fn destroy_buffer(mut self, f: vk::PFN_vkDestroyBuffer) -> Self {
        self.functions.vkDestroyBuffer = Some(f);
        self
    }

    ///
    ///
    ///
    #[inline]
    pub fn create_image(mut self, f: vk::PFN_vkCreateImage) -> Self {
        self.functions.vkCreateImage = Some(f);
        self
    }

    ///
    ///
    ///
    #[inline]
    pub fn destroy_image(mut self, f: vk::PFN_vkDestroyImage) -> Self {
        self.functions.vkDestroyImage = Some(f);
        self
    }

    ///
    ///
    ///
    #[inline]
    pub fn cmd_copy_buffer(mut self, f: vk::PFN_vkCmdCopyBuffer) -> Self {
        self.functions.vkCmdCopyBuffer = Some(f);
        self
    }

    ///
    ///
    ///
    #[inline]
    pub fn get_buffer_memory_requirements2_khr(
        mut self,
        f: vk::PFN_vkGetBufferMemoryRequirements2,
    ) -> Self {
        self.functions.vkGetBufferMemoryRequirements2KHR = Some(f);
        self
    }

    ///
    ///
    ///
    #[inline]
    pub fn get_image_memory_requirements2_khr(
        mut self,
        f: vk::PFN_vkGetImageMemoryRequirements2,
    ) -> Self {
        self.functions.vkGetImageMemoryRequirements2KHR = Some(f);
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
