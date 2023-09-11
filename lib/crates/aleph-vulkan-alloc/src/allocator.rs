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
use crate::raw::{
    AllocationH, AllocationInfo, AllocatorCreateFlags, AllocatorCreateInfo, AllocatorH,
};
use crate::utils;
use crate::vma;
use crate::vulkan_functions::VulkanFunctionsBuilder;
use ash::prelude::VkResult;
use ash::vk;
use core::mem;
use core::ptr;
use std::ffi::c_void;
use std::marker::PhantomData;
use std::ops::Deref;
use std::ptr::NonNull;
use std::sync::Arc;
use thiserror::Error;

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
pub struct AllocatorBuilder<'a> {
    create_info: AllocatorCreateInfo,
    phantom: PhantomData<&'a ()>,
}

impl<'a> AllocatorBuilder<'a> {
    /// New instance of the builder
    ///
    /// User must give builder an instance, device and physical device otherwise it will fail to
    /// build
    pub const fn new() -> Self {
        let create_info = AllocatorCreateInfo {
            flags: AllocatorCreateFlags::empty(),
            physical_device: vk::PhysicalDevice::null(),
            device: vk::Device::null(),
            preferred_large_heap_block_size: 0,
            p_allocation_callbacks: ptr::null_mut(),
            p_device_memory_callbacks: ptr::null_mut(),
            p_heap_size_limit: ptr::null_mut(),
            p_vulkan_functions: ptr::null_mut(),
            vulkan_api_version: vk::make_api_version(1, 1, 0, 0),
            instance: vk::Instance::null(),
            p_type_external_memory_handle_types: ptr::null(),
        };

        AllocatorBuilder {
            create_info,
            phantom: PhantomData,
        }
    }

    /// Set the vulkan version being used
    pub fn vulkan_api_version(mut self, version: u32) -> Self {
        self.create_info.vulkan_api_version = version;
        self
    }

    /// Build the VmaAllocator
    ///
    /// ## Errors
    ///
    /// - If there is no valid physical device handle
    /// - If there is no valid device handle
    /// - If not all required function pointers are provided
    /// - If KHR_DEDICATED_ALLOCATION_BIT is set and the required function pointers are not given
    #[inline]
    pub fn build(
        mut self,
        entry_loader: &ash::Entry,
        instance_loader: &ash::Instance,
        device_loader: &ash::Device,
        physical_device: vk::PhysicalDevice,
    ) -> Result<Allocator, AllocatorBuilderError> {
        self.create_info.instance = instance_loader.handle();
        self.create_info.device = device_loader.handle();
        self.create_info.physical_device = physical_device;

        let mut functions =
            VulkanFunctionsBuilder::new().ash_vk_1_0(entry_loader, instance_loader, device_loader);
        if self.create_info.vulkan_api_version >= vk::API_VERSION_1_1 {
            functions = functions.ash_vk_1_1(instance_loader, device_loader);
        }
        if self.create_info.vulkan_api_version >= vk::API_VERSION_1_3 {
            functions = functions.ash_vk_1_3(instance_loader, device_loader);
        }
        let functions = functions.build();

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

        let mut raw_alloc: Option<AllocatorH> = None;
        unsafe {
            let result = raw::vmaCreateAllocator(
                NonNull::from(&self.create_info),
                NonNull::from(&mut raw_alloc),
            );
            if result != vk::Result::SUCCESS {
                panic!("Failed on call to vmaCreateAllocator with error: {result:?}");
            }
        }

        let alloc = Allocator {
            inner: Arc::new(Inner {
                allocator: raw_alloc.unwrap(),
            }),
        };

        Ok(alloc)
    }
}

impl<'a> Deref for AllocatorBuilder<'a> {
    type Target = AllocatorCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.create_info
    }
}

impl<'a> AsRef<AllocatorCreateInfo> for AllocatorBuilder<'a> {
    fn as_ref(&self) -> &AllocatorCreateInfo {
        &self.create_info
    }
}

