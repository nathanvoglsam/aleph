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
use crate::utils;
use crate::vma;
use crate::vulkan_functions::VulkanFunctionsBuilder;
use core::mem;
use core::ptr;
use erupt::utils::VulkanResult;
use erupt::vk;
use std::ffi::c_void;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::sync::Arc;
use thiserror::Error;

pub use raw::AllocatorCreateFlags;

///
/// Errors that can be produced by trying to construct a vma Allocator incorrectly
///
#[derive(Copy, Clone, Debug, PartialEq, Eq, Error)]
pub enum AllocatorBuilderError {
    #[error("The provided set of function pointers are not valid")]
    InvalidFunctionPointers,

    #[error("The provided set of extension function pointers are not valid")]
    InvalidExtensionFunctionPointers,
}

///
/// Builder utility for simple, safe construction of a vma Allocator
///
pub struct AllocatorBuilder {
    create_info: raw::VmaAllocatorCreateInfo,
}

impl AllocatorBuilder {
    ///
    /// New instance of the builder
    ///
    /// User must give builder an instance, device and physical device otherwise it will fail to
    /// build
    ///
    pub const fn new() -> Self {
        let create_info = raw::VmaAllocatorCreateInfo {
            flags: AllocatorCreateFlags::empty(),
            physical_device: vk::PhysicalDevice::null(),
            device: vk::Device::null(),
            preferred_large_heap_block_size: 0,
            p_allocation_callbacks: ptr::null_mut(),
            p_device_memory_callbacks: ptr::null_mut(),
            frame_in_use_count: 0,
            p_heap_size_limit: ptr::null_mut(),
            p_vulkan_functions: ptr::null_mut(),
            p_record_settings: ptr::null_mut(),
            vulkan_api_version: vk::make_api_version(1, 1, 0, 0),
        };

        AllocatorBuilder { create_info }
    }

    /// Set the vulkan version being used
    pub fn vulkan_api_version(mut self, version: u32) -> Self {
        self.create_info.vulkan_api_version = version;
        self
    }

    ///
    /// Build the VmaAllocator
    ///
    /// ## Errors
    ///
    /// - If there is no valid physical device handle
    /// - If there is no valid device handle
    /// - If not all required function pointers are provided
    /// - If KHR_DEDICATED_ALLOCATION_BIT is set and the required function pointers are not given
    ///
    #[inline]
    pub fn build(
        mut self,
        instance_loader: &erupt::InstanceLoader,
        device_loader: &erupt::DeviceLoader,
        physical_device: vk::PhysicalDevice,
    ) -> Result<Allocator, AllocatorBuilderError> {
        self.create_info.device = device_loader.handle;
        self.create_info.physical_device = physical_device;

        let functions = VulkanFunctionsBuilder::new()
            .erupt_tables(instance_loader, device_loader)
            .build();

        // The library will require the 'requirements 2' functions either if required based on the
        // config flag, or unconditionally if the API version is 1.1 or higher (extension became
        // core).
        let wants_dedicated_alloc = self
            .create_info
            .flags
            .contains(AllocatorCreateFlags::KHR_DEDICATED_ALLOCATION_BIT);
        let vulkan_1_1 = self.create_info.vulkan_api_version >= vk::API_VERSION_1_1;
        let requirements_2 = wants_dedicated_alloc || vulkan_1_1;

        if !utils::allocator_functions_valid(&functions, requirements_2) {
            return Err(AllocatorBuilderError::InvalidFunctionPointers);
        }

        self.create_info.p_vulkan_functions = &functions;

        if self
            .create_info
            .flags
            .contains(AllocatorCreateFlags::KHR_DEDICATED_ALLOCATION_BIT)
        {
            let has_buffer_req_2 = functions.vkGetBufferMemoryRequirements2KHR.is_none();
            let has_image_req_2 = functions.vkGetImageMemoryRequirements2KHR.is_none();
            if has_buffer_req_2 || has_image_req_2 {
                return Err(AllocatorBuilderError::InvalidExtensionFunctionPointers);
            }
        }

        let mut raw_alloc: raw::VmaAllocator = ptr::null_mut();
        unsafe {
            let result = raw::vmaCreateAllocator(&self.create_info, &mut raw_alloc as *mut _);
            if result != vk::Result::SUCCESS {
                panic!("Failed on call to vmaCreateAllocator with error: {result:?}");
            }
        }

        let alloc = Allocator {
            inner: Arc::new(Inner {
                allocator: raw_alloc,
                device: device_loader.handle,
            }),
        };

        Ok(alloc)
    }
}