impl<'a> Into<AllocatorCreateInfo> for AllocatorBuilder<'a> {
    fn into(self) -> AllocatorCreateInfo {
        self.create_info
    }
}

impl<'a> Default for AllocatorBuilder<'a> {
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
    /// Return a builder for the vma allocator
    pub const fn builder<'a>() -> AllocatorBuilder<'a> {
        AllocatorBuilder::new()
    }

    /// Get the raw VmaAllocator handle
    #[inline]
    pub fn as_raw(&self) -> AllocatorH {
        self.inner.allocator
    }

    /// vmaGetPhysicalDeviceProperties
    #[inline]
    pub unsafe fn get_physical_device_properties(&self) -> &vk::PhysicalDeviceProperties {
        let mut reference: Option<NonNull<vk::PhysicalDeviceProperties>> = None;

        raw::vmaGetPhysicalDeviceProperties(self.inner.allocator, NonNull::from(&mut reference));

        reference
            .expect("Given nullptr by vmaGetPhysicalDeviceProperties")
            .as_ref()
    }

    /// vmaGetPhysicalDeviceProperties
    #[inline]
    pub unsafe fn get_memory_properties(&self) -> &vk::PhysicalDeviceMemoryProperties {
        let mut reference: Option<NonNull<vk::PhysicalDeviceMemoryProperties>> = None;

        raw::vmaGetMemoryProperties(self.inner.allocator, NonNull::from(&mut reference));

        reference
            .expect("Given nullptr by vmaGetMemoryProperties")
            .as_ref()
    }

    // /// Returns a rusty String object that is much easier to pass around than a CStr reference. It
    // /// will also automatically de-allocate making it much safer, but will incur an extra allocation
    // /// and utf-8 conversion so be aware of the overhead.
    // #[inline]
    // pub unsafe fn get_stats_string(&self, detailed_map: bool) -> String {
    //     let c_str = self.build_stats_string(detailed_map);
    //
    //     let string = c_str.to_str().expect("Invalid utf-8 chars in stats string");
    //     let string = string.to_string();
    //
    //     self.free_stats_string(c_str);
    //
    //     string
    // }
    //
    // /// vmaBuildStatsString
    // #[inline]
    // pub unsafe fn build_stats_string(&self, detailed_map: bool) -> &CStr {
    //     let mut c_str_ptr: Option<NonNull<c_char>> = None;
    //
    //     raw::vmaBuildStatsString(
    //         self.inner.allocator,
    //         NonNull::from(&mut c_str_ptr),
    //         if detailed_map { 1 } else { 0 },
    //     );
    //
    //     if let Some(ptr) = c_str_ptr {
    //         CStr::from_ptr(ptr.as_ptr() as *const _)
    //     } else {
    //         unreachable!()
    //     }
    // }
    //
    // /// vmaFreeStatsString
    // #[inline]
    // pub unsafe fn free_stats_string(&self, str: &CStr) {
    //     let c_str_ptr = NonNull::new(str.as_ptr().cast_mut());
    //     raw::vmaFreeStatsString(self.inner.allocator, c_str_ptr);
    // }

    /// vmaFindMemoryTypeIndex
    #[inline]
    pub unsafe fn find_memory_type_index(
        &self,
        memory_type_bits: u32,
        allocation_create_info: &vma::AllocationCreateInfo,
    ) -> VkResult<u32> {
        let mut idx = 0u32;
        let result = raw::vmaFindMemoryTypeIndex(
            self.inner.allocator,
            memory_type_bits,
            NonNull::from(allocation_create_info),
            NonNull::from(&mut idx),
        );

        result.result_with_success(idx)
    }

    /// vmaFindMemoryTypeIndexForBufferInfo
    #[inline]
    pub unsafe fn find_memory_type_index_for_buffer_info(
        &self,
        buffer_create_info: &vk::BufferCreateInfo,
        allocation_create_info: &vma::AllocationCreateInfo,
    ) -> VkResult<u32> {
        let mut idx = 0u32;
        let result = raw::vmaFindMemoryTypeIndexForBufferInfo(
            self.inner.allocator,
            NonNull::from(buffer_create_info),
            NonNull::from(allocation_create_info),
            NonNull::from(&mut idx),
        );

        result.result_with_success(idx)
    }

    /// vmaFindMemoryTypeIndexForImageInfo
    #[inline]
    pub unsafe fn find_memory_type_index_for_image_info(
        &self,
        image_create_info: &vk::ImageCreateInfo,
        allocation_create_info: &vma::AllocationCreateInfo,
    ) -> VkResult<u32> {
        let mut idx = 0u32;
        let result = raw::vmaFindMemoryTypeIndexForImageInfo(
            self.inner.allocator,
            NonNull::from(image_create_info),
            NonNull::from(allocation_create_info),
            NonNull::from(&mut idx),
        );

        result.result_with_success(idx)
    }

    /// vmaAllocateMemory
    #[inline]
    pub unsafe fn allocate_memory(
        &self,
        memory_requirements: &vk::MemoryRequirements,
        create_info: &vma::AllocationCreateInfo,
    ) -> VkResult<(vma::Allocation, AllocationInfo)> {
        let mut allocation: Option<AllocationH> = None;
        let mut allocation_info = AllocationInfo::default();
        let result = raw::vmaAllocateMemory(
            self.inner.allocator,
            NonNull::from(memory_requirements),
            NonNull::from(create_info),
            NonNull::from(&mut allocation),
            Some(NonNull::from(&mut allocation_info)),
        );

        result.result_with_success((
            vma::Allocation {
                allocation: allocation.unwrap_unchecked(),
            },
            allocation_info,
        ))
    }

    /// vmaAllocateMemoryPages
    #[inline]
    pub unsafe fn allocate_memory_pages(
        &self,
        memory_requirements: &vk::MemoryRequirements,
        create_info: &vma::AllocationCreateInfo,
        allocations: &mut [Option<vma::Allocation>],
        allocation_infos: &mut [AllocationInfo],
    ) -> VkResult<()> {
        if allocations.len() > 0 {
            let get_infos = if allocation_infos.len() > 0 {
                assert_eq!(allocations.len(), allocation_infos.len());
                true
            } else {
                false
            };

            let ptr = NonNull::new(allocations.as_ptr().cast_mut()).unwrap_unchecked();
            let ptr = ptr.cast::<Option<AllocationH>>();
            let infos_ptr = NonNull::new(allocation_infos.as_ptr().cast_mut()).unwrap_unchecked();
            let infos_ptr = infos_ptr.cast::<AllocationInfo>();

            let result = raw::vmaAllocateMemoryPages(
                self.inner.allocator,
                Some(NonNull::from(memory_requirements)),
                Some(NonNull::from(create_info)),
                allocations.len() as _,
                ptr,
                if get_infos { None } else { Some(infos_ptr) },
            );

            // Enforce that everything has been written and is non-null
            if cfg!(debug_assertions) {
                for v in allocations.iter() {
                    debug_assert!(v.is_some())
                }
            }

            result.result()
        } else {
            Ok(())
        }
    }

    /// vmaAllocateMemoryPages
    #[inline]
    pub unsafe fn allocate_memory_pages_dyn(
        &self,
        memory_requirements: &vk::MemoryRequirements,
        create_info: &vma::AllocationCreateInfo,
        allocation_count: usize,
    ) -> VkResult<(Vec<vma::Allocation>, Vec<AllocationInfo>)> {
        let mut allocations: Vec<_> = Vec::with_capacity(allocation_count);
        allocations.resize(allocation_count, None);
        let mut allocation_infos: Vec<_> = Vec::with_capacity(allocation_count);
        allocation_infos.resize(allocation_count, AllocationInfo::default());

        self.allocate_memory_pages(
            memory_requirements,
            create_info,
            &mut allocations,
            &mut allocation_infos,
        ).map(|_| {
            (mem::transmute(allocations), allocation_infos)
        })
    }