impl Default for AllocatorBuilder {
    fn default() -> Self {
        Self::new()
    }
}

///
/// Holds the internal reference to the vma allocator as well as managing correct lifetime
///
#[derive(Clone)]
pub struct Allocator {
    inner: Arc<Inner>,
}

impl Allocator {
    ///
    /// Return a builder for the vma allocator
    ///
    pub const fn builder() -> AllocatorBuilder {
        AllocatorBuilder::new()
    }

    ///
    /// Get the raw VmaAllocator handle
    ///
    #[inline]
    pub fn as_raw(&self) -> raw::VmaAllocator {
        self.inner.allocator
    }

    ///
    /// vmaGetPhysicalDeviceProperties
    ///
    #[inline]
    pub unsafe fn get_physical_device_properties<'alloc>(
        &'alloc self,
    ) -> &'alloc vk::PhysicalDeviceProperties {
        let mut reference: *const vk::PhysicalDeviceProperties = ptr::null_mut();

        raw::vmaGetPhysicalDeviceProperties(self.inner.allocator, &mut reference);

        reference
            .as_ref::<'alloc>()
            .expect("Given nullptr by vmaGetPhysicalDeviceProperties")
    }

    ///
    /// vmaGetPhysicalDeviceProperties
    ///
    #[inline]
    pub unsafe fn get_memory_properties<'alloc>(
        &'alloc self,
    ) -> &'alloc vk::PhysicalDeviceMemoryProperties {
        let mut reference: *const vk::PhysicalDeviceMemoryProperties = ptr::null_mut();

        raw::vmaGetMemoryProperties(self.inner.allocator, &mut reference);

        reference
            .as_ref::<'alloc>()
            .expect("Given nullptr by vmaGetMemoryProperties")
    }

    ///
    /// vmaCalculateStats
    ///
    #[inline]
    pub unsafe fn calculate_stats(&self) -> vma::Stats {
        let mut stats = vma::Stats::default();
        let stats_ptr = &mut stats as *mut vma::Stats;
        let stats_ptr = stats_ptr as *mut raw::VmaStats;

        raw::vmaCalculateStats(self.inner.allocator, stats_ptr);

        stats
    }

    ///
    /// Returns a rusty String object that is much easier to pass around than a CStr reference. It
    /// will also automatically de-allocate making it much safer, but will incur an extra allocation
    /// and utf-8 conversion so be aware of the overhead.
    ///
    #[inline]
    pub unsafe fn get_stats_string(&self, detailed_map: bool) -> String {
        let c_str = self.build_stats_string(detailed_map);

        let string = c_str.to_str().expect("Invalid utf-8 chars in stats string");
        let string = string.to_string();

        self.free_stats_string(c_str);

        string
    }

    ///
    /// vmaBuildStatsString
    ///
    #[inline]
    pub unsafe fn build_stats_string(&self, detailed_map: bool) -> &CStr {
        let mut c_str_ptr: *mut c_char = ptr::null_mut();

        raw::vmaBuildStatsString(
            self.inner.allocator,
            &mut c_str_ptr,
            if detailed_map { 1 } else { 0 },
        );

        CStr::from_ptr(c_str_ptr)
    }

    ///
    /// vmaFreeStatsString
    ///
    #[inline]
    pub unsafe fn free_stats_string(&self, str: &CStr) {
        let c_str_ptr = str.as_ptr();
        raw::vmaFreeStatsString(self.inner.allocator, c_str_ptr as *mut c_char);
    }

    ///
    /// vmaFindMemoryTypeIndex
    ///
    #[inline]
    pub unsafe fn find_memory_type_index(
        &self,
        memory_type_bits: u32,
        allocation_create_info: &vma::AllocationCreateInfo,
    ) -> VulkanResult<u32> {
        let mut idx = 0u32;

        let alloc_create_info = allocation_create_info.into_raw();
        let alloc_ptr = &alloc_create_info as *const raw::VmaAllocationCreateInfo;

        let result = raw::vmaFindMemoryTypeIndex(
            self.inner.allocator,
            memory_type_bits,
            alloc_ptr,
            &mut idx,
        );

        VulkanResult::new(result, idx)
    }

    ///
    /// vmaFindMemoryTypeIndexForBufferInfo
    ///
    #[inline]
    pub unsafe fn find_memory_type_index_for_buffer_info(
        &self,
        buffer_create_info: &vk::BufferCreateInfo,
        allocation_create_info: &vma::AllocationCreateInfo,
    ) -> VulkanResult<u32> {
        let mut idx = 0u32;

        let alloc_create_info = allocation_create_info.into_raw();
        let alloc_ptr = &alloc_create_info as *const raw::VmaAllocationCreateInfo;

        let result = raw::vmaFindMemoryTypeIndexForBufferInfo(
            self.inner.allocator,
            buffer_create_info,
            alloc_ptr,
            &mut idx,
        );

        VulkanResult::new(result, idx)
    }

    ///
    /// vmaFindMemoryTypeIndexForImageInfo
    ///
    #[inline]
    pub unsafe fn find_memory_type_index_for_image_info(
        &self,
        image_create_info: &vk::ImageCreateInfo,
        allocation_create_info: &vma::AllocationCreateInfo,
    ) -> VulkanResult<u32> {
        let mut idx = 0u32;

        let allocation_create_info = allocation_create_info.into_raw();
        let alloc_ptr = &allocation_create_info as *const raw::VmaAllocationCreateInfo;

        let result = raw::vmaFindMemoryTypeIndexForImageInfo(
            self.inner.allocator,
            image_create_info,
            alloc_ptr,
            &mut idx,
        );

        VulkanResult::new(result, idx)
    }

    ///
    /// vmaAllocateMemory
    ///
    #[inline]
    pub unsafe fn allocate_memory(
        &self,
        memory_requirements: &vk::MemoryRequirements,
        create_info: &vma::AllocationCreateInfo,
    ) -> VulkanResult<vma::Allocation> {
        let mut allocation: raw::VmaAllocation = ptr::null_mut();

        let create_info = create_info.into_raw();
        let create_info_ptr = &create_info as *const raw::VmaAllocationCreateInfo;

        let result = raw::vmaAllocateMemory(
            self.inner.allocator,
            memory_requirements,
            create_info_ptr,
            &mut allocation,
            ptr::null_mut(),
        );

        VulkanResult::new(result, vma::Allocation::from_raw(allocation))
    }

    ///
    /// vmaAllocateMemoryPages
    ///
    #[inline]
    pub unsafe fn allocate_memory_pages(
        &self,
        memory_requirements: &vk::MemoryRequirements,
        create_info: &vma::AllocationCreateInfo,
        allocation_count: usize,
    ) -> VulkanResult<Vec<vma::Allocation>> {
        let mut ret: Vec<vma::Allocation> = Vec::with_capacity(allocation_count);
        ret.resize(allocation_count, vma::Allocation::from_raw(ptr::null_mut()));

        let create_info = create_info.into_raw();
        let create_info_ptr = &create_info as *const raw::VmaAllocationCreateInfo;

        let result = raw::vmaAllocateMemoryPages(
            self.inner.allocator,
            memory_requirements,
            create_info_ptr,
            allocation_count as _,
            ret.as_mut_ptr() as *mut raw::VmaAllocation,
            ptr::null_mut(),
        );

        VulkanResult::new(result, ret)
    }

    ///
    /// vmaAllocateMemoryForBuffer
    ///
    #[inline]
    pub unsafe fn allocate_memory_for_buffer(
        &self,
        buffer: vk::Buffer,
        create_info: &vma::AllocationCreateInfo,
    ) -> VulkanResult<vma::Allocation> {
        let mut allocation: raw::VmaAllocation = ptr::null_mut();

        let create_info = create_info.into_raw();
        let create_info_ptr = &create_info as *const raw::VmaAllocationCreateInfo;

        let result = raw::vmaAllocateMemoryForBuffer(
            self.inner.allocator,
            buffer,
            create_info_ptr,
            &mut allocation,
            ptr::null_mut(),
        );

        VulkanResult::new(result, vma::Allocation::from_raw(allocation))
    }

    ///
    /// vmaAllocateMemoryforImage
    ///
    #[inline]
    pub unsafe fn allocate_memory_for_image(
        &self,
        image: vk::Image,
        create_info: &vma::AllocationCreateInfo,
    ) -> VulkanResult<vma::Allocation> {
        let mut allocation: raw::VmaAllocation = ptr::null_mut();

        let create_info = create_info.into_raw();
        let create_info_ptr = &create_info as *const raw::VmaAllocationCreateInfo;

        let result = raw::vmaAllocateMemoryForImage(
            self.inner.allocator,
            image,
            create_info_ptr,
            &mut allocation,
            ptr::null_mut(),
        );

        VulkanResult::new(result, vma::Allocation::from_raw(allocation))
    }

    ///
    /// vmaFreeMemory
    ///
    #[inline]
    pub unsafe fn free_memory(&self, allocation: vma::Allocation) {
        raw::vmaFreeMemory(self.inner.allocator, allocation.into_raw());
    }

    ///
    /// vmaFreeMemoryPages
    ///
    #[inline]
    pub unsafe fn free_memory_pages(&self, allocations: &[vma::Allocation]) {
        let pointer = allocations.as_ptr();
        let pointer = pointer as *mut vma::Allocation;
        let pointer = pointer as *mut raw::VmaAllocation;

        raw::vmaFreeMemoryPages(self.inner.allocator, allocations.len() as _, pointer)
    }

    ///
    /// vmaResizeAllocation
    ///
    #[inline]
    pub unsafe fn resize_allocation(
        &self,
        allocation: &vma::Allocation,
        new_size: vk::DeviceSize,
    ) -> VulkanResult<()> {
        let result =
            raw::vmaResizeAllocation(self.inner.allocator, allocation.into_raw(), new_size);

        VulkanResult::new(result, ())
    }

    ///
    /// vmaGetAllocationInfo
    ///
    #[inline]
    pub unsafe fn get_allocation_info(&self, allocation: &vma::Allocation) -> vma::AllocationInfo {
        let mut info = vma::AllocationInfo {
            memory_type: 0,
            device_memory: vk::DeviceMemory::default(),
            offset: 0,
            size: 0,
            p_mapped_data: ptr::null_mut(),
            p_user_data: ptr::null_mut(),
        };
        let info_ptr = &mut info as *mut vma::AllocationInfo;
        let info_ptr = info_ptr as *mut raw::VmaAllocationInfo;

        raw::vmaGetAllocationInfo(self.inner.allocator, allocation.into_raw(), info_ptr);

        info
    }

    ///
    /// vmaTouchAllocation
    ///
    #[inline]
    pub unsafe fn touch_allocation(&self, allocation: &vma::Allocation) -> bool {
        let result = raw::vmaTouchAllocation(self.inner.allocator, allocation.into_raw());

        result != 0
    }

    // TODO : vmaSetAllocationUserData (PROBABLY WONT EXPOSE THIS, NOT VERY RUSTY)

    ///
    /// vmaCreateLostAllocation
    ///
    #[inline]
    pub unsafe fn create_lost_allocation(&self) -> vma::Allocation {
        let mut allocation: raw::VmaAllocation = ptr::null_mut();
        raw::vmaCreateLostAllocation(self.inner.allocator, &mut allocation);

        let allocation: vma::Allocation = mem::transmute(allocation);
        allocation
    }

    ///
    /// vmaMapMemory
    ///
    #[inline]
    pub unsafe fn map_memory(&self, allocation: &vma::Allocation) -> VulkanResult<*mut u8> {
        let mut pointer: *mut c_void = ptr::null_mut();

        let result = raw::vmaMapMemory(self.inner.allocator, allocation.into_raw(), &mut pointer);

        VulkanResult::new(result, pointer as *mut u8)
    }

    ///
    /// vmaUnmapMemory
    ///
    #[inline]
    pub unsafe fn unmap_memory(&self, allocation: &vma::Allocation) {
        raw::vmaUnmapMemory(self.inner.allocator, allocation.into_raw());
    }

    ///
    /// vmaFlushAllocation
    ///
    #[inline]
    pub unsafe fn flush_allocation(
        &self,
        allocation: &vma::Allocation,
        offset: vk::DeviceSize,
        size: vk::DeviceSize,
    ) {
        raw::vmaFlushAllocation(self.inner.allocator, allocation.into_raw(), offset, size);
    }
    ///
    /// vmaInvalidateAllocation
    ///
    #[inline]
    pub unsafe fn invalidate_allocation(
        &self,
        allocation: &vma::Allocation,
        offset: vk::DeviceSize,
        size: vk::DeviceSize,
    ) {
        raw::vmaInvalidateAllocation(self.inner.allocator, allocation.into_raw(), offset, size);
    }

    ///
    /// vmaCheckCorruption
    ///
    #[inline]
    pub unsafe fn check_corruption(&self, memory_type_bits: u32) -> VulkanResult<()> {
        let result = raw::vmaCheckCorruption(self.inner.allocator, memory_type_bits);

        VulkanResult::new(result, ())
    }

    // TODO : vmaDefragmentationBegin
    // TODO : vmaDefragmentationEnd
    // TODO : vmaDefragment (Deprecated)

    ///
    /// vmaBindBufferMemory
    ///
    #[inline]
    pub unsafe fn bind_buffer_memory(
        &self,
        allocation: &vma::Allocation,
        buffer: vk::Buffer,
    ) -> VulkanResult<()> {
        let result = raw::vmaBindBufferMemory(self.inner.allocator, allocation.into_raw(), buffer);

        VulkanResult::new(result, ())
    }

    ///
    /// vmaBindImageMemory
    ///
    #[inline]
    pub unsafe fn bind_image_memory(
        &self,
        allocation: &vma::Allocation,
        image: vk::Image,
    ) -> VulkanResult<()> {
        let result = raw::vmaBindImageMemory(
            self.inner.allocator,
            allocation.into_raw(),
            mem::transmute(image),
        );

        VulkanResult::new(result, ())
    }

    ///
    /// vmaCreateBuffer
    ///
    #[inline]
    pub unsafe fn create_buffer(
        &self,
        buffer_create_info: &vk::BufferCreateInfo,
        alloc_create_info: &vma::AllocationCreateInfo,
    ) -> VulkanResult<(vk::Buffer, vma::Allocation)> {
        let alloc_create_info = alloc_create_info.into_raw();
        let a_create_info_ptr = &alloc_create_info as *const raw::VmaAllocationCreateInfo;

        let mut buffer = vk::Buffer::null();
        let mut allocation: raw::VmaAllocation = ptr::null_mut();

        let result = raw::vmaCreateBuffer(
            self.inner.allocator,
            buffer_create_info,
            a_create_info_ptr,
            &mut buffer,
            &mut allocation,
            ptr::null_mut(),
        );

        VulkanResult::new(result, (buffer, vma::Allocation::from_raw(allocation)))
    }

    ///
    /// vmaDestroyBuffer
    ///
    #[inline]
    pub unsafe fn destroy_buffer(&self, buffer: vk::Buffer, alloc: vma::Allocation) {
        raw::vmaDestroyBuffer(
            self.inner.allocator,
            mem::transmute(buffer),
            alloc.into_raw(),
        );
    }

    ///
    /// vmaCreateImage
    ///
    #[inline]
    pub unsafe fn create_image(
        &self,
        image_create_info: &vk::ImageCreateInfo,
        alloc_create_info: &vma::AllocationCreateInfo,
    ) -> VulkanResult<(vk::Image, vma::Allocation)> {
        let alloc_create_info = alloc_create_info.into_raw();
        let a_create_info_ptr = &alloc_create_info as *const raw::VmaAllocationCreateInfo;

        let mut image = vk::Image::null();
        let mut allocation: raw::VmaAllocation = ptr::null_mut();

        let result = raw::vmaCreateImage(
            self.inner.allocator,
            image_create_info,
            a_create_info_ptr,
            &mut image,
            &mut allocation,
            ptr::null_mut(),
        );

        VulkanResult::new(result, (image, vma::Allocation::from_raw(allocation)))
    }

    ///
    /// vmaDestroyImage
    ///
    #[inline]
    pub unsafe fn destroy_image(&self, buffer: vk::Image, alloc: vma::Allocation) {
        raw::vmaDestroyImage(
            self.inner.allocator,
            mem::transmute(buffer),
            alloc.into_raw(),
        );
    }

    ///
    /// vmaSetCurrentFrameIndex
    ///
    #[inline]
    pub unsafe fn set_current_frame_index(self, index: u32) {
        raw::vmaSetCurrentFrameIndex(self.inner.allocator, index);
    }

    ///
    /// Get a reference to the device this allocator was created with
    ///
    #[inline]
    pub fn device(&self) -> &vk::Device {
        &self.inner.device
    }
}

struct Inner {
    allocator: raw::VmaAllocator,
    device: vk::Device,
}

//
// Implementing these is safe because vma internally synchronizes access
//
unsafe impl Send for Inner {}
unsafe impl Sync for Inner {}

impl Drop for Inner {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            raw::vmaDestroyAllocator(self.allocator);
        }
    }
}