    /// vmaAllocateMemoryForBuffer
    #[inline]
    pub unsafe fn allocate_memory_for_buffer(
        &self,
        buffer: vk::Buffer,
        create_info: &vma::AllocationCreateInfo,
    ) -> VkResult<(vma::Allocation, AllocationInfo)> {
        let mut allocation: Option<AllocationH> = None;
        let mut allocation_info = AllocationInfo::default();

        let result = raw::vmaAllocateMemoryForBuffer(
            self.inner.allocator,
            buffer,
            NonNull::from(create_info),
            NonNull::from(&mut allocation),
            Some(NonNull::from(&mut allocation_info)),
        );

        result.result_with_success((
            vma::Allocation {
                allocation: allocation.unwrap_unchecked(),
            },
            allocation_info,
        ))
    }

    /// vmaAllocateMemoryforImage
    #[inline]
    pub unsafe fn allocate_memory_for_image(
        &self,
        image: vk::Image,
        create_info: &vma::AllocationCreateInfo,
    ) -> VkResult<(vma::Allocation, AllocationInfo)> {
        let mut allocation: Option<AllocationH> = None;
        let mut allocation_info = AllocationInfo::default();

        let result = raw::vmaAllocateMemoryForImage(
            self.inner.allocator,
            image,
            NonNull::from(create_info),
            NonNull::from(&mut allocation),
            Some(NonNull::from(&mut allocation_info)),
        );

        result.result_with_success((
            vma::Allocation {
                allocation: allocation.unwrap_unchecked(),
            },
            allocation_info,
        ))
    }

    /// vmaFreeMemory
    #[inline]
    pub unsafe fn free_memory(&self, allocation: vma::Allocation) {
        raw::vmaFreeMemory(self.inner.allocator, Some(allocation.allocation))
    }

    /// vmaFreeMemoryPages
    #[inline]
    pub unsafe fn free_memory_pages(&self, allocations: &[vma::Allocation]) {
        if !allocations.is_empty() {
            // Safety: Must be non-null for len > 1
            let ptr = NonNull::new(allocations.as_ptr().cast_mut()).unwrap_unchecked();
            let ptr = ptr.cast::<AllocationH>();
            raw::vmaFreeMemoryPages(self.inner.allocator, allocations.len() as _, Some(ptr))
        }
    }

    /// vmaGetAllocationInfo
    #[inline]
    pub unsafe fn get_allocation_info(&self, allocation: vma::Allocation) -> AllocationInfo {
        let mut info = AllocationInfo::default();

        raw::vmaGetAllocationInfo(
            self.inner.allocator,
            allocation.allocation,
            NonNull::from(&mut info),
        );

        info
    }

    // TODO : vmaSetAllocationUserData (PROBABLY WONT EXPOSE THIS, NOT VERY RUSTY)

    /// vmaMapMemory
    #[inline]
    pub unsafe fn map_memory(
        &self,
        allocation: vma::Allocation,
    ) -> VkResult<Option<NonNull<c_void>>> {
        let mut pointer: Option<NonNull<c_void>> = None;

        let result = raw::vmaMapMemory(
            self.inner.allocator,
            allocation.allocation,
            NonNull::from(&mut pointer),
        );

        result.result_with_success(pointer)
    }

    /// vmaUnmapMemory
    #[inline]
    pub unsafe fn unmap_memory(&self, allocation: vma::Allocation) {
        raw::vmaUnmapMemory(self.inner.allocator, allocation.allocation)
    }

    /// vmaFlushAllocation
    #[inline]
    pub unsafe fn flush_allocation(
        &self,
        allocation: vma::Allocation,
        offset: vk::DeviceSize,
        size: vk::DeviceSize,
    ) -> VkResult<()> {
        raw::vmaFlushAllocation(self.inner.allocator, allocation.allocation, offset, size).result()
    }
    /// vmaInvalidateAllocation
    #[inline]
    pub unsafe fn invalidate_allocation(
        &self,
        allocation: vma::Allocation,
        offset: vk::DeviceSize,
        size: vk::DeviceSize,
    ) -> VkResult<()> {
        raw::vmaInvalidateAllocation(self.inner.allocator, allocation.allocation, offset, size)
            .result()
    }

    /// vmaCheckCorruption
    #[inline]
    pub unsafe fn check_corruption(&self, memory_type_bits: u32) -> VkResult<()> {
        raw::vmaCheckCorruption(self.inner.allocator, memory_type_bits).result()
    }

    // TODO : vmaDefragmentationBegin
    // TODO : vmaDefragmentationEnd
    // TODO : vmaDefragment (Deprecated)

    /// vmaBindBufferMemory
    #[inline]
    pub unsafe fn bind_buffer_memory(
        &self,
        allocation: vma::Allocation,
        buffer: vk::Buffer,
    ) -> VkResult<()> {
        raw::vmaBindBufferMemory(self.inner.allocator, allocation.allocation, buffer).result()
    }

    /// vmaBindImageMemory
    #[inline]
    pub unsafe fn bind_image_memory(
        &self,
        allocation: vma::Allocation,
        image: vk::Image,
    ) -> VkResult<()> {
        raw::vmaBindImageMemory(
            self.inner.allocator,
            allocation.allocation,
            mem::transmute(image),
        )
        .result()
    }

    /// vmaCreateBuffer
    #[inline]
    pub unsafe fn create_buffer(
        &self,
        buffer_create_info: &vk::BufferCreateInfo,
        alloc_create_info: &vma::AllocationCreateInfo,
    ) -> VkResult<(vk::Buffer, vma::Allocation, AllocationInfo)> {
        let mut buffer = vk::Buffer::null();
        let mut allocation: Option<AllocationH> = None;
        let mut allocation_info = AllocationInfo::default();

        let result = raw::vmaCreateBuffer(
            self.inner.allocator,
            NonNull::from(buffer_create_info),
            NonNull::from(alloc_create_info),
            NonNull::from(&mut buffer),
            NonNull::from(&mut allocation),
            Some(NonNull::from(&mut allocation_info)),
        );

        result.result_with_success((
            buffer,
            vma::Allocation {
                allocation: allocation.unwrap_unchecked(),
            },
            allocation_info,
        ))
    }

    /// vmaDestroyBuffer
    #[inline]
    pub unsafe fn destroy_buffer(&self, buffer: vk::Buffer, alloc: vma::Allocation) {
        raw::vmaDestroyBuffer(
            self.inner.allocator,
            mem::transmute(buffer),
            Some(alloc.allocation),
        )
    }

    /// vmaCreateImage
    #[inline]
    pub unsafe fn create_image(
        &self,
        image_create_info: &vk::ImageCreateInfo,
        alloc_create_info: &vma::AllocationCreateInfo,
    ) -> VkResult<(vk::Image, vma::Allocation, AllocationInfo)> {
        let mut image = vk::Image::null();
        let mut allocation: Option<AllocationH> = None;
        let mut allocation_info = AllocationInfo::default();

        let result = raw::vmaCreateImage(
            self.inner.allocator,
            NonNull::from(image_create_info),
            NonNull::from(alloc_create_info),
            NonNull::from(&mut image),
            NonNull::from(&mut allocation),
            Some(NonNull::from(&mut allocation_info)),
        );

        result.result_with_success((
            image,
            vma::Allocation {
                allocation: allocation.unwrap_unchecked(),
            },
            allocation_info,
        ))
    }

    /// vmaDestroyImage
    #[inline]
    pub unsafe fn destroy_image(&self, buffer: vk::Image, alloc: vma::Allocation) {
        raw::vmaDestroyImage(
            self.inner.allocator,
            mem::transmute(buffer),
            Some(alloc.allocation),
        )
    }

    /// vmaSetCurrentFrameIndex
    #[inline]
    pub unsafe fn set_current_frame_index(self, index: u32) {
        raw::vmaSetCurrentFrameIndex(self.inner.allocator, index)
    }
}

#[repr(transparent)]
struct Inner {
    allocator: AllocatorH,
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
            raw::vmaDestroyAllocator(Some(self.allocator));
        }
    }
}
